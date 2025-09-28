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

> **💡 For detailed VS Code setup and visual guides, see [VSCODE_SETUP.md](../VSCODE_SETUP.md)**

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

## Key Files

- **`src/main.rs`** - Your Rust code that blinks the LED
- **`.cargo/config.toml`** - Build configuration (ARM target, probe-rs runner)  
- **`Cargo.toml`** - Dependencies and project metadata
- **`Embed.toml`** - probe-rs flashing configuration

> **💡 Tip**: This example is completely self-contained. You can copy this entire directory and use it as a starting point for your own micro:bit projects!

## Want to Learn More?

- **[DEEP_DIVE.md](../DEEP_DIVE.md)** - Technical explanation of how Rust becomes running hardware code
- **[VSCODE_SETUP.md](../VSCODE_SETUP.md)** - Complete VS Code configuration guide