# Example 09 - Technical Details

This document explains the I²C communication, sensor initialization, and the trait magic behind reading the LSM303AGR accelerometer.

## Project Structure

This example contains everything needed to build and run independently:

```
example_09_onboard_triax/
├── src/
│   └── main.rs          # Your Rust code
├── Cargo.toml           # Dependencies and project metadata
├── Embed.toml           # probe-rs flashing configuration
└── readme.md            # This file
```

## I²C Communication on the micro:bit v2

The micro:bit v2 has two I²C buses:

### Internal I²C Bus
- **Purpose**: Connects the nRF52833 to onboard sensors
- **Devices**: LSM303AGR (accelerometer/magnetometer)
- **Pins**: 
  - SCL (clock): P0.08
  - SDA (data): P0.16
- **Access**: `board.i2c_internal`

### External I²C Bus  
- **Purpose**: Connect external I²C devices via the edge connector
- **Pins**: 
  - SCL (clock): P0.26 (edge connector pin 19)
  - SDA (data): P0.25 (edge connector pin 20)
- **Access**: `board.i2c_external`

This example uses the internal bus to communicate with the LSM303AGR sensor.

## The LSM303AGR Sensor

The LSM303AGR is actually two sensors in one chip:
- **Accelerometer**: Measures acceleration/tilt (address 0x19)
- **Magnetometer**: Measures magnetic field/compass (address 0x1E)

### Communication Protocol
Each sensor responds to its own I²C address and has separate registers for configuration and data.

## How TWIM (I²C) Works

The nRF52833's TWIM (Two-Wire Interface Master) peripheral handles I²C communication:

```rust
let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
```

This configures:
- **TWIM0**: The hardware peripheral (nRF52 has two: TWIM0 and TWIM1)
- **Pins**: P0.08 (SCL) and P0.16 (SDA) from `board.i2c_internal`
- **Frequency**: 100kHz (standard I²C speed)

### I²C Transaction Types

**Write transaction** (send data to sensor):
```
START -> [ADDR + WRITE] -> [DATA] -> STOP
```

**Read transaction** (read data from sensor):
```
START -> [ADDR + WRITE] -> [REG] -> RESTART -> [ADDR + READ] -> [DATA] -> STOP
```

The `write_read()` method combines both: write a register address, then read the response.

## Tracing `sensor.accelerometer_id()` Call

Let's follow the complete path from your call to the actual I²C communication - this demonstrates the power of Rust's trait system and zero-cost abstractions.

### 1. Your Code
```rust
let id = sensor.accelerometer_id().unwrap();
```

### 2. Method Call (`lsm303agr` crate - `device_impl.rs:217`)
```rust
pub fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE>> {
    self.iface.read_accel_register::<WhoAmIA>()
}
```
**What happens:** Calls `read_accel_register` with the type parameter `WhoAmIA`.

### 3. Register Type Definition (`register_address.rs:80`)
```rust
register! {
  /// WHO_AM_I_A
  pub type WhoAmIA: 0x0F = AccelerometerId;
}

impl WhoAmIA {
    pub(crate) const ID: u8 = 0b00110011;  // This is 51 decimal, 0x33 hex
}
```
**What happens:** The `register!` macro expands to create:
- An empty enum `WhoAmIA` (just a type marker)
- Implementation of `RegRead` trait for `WhoAmIA`

The macro expansion creates:
```rust
impl RegRead for WhoAmIA {
    type Output = AccelerometerId;
    const ADDR: u8 = 0x0F;
    
    fn from_data(data: u8) -> Self::Output {
        AccelerometerId::from_bits_truncate(data)
    }
}
```

### 4. Read Accelerometer Register (`interface.rs:139`)
```rust
fn read_accel_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {
    self.read_register::<R>(ACCEL_ADDR)
}
```
**What happens:** 
- `R` is `WhoAmIA`
- `ACCEL_ADDR` is the I²C address of the accelerometer (0x19)
- Returns `R::Output` which is `AccelerometerId`

### 5. Generic Read Register (`interface.rs:174`)
```rust
fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error<E>> {
    let mut data = [0];
    self.i2c
        .write_read(address, &[R::ADDR], &mut data)
        .map_err(Error::Comm)?;

    Ok(R::from_data(data[0]))
}
```
**What happens:**
1. Creates a buffer `data` for receiving one byte
2. Calls I²C `write_read`:
   - **address**: `0x19` (accelerometer I²C address)
   - **write data**: `[0x0F]` (the WHO_AM_I register address from `WhoAmIA::ADDR`)
   - **read buffer**: `&mut data` (reads 1 byte into this)
3. Calls `R::from_data(data[0])` which is `WhoAmIA::from_data(data[0])`

#### Deep Dive: `write_read()` Implementation

The `write_read()` method is part of the `embedded_hal::i2c::I2c` trait. For the nRF52833, it's implemented in the HAL crate and performs these steps:

**High-level flow:**
```rust
// Simplified view of what write_read does:
fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Error> {
    // 1. Set I²C slave address
    // 2. Setup TX buffer (what to write)
    // 3. Setup RX buffer (where to read into)
    // 4. Start transmission
    // 5. Wait for completion
    // 6. Check for errors
}
```

**Detailed TWIM peripheral operations:**

1. **Configure slave address:**
   ```rust
   // Write to TWIM ADDRESS register
   twim.address.write(|w| w.address().bits(address));  // 0x19
   ```

2. **Setup TX buffer (write phase):**
   ```rust
   // Point TWIM to our write data
   twim.txd.ptr.write(|w| unsafe { w.ptr().bits(write.as_ptr() as u32) });
   twim.txd.maxcnt.write(|w| w.maxcnt().bits(write.len() as u8));  // 1 byte: [0x0F]
   ```

3. **Setup RX buffer (read phase):**
   ```rust
   // Point TWIM to our read buffer
   twim.rxd.ptr.write(|w| unsafe { w.ptr().bits(read.as_mut_ptr() as u32) });
   twim.rxd.maxcnt.write(|w| w.maxcnt().bits(read.len() as u8));  // 1 byte
   ```

4. **Configure for write-then-read (no stop between):**
   ```rust
   // Enable shortcut: LASTTX -> STARTRX (automatically start read after write)
   twim.shorts.write(|w| w.lasttx_startrx().enabled());
   ```
   This tells the TWIM: "After sending the last TX byte, automatically start receiving without sending a STOP condition." This creates the RESTART condition needed for register reads.

5. **Start the transaction:**
   ```rust
   // Trigger STARTTX task - begins the I²C transaction
   twim.tasks_starttx.write(|w| w.tasks_starttx().set_bit());
   ```

6. **Wait for completion:**
   ```rust
   // Poll or wait on interrupt for the STOPPED event
   while twim.events_stopped.read().events_stopped().bit_is_clear() {
       // Check for errors (NACK, bus error, etc.)
       if twim.events_error.read().events_error().bit_is_set() {
           return Err(Error::Nack);
       }
   }
   ```

7. **Clean up:**
   ```rust
   // Clear the STOPPED event for next transaction
   twim.events_stopped.write(|w| w.events_stopped().clear_bit());
   ```

**What's happening on the I²C bus during all this:**

```
Time →

1. START condition (SDA falls while SCL high)
2. Send 7-bit address + WRITE bit: 0x19 << 1 | 0 = 0b00110010
3. Sensor ACKs (pulls SDA low)
4. Send register address: 0x0F
5. Sensor ACKs
6. REPEATED START (no STOP - this is the shortcut magic!)
7. Send 7-bit address + READ bit: 0x19 << 1 | 1 = 0b00110011
8. Sensor ACKs
9. Sensor sends data byte: 0x33
10. Master NACKs (signals "last byte")
11. STOP condition (SDA rises while SCL high)
```

**Why the shortcut matters:**

Without the LASTTX → STARTRX shortcut, you'd need two separate I²C transactions:
```
Transaction 1: START -> [0x19+W] -> [0x0F] -> STOP
Transaction 2: START -> [0x19+R] -> [data] -> STOP
```

The shortcut combines them into one transaction with a RESTART:
```
START -> [0x19+W] -> [0x0F] -> RESTART -> [0x19+R] -> [data] -> STOP
```

This is important because many I²C devices (including the LSM303AGR) expect register reads to be atomic operations without a STOP condition in between.

### 6. I²C Transaction (HAL Level)
The `write_read` is from the `embedded-hal` I²C trait and ultimately calls the nRF52 TWIM peripheral:
```
I²C Bus Transaction:
  START -> [0x19 + WRITE] -> [0x0F] -> RESTART -> [0x19 + READ] -> [data byte] -> STOP
```
The sensor responds with `0x33` (51 decimal).

### 7. Convert Raw Data to Type (`types.rs:46`)
```rust
impl AccelerometerId {
    pub(crate) fn from_bits_truncate(raw: u8) -> Self {
        Self { raw }
    }

    pub const fn raw(&self) -> u8 {
        self.raw
    }

    pub const fn is_correct(&self) -> bool {
        self.raw == WhoAmIA::ID  // Checks if raw == 0x33
    }
}
```
**What happens:** The raw `u8` value (0x33) is wrapped in the `AccelerometerId` struct.

### 8. Result
Your `id` variable contains:
```rust
AccelerometerId { raw: 51 }  // 0x33 in hex
```

## Key Trait Magic

### The `RegRead` Trait
```rust
pub trait RegRead<D = u8> {
    type Output;           // What type to return (AccelerometerId)
    const ADDR: u8;        // Which register address (0x0F)
    fn from_data(data: D) -> Self::Output;  // How to convert raw byte
}
```

This trait allows the library to be generic over different register types while maintaining type safety.

### Why All The Traits?

1. **Type Safety**: Each register has its own type, so you can't accidentally read the wrong register
2. **Code Reuse**: One `read_register` function works for all register types
3. **Compile-Time Guarantees**: The register address and output type are known at compile time
4. **Zero Cost Abstraction**: All this compiles down to efficient code with no runtime overhead

### The Magic Flow
```
WhoAmIA (empty enum, type marker)
  ↓
implements RegRead trait
  ↓  
  ADDR = 0x0F (constant)
  Output = AccelerometerId (associated type)
  from_data = AccelerometerId::from_bits_truncate (method)
  ↓
read_register::<WhoAmIA>()
  ↓
uses WhoAmIA::ADDR to know which register
uses WhoAmIA::from_data to convert result
returns WhoAmIA::Output (AccelerometerId)
```

### Why Not Just Return u8?

The library wraps the `u8` in `AccelerometerId` to provide:
1. **Type safety** - can't confuse it with other IDs
2. **Helper methods** like `is_correct()` to validate the ID
3. **Semantic meaning** - it's not just any byte, it's specifically an accelerometer ID
4. **Debug formatting** with `{:?}` through the `Debug` trait

To get the raw value, you'd call:
```rust
let raw_value: u8 = id.raw();
```

## Sensor Initialization Sequence

The LSM303AGR requires a specific initialization sequence:

```rust
let mut sensor = Lsm303agr::new_with_i2c(i2c);  // Create driver instance

// Configure accelerometer
sensor.set_accel_mode_and_odr(&mut timer0, AccelMode::HighResolution, AccelOutputDataRate::Hz50).unwrap();

// Enable magnetometer (accelerometer continues working)
let mut sensor = sensor.into_mag_continuous().ok().unwrap();
```

### What Each Step Does

1. **`new_with_i2c(i2c)`**: 
   - Creates the sensor driver
   - Takes ownership of the I²C peripheral
   - Doesn't perform any I²C transactions yet

2. **`set_accel_mode_and_odr()`**:
   - Writes configuration registers
   - Sets power mode (normal, high-resolution, low-power)
   - Sets output data rate (1Hz to 400Hz)
   - Uses timer for required delays between register writes

3. **`into_mag_continuous()`**:
   - Consumes the sensor (takes ownership)
   - Configures magnetometer for continuous measurement
   - Returns a new sensor instance that can read both sensors
   - This pattern uses Rust's type system to enforce correct initialization order

### Type States Pattern

The `lsm303agr` crate uses the **type state pattern** to enforce correct usage at compile time:

- `Lsm303agr<I2C, MagModeUnknown>` - Initial state, accelerometer only
- `Lsm303agr<I2C, MagContinuous>` - Magnetometer in continuous mode

You can't call magnetometer methods until you've called `into_mag_continuous()`. The compiler enforces this!

## Reading Acceleration Data

```rust
let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
```

Behind the scenes:
1. Reads 6 bytes from consecutive registers (2 bytes per axis)
2. Converts 16-bit two's complement values to signed integers
3. Scales values to milligravities based on the configured range
4. Returns a tuple with the three axis values

## Assembly Code Generation

The high-level Rust code compiles to efficient ARM assembly. Here's a simplified view of an I²C register read:

```rust
self.i2c.write_read(0x19, &[0x0F], &mut data)
```

Compiles roughly to:
```assembly
; Configure TWIM peripheral
mov r0, #0x40003000      ; TWIM0 base address
mov r1, #0x19            ; I²C slave address
str r1, [r0, #0x588]     ; Write to ADDRESS register

mov r1, #0x0F            ; Register address to read
str r1, [r0, #0x534]     ; Write to TXD.PTR

mov r1, #1               ; Send 1 byte
str r1, [r0, #0x538]     ; Write to TXD.MAXCNT

; ... setup RXD buffer and maxcnt ...

mov r1, #1               ; Start transmission
str r1, [r0, #0x000]     ; Write to TASKS_STARTTX

; ... wait for completion via interrupts or polling ...
```

The HAL abstracts all of this complexity into simple, safe function calls!

## Why This Example Uses High-Level Crates

This example relies heavily on the `lsm303agr` driver crate, which provides:

- **Initialization sequences**: The sensor requires specific power-up and configuration steps
- **Register abstraction**: The type system prevents reading/writing wrong registers
- **Unit conversion**: Automatic conversion from raw sensor values to meaningful units (mg, µT)
- **Error handling**: I²C errors are properly propagated

## What's Next?

> **🔬 Want to Experiment?** Try modifying the code to:
> - Read magnetometer data as well
> - Change the sampling rate or resolution
> - Detect motion by checking if acceleration exceeds a threshold
> - Calculate the tilt angle from x, y, z values

## Additional Resources

- **[deep_dive.md](../deep_dive.md)** - Complete technical explanation of Rust trait system and zero-cost abstractions
- **[LSM303AGR Datasheet](https://www.st.com/resource/en/datasheet/lsm303agr.pdf)** - Complete sensor register map and specifications
- **[nRF52833 TWIM Documentation](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.7.pdf)** - I²C/TWIM peripheral reference (Section 6.34)
- **[I²C Specification](https://www.nxp.com/docs/en/user-guide/UM10204.pdf)** - Official I²C protocol documentation