# Tracing `sensor.accelerometer_id()` Call

Let's follow the complete path from your call to the actual I²C communication:

## 1. Your Code
```rust
let id = sensor.accelerometer_id().unwrap();
```

## 2. Method Call (`device_impl.rs:217`)
```rust
pub async fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE>> {
    self.iface.read_accel_register::<WhoAmIA>().await
}
```
**What happens:** Calls `read_accel_register` with the type parameter `WhoAmIA`.

## 3. Register Type Definition (`register_address.rs:80`)
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

## 4. Read Accelerometer Register (`interface.rs:139`)
```rust
async fn read_accel_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {
    self.read_register::<R>(ACCEL_ADDR).await
}
```
**What happens:** 
- `R` is `WhoAmIA`
- `ACCEL_ADDR` is the I²C address of the accelerometer (0x19)
- Returns `R::Output` which is `AccelerometerId`

## 5. Generic Read Register (`interface.rs:174`)
```rust
async fn read_register<R: RegRead>(&mut self, address: u8) -> Result<R::Output, Error<E>> {
    let mut data = [0];
    self.i2c
        .write_read(address, &[R::ADDR], &mut data)
        .await
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

## 6. I²C Transaction (HAL Level)

The `write_read` method is from the `embedded-hal` I²C trait. Here's how it works through the layers:

### 6a. The embedded-hal I2c Trait
```rust
pub trait I2c {
    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) 
        -> Result<(), Self::Error>;
}
```

### 6b. nrf52833-hal Implementation
The nrf52833-hal crate implements this trait for the TWIM peripheral:

```rust
impl<T: Instance> I2c for Twim<T> {
    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) 
        -> Result<(), Error> {
        // 1. Configure the I²C address
        self.twim.address.write(|w| unsafe { w.address().bits(address) });
        
        // 2. Set up TX (transmit) buffer - DMA will read from here
        self.twim.txd.ptr.write(|w| unsafe { w.ptr().bits(bytes.as_ptr() as u32) });
        self.twim.txd.maxcnt.write(|w| unsafe { w.maxcnt().bits(bytes.len() as u8) });
        
        // 3. Set up RX (receive) buffer - DMA will write to here
        self.twim.rxd.ptr.write(|w| unsafe { w.ptr().bits(buffer.as_mut_ptr() as u32) });
        self.twim.rxd.maxcnt.write(|w| unsafe { w.maxcnt().bits(buffer.len() as u8) });
        
        // 4. Configure shortcut: automatically start RX after TX completes
        self.twim.shorts.write(|w| w.lasttx_startrx().enabled());
        
        // 5. Clear any previous events
        self.twim.events_stopped.reset();
        self.twim.events_error.reset();
        
        // 6. Start the TX transaction (RX will auto-start via shortcut)
        self.twim.tasks_starttx.write(|w| unsafe { w.bits(1) });
        
        // 7. Wait for completion or error
        while self.twim.events_stopped.read().bits() == 0 {
            if self.twim.events_error.read().bits() != 0 {
                return Err(Error::Transmit);
            }
        }
        
        // 8. Check for errors
        let err = self.twim.errorsrc.read();
        if err.overrun().bit_is_set() {
            return Err(Error::Overrun);
        }
        if err.anack().bit_is_set() || err.dnack().bit_is_set() {
            return Err(Error::AddressNack);
        }
        
        Ok(())
    }
}
```

### 6c. What Happens in Hardware

For the call `write_read(0x19, &[0x0F], &mut data)`:

**Step 1: Setup Phase**
- ADDRESS register ← 0x19 (accelerometer I²C address)
- TXD.PTR ← memory address of `[0x0F]`
- TXD.MAXCNT ← 1 (one byte to transmit)
- RXD.PTR ← memory address of `data` buffer
- RXD.MAXCNT ← 1 (one byte to receive)
- SHORTS register ← LASTTX_STARTRX enabled

**Step 2: TX Transaction**
- TASKS_STARTTX triggered
- TWIM state machine generates:
  - START condition on I²C bus
  - Address byte: 0x19 with WRITE bit (0x32 on the wire)
  - Wait for ACK from sensor
  - Data byte: 0x0F (register address)
  - Wait for ACK from sensor
  - EVENTS_LASTTX fires

**Step 3: Automatic RX Transaction** (via shortcut)
- LASTTX_STARTRX shortcut triggers TASKS_STARTRX automatically
- TWIM state machine generates:
  - REPEATED START condition
  - Address byte: 0x19 with READ bit (0x33 on the wire)
  - Wait for ACK from sensor
  - Read one byte from sensor via DMA
  - Master sends NACK (last byte indicator)
  - STOP condition
  - EVENTS_LASTRX fires
  - EVENTS_STOPPED fires

**Step 4: Completion**
- DMA has written sensor response (0x33) to `data[0]`
- Function returns Ok(())

### 6d. The I²C Bus Signals

Actual electrical signals on SCL/SDA lines:
```
START -> [0x32] -> ACK -> [0x0F] -> ACK -> RESTART -> [0x33] -> ACK -> [0x33] -> NACK -> STOP
         │                │                           │                 │
         └─ 0x19 + W       └─ WHO_AM_I reg            └─ 0x19 + R       └─ sensor data
```

Where:
- `0x32` = `(0x19 << 1) | 0` (7-bit address + WRITE bit)
- `0x33` = `(0x19 << 1) | 1` (7-bit address + READ bit)
- ACK/NACK are single-bit responses
- Master (nRF52833) sends NACK after the last read byte

The sensor responds with `0x33` (51 decimal), which is its WHO_AM_I identification value.

## 7. Convert Raw Data to Type (`types.rs:46`)
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

## 8. Result
Your `id` variable contains:
```rust
AccelerometerId { raw: 0x33 }
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

## Why Not Just Return u8?

The library wraps the `u8` in `AccelerometerId` to provide:
1. Type safety - can't confuse it with other IDs
2. Helper methods like `is_correct()` to validate the ID
3. Semantic meaning - it's not just any byte, it's specifically an accelerometer ID
4. Debug formatting with `{:?}` through the `Debug` trait

To get the raw value, you'd call:
```rust
let raw_value: u8 = id.raw();
```
