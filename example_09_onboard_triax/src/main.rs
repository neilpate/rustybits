#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut timer0 = Timer::new(board.TIMER0);

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    let id = sensor.accelerometer_id().unwrap();
    rprintln!("Accelerometer ID: {:?} (expected: 51)", id);

    sensor
        .set_accel_mode_and_odr(&mut timer0, AccelMode::HighResolution, AccelOutputDataRate::Hz50)
        .unwrap();

    loop {
        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
        rprintln!("Accelerometer: x {} y {} z {}", x, y, z);
        timer0.delay_ms(250);
    }
}
