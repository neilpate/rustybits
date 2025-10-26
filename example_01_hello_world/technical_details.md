# Example 01 - Technical Details

This document explains what's happening in the Hello World LED blink example, focused on understanding the specific code and concepts in this example.

## Project Structure

This example contains everything needed to build and run independently:

```
example_01_hello_world/
├── .cargo/
│   └── config.toml      # Build configuration (ARM target, probe-rs runner)
├── src/
│   └── main.rs          # Your Rust code
├── Cargo.toml           # Dependencies and project metadata
├── Cargo.lock           # Locked dependency versions (for reproducible builds)
└── Embed.toml           # probe-rs flashing configuration
```

## How The LED Matrix Works

The micro:bit's 5x5 LED display works as a matrix - each LED is controlled by setting its row HIGH and column LOW. This example controls just one LED at position (1,1).

When you configure:
- `row1` as HIGH (3.3V)
- `col1` as LOW (0V)

Current flows from row1 → through the LED → to col1, lighting up the LED at their intersection.

## What the Crates Are Doing

While this code looks simple, the Rust embedded ecosystem handles complex work behind the scenes:

### 1. Memory Layout (Automatic)

Example 01 works without a local `memory.x` file because:
- The `microbit-v2` crate automatically generates one during compilation
- It contains the nRF52833's memory layout: 512KB Flash, 128KB RAM
- The linker uses this to place your code at the right memory addresses

### 2. Cross-Compilation

Your Rust code gets compiled to ARM assembly:
- Target: `thumbv7em-none-eabihf` (ARM Cortex-M4F)
- Your `set_high()` calls become direct register writes
- All type checking happens at compile time - no runtime overhead

### 3. Hardware Startup

Before your `main()` runs, the `cortex-m-rt` crate:
- Sets up the ARM processor after power-on
- Initializes memory and the stack
- Calls your `main()` function when ready

### 4. Board Initialization (`microbit::Board::take()`)

When you call `microbit::Board::take().unwrap()`:
- **Singleton Pattern**: Returns `Some(Board)` the first time, `None` on subsequent calls - ensures only one part of your code can control the hardware
- **Pin Mapping**: Provides convenient access to GPIO pins with their micro:bit functions (already configured as push-pull outputs)
- **Peripheral Access**: Gives you ownership of the hardware peripherals (TIMER0, TWIM0, etc.)
- **Zero Cost**: All of this is just compile-time organization - no runtime checking happens

### 5. Pin Usage and Timer Setup

The pins returned from `board.display_pins` are ready to use:
- Already configured as outputs (can drive voltage)
- Already set to push-pull mode (can drive both HIGH and LOW)
- The `embedded_hal::digital::OutputPin` trait provides the `set_high()` and `set_low()` methods
- You can directly call these methods without further configuration

The timer provides precise delays:
- Takes ownership of the hardware timer (prevents conflicts)
- `delay_ms(300)` uses the hardware timer for accurate timing
- Much more precise than software loops

## The Power of HAL Crates

**Without these crates**, you'd need to:
- Read the 400+ page nRF52833 reference manual
- Write hundreds of lines of register manipulation code  
- Handle ARM assembly startup code and linker scripts manually

**With HAL crates**: The simple `board.display_pins.row1.set_high()` compiles to just 2-3 ARM assembly instructions while maintaining safe, high-level Rust code.

## The Crate Ecosystem

This example uses several crates working together:

### Key Dependencies

**`microbit-v2`**: Board support for the micro:bit v2
- Provides `Board::take()` for easy hardware access
- Pre-configures all the pins for micro:bit layout
- Includes the nRF52833 HAL functionality

**`cortex-m-rt`**: ARM Cortex-M runtime
- Handles processor startup sequence
- Sets up memory layout and stack
- Calls your `main()` function

**`panic-halt`**: What happens on errors
- Defines what to do if the program panics
- Simply halts execution (good for beginners)

## What Happens During Runtime

When your program runs:

1. **Hardware Startup**: The `cortex-m-rt` crate sets up the ARM processor
2. **Board Initialization**: `Board::take()` claims exclusive access to hardware
3. **LED Loop**: Your code runs in the infinite `loop`
   - `row1.set_high()` → GPIO register write (turns on voltage)
   - `col1.set_low()` → GPIO register write (creates current path)  
   - `delay_ms(100)` → Hardware timer provides precise 100ms delay
   - Set the row output low to turn off the LED
   - Repeat forever



## How `delay_ms()` Works Through the HAL

When you call `delay.delay_ms(100)`, it uses the hardware timer for precise timing:

### 1. Your Code
```rust
let mut delay = Delay::new(board.TIMER0);
delay.delay_ms(100_u32);
```

### 2. The DelayMs Trait (from embedded-hal)

The `embedded_hal::delay::DelayMs` trait defines the interface:
```rust
pub trait DelayMs<UXX> {
    fn delay_ms(&mut self, ms: UXX);
}
```

This trait is generic over the time unit type (u8, u16, u32, etc.), allowing flexibility in delay duration.

### 3. The HAL Implementation (nrf52833-hal)

The `nrf52833-hal` crate implements this trait using the TIMER peripheral:
```rust
impl<T: Instance> DelayMs<u32> for Delay<T> {
    fn delay_ms(&mut self, ms: u32) {
        // Convert milliseconds to microseconds
        self.delay_us(ms * 1_000);
    }
}

impl<T: Instance> DelayUs<u32> for Delay<T> {
    fn delay_us(&mut self, us: u32) {
        // Configure timer for 1 MHz (1 tick per microsecond)
        self.timer.set_frequency(Frequency::F1MHz);
        
        // Set the compare register for the target delay
        self.timer.cc(0).write(|w| unsafe { w.bits(us) });
        
        // Clear the event flag
        self.timer.events_compare[0].reset();
        
        // Start the timer
        self.timer.tasks_start.write(|w| unsafe { w.bits(1) });
        
        // Wait for the compare event (busy-wait)
        while self.timer.events_compare[0].read().bits() == 0 {}
        
        // Stop the timer
        self.timer.tasks_stop.write(|w| unsafe { w.bits(1) });
    }
}
```

### 4. How the TIMER Peripheral Works

The nRF52833 TIMER peripheral is a hardware counter:
- **Clock Source**: Runs at 1 MHz (configured via prescaler)
- **Counter Register**: Increments every microsecond
- **Compare Register (CC[0])**: Holds the target count value
- **Event**: Fires when counter equals compare value

For a 100ms delay:
1. Set timer frequency to 1 MHz (1 tick = 1 microsecond)
2. Load compare register with 100,000 (100ms × 1000 μs/ms)
3. Clear the compare event flag
4. Start the timer counting from 0
5. Busy-wait until the compare event flag is set
6. Stop the timer

### 5. Why Hardware Timers Are Better

Compared to software loops:
- **Accurate**: Not affected by code optimization or interrupts
- **Consistent**: Always takes exactly the specified time
- **Efficient**: CPU can do other work (though this example busy-waits)
- **Predictable**: Timing doesn't change with compiler settings

The hardware timer counts independently while your code waits, ensuring precise timing regardless of what else happens in your program.

## How `set_high()` Works Through the HAL

When you call `row1.set_high()`, it goes through several layers:

### 1. Your Code
```rust
row1.set_high().unwrap();
```

### 2. The OutputPin Trait (from embedded-hal)

The `embedded_hal::digital::OutputPin` trait defines the interface:
```rust
pub trait OutputPin {
    fn set_high(&mut self) -> Result<(), Self::Error>;
    fn set_low(&mut self) -> Result<(), Self::Error>;
}
```

This trait provides a generic interface that works across different microcontrollers. Your code doesn't need to know about nRF52833-specific details.

### 3. The HAL Implementation (nrf52833-hal)

The `nrf52833-hal` crate implements this trait for GPIO pins:
```rust
impl<MODE> OutputPin for Pin<Output<MODE>> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // Safety: We own this pin, so we have exclusive access
        unsafe { (*P0::ptr()).outset.write(|w| w.bits(1 << self.pin)) }
        Ok(())
    }
}
```

This code:
- Gets a pointer to the GPIO peripheral registers
- Writes to the OUTSET register (sets specific bits high without affecting others)
- Uses bit shifting to target the correct pin (e.g., pin 21 → bit 21)

### 4. What OUTSET Does

The OUTSET register is special:
- Writing a `1` to a bit sets that GPIO pin HIGH
- Writing a `0` has no effect (doesn't change the pin)
- This allows setting individual pins without reading the current state first

For pin 21 (row1):
- Bit mask: `1 << 21` = `0x00200000`
- Writing this to OUTSET sets pin 21 HIGH while leaving all other pins unchanged

## Assembly Code Generation

When you call `row1.set_high()`, here's roughly what the compiler generates:

```assembly
// Simplified ARM assembly for row1.set_high():
mov r0, #0x50000000    ; GPIO P0 base address
mov r1, #0x200000      ; Pin 21 bit mask (1 << 21)
str r1, [r0, #0x508]   ; Write to OUTSET register (P0_BASE + 0x508)
```

This is just 3 ARM instructions - incredibly efficient for such high-level Rust code!

## Why This Example Uses High-Level Crates

Example 01 heavily uses the HAL ecosystem, which hides the low-level details. This is intentional:

- **Focus on Learning**: You can focus on Rust concepts without getting lost in hardware registers
- **Safety**: Compile-time guarantees prevent common embedded programming mistakes
- **Productivity**: Simple functions like `set_high()` instead of complex register manipulation

## What's Next?

> **🔬 Want to See the Low-Level Details?** [Example 02](../example_02_hello_world_minimal_dependencies/) shows the same LED blinking functionality but with fewer dependencies and more direct hardware control. You'll see exactly what these high-level abstractions are doing behind the scenes!

## Additional Resources

- **[deep_dive.md](../deep_dive.md)** - Complete technical explanation of the compilation process and embedded Rust ecosystem
- **[hardware.md](../hardware.md)** - How memory mapping and address buses work
- **[nRF52833 Product Specification](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.7.pdf)** - Complete hardware reference