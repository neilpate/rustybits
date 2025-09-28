# Example 01 - Hello World

A simple LED blinking example that blinks one LED on the micro:bit's LED matrix.

## What it does

This is the "Hello World" for embedded systems - it blinks an LED! The program:

1. Initializes the micro:bit board
2. Configures row 1 and column 1 of the LED matrix 
3. Continuously blinks the LED at the intersection (500ms on, 100ms off)

## Running this example

This example is completely self-contained - it includes all necessary configuration files:

### From Command Line
```bash
cd example_01_hello_world
cargo run
```

### From VS Code
1. Open `src/main.rs` in VS Code
2. Click the ▶️ **Run** button above the `#[entry]` function
3. Or use `Ctrl+Shift+P` → "Tasks: Run Task" → "Run Example 01"

> **💡 For detailed VS Code setup and visual guides, see [vscode_setup.md](../vscode_setup.md)**

### What Happens
- **Builds** the project for ARM Cortex-M4 (`thumbv7em-none-eabihf` target)
- **Flashes** the binary to your micro:bit via probe-rs  
- **Runs** the program on the micro:bit hardware
- **LED blinks** at row 1, column 1 position

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
## Code Overview

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::{gpio, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(board.TIMER0);

    loop {
        timer0.delay_ms(100);
        row1.set_high().unwrap();
        timer0.delay_ms(100);
        row1.set_low().unwrap();
    }
}
```

## How It Works

This example demonstrates the basics of micro:bit LED control:

1. **Board Initialization**: `microbit::Board::take().unwrap()` - Gets exclusive access to the micro:bit hardware
2. **LED Setup**: Configures row 1 and column 1 of the LED matrix - an LED lights when its row is HIGH and column is LOW
3. **Timer**: Uses the hardware timer for precise delays
4. **Blink Loop**: Toggles the LED every 100ms to create a blinking effect

### The micro:bit LED Matrix
The micro:bit's 5x5 LED display works as a matrix - each LED is controlled by setting its row HIGH and column LOW. This example controls just one LED at position (1,1).

### What the Crates Are Doing Behind the Scenes

While this code looks simple, here's the logical sequence of complex work the Rust embedded ecosystem handles for you:

#### 1. **Compilation Magic** (What Happens When You Build):

The crates do enormous work during compilation to transform your Rust code into efficient embedded machine code:

**Memory Layout Generation** (`cortex-m-rt`):
- Automatically generates `memory.x` linker script with nRF52833's exact flash/RAM layout
- Creates the ARM Cortex-M interrupt vector table with your reset handler
- Sets up proper memory sections (.text, .data, .bss, .rodata) for embedded systems

**Hardware Abstraction Compilation** (`microbit-v2`, `nrf52833-hal`):
- Your high-level `set_high()` calls compile to single ARM assembly instructions like `str r1, [r0]` (store register to memory)
- GPIO pin abstractions become direct register addresses (e.g., `0x50000000` for GPIO peripheral)
- Type safety checks happen at compile time - impossible to accidentally use wrong pin types

**Cross-Compilation Coordination**:
- Configures the Rust compiler for `thumbv7em-none-eabihf` (ARM Cortex-M4F with hardware float)
- Links against newlib-nano for minimal C library functions
- Strips out all standard library dependencies that assume an operating system
- Optimizes for code size (embedded systems have limited flash memory)

**Link-Time Optimization**:
- Dead code elimination removes unused functions (even from the HAL crates)
- Inlines small functions for maximum performance
- Resolves all hardware register addresses to compile-time constants
- Creates a single, self-contained binary with no external dependencies

#### 2. **Before Your Code Even Runs** (cortex-m-rt crate):
- Sets up the ARM Cortex-M4 processor after power on
- Initializes the stack pointer and memory layout
- Copies initial data from flash to RAM
- Zeros out uninitialized memory sections
- Calls your `main()` function with everything ready

#### 2. **Board Initialization** (`microbit::Board::take()`):
- **Clock System**: Configures the nRF52833's complex clock tree (high-frequency, low-frequency, and peripheral clocks)
- **Power Management**: Sets up voltage regulators and power domains
- **GPIO Configuration**: Maps the physical pins to their micro:bit functions (LED matrix, buttons, etc.)
- **Hardware Verification**: Ensures the chip is the expected nRF52833 variant
- **Resource Management**: Creates a singleton to prevent multiple access to hardware

#### 3. **Pin Configuration** (`into_push_pull_output()`):
- **Multiplexer Setup**: Configures the pin to be a GPIO (not UART, SPI, etc.)
- **Direction Setting**: Programs the GPIO direction register for output
- **Drive Strength**: Sets electrical characteristics (how much current the pin can supply)
- **Initial State**: Sets the pin to the specified starting voltage level
- **Safety Checks**: Ensures the pin isn't already in use elsewhere

#### 4. **Timer Initialization** (`Timer::new(board.TIMER0)`):
- **Hardware Allocation**: Claims one of the nRF52833's dedicated timer peripherals
- **Clock Configuration**: Sets up the timer's clock source and prescaler for precise timing
- **Register Setup**: Configures the 32-bit timer registers for delay functionality
- **Mode Selection**: Programs the timer for blocking delay operation

#### 5. **Runtime Operations** (Every `set_high()`, `delay_ms()` call):
- **GPIO Control**: Direct register writes to toggle pin voltage instantly
- **Timer Operations**: Hardware-based delays with microsecond precision
- **No OS Overhead**: Direct hardware access with zero operating system latency

**Without these crates**, you'd need to:
- Read the 400+ page nRF52833 reference manual
- Write hundreds of lines of register manipulation code  
- Debug timing issues and hardware conflicts with expensive equipment
- Handle ARM assembly startup code and linker scripts manually

**The Result**: Your simple `board.display_pins.row1.set_high()` compiles to just 2-3 ARM assembly instructions that directly flip hardware bits, but you get to write safe, high-level Rust code that's checked at compile time!

> **🔬 Want to See the Low-Level Details?** Check out [Example 02](../example_02_hello_world_minimal_dependencies/) which shows the same LED blinking functionality but using as few dependencies as possible and direct register manipulation. You'll see exactly what this high-level code is doing behind the scenes!

## Key Files

- **`src/main.rs`** - Your Rust code that blinks the LED
- **`.cargo/config.toml`** - Build configuration (ARM target, probe-rs runner)  
- **`Cargo.toml`** - Dependencies and project metadata
- **`Embed.toml`** - probe-rs flashing configuration

> **💡 Tip**: This example is completely self-contained. You can copy this entire directory and use it as a starting point for your own micro:bit projects!

## Want to Learn More?

- **[deep_dive.md](../deep_dive.md)** - Technical explanation of how Rust becomes running hardware code
- **[vscode_setup.md](../vscode_setup.md)** - Complete VS Code configuration guide