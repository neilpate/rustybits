# Example 09 - Onboard Accelerometer

Read acceleration data from the micro:bit v2's onboard LSM303AGR accelerometer using I²C communication!

## What it does

This program reads acceleration data from the LSM303AGR accelerometer that's built into the micro:bit v2. It continuously outputs acceleration values in milligravities (mg) for all three axes via RTT (Real-Time Transfer) debugging.

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
```

## How it works

1. **Initialize RTT**: Set up Real-Time Transfer for debugging output
2. **Get board peripherals**: Claim exclusive access to hardware
3. **Create timer**: For delays between readings
4. **Create I²C interface**: Configure the TWIM peripheral at 100kHz with the internal I²C pins
5. **Initialize sensor**: Create an LSM303AGR driver instance with the I²C interface
6. **Read sensor ID**: Verify communication by reading the WHO_AM_I register (should be 51/0x33)
7. **Configure accelerometer**: Set to high-resolution mode at 50Hz sampling rate
8. **Read loop**: Continuously read x, y, z acceleration values and print them every 250 ms

## Understanding the Output

The acceleration values are in milligravities (mg):
- **1000 mg = 1g** (Earth's gravity)
- When the micro:bit is flat on a table, you'll see approximately:
  - `z ≈ 1000 mg` (gravity pointing down)
  - `x ≈ 0 mg`, `y ≈ 0 mg`
- Tilt or move the micro:bit to see the values change!

## Key Concepts

### I²C Communication
- **TWIM**: Nordic's name for their I²C master peripheral with DMA support (Two-Wire Interface Master)
- **TWI vs TWIM**: The nRF52833 has both TWI (non-DMA) and TWIM (DMA-enabled). This example uses TWIM for efficiency
- **Internal bus**: The micro:bit v2 has an internal I²C bus connecting the nRF52 to the LSM303AGR
- **Pins**: SCL (clock) on P0.08, SDA (data) on P0.16

### LSM303AGR Sensor
- **Combined sensor**: Contains both accelerometer (motion) and magnetometer (compass) in one chip
- **This example**: Uses only the accelerometer functionality
- **I²C addresses**: 0x19 for accelerometer, 0x1E for magnetometer
- **High-level driver**: The `lsm303agr` crate handles all the register-level details

## Understanding the Abstraction Layers

Reading the accelerometer ID with `sensor.accelerometer_id()` looks simple, but it's quite complicated under the hood! The call goes through multiple layers of abstraction: the lsm303agr driver → embedded-hal I2c trait → nrf52833-hal TWIM implementation → hardware registers → I²C bus signals. Each layer provides type safety and hardware independence while compiling down to efficient code. See `accelerometer_id_trace.md` for a complete step-by-step trace through all these layers.

## Additional Resources

- **[accelerometer_id_trace.md](accelerometer_id_trace.md)** - Step-by-step trace of how `accelerometer_id()` works through all the layers
- **[LSM303AGR Datasheet](../doc/lsm303agr.pdf)** - Complete sensor documentation
- **[I²C Protocol Specification](https://www.nxp.com/docs/en/user-guide/UM10204.pdf)** - I²C bus specification
- **[Technical Details](technical_details.md)** - Deep dive from Rust code to electrical signals