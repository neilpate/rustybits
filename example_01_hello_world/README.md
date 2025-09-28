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

> **💡 VS Code Tip**: When viewing the source code, you'll see a small ▶️ "Run" arrow above the `#[entry]` function. This is provided by rust-analyzer and lets you run the example with a single click!

<img width="1660" height="773" alt="VS Code Run button in source code" src="https://github.com/user-attachments/assets/744fbe24-fdd4-4cfb-af37-8be0536d5d28" />

#### Debugging in VS Code

You can also debug this example directly on the micro:bit hardware. Start the session using the preconfigured launch command (or just press F5).

<img width="2257" height="1084" alt="VS Code debugging session" src="https://github.com/user-attachments/assets/84128dfc-99a1-4703-adae-b770a1a1c9fa" />

This command is set up to halt the CPU on load. You can resume as usual by pressing the run arrow at the top.

If you don't like this behaviour you can disable this automatic halting. Then use breakpoints as usual to pause execution.

<img width="1633" height="800" alt="VS Code breakpoint debugging" src="https://github.com/user-attachments/assets/25fded61-5eb7-4921-a8c8-90032c17cc9f" />

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

## Key Concepts

### Embedded Rust Attributes

- **`#![no_main]`** - Tells Rust not to use the standard `main()` function. In embedded systems, we need custom startup code and memory layout, so we bypass Rust's default runtime.
- **`#![no_std]`** - Disables the standard library since it assumes an operating system with heap allocation, file systems, etc. We use only the core library which works in bare metal environments.

### Import Breakdown

- **`use cortex_m_rt::entry;`** - Provides the `#[entry]` macro for defining our custom entry point. This crate handles ARM Cortex-M startup code and vector tables.
- **`use embedded_hal::{delay::DelayNs, digital::OutputPin};`** - Hardware Abstraction Layer (HAL) traits that define common embedded interfaces. `DelayNs` provides timing functions, `OutputPin` provides GPIO output operations.
- **`use microbit::hal::{gpio, timer};`** - micro:bit specific implementations of GPIO pins and hardware timers, built on top of the nRF52833 chip drivers.
- **`use panic_halt as _;`** - Panic handler for `no_std` environments. When a panic occurs, this handler simply halts execution (enters infinite loop) rather than unwinding the stack.

### Entry Point

- **`#[entry]`** - This macro marks our `main()` function as the program entry point. It generates the necessary startup code, sets up the stack pointer, initializes RAM, and calls our function. The function signature `fn main() -> !` indicates it never returns (infinite loop).

### Code Explanation

1. **Board Initialization**: `microbit::Board::take().unwrap()` - Singleton pattern that gives us exclusive access to the micro:bit's hardware peripherals. Can only be called once.

2. **LED Matrix Setup**: The micro:bit's 5x5 LED display uses a matrix scanning approach:
   - `row1.into_push_pull_output(gpio::Level::High)` - Configure row 1 as output, initially high
   - `col1.into_push_pull_output(gpio::Level::Low)` - Configure column 1 as output, initially low
   - An LED lights up when its row is HIGH and column is LOW

3. **Hardware Timer**: `timer::Timer::new(board.TIMER0)` - Creates a blocking delay timer using the nRF52833's hardware timer peripheral. More accurate than software delays.

4. **Main Loop**: Embedded systems typically run forever, so we use an infinite loop:
   - `timer0.delay_ms(100)` - Precise 100ms delay using hardware timer
   - `row1.set_high()/.set_low()` - Toggle the LED state to create blinking effect

## Configuration Files

### `.cargo/config.toml`
```toml
[build]
target = "thumbv7em-none-eabihf"  # ARM Cortex-M4 with hardware floating point

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52833_xxAA"  # Flash and run on micro:bit
rustflags = ["-C", "linker=rust-lld", "-C", "link-arg=-Tlink.x"]
```

### `Embed.toml`
```toml
[default.general]
chip = "nrf52833_xxAA"    # micro:bit v2 chip

[default.reset]
halt_afterwards = false   # Continue running after flash

[default.rtt]
enabled = false          # Real-Time Transfer (for debugging output)

[default.gdb]
enabled = false          # GDB debugging interface
```

## Dependencies

This example uses several key embedded Rust crates:

- **`microbit-v2 = "0.13.0"`** - Board Support Package for micro:bit v2
- **`cortex-m-rt = "0.7.3"`** - Runtime and startup code for ARM Cortex-M
- **`embedded-hal = "0.2.7"`** - Hardware Abstraction Layer traits
- **`panic-halt = "0.2.0"`** - Simple panic handler for embedded systems

The `microbit-v2` crate provides high-level access to micro:bit hardware while the other crates provide the embedded Rust foundation.