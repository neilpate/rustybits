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
The `write_read` is from the `embedded-hal` I²C trait and ultimately calls the nRF52 TWIM peripheral:
```
I²C Bus Transaction:
  START -> [0x19 + WRITE] -> [0x0F] -> RESTART -> [0x19 + READ] -> [data byte] -> STOP
```
The sensor responds with `0x33` (51 decimal).

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
