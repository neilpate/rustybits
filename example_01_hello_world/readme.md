# Example 01 - Hello World

The classic "Hello World" for embedded systems - blink an LED!

## What it does

This program blinks one LED on your micro:bit's 5×5 LED display. The LED turns on for 100ms, then off for 100ms, and repeats forever.

## Running this example

### Quick Start
1. Connect your micro:bit via USB
2. Open a terminal and run:
```bash
cd example_01_hello_world
cargo run
```

### From VS Code
1. Open `src/main.rs` in VS Code
2. Click the ▶️ **Run** button above the `#[entry]` function

> **💡 Need VS Code setup help?** See [vscode_setup.md](../vscode_setup.md)

## The Code

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

## How it works

1. **Get the board**: `microbit::Board::take()` gives us access to the micro:bit hardware
2. **Set up the LED**: Configure row 1 and column 1 of the LED matrix
3. **Create a timer**: Use the hardware timer for precise delays  
4. **Blink loop**: Turn LED on, wait, turn off, wait, repeat

> **🦀 New to embedded Rust?** Check out the **[Embedded Rust Primer](../embedded_rust_primer.md)** to understand `#![no_std]`, `#[entry]`, and other embedded essentials!

## Next Steps

- Try changing the delay times to make it blink faster or slower
- Want to understand what's happening under the hood? See [technical_details.md](technical_details.md)
- This example uses high-level crates that hide the hardware details. Ready to see what's happening at a lower level? Check out [Example 02](../example_02_hello_world_minimal_dependencies/) which shows the same LED blinking with fewer dependencies and more direct hardware control

## Additional Resources

- **[Embedded Rust Primer](../embedded_rust_primer.md)** - Essential embedded Rust concepts  
- **[VS Code Setup Guide](../vscode_setup.md)** - Complete development environment setup