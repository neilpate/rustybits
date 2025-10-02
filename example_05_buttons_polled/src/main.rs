#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::gpio;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    // Configure LED (top-left LED in 5x5 matrix)
    let mut row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(gpio::Level::Low);

    // Configure button A with pull-up resistor
    // Note: `mut` is required even for input pins because GPIO read operations
    // like `is_low()` require mutable access to the underlying hardware registers
    let mut button_a = board.buttons.button_a.into_floating_input();

    // Track LED state and button state for edge detection
    let mut led_on = false;
    let mut button_was_pressed = false;

    loop {
        // Read button state (button is active low - pressed = false)
        let button_pressed = button_a.is_low().unwrap();

        // Detect button press (transition from not pressed to pressed)
        if button_pressed && !button_was_pressed {
            // Toggle LED state
            led_on = !led_on;

            if led_on {
                row1.set_low().unwrap(); // Turn LED on (row low, col already low)
            } else {
                row1.set_high().unwrap(); // Turn LED off (row high)
            }
        }

        // Update button state for next iteration
        button_was_pressed = button_pressed;
    }
}
