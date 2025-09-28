#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

// GPIO registers for nRF52833
const GPIO_P0_OUTSET: *mut u32 = 0x5000_0508 as *mut u32;
const GPIO_P0_OUTCLR: *mut u32 = 0x5000_050C as *mut u32;
const GPIO_P0_PIN_CNF: *mut u32 = 0x5000_0700 as *mut u32;

// micro:bit LED matrix pins
const ROW1_PIN: u32 = 21; // P0.21
const COL1_PIN: u32 = 28; // P0.28

#[entry]
fn main() -> ! {
    unsafe {
        // Configure P0.21 (Row 1) as output
        let pin_cnf_21 = GPIO_P0_PIN_CNF.add(ROW1_PIN as usize);
        core::ptr::write_volatile(pin_cnf_21, 1); // DIR=1 (output)

        // Configure P0.28 (Col 1) as output and set low (column active)
        let pin_cnf_28 = GPIO_P0_PIN_CNF.add(COL1_PIN as usize);
        core::ptr::write_volatile(pin_cnf_28, 1); // DIR=1 (output)
        core::ptr::write_volatile(GPIO_P0_OUTCLR, 1 << COL1_PIN); // Set column active
    }

    loop {
        unsafe {
            // Turn LED on (set row low)
            core::ptr::write_volatile(GPIO_P0_OUTCLR, 1 << ROW1_PIN);
        }

        // Delay ~1s (on time)
        for _ in 0..400_00 {
            unsafe {
                core::arch::asm!("nop");
            }
        }

        unsafe {
            // Turn LED off (set row high)
            core::ptr::write_volatile(GPIO_P0_OUTSET, 1 << ROW1_PIN);
        }

        // Delay ~1s (off time)
        for _ in 0..800_00 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}
