#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::timer;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut row1 = board.display_pins.row1;

    let mut col1 = board.display_pins.col1;
    col1.set_low().unwrap(); // Activate column (active low)

    let mut timer0 = timer::Timer::new(board.TIMER0);

    loop {
        timer0.delay_ms(300);
        row1.set_high().unwrap();
        timer0.delay_ms(100);
        row1.set_low().unwrap();
    }
}
