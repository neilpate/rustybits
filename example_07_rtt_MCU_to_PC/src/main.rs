#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::hal::timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer0 = timer::Timer::new(board.TIMER0);

    let mut loop_count: u32 = 0;

    rprintln!("RTT Example Started!");

    loop {
        timer0.delay_ms(1000);
        rprintln!("Count: {}", loop_count); // Note we do not explicitly write the timestamp
        loop_count += 1;
    }
}
