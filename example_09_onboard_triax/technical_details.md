# Example 09 - Technical Details# Example 09 - Technical Details



This document traces the complete journey from high-level Rust code down to physical electrical signals on the I²C bus, demonstrating how Rust's zero-cost abstractions work in embedded systems.This document traces the complete journey from high-level Rust code down to physical electrical signals on the I²C bus, demonstrating how Rust's zero-cost abstractions work in embedded systems.



## Project Structure



```## Project Structure## Project Structure

example_09_onboard_triax/

├── src/

│   └── main.rs          # Your Rust code

├── Cargo.toml           # Dependencies and project metadata  ```This example contains everything needed to build and run independently:

├── Embed.toml           # probe-rs flashing configuration

└── readme.md            # Example overview and usageexample_09_onboard_triax/

```

├── src/```

## Hardware Overview

│   └── main.rs          # Your Rust codeexample_09_onboard_triax/

### The LSM303AGR Sensor

├── Cargo.toml           # Dependencies and project metadata  ├── src/

The LSM303AGR is a combined motion sensor containing two devices in one chip:

- **Accelerometer**: Measures acceleration/tilt (I²C address 0x19)├── Embed.toml           # probe-rs flashing configuration│   └── main.rs          # Your Rust code

- **Magnetometer**: Measures magnetic field/compass (I²C address 0x1E)

└── readme.md            # Example overview and usage├── Cargo.toml           # Dependencies and project metadata

Each sensor responds to its own I²C address and has separate registers for configuration and data.

```├── Embed.toml           # probe-rs flashing configuration

### I²C Buses on the micro:bit v2

└── readme.md            # Example overview and usage

The micro:bit v2 has two I²C buses:

## Hardware Overview```

**Internal I²C Bus** (used in this example):

- **Purpose**: Connects nRF52833 to onboard sensors

- **Device**: LSM303AGR accelerometer/magnetometer

- **Pins**: SCL (clock) on P0.08, SDA (data) on P0.16### The LSM303AGR Sensor## Hardware Overview

- **Access in code**: `board.i2c_internal`



**External I²C Bus**:

- **Purpose**: Connect external I²C devices via edge connectorThe LSM303AGR is a combined motion sensor containing two devices in one chip:### The LSM303AGR Sensor

- **Pins**: SCL on P0.26 (pin 19), SDA on P0.25 (pin 20)

- **Access in code**: `board.i2c_external`- **Accelerometer**: Measures acceleration/tilt (I²C address 0x19)



### The TWIM Peripheral- **Magnetometer**: Measures magnetic field/compass (I²C address 0x1E)The LSM303AGR is actually two sensors in one chip:



The nRF52833's TWIM (Two-Wire Interface Master) is dedicated hardware that implements the I²C protocol:- **Accelerometer**: Measures acceleration/tilt (I²C address 0x19)

- Two peripherals available: TWIM0 and TWIM1

- Supports standard (100kHz) and fast (400kHz) speedsEach sensor responds to its own I²C address and has separate registers for configuration and data.- **Magnetometer**: Measures magnetic field/compass (I²C address 0x1E)

- Hardware state machine handles protocol timing automatically

- DMA support for efficient data transfer

- Configurable shortcuts for common operations

### I²C Buses on the micro:bit v2Each sensor responds to its own I²C address and has separate registers for configuration and data.

## Complete Hardware Flow: From Rust to Silicon



Let's trace one simple operation through all layers - reading the sensor ID register. This demonstrates the complete path from your one-line Rust call down to electrical signals on the wire and back.

The micro:bit v2 has two I²C buses:### I²C Buses on the micro:bit v2

### Your Starting Point



```rust

let id = sensor.accelerometer_id().unwrap();**Internal I²C Bus** (used in this example):The micro:bit v2 has two I²C buses:

```

- **Purpose**: Connects nRF52833 to onboard sensors

One simple function call. Behind it lies nine layers of abstraction that ultimately manipulate silicon and electrical signals.

- **Device**: LSM303AGR accelerometer/magnetometer**Internal I²C Bus** (used in this example):

### Layer 1: Driver Crate (`lsm303agr`)

- **Pins**: SCL (clock) on P0.08, SDA (data) on P0.16- **Purpose**: Connects the nRF52833 to onboard sensors

```rust

// Inside the lsm303agr crate- **Access in code**: `board.i2c_internal`- **Device**: LSM303AGR (accelerometer/magnetometer)

pub fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE>> {

    self.iface.read_accel_register::<WhoAmIA>()- **Pins**: SCL (clock) on P0.08, SDA (data) on P0.16

}

```**External I²C Bus**:- **Access**: `board.i2c_internal`



**What's happening:**- **Purpose**: Connect external I²C devices via edge connector

- Calls `read_accel_register` with type parameter `WhoAmIA`

- `WhoAmIA` is a zero-sized type (marker) that encodes register information- **Pins**: SCL on P0.26 (pin 19), SDA on P0.25 (pin 20)**External I²C Bus**:

- Uses Rust's type system to make register access type-safe

- **Access in code**: `board.i2c_external`- **Purpose**: Connect external I²C devices via the edge connector

**The register type definition:**

```rust- **Pins**: SCL on P0.26 (pin 19), SDA on P0.25 (pin 20)

register! {

  /// WHO_AM_I_A register at address 0x0F### The TWIM Peripheral- **Access**: `board.i2c_external`

  pub type WhoAmIA: 0x0F = AccelerometerId;

}



impl WhoAmIA {The nRF52833's TWIM (Two-Wire Interface Master) is dedicated hardware that implements the I²C protocol:### The TWIM Peripheral

    pub(crate) const ID: u8 = 0x33;  // Expected value

}- Two peripherals available: TWIM0 and TWIM1

```

- Supports standard (100kHz) and fast (400kHz) speedsThe nRF52833's TWIM (Two-Wire Interface Master) is a dedicated hardware block that implements the I²C protocol:

**The `register!` macro expands to:**

```rust- Hardware state machine handles protocol timing automatically- Two TWIM peripherals available: TWIM0 and TWIM1

impl RegRead for WhoAmIA {

    type Output = AccelerometerId;  // Return type- DMA support for efficient data transfer- Supports standard (100kHz) and fast (400kHz) I²C speeds

    const ADDR: u8 = 0x0F;         // Register address

    - Configurable shortcuts for common operations- Hardware state machine handles protocol timing

    fn from_data(data: u8) -> Self::Output {

        AccelerometerId::from_bits_truncate(data)- DMA support for efficient data transfer

    }

}## Complete Hardware Flow: From Rust to Silicon

```

## Complete Hardware Flow: From Rust to Silicon

This is the **trait magic** - the type system encodes:

- Which register to read (0x0F)Let's trace one simple operation through all layers - reading the sensor ID register. This demonstrates the complete path from your one-line Rust call down to electrical signals on the wire and back.

- What type to return (AccelerometerId)

- How to convert raw bytes (from_data method)Let's trace one simple operation - reading the sensor ID - through all layers from your code down to the electrical signals.



### Layer 2: Generic Register Read### Your Starting Point



```rust### 1. Your Code

fn read_accel_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {

    self.read_register::<R>(ACCEL_ADDR)  // ACCEL_ADDR = 0x19```rust```rust

}

```let id = sensor.accelerometer_id().unwrap();let id = sensor.accelerometer_id().unwrap();



**What's happening:**``````

- Still generic over register type `R` (which is `WhoAmIA`)

- Adds the I²C slave address (0x19 for accelerometer)

- Calls more generic `read_register` function

One simple function call. Behind it lies nine layers of abstraction that ultimately manipulate silicon and electrical signals.### 2. Method Call (`lsm303agr` crate - `device_impl.rs:217`)

### Layer 3: HAL Trait Call

```rust

```rust

fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error<E>> {### Layer 1: Driver Crate (`lsm303agr`)pub fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE>> {

    let mut data = [0];

    self.i2c    self.iface.read_accel_register::<WhoAmIA>()

        .write_read(address, &[R::ADDR], &mut data)

        .map_err(Error::Comm)?;```rust}

    

    Ok(R::from_data(data[0]))// Inside the lsm303agr crate```

}

```pub fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE>> {**What happens:** Calls `read_accel_register` with the type parameter `WhoAmIA`.



**What's happening:**    self.iface.read_accel_register::<WhoAmIA>()

- Creates buffer for one byte

- Calls `embedded-hal` I²C trait method `write_read()`}### 3. Register Type Definition (`register_address.rs:80`)

- Parameters:

  - `address`: 0x19 (accelerometer I²C address)``````rust

  - `&[R::ADDR]`: &[0x0F] (register address to read)

  - `&mut data`: buffer to receive responseregister! {

- Converts result using `R::from_data()`

**What's happening:**  /// WHO_AM_I_A

This is where we cross from driver code into hardware abstraction layer (HAL).

- Calls `read_accel_register` with type parameter `WhoAmIA`  pub type WhoAmIA: 0x0F = AccelerometerId;

### Layer 4: HAL Implementation (nRF52833-HAL)

- `WhoAmIA` is a zero-sized type (marker) that encodes register information}

```rust

// Simplified version of the actual HAL code- Uses Rust's type system to make register access type-safe

pub fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {

    // 1. Configure slave addressimpl WhoAmIA {

    self.0.address.write(|w| w.address().bits(address));

    **The register type definition:**    pub(crate) const ID: u8 = 0b00110011;  // This is 51 decimal, 0x33 hex

    // 2. Setup TX buffer (what to write - the register address)

    self.0.txd.ptr.write(|w| unsafe { w.ptr().bits(bytes.as_ptr() as u32) });```rust}

    self.0.txd.maxcnt.write(|w| w.maxcnt().bits(bytes.len() as u8));

    register! {```

    // 3. Setup RX buffer (where to read into)

    self.0.rxd.ptr.write(|w| unsafe { w.ptr().bits(buffer.as_mut_ptr() as u32) });  /// WHO_AM_I_A register at address 0x0F**What happens:** The `register!` macro expands to create:

    self.0.rxd.maxcnt.write(|w| w.maxcnt().bits(buffer.len() as u8));

      pub type WhoAmIA: 0x0F = AccelerometerId;- An empty enum `WhoAmIA` (just a type marker)

    // 4. Enable shortcut: automatically start read after write

    self.0.shorts.write(|w| w.lasttx_startrx().enabled());}- Implementation of `RegRead` trait for `WhoAmIA`

    

    // 5. Start transmission

    self.0.tasks_starttx.write(|w| w.tasks_starttx().set_bit());

    impl WhoAmIA {The macro expansion creates:

    // 6. Wait for completion

    while self.0.events_stopped.read().bits() == 0 {    pub(crate) const ID: u8 = 0x33;  // Expected value```rust

        if self.0.events_error.read().bits() != 0 {

            return Err(Error::Nack);}impl RegRead for WhoAmIA {

        }

    }```    type Output = AccelerometerId;

    

    // 7. Clear event for next transaction    const ADDR: u8 = 0x0F;

    self.0.events_stopped.write(|w| w.events_stopped().clear_bit());

    **The `register!` macro expands to:**    

    Ok(())

}```rust    fn from_data(data: u8) -> Self::Output {

```

impl RegRead for WhoAmIA {        AccelerometerId::from_bits_truncate(data)

**What's happening:**

- Each `write()` call modifies a memory-mapped hardware register    type Output = AccelerometerId;  // Return type    }

- These registers control the TWIM peripheral

- The `LASTTX_STARTRX` shortcut is key - creates atomic write-read operation    const ADDR: u8 = 0x0F;         // Register address}



**Why the shortcut matters:**    ```



Without shortcut (two separate transactions):    fn from_data(data: u8) -> Self::Output {

```

Transaction 1: START -> [0x19+W] -> [0x0F] -> STOP        AccelerometerId::from_bits_truncate(data)### 4. Read Accelerometer Register (`interface.rs:139`)

Transaction 2: START -> [0x19+R] -> [data] -> STOP

```    }```rust



With shortcut (atomic transaction):}fn read_accel_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {

```

START -> [0x19+W] -> [0x0F] -> RESTART -> [0x19+R] -> [data] -> STOP```    self.read_register::<R>(ACCEL_ADDR)

```

}

The sensor expects register reads to be atomic (no STOP between write and read).

This is the **trait magic** - the type system encodes:```

### Layer 5: Memory-Mapped Registers

- Which register to read (0x0F)**What happens:** 

The `self.0.address.write()` calls are writing to specific memory addresses:

- What type to return (AccelerometerId)- `R` is `WhoAmIA`

| Register | Memory Address | Purpose | Value |

|----------|---------------|---------|-------|- How to convert raw bytes (from_data method)- `ACCEL_ADDR` is the I²C address of the accelerometer (0x19)

| `ADDRESS` | 0x40003588 | I²C slave address | 0x19 |

| `TXD.PTR` | 0x40003534 | Pointer to TX data | &[0x0F] |- Returns `R::Output` which is `AccelerometerId`

| `TXD.MAXCNT` | 0x40003538 | Bytes to transmit | 1 |

| `RXD.PTR` | 0x40003544 | Pointer to RX buffer | &data |### Layer 2: Generic Register Read

| `RXD.MAXCNT` | 0x40003548 | Bytes to receive | 1 |

| `SHORTS` | 0x40003200 | Shortcut config | LASTTX_STARTRX |### 5. Generic Read Register (`interface.rs:174`)

| `TASKS_STARTTX` | 0x40003008 | Start trigger | 1 (write any value) |

| `EVENTS_STOPPED` | 0x40003104 | Done flag | (poll until 1) |```rust```rust



**These aren't just variables - they're physical flip-flops in silicon!**fn read_accel_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error<E>> {



When ARM Cortex-M4 executes a store instruction to address 0x40003588, electrical signals propagate through the chip's address decoder to the TWIM peripheral, which detects the write and updates its configuration registers.    self.read_register::<R>(ACCEL_ADDR)  // ACCEL_ADDR = 0x19    let mut data = [0];



### Layer 6: TWIM Peripheral State Machine}    self.i2c



The TWIM is a hardware block with its own state machine that runs independently. When you write 1 to `TASKS_STARTTX`, you trigger this sequence:```        .write_read(address, &[R::ADDR], &mut data)



**1. IDLE** → **START**        .map_err(Error::Comm)?;

- Drive SDA LOW while SCL is HIGH

- This is the I²C START condition**What's happening:**



**2. START** → **TX ADDRESS**- Still generic over register type `R` (which is `WhoAmIA`)    Ok(R::from_data(data[0]))

- Shift out 8 bits: 7-bit address + 1 write bit

- Address: 0x19 = `0b0011001`- Adds the I²C slave address (0x19 for accelerometer)}

- Plus write bit (0): `0b00110010` = 0x32

- For each bit:- Calls more generic `read_register` function```

  - Set SDA to bit value

  - Pulse SCL HIGH then LOW**What happens:**

  - Slave samples on SCL rising edge

### Layer 3: HAL Trait Call1. Creates a buffer `data` for receiving one byte

**3. TX ADDRESS** → **WAIT ACK**

- Release SDA (tri-state, pull-up pulls HIGH)2. Calls I²C `write_read`:

- Pulse SCL

- Sample SDA on HIGH - slave pulls LOW to ACK```rust   - **address**: `0x19` (accelerometer I²C address)



**4. ACK** → **TX DATA**fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error<E>> {   - **write data**: `[0x0F]` (the WHO_AM_I register address from `WhoAmIA::ADDR`)

- Shift out register address byte: 0x0F = `0b00001111`

- 8 clock pulses, MSB first    let mut data = [0];   - **read buffer**: `&mut data` (reads 1 byte into this)



**5. TX DATA** → **WAIT ACK**    self.i2c3. Calls `R::from_data(data[0])` which is `WhoAmIA::from_data(data[0])`

- Slave ACKs the byte

        .write_read(address, &[R::ADDR], &mut data)

**6. Shortcut: LASTTX** → **STARTRX**

- Instead of STOP, immediately send RESTART        .map_err(Error::Comm)?;#### Deep Dive: `write_read()` Implementation

- RESTART looks like START (SDA LOW while SCL HIGH)

    

**7. STARTRX** → **TX ADDRESS (read)**

- Send same address with read bit: `0b00110011` = 0x33    Ok(R::from_data(data[0]))The `write_read()` method is part of the `embedded_hal::i2c::I2c` trait. For the nRF52833, it's implemented in the HAL crate and performs these steps:



**8. WAIT ACK** → **RX DATA**}

- Slave ACKs

- Master releases SDA, becomes receiver```**High-level flow:**

- 8 clock pulses

- Sample SDA on each SCL rising edge```rust

- Build byte in shift register

- DMA writes byte to RXD.PTR address**What's happening:**// Simplified view of what write_read does:



**9. RX DATA** → **SEND NACK**- Creates buffer for one bytefn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Error> {

- Master keeps SDA HIGH during ACK clock

- Signals "this is the last byte"- Calls `embedded-hal` I²C trait method `write_read()`    // 1. Set I²C slave address



**10. NACK** → **STOP**- Parameters:    // 2. Setup TX buffer (what to write)

- SDA rises while SCL is HIGH

- I²C STOP condition  - `address`: 0x19 (accelerometer I²C address)    // 3. Setup RX buffer (where to read into)

- Set `EVENTS_STOPPED` flag

  - `&[R::ADDR]`: &[0x0F] (register address to read)    // 4. Start transmission

### Layer 7: Physical Electrical Signals

  - `&mut data`: buffer to receive response    // 5. Wait for completion

Here's what's actually happening on the two wires (oscilloscope view):

- Converts result using `R::from_data()`    // 6. Check for errors

```

Time →}



SCL: ‾‾‾‾\_/\_/\_/\_/\_/\_/\_/\_/‾\_/\_/\_/\_/\_/\_/\_/\_/‾‾\__/\_/\_/\_/\_/\_/\_/\_/‾‾‾‾This is where we cross from driver code into hardware abstraction layer (HAL).```

SDA: ‾‾\_0_0_1_1_0_0_1_0_A_0_0_0_0_1_1_1_1_A_‾\_0_0_1_1_0_0_1_1_A_0_0_1_1_0_0_1_1_N‾‾

     S  |  Address+W  |A|Reg Addr |A R |  Address+R  |A|   Data    |N P

     T                           S                           O

     A                           T                           P### Layer 4: HAL Implementation (nRF52833-HAL)**Detailed TWIM peripheral operations:**

     R                           A

     T                           R

                                 T

``````rust1. **Configure slave address:**



**Legend:**// Simplified version of the actual HAL code   ```rust

- S = START (SDA falls while SCL high)

- A = ACK (slave pulls SDA low)pub fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {   // Write to TWIM ADDRESS register

- R = RESTART

- N = NACK (master leaves SDA high)    // 1. Configure slave address   twim.address.write(|w| w.address().bits(address));  // 0x19

- P = STOP (SDA rises while SCL high)

- 0/1 = Data bits    self.0.address.write(|w| w.address().bits(address));   ```

- _ = LOW (~0V)

- ‾ = HIGH (~3.3V)    



**Voltage levels:**    // 2. Setup TX buffer (what to write - the register address)2. **Setup TX buffer (write phase):**

- Logic HIGH: ~3.3V (provided by pull-up resistors)

- Logic LOW: ~0V (actively driven)    self.0.txd.ptr.write(|w| unsafe { w.ptr().bits(bytes.as_ptr() as u32) });   ```rust

- Both lines have pull-up resistors (typically 4.7kΩ)

- Any device can pull LOW (open-drain/open-collector)    self.0.txd.maxcnt.write(|w| w.maxcnt().bits(bytes.len() as u8));   // Point TWIM to our write data



**Timing at 100kHz I²C:**       twim.txd.ptr.write(|w| unsafe { w.ptr().bits(write.as_ptr() as u32) });

- Clock period: 10 microseconds

- Each bit: ~10µs    // 3. Setup RX buffer (where to read into)   twim.txd.maxcnt.write(|w| w.maxcnt().bits(write.len() as u8));  // 1 byte: [0x0F]

- Full transaction: ~50 clock cycles = ~500µs

    self.0.rxd.ptr.write(|w| unsafe { w.ptr().bits(buffer.as_mut_ptr() as u32) });   ```

### Layer 8: Inside the LSM303AGR Sensor

    self.0.rxd.maxcnt.write(|w| w.maxcnt().bits(buffer.len() as u8));

The sensor has its own I²C slave state machine:

    3. **Setup RX buffer (read phase):**

**1. Monitors bus** → Detects START condition

    // 4. Enable shortcut: automatically start read after write   ```rust

**2. Receives address** → Compares to 0x19

   - Match! Responds with ACK    self.0.shorts.write(|w| w.lasttx_startrx().enabled());   // Point TWIM to our read buffer



**3. Receives byte** → Stores as register address (0x0F)       twim.rxd.ptr.write(|w| unsafe { w.ptr().bits(read.as_mut_ptr() as u32) });

   - ACKs

    // 5. Start transmission   twim.rxd.maxcnt.write(|w| w.maxcnt().bits(read.len() as u8));  // 1 byte

**4. Detects RESTART** → Prepares for read operation

    self.0.tasks_starttx.write(|w| w.tasks_starttx().set_bit());   ```

**5. Receives address+read** → Matches 0x19

   - ACKs    



**6. Shifts out data** → Reads WHO_AM_I register (hardwired to 0x33)    // 6. Wait for completion4. **Configure for write-then-read (no stop between):**

   - Sends: `0b00110011`

    while self.0.events_stopped.read().bits() == 0 {   ```rust

**7. Receives NACK** → Master done reading

           if self.0.events_error.read().bits() != 0 {   // Enable shortcut: LASTTX -> STARTRX (automatically start read after write)

**8. Detects STOP** → Returns to idle

            return Err(Error::Nack);   twim.shorts.write(|w| w.lasttx_startrx().enabled());

The WHO_AM_I register at address 0x0F is hardwired in the sensor's ROM - it always returns 0x33. This allows software to verify it's communicating with the correct sensor.

        }   ```

### Layer 9: Journey Back Up

    }   This tells the TWIM: "After sending the last TX byte, automatically start receiving without sending a STOP condition." This creates the RESTART condition needed for register reads.

Once the TWIM peripheral sets `EVENTS_STOPPED`:

    

**1. TWIM Peripheral:**

- DMA has already written 0x33 to your `data` buffer in RAM    // 7. Clear event for next transaction5. **Start the transaction:**



**2. HAL Code:**    self.0.events_stopped.write(|w| w.events_stopped().clear_bit());   ```rust

- Loop exits (events_stopped is now 1)

- Returns `Ok(())`       // Trigger STARTTX task - begins the I²C transaction



**3. Driver Code:**    Ok(())   twim.tasks_starttx.write(|w| w.tasks_starttx().set_bit());

- Receives raw byte 0x33

- Calls `AccelerometerId::from_bits_truncate(0x33)`}   ```

- Wraps in type-safe struct

```

**4. Your Code:**

- Receives `AccelerometerId { raw: 51 }`6. **Wait for completion:**

- Can call methods like `id.is_correct()` to validate

**What's happening:**   ```rust

### The Complete Journey Summary

- Each `write()` call modifies a memory-mapped hardware register   // Poll or wait on interrupt for the STOPPED event

From your one line of Rust:

- These registers control the TWIM peripheral   while twim.events_stopped.read().events_stopped().bit_is_clear() {

```rust

let id = sensor.accelerometer_id().unwrap();- The `LASTTX_STARTRX` shortcut is key - creates atomic write-read operation       // Check for errors (NACK, bus error, etc.)

```

       if twim.events_error.read().events_error().bit_is_set() {

To physical reality and back:

**Why the shortcut matters:**           return Err(Error::Nack);

- **9 software layers** of abstraction

- **~30 memory-mapped register operations**       }

- **~50 clock cycles** on the I²C bus

- **~500 microseconds** of real timeWithout shortcut (two separate transactions):   }

- **0 bytes** of runtime overhead (all abstractions compile away)

```   ```

**Every layer compiles to direct hardware access with zero abstraction cost!**

Transaction 1: START -> [0x19+W] -> [0x0F] -> STOP

## Understanding the Rust Trait Magic

Transaction 2: START -> [0x19+R] -> [data] -> STOP7. **Clean up:**

### The `RegRead` Trait

```   ```rust

```rust

pub trait RegRead<D = u8> {   // Clear the STOPPED event for next transaction

    type Output;                        // What type to return

    const ADDR: u8;                     // Which register addressWith shortcut (atomic transaction):   twim.events_stopped.write(|w| w.events_stopped().clear_bit());

    fn from_data(data: D) -> Self::Output;  // How to convert

}```   ```

```

START -> [0x19+W] -> [0x0F] -> RESTART -> [0x19+R] -> [data] -> STOP

This trait is the key to type-safe register access.

```**What's happening on the I²C bus during all this:**

### Why Use All These Traits?



**1. Type Safety:**

```rustThe sensor expects register reads to be atomic (no STOP between write and read).```

// Can't accidentally read wrong register

sensor.read_accel_register::<MagnetometerReg>()  // Compile error!Time →

```

### Layer 5: Memory-Mapped Registers

**2. Code Reuse:**

```rust1. START condition (SDA falls while SCL high)

// One generic function handles ALL registers

fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error>The `self.0.address.write()` calls are writing to specific memory addresses:2. Send 7-bit address + WRITE bit: 0x19 << 1 | 0 = 0b00110010

```

3. Sensor ACKs (pulls SDA low)

**3. Compile-Time Guarantees:**

```rust| Register | Memory Address | Purpose | Value |4. Send register address: 0x0F

// Register address and type known at compile time

const ADDR: u8 = 0x0F;          // No runtime lookup needed|----------|---------------|---------|-------|5. Sensor ACKs

type Output = AccelerometerId;   // Type checking at compile time

```| `ADDRESS` | 0x40003588 | I²C slave address | 0x19 |6. REPEATED START (no STOP - this is the shortcut magic!)



**4. Zero-Cost Abstraction:**| `TXD.PTR` | 0x40003534 | Pointer to TX data | &[0x0F] |7. Send 7-bit address + READ bit: 0x19 << 1 | 1 = 0b00110011

- All type parameters resolved at compile time

- No virtual dispatch, no runtime cost| `TXD.MAXCNT` | 0x40003538 | Bytes to transmit | 1 |8. Sensor ACKs

- Compiles to same code as hand-written register access

| `RXD.PTR` | 0x40003544 | Pointer to RX buffer | &data |9. Sensor sends data byte: 0x33

### The Type System Encodes Hardware

| `RXD.MAXCNT` | 0x40003548 | Bytes to receive | 1 |10. Master NACKs (signals "last byte")

```

WhoAmIA (zero-sized marker type)| `SHORTS` | 0x40003200 | Shortcut config | LASTTX_STARTRX |11. STOP condition (SDA rises while SCL high)

  ↓

implements RegRead trait| `TASKS_STARTTX` | 0x40003008 | Start trigger | 1 (write any value) |```

  ↓

  ADDR = 0x0F          (compile-time constant)| `EVENTS_STOPPED` | 0x40003104 | Done flag | (poll until 1) |

  Output = AccelerometerId (compile-time type)

  from_data = ...      (compile-time function)**Why the shortcut matters:**

  ↓

Generic code becomes concrete**These aren't just variables - they're physical flip-flops in silicon!**

  ↓

Compiles to direct hardware accessWithout the LASTTX → STARTRX shortcut, you'd need two separate I²C transactions:

```

When ARM Cortex-M4 executes a store instruction to address 0x40003588, electrical signals propagate through the chip's address decoder to the TWIM peripheral, which detects the write and updates its configuration registers.```

### Why Not Just Use u8?

Transaction 1: START -> [0x19+W] -> [0x0F] -> STOP

The driver wraps raw bytes in type-safe structs:

### Layer 6: TWIM Peripheral State MachineTransaction 2: START -> [0x19+R] -> [data] -> STOP

```rust

pub struct AccelerometerId {```

    raw: u8,

}The TWIM is a hardware block with its own state machine that runs independently. When you write 1 to `TASKS_STARTTX`, you trigger this sequence:



impl AccelerometerId {The shortcut combines them into one transaction with a RESTART:

    pub const fn raw(&self) -> u8 {

        self.raw**1. IDLE** → **START**```

    }

    - Drive SDA LOW while SCL is HIGHSTART -> [0x19+W] -> [0x0F] -> RESTART -> [0x19+R] -> [data] -> STOP

    pub const fn is_correct(&self) -> bool {

        self.raw == 0x33- This is the I²C START condition```

    }

}

```

**2. START** → **TX ADDRESS**This is important because many I²C devices (including the LSM303AGR) expect register reads to be atomic operations without a STOP condition in between.

**Benefits:**

1. **Type safety**: Can't confuse with other u8 values- Shift out 8 bits: 7-bit address + 1 write bit

2. **Helper methods**: Like `is_correct()` for validation

3. **Semantic meaning**: It's specifically an accelerometer ID- Address: 0x19 = `0b0011001`### 6. I²C Transaction (HAL Level)

4. **Future flexibility**: Can change implementation without breaking API

- Plus write bit (0): `0b00110010` = 0x32The `write_read` is from the `embedded-hal` I²C trait and ultimately calls the nRF52 TWIM peripheral:

**Debug formatting:**

```rust- For each bit:```

rprintln!("ID: {:?}", id);  // Works via Debug trait

// Output: AccelerometerId { raw: 51 }  - Set SDA to bit valueI²C Bus Transaction:

```

  - Pulse SCL HIGH then LOW  START -> [0x19 + WRITE] -> [0x0F] -> RESTART -> [0x19 + READ] -> [data byte] -> STOP

## Sensor Initialization and Type States

  - Slave samples on SCL rising edge```

### The Initialization Sequence

The sensor responds with `0x33` (51 decimal).

```rust

// 1. Create driver with I²C peripheral**3. TX ADDRESS** → **WAIT ACK**

let mut sensor = Lsm303agr::new_with_i2c(i2c);

- Release SDA (tri-state, pull-up pulls HIGH)### 7. Convert Raw Data to Type (`types.rs:46`)

// 2. Configure accelerometer

sensor.set_accel_mode_and_odr(- Pulse SCL```rust

    &mut timer0, 

    AccelMode::HighResolution, - Sample SDA on HIGH - slave pulls LOW to ACKimpl AccelerometerId {

    AccelOutputDataRate::Hz50

).unwrap();    pub(crate) fn from_bits_truncate(raw: u8) -> Self {



// 3. Enable magnetometer**4. ACK** → **TX DATA**        Self { raw }

let mut sensor = sensor.into_mag_continuous().ok().unwrap();

```- Shift out register address byte: 0x0F = `0b00001111`    }



### Type States Pattern- 8 clock pulses, MSB first



The driver uses Rust's type system to enforce correct initialization order:    pub const fn raw(&self) -> u8 {



```rust**5. TX DATA** → **WAIT ACK**        self.raw

// Initial state - accelerometer only

Lsm303agr<I2C, MagModeUnknown>- Slave ACKs the byte    }



// After into_mag_continuous() - both sensors available

Lsm303agr<I2C, MagContinuous>

```**6. Shortcut: LASTTX** → **STARTRX**    pub const fn is_correct(&self) -> bool {



**The compiler enforces this:**- Instead of STOP, immediately send RESTART        self.raw == WhoAmIA::ID  // Checks if raw == 0x33

```rust

let sensor = Lsm303agr::new_with_i2c(i2c);- RESTART looks like START (SDA LOW while SCL HIGH)    }

sensor.magnetic_field();  // Compile error! Mag not initialized yet

}

let sensor = sensor.into_mag_continuous().ok().unwrap();

sensor.magnetic_field();  // OK! Type system knows mag is ready**7. STARTRX** → **TX ADDRESS (read)**```

```

- Send same address with read bit: `0b00110011` = 0x33**What happens:** The raw `u8` value (0x33) is wrapped in the `AccelerometerId` struct.

### What Each Step Does



**1. `new_with_i2c(i2c)`:**

- Takes ownership of I²C peripheral**8. WAIT ACK** → **RX DATA**### 8. Result

- Creates driver struct

- No I²C transactions yet (just setup)- Slave ACKsYour `id` variable contains:



**2. `set_accel_mode_and_odr()`:**- Master releases SDA, becomes receiver```rust

- Writes to CTRL_REG1_A (0x20)

- Sets power mode (normal/high-res/low-power)- 8 clock pulsesAccelerometerId { raw: 51 }  // 0x33 in hex

- Sets output data rate (1Hz to 400Hz)

- Uses timer for required delays- Sample SDA on each SCL rising edge```



**3. `into_mag_continuous()`:**- Build byte in shift register

- **Consumes** the sensor (takes ownership)

- Writes to CFG_REG_A_M (0x60) and CFG_REG_C_M (0x62)- DMA writes byte to RXD.PTR address## Key Trait Magic

- Enables continuous measurement mode

- Returns **new** sensor with different type



## Reading Acceleration Data**9. RX DATA** → **SEND NACK**### The `RegRead` Trait



```rust- Master keeps SDA HIGH during ACK clock```rust

let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();

```- Signals "this is the last byte"pub trait RegRead<D = u8> {



### What Happens Behind the Scenes    type Output;           // What type to return (AccelerometerId)



**1. Read 6 consecutive registers:****10. NACK** → **STOP**    const ADDR: u8;        // Which register address (0x0F)

- OUT_X_L_A (0x28): X low byte

- OUT_X_H_A (0x29): X high byte- SDA rises while SCL is HIGH    fn from_data(data: D) -> Self::Output;  // How to convert raw byte

- OUT_Y_L_A (0x2A): Y low byte

- OUT_Y_H_A (0x2B): Y high byte- I²C STOP condition}

- OUT_Z_L_A (0x2C): Z low byte

- OUT_Z_H_A (0x2D): Z high byte- Set `EVENTS_STOPPED` flag```



**2. Combine into 16-bit values:**

```rust

let x_raw = ((x_h as i16) << 8) | (x_l as i16);### Layer 7: Physical Electrical SignalsThis trait allows the library to be generic over different register types while maintaining type safety.

```



**3. Convert from two's complement to signed integers**

Here's what's actually happening on the two wires (oscilloscope view):### Why All The Traits?

**4. Scale to milligravities:**

- Depends on configured range (±2g, ±4g, etc.)

- Default ±2g: LSB = ~1 mg

```1. **Type Safety**: Each register has its own type, so you can't accidentally read the wrong register

**5. Return as tuple:**

```rustTime →2. **Code Reuse**: One `read_register` function works for all register types

(x_mg, y_mg, z_mg)  // Each is i32

```3. **Compile-Time Guarantees**: The register address and output type are known at compile time



## Why High-Level Crates MatterSCL: ‾‾‾‾\_/\_/\_/\_/\_/\_/\_/\_/‾\_/\_/\_/\_/\_/\_/\_/\_/‾‾\__/\_/\_/\_/\_/\_/\_/\_/‾‾‾‾4. **Zero Cost Abstraction**: All this compiles down to efficient code with no runtime overhead



This example relies heavily on the `lsm303agr` driver crate. Without it, you'd need to:SDA: ‾‾\_0_0_1_1_0_0_1_0_A_0_0_0_0_1_1_1_1_A_‾\_0_0_1_1_0_0_1_1_A_0_0_1_1_0_0_1_1_N‾‾



**1. Implement I²C Protocol:**     S  |  Address+W  |A|Reg Addr |A R |  Address+R  |A|   Data    |N P### The Magic Flow

- Start/stop conditions

- Bit timing     T                           S                           O```

- ACK/NACK handling

- Bus arbitration     A                           T                           PWhoAmIA (empty enum, type marker)



**2. Understand Sensor Internals:**     R                           A  ↓

- 50+ configuration registers

- Power-up sequence requirements     T                           Rimplements RegRead trait

- Mode transition requirements

- Register interdependencies                                 T  ↓  



**3. Handle Data Conversion:**```  ADDR = 0x0F (constant)

- Two's complement arithmetic

- Scaling factors for different ranges  Output = AccelerometerId (associated type)

- Temperature compensation

- Calibration offsets**Legend:**  from_data = AccelerometerId::from_bits_truncate (method)



**4. Manage Errors:**- S = START (SDA falls while SCL high)  ↓

- I²C bus errors

- Sensor errors- A = ACK (slave pulls SDA low)read_register::<WhoAmIA>()

- Timeout handling

- Recovery procedures- R = RESTART  ↓



**The driver crate handles ALL of this**, giving you a simple, safe API:- N = NACK (master leaves SDA high)uses WhoAmIA::ADDR to know which register



```rust- P = STOP (SDA rises while SCL high)uses WhoAmIA::from_data to convert result

sensor.acceleration()  // One method call

```- 0/1 = Data bitsreturns WhoAmIA::Output (AccelerometerId)



Instead of hundreds of lines of error-prone register manipulation.- _ = LOW (~0V)```



## Summary: The Power of Embedded Rust- ‾ = HIGH (~3.3V)



This example demonstrates embedded Rust's key strengths:### Why Not Just Return u8?



**Zero-Cost Abstractions:****Voltage levels:**

- High-level code compiles to direct hardware access

- No runtime overhead for type safety- Logic HIGH: ~3.3V (provided by pull-up resistors)The library wraps the `u8` in `AccelerometerId` to provide:

- Same performance as hand-written C

- Logic LOW: ~0V (actively driven)1. **Type safety** - can't confuse it with other IDs

**Type Safety:**

- Can't read wrong registers- Both lines have pull-up resistors (typically 4.7kΩ)2. **Helper methods** like `is_correct()` to validate the ID

- Can't use uninitialized sensors

- Compiler catches errors before runtime- Any device can pull LOW (open-drain/open-collector)3. **Semantic meaning** - it's not just any byte, it's specifically an accelerometer ID



**Memory Safety:**4. **Debug formatting** with `{:?}` through the `Debug` trait

- No buffer overflows

- No use-after-free**Timing at 100kHz I²C:**

- Ownership prevents races

- Clock period: 10 microsecondsTo get the raw value, you'd call:

**Ergonomics:**

- One-line function calls- Each bit: ~10µs```rust

- Automatic error propagation

- Clear, readable code- Full transaction: ~50 clock cycles = ~500µslet raw_value: u8 = id.raw();



From `sensor.accelerometer_id()` to electrical signals and back, every layer adds safety and ergonomics while maintaining zero runtime cost.```



## Additional Resources### Layer 8: Inside the LSM303AGR Sensor



- **[deep_dive.md](../deep_dive.md)** - More on Rust traits and zero-cost abstractions## Sensor Initialization Sequence

- **[LSM303AGR Datasheet](https://www.st.com/resource/en/datasheet/lsm303agr.pdf)** - Complete sensor register map

- **[nRF52833 TWIM](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.7.pdf)** - TWIM peripheral reference (Section 6.34)The sensor has its own I²C slave state machine:

- **[I²C Specification](https://www.nxp.com/docs/en/user-guide/UM10204.pdf)** - Official protocol documentation

The LSM303AGR requires a specific initialization sequence:

**1. Monitors bus** → Detects START condition

```rust

**2. Receives address** → Compares to 0x19let mut sensor = Lsm303agr::new_with_i2c(i2c);  // Create driver instance

   - Match! Responds with ACK

// Configure accelerometer

**3. Receives byte** → Stores as register address (0x0F)sensor.set_accel_mode_and_odr(&mut timer0, AccelMode::HighResolution, AccelOutputDataRate::Hz50).unwrap();

   - ACKs

// Enable magnetometer (accelerometer continues working)

**4. Detects RESTART** → Prepares for read operationlet mut sensor = sensor.into_mag_continuous().ok().unwrap();

```

**5. Receives address+read** → Matches 0x19

   - ACKs### What Each Step Does



**6. Shifts out data** → Reads WHO_AM_I register (hardwired to 0x33)1. **`new_with_i2c(i2c)`**: 

   - Sends: `0b00110011`   - Creates the sensor driver

   - Takes ownership of the I²C peripheral

**7. Receives NACK** → Master done reading   - Doesn't perform any I²C transactions yet

   

**8. Detects STOP** → Returns to idle2. **`set_accel_mode_and_odr()`**:

   - Writes configuration registers

The WHO_AM_I register at address 0x0F is hardwired in the sensor's ROM - it always returns 0x33. This allows software to verify it's communicating with the correct sensor.   - Sets power mode (normal, high-resolution, low-power)

   - Sets output data rate (1Hz to 400Hz)

### Layer 9: Journey Back Up   - Uses timer for required delays between register writes



Once the TWIM peripheral sets `EVENTS_STOPPED`:3. **`into_mag_continuous()`**:

   - Consumes the sensor (takes ownership)

**1. TWIM Peripheral:**   - Configures magnetometer for continuous measurement

- DMA has already written 0x33 to your `data` buffer in RAM   - Returns a new sensor instance that can read both sensors

   - This pattern uses Rust's type system to enforce correct initialization order

**2. HAL Code:**

- Loop exits (events_stopped is now 1)### Type States Pattern

- Returns `Ok(())`

The `lsm303agr` crate uses the **type state pattern** to enforce correct usage at compile time:

**3. Driver Code:**

- Receives raw byte 0x33- `Lsm303agr<I2C, MagModeUnknown>` - Initial state, accelerometer only

- Calls `AccelerometerId::from_bits_truncate(0x33)`- `Lsm303agr<I2C, MagContinuous>` - Magnetometer in continuous mode

- Wraps in type-safe struct

You can't call magnetometer methods until you've called `into_mag_continuous()`. The compiler enforces this!

**4. Your Code:**

- Receives `AccelerometerId { raw: 51 }`## Reading Acceleration Data

- Can call methods like `id.is_correct()` to validate

```rust

### The Complete Journey Summarylet (x, y, z) = sensor.acceleration().unwrap().xyz_mg();

```

From your one line of Rust:

Behind the scenes:

```rust1. Reads 6 bytes from consecutive registers (2 bytes per axis)

let id = sensor.accelerometer_id().unwrap();2. Converts 16-bit two's complement values to signed integers

```3. Scales values to milligravities based on the configured range

4. Returns a tuple with the three axis values

To physical reality and back:

## Complete Hardware Flow: From Rust to Silicon

- **9 software layers** of abstraction

- **~30 memory-mapped register operations**Let's trace the entire path from your high-level Rust code down to the physical electrical signals on the I²C bus.

- **~50 clock cycles** on the I²C bus

- **~500 microseconds** of real time### Layer 1: Your Rust Code

- **0 bytes** of runtime overhead (all abstractions compile away)```rust

let id = sensor.accelerometer_id().unwrap();

**Every layer compiles to direct hardware access with zero abstraction cost!**```



## Understanding the Rust Trait Magic### Layer 2: Driver Crate (`lsm303agr`)

```rust

### The `RegRead` Trait// Calls read_accel_register with type WhoAmIA

self.iface.read_accel_register::<WhoAmIA>()

```rust```

pub trait RegRead<D = u8> {- Uses Rust's type system to encode which register to read (0x0F)

    type Output;                        // What type to return- Specifies the return type (AccelerometerId)

    const ADDR: u8;                     // Which register address

    fn from_data(data: D) -> Self::Output;  // How to convert### Layer 3: HAL Trait (`embedded_hal::i2c::I2c`)

}```rust

```// Generic I²C interface - works with any HAL implementation

self.i2c.write_read(0x19, &[0x0F], &mut data)

This trait is the key to type-safe register access.```

- `0x19`: I²C slave address of the accelerometer

### Why Use All These Traits?- `&[0x0F]`: Register address to read from

- `&mut data`: Buffer to receive the response

**1. Type Safety:**

```rust### Layer 4: nRF52 HAL Implementation (`nrf52833-hal`)

// Can't accidentally read wrong register```rust

sensor.read_accel_register::<MagnetometerReg>()  // Compile error!// Configures the TWIM peripheral registers

```pub fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {

    // Configure peripheral

**2. Code Reuse:**    self.0.address.write(|w| w.address().bits(address));

```rust    self.0.txd.ptr.write(|w| unsafe { w.ptr().bits(bytes.as_ptr() as u32) });

// One generic function handles ALL registers    self.0.txd.maxcnt.write(|w| w.maxcnt().bits(bytes.len() as u8));

fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error>    self.0.rxd.ptr.write(|w| unsafe { w.ptr().bits(buffer.as_mut_ptr() as u32) });

```    self.0.rxd.maxcnt.write(|w| w.maxcnt().bits(buffer.len() as u8));

    

**3. Compile-Time Guarantees:**    // Enable shortcut: automatically start read after write completes

```rust    self.0.shorts.write(|w| w.lasttx_startrx().enabled());

// Register address and type known at compile time    

const ADDR: u8 = 0x0F;          // No runtime lookup needed    // Start transmission

type Output = AccelerometerId;   // Type checking at compile time    self.0.tasks_starttx.write(|w| w.tasks_starttx().set_bit());

```    

    // Wait for completion

**4. Zero-Cost Abstraction:**    while self.0.events_stopped.read().bits() == 0 { }

- All type parameters resolved at compile time    

- No virtual dispatch, no runtime cost    Ok(())

- Compiles to same code as hand-written register access}

```

### The Type System Encodes Hardware

### Layer 5: Memory-Mapped Hardware Registers

```

WhoAmIA (zero-sized marker type)The writes to `self.0.address`, `self.0.txd.ptr`, etc. are writing to specific memory addresses that the TWIM peripheral monitors:

  ↓

implements RegRead trait| Register | Address | Purpose | Value Written |

  ↓|----------|---------|---------|---------------|

  ADDR = 0x0F          (compile-time constant)| `ADDRESS` | 0x40003588 | I²C slave address | 0x19 |

  Output = AccelerometerId (compile-time type)| `TXD.PTR` | 0x40003534 | Pointer to transmit buffer | Address of `&[0x0F]` |

  from_data = ...      (compile-time function)| `TXD.MAXCNT` | 0x40003538 | Number of bytes to transmit | 1 |

  ↓| `RXD.PTR` | 0x40003544 | Pointer to receive buffer | Address of `data` |

Generic code becomes concrete| `RXD.MAXCNT` | 0x40003548 | Number of bytes to receive | 1 |

  ↓| `SHORTS` | 0x40003200 | Shortcuts configuration | LASTTX_STARTRX |

Compiles to direct hardware access| `TASKS_STARTTX` | 0x40003008 | Trigger to start TX | 1 |

```| `EVENTS_STOPPED` | 0x40003104 | Transaction complete flag | (read until set) |



### Why Not Just Use u8?**These are actual hardware registers in silicon!** When you write `0x19` to address `0x40003588`, you're changing the state of physical flip-flops in the TWIM peripheral.



The driver wraps raw bytes in type-safe structs:### Layer 6: TWIM Peripheral State Machine



```rustThe TWIM peripheral is a dedicated hardware block with its own state machine. When you write to `TASKS_STARTTX`:

pub struct AccelerometerId {

    raw: u8,1. **Idle State** → **Sending START**

}   - Drives SDA low while SCL is high (START condition)

   

impl AccelerometerId {2. **Sending START** → **Sending Address**

    pub const fn raw(&self) -> u8 {   - Shifts out 7-bit address (0x19) + write bit (0)

        self.raw   - Clocks each bit: set SDA, pulse SCL high then low

    }   - Total: 8 bits = `0b00110010`

    

    pub const fn is_correct(&self) -> bool {3. **Sending Address** → **Waiting for ACK**

        self.raw == 0x33   - Releases SDA (sets as input)

    }   - Pulses SCL

}   - Checks if slave pulled SDA low (ACK)

```

4. **ACK Received** → **Sending Data**

**Benefits:**   - Shifts out register address (0x0F)

1. **Type safety**: Can't confuse with other u8 values   - 8 clock pulses, MSB first

2. **Helper methods**: Like `is_correct()` for validation

3. **Semantic meaning**: It's specifically an accelerometer ID5. **Data Sent** → **Waiting for ACK**

4. **Future flexibility**: Can change implementation without breaking API   - Slave ACKs the byte



**Debug formatting:**6. **Shortcut Triggered: LASTTX → STARTRX**

```rust   - Instead of sending STOP, immediately sends RESTART

rprintln!("ID: {:?}", id);  // Works via Debug trait

// Output: AccelerometerId { raw: 51 }7. **Sending RESTART** → **Sending Address (Read)**

```   - SDA low while SCL high (looks like START)

   - Sends address with read bit: `0b00110011`

## Sensor Initialization and Type States

8. **ACK Received** → **Receiving Data**

### The Initialization Sequence   - Sets SDA as input

   - Pulses SCL 8 times

```rust   - Samples SDA on each high pulse

// 1. Create driver with I²C peripheral   - Stores bits in shift register

let mut sensor = Lsm303agr::new_with_i2c(i2c);   - Uses DMA to write result to RXD.PTR address



// 2. Configure accelerometer9. **Data Received** → **Sending NACK**

sensor.set_accel_mode_and_odr(   - Master keeps SDA high during ACK clock

    &mut timer0,    - Signals "this is the last byte"

    AccelMode::HighResolution, 

    AccelOutputDataRate::Hz5010. **NACK Sent** → **Sending STOP**

).unwrap();    - SDA rises while SCL is high

    - Sets `EVENTS_STOPPED` flag

// 3. Enable magnetometer

let mut sensor = sensor.into_mag_continuous().ok().unwrap();### Layer 7: Physical Electrical Signals

```

Here's what's actually happening on the two wires:

### Type States Pattern

```

The driver uses Rust's type system to enforce correct initialization order:Time →



```rustSCL: ‾‾‾‾\_/\_/\_/\_/\_/\_/\_/\_/‾\_/\_/\_/\_/\_/\_/\_/\_/‾‾\__/\_/\_/\_/\_/\_/\_/\_/‾‾‾‾

// Initial state - accelerometer onlySDA: ‾‾\_0_0_1_1_0_0_1_0_A_0_0_0_0_1_1_1_1_A_‾\_0_0_1_1_0_0_1_1_A_0_0_1_1_0_0_1_1_N‾‾

Lsm303agr<I2C, MagModeUnknown>     S  |  Address+W  |A|Reg Addr |A R |  Address+R  |A|   Data    |N P

     T                           S                           O

// After into_mag_continuous() - both sensors available     A                           T                           P

Lsm303agr<I2C, MagContinuous>     R                           A

```     T                           R

                                 T

**The compiler enforces this:**```

```rust

let sensor = Lsm303agr::new_with_i2c(i2c);**Legend:**

sensor.magnetic_field();  // Compile error! Mag not initialized yet- `S` = START condition (SDA falls while SCL high)

- `A` = ACK (slave pulls SDA low)

let sensor = sensor.into_mag_continuous().ok().unwrap();- `R` = RESTART condition  

sensor.magnetic_field();  // OK! Type system knows mag is ready- `N` = NACK (master leaves SDA high)

```- `P` = STOP condition (SDA rises while SCL high)

- `0`, `1` = Data bits

### What Each Step Does

**Voltage levels:**

**1. `new_with_i2c(i2c)`:**- Logic HIGH: ~3.3V

- Takes ownership of I²C peripheral- Logic LOW: ~0V

- Creates driver struct- Pull-up resistors keep lines HIGH when not driven LOW

- No I²C transactions yet (just setup)

### Layer 8: Inside the LSM303AGR Sensor

**2. `set_accel_mode_and_odr()`:**

- Writes to CTRL_REG1_A (0x20)The sensor has its own internal state machine monitoring the I²C bus:

- Sets power mode (normal/high-res/low-power)

- Sets output data rate (1Hz to 400Hz)1. **Detects START** → Begins monitoring address bits

- Uses timer for required delays2. **Address matches 0x19** → Responds with ACK

3. **Receives register address 0x0F** → ACKs, prepares data

**3. `into_mag_continuous()`:**4. **Detects RESTART** → Ready for read operation

- **Consumes** the sensor (takes ownership)5. **Address matches 0x19 with read bit** → ACKs

- Writes to CFG_REG_A_M (0x60) and CFG_REG_C_M (0x62)6. **Shifts out WHO_AM_I register value** → Sends 0x33 (0b00110011)

- Enables continuous measurement mode7. **Receives NACK** → Knows to stop sending

- Returns **new** sensor with different type8. **Detects STOP** → Returns to idle



## Reading Acceleration DataThe sensor's WHO_AM_I register at 0x0F is hardwired in ROM to always return 0x33.



```rust### Layer 9: Back Up the Stack

let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();

```Once `EVENTS_STOPPED` is set:



### What Happens Behind the Scenes1. **TWIM peripheral**: DMA has already written 0x33 to your `data` buffer

2. **HAL**: Returns `Ok(())` from `write_read()`

**1. Read 6 consecutive registers:**3. **Driver**: Calls `AccelerometerId::from_bits_truncate(0x33)`

- OUT_X_L_A (0x28): X low byte4. **Your code**: Gets `AccelerometerId { raw: 51 }`

- OUT_X_H_A (0x29): X high byte

- OUT_Y_L_A (0x2A): Y low byte### The Complete Round Trip

- OUT_Y_H_A (0x2B): Y high byte

- OUT_Z_L_A (0x2C): Z low byteFrom your one line of Rust code:

- OUT_Z_H_A (0x2D): Z high byte- **10 layers of abstraction** (Rust → driver → HAL → register writes → state machine → electrical signals → sensor → back)

- **~30 memory-mapped register operations** in the nRF52

**2. Combine into 16-bit values:**- **~50 clock cycles** on the I²C bus

```rust- **~500 microseconds** of real time (at 100kHz I²C speed)

let x_raw = ((x_h as i16) << 8) | (x_l as i16);- **Zero runtime overhead** for all the Rust abstractions (compiled away)

```

This is the power of embedded Rust: you write safe, high-level code that compiles down to direct hardware manipulation!

**3. Convert from two's complement to signed integers**

## Why This Example Uses High-Level Crates

**4. Scale to milligravities:**

- Depends on configured range (±2g, ±4g, etc.)This example relies heavily on the `lsm303agr` driver crate, which provides:

- Default ±2g: LSB = ~1 mg

- **Initialization sequences**: The sensor requires specific power-up and configuration steps

**5. Return as tuple:**- **Register abstraction**: The type system prevents reading/writing wrong registers

```rust- **Unit conversion**: Automatic conversion from raw sensor values to meaningful units (mg, µT)

(x_mg, y_mg, z_mg)  // Each is i32- **Error handling**: I²C errors are properly propagated

```

## What's Next?

## Why High-Level Crates Matter

> **🔬 Want to Experiment?** Try modifying the code to:

This example relies heavily on the `lsm303agr` driver crate. Without it, you'd need to:> - Read magnetometer data as well

> - Change the sampling rate or resolution

**1. Implement I²C Protocol:**> - Detect motion by checking if acceleration exceeds a threshold

- Start/stop conditions> - Calculate the tilt angle from x, y, z values

- Bit timing

- ACK/NACK handling## Additional Resources

- Bus arbitration

- **[deep_dive.md](../deep_dive.md)** - Complete technical explanation of Rust trait system and zero-cost abstractions

**2. Understand Sensor Internals:**- **[LSM303AGR Datasheet](https://www.st.com/resource/en/datasheet/lsm303agr.pdf)** - Complete sensor register map and specifications

- 50+ configuration registers- **[nRF52833 TWIM Documentation](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.7.pdf)** - I²C/TWIM peripheral reference (Section 6.34)

- Power-up sequence requirements- **[I²C Specification](https://www.nxp.com/docs/en/user-guide/UM10204.pdf)** - Official I²C protocol documentation
- Mode transition requirements
- Register interdependencies

**3. Handle Data Conversion:**
- Two's complement arithmetic
- Scaling factors for different ranges
- Temperature compensation
- Calibration offsets

**4. Manage Errors:**
- I²C bus errors
- Sensor errors
- Timeout handling
- Recovery procedures

**The driver crate handles ALL of this**, giving you a simple, safe API:

```rust
sensor.acceleration()  // One method call
```

Instead of hundreds of lines of error-prone register manipulation.

## Summary: The Power of Embedded Rust

This example demonstrates embedded Rust's key strengths:

**Zero-Cost Abstractions:**
- High-level code compiles to direct hardware access
- No runtime overhead for type safety
- Same performance as hand-written C

**Type Safety:**
- Can't read wrong registers
- Can't use uninitialized sensors
- Compiler catches errors before runtime

**Memory Safety:**
- No buffer overflows
- No use-after-free
- Ownership prevents races

**Ergonomics:**
- One-line function calls
- Automatic error propagation
- Clear, readable code

From `sensor.accelerometer_id()` to electrical signals and back, every layer adds safety and ergonomics while maintaining zero runtime cost.

## Additional Resources

- **[deep_dive.md](../deep_dive.md)** - More on Rust traits and zero-cost abstractions
- **[LSM303AGR Datasheet](../doc/lsm303agr.pdf)** - Complete sensor register map
- **[nRF52833 Product Specification](../doc/nRF52833_PS_v1.7.pdf)** - TWIM peripheral reference (Section 6.34)
- **[I²C Specification](https://www.nxp.com/docs/en/user-guide/UM10204.pdf)** - Official protocol documentation
