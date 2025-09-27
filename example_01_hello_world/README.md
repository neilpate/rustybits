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

Make sure your micro:bit v2 is connected via USB.

## Code Overview

```rust
#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(board.TIMER0);

    loop {
        timer0.delay_ms(500);
        row1.set_high().unwrap();
        timer0.delay_ms(100);
        row1.set_low().unwrap();
    }
}
```

## Key Concepts

- **Board initialization** with `microbit::Board::take()`
- **GPIO configuration** for LED matrix control
- **Hardware timer** usage for delays
- **Infinite loops** in embedded systems