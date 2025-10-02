#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::hal::{
    gpio::Level,
    gpiote::Gpiote,
    pac::{self, interrupt},
};
use panic_halt as _;

// Simple atomic boolean for LED state
// This is needed as the ISR and main loop run in different contexts
static LED_STATE: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    // Configure LED pins (top-left LED in 5x5 matrix)
    let row1 = board.display_pins.row1.into_push_pull_output(Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(Level::Low);

    // Configure button A as input with pull-up resistor and degrade to generic Pin
    let button_a = board.buttons.button_a.into_pullup_input().degrade();

    // Initialize GPIOTE (GPIO Tasks and Events) for interrupt handling
    let gpiote = Gpiote::new(board.GPIOTE);

    // Configure button A for interrupt on falling edge (button press)
    // Channel 0 will trigger on HiToLo transition (button press)
    gpiote.channel0().input_pin(&button_a).hi_to_lo().enable_interrupt();

    // Enable GPIOTE interrupt in NVIC (Nested Vector Interrupt Controller)
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIOTE);
    }

    // Keep the LED pin for main loop use
    let mut led = row1;

    // Main loop - check for state changes and update LED
    // Interrupt only sets the flag, main loop does the LED control
    let mut last_state = false;

    loop {
        // Check if LED state changed (set by interrupt)
        // Ordering::Relaxed = fastest atomic read, only guarantees the read itself is atomic
        let current_state = LED_STATE.load(Ordering::Relaxed);

        if current_state != last_state {
            // Update LED based on new state
            if current_state {
                led.set_low().ok(); // Turn LED on (row low, col already low)
            } else {
                led.set_high().ok(); // Turn LED off (row high)
            }
            last_state = current_state;
        }

        // Optional: CPU can sleep briefly between checks
        cortex_m::asm::wfi(); // Wait For Interrupt
    }
}

// GPIOTE interrupt handler
// This function only runs when button A is pressed (our only configured source)
#[interrupt]
fn GPIOTE() {
    // Access GPIOTE directly through PAC (no sharing needed)
    let gpiote = unsafe { &*pac::GPIOTE::ptr() };

    // Clear the event flag (we know it's channel 0/button A)
    gpiote.events_in[0].write(|w| unsafe { w.bits(0) });

    // Simply toggle the atomic boolean - main loop handles LED
    // Ordering::Relaxed = fastest atomic operations, perfect for simple flags
    let current_state = LED_STATE.load(Ordering::Relaxed);
    LED_STATE.store(!current_state, Ordering::Relaxed);
}
