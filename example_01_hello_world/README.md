# Example 01 - Hello World

A simple LED blinking example that blinks one LED on the micro:bit's LED matrix.

## What it does

This is the "Hello World" for embedded systems - it blinks an LED! The program:

1. Initializes the micro:bit board
2. Configures row 1 and column 1 of the LED matrix 
3. Continuously blinks the LED at the intersection (500ms on, 100ms off)

## Running this example

```bash
cd example_01_hello_world
cargo run
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