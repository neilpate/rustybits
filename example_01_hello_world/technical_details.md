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
- **Hardware Detection**: Verifies the nRF52833 chip is present and working
- **Pin Mapping**: Sets up all GPIO pins with their micro:bit functions (already configured as push-pull outputs)
- **Clock Setup**: Configures the chip's internal clocks for proper operation
- **Safety**: Creates a singleton - only one part of your code can control the hardware

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

## Assembly Code Generation

Your high-level Rust code compiles to efficient ARM assembly:

```rust
row1.set_high();  // Becomes just 2-3 ARM instructions:
```

```assembly
mov r0, #0x50000000    ; GPIO P0 base address
mov r1, #0x200000      ; Pin 21 bit mask (1 << 21)  
str r1, [r0, #0x508]   ; Write to OUTSET register
```

This is incredibly efficient for such high-level, safe Rust code!

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