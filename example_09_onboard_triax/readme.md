# Example 09 - Onboard Accelerometer/Magnetometer

Read acceleration data from the micro:bit v2's onboard LSM303AGR sensor using I²C communication!

## What it does

This program reads data from the LSM303AGR accelerometer/magnetometer sensor that's built into the micro:bit v2. It continuously outputs acceleration values in milligravities (mg) for all three axes via RTT (Real-Time Transfer) debugging.

## Running this example

```bash
cd example_09_onboard_triax
cargo embed
```

The RTT output will show:
```
Accelerometer ID: AccelerometerId { raw: 51 } (expected: 51)
Accelerometer: x -16 y 32 z 1008
Accelerometer: x -48 y 0 z 992
...
```
## The Code

```rust
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

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    let id = sensor.accelerometer_id().unwrap();
    rprintln!("Accelerometer ID: {:?} (expected: 51)", id);

    sensor
        .set_accel_mode_and_odr(&mut timer0, AccelMode::HighResolution, AccelOutputDataRate::Hz50)
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
        rprintln!("Accelerometer: x {} y {} z {}", x, y, z);
        timer0.delay_ms(1000);
    }
}
```

## How it works

1. **Initialize RTT**: Set up Real-Time Transfer for debugging output
2. **Create I²C interface**: Configure the TWIM (Two-Wire Interface Master) peripheral at 100kHz
3. **Initialize sensor**: Create an LSM303AGR driver instance with the I²C interface
4. **Read sensor ID**: Verify communication by reading the WHO_AM_I register (should be 51/0x33)
5. **Configure accelerometer**: Set to high-resolution mode at 50Hz sampling rate
6. **Enable magnetometer**: Switch to continuous magnetometer mode (accelerometer still works)
7. **Read loop**: Continuously read x, y, z acceleration values and print them

## Understanding the Output

The acceleration values are in milligravities (mg):
- **1000 mg = 1g** (Earth's gravity)
- When the micro:bit is flat on a table, you'll see approximately:
  - `z ≈ 1000 mg` (gravity pointing down)
  - `x ≈ 0 mg`, `y ≈ 0 mg`
- Tilt or move the micro:bit to see the values change!

## Key Concepts

### I²C Communication
- **I²C (Inter-Integrated Circuit)**: A two-wire serial protocol for chip-to-chip communication
- **TWIM**: Nordic's I²C master peripheral on the nRF52833
- **Internal bus**: The micro:bit v2 has an internal I²C bus connecting the nRF52 to the LSM303AGR
- **Pins**: SCL (clock) on P0.08, SDA (data) on P0.16

### LSM303AGR Sensor
- **Combined sensor**: Both accelerometer (motion) and magnetometer (compass) in one chip
- **I²C addresses**: 0x19 for accelerometer, 0x1E for magnetometer
- **High-level driver**: The `lsm303agr` crate handles all the register-level details

## Additional Resources

- **[LSM303AGR Datasheet](../doc/lsm303agr.pdf)** - Complete sensor documentation
- **[I²C Protocol Specification](https://www.nxp.com/docs/en/user-guide/UM10204.pdf)** - I²C bus specification
- **[Technical Details](technical_details.md)** - Deep dive from Rust code to electrical signals