# Example 05 - Button-Controlled LED

An interactive GPIO input demonstration that toggles an LED in response to button presses using polled input with edge detection.

## What it does

This example demonstrates fundamental embedded input processing techniques. The program:

1. Initializes the micro:bit board and GPIO peripherals
2. Configures row 1 and column 1 of the LED matrix for output control
3. Configures button A as a floating input with pull-up resistor
4. Implements polled input with edge detection and software debouncing
5. Toggles LED state on each button press (press to turn on, press again to turn off)

## Running this example

This example is completely self-contained - it includes all necessary configuration files:

### From Command Line
```bash
cd example_05_buttons_polled
cargo run
```

### From VS Code
1. Open `src/main.rs` in VS Code
2. Click the ▶️ **Run** button above the `#[entry]` function

## Code Overview

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::gpio;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    // Configure LED (top-left LED in 5x5 matrix)
    let mut row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(gpio::Level::Low);

    // Configure button A with pull-up resistor
    // Note: `mut` is required even for input pins because GPIO read operations
    // like `is_low()` require mutable access to the underlying hardware registers
    let mut button_a = board.buttons.button_a.into_floating_input();

    // Track LED state and button state for edge detection
    let mut led_on = false;
    let mut button_was_pressed = false;

    loop {
        // Read button state (button is active low - pressed = false)
        let button_pressed = button_a.is_low().unwrap();

        // Detect button press (transition from not pressed to pressed)
        if button_pressed && !button_was_pressed {
            // Toggle LED state
            led_on = !led_on;

            if led_on {
                row1.set_low().unwrap(); // Turn LED on (row low, col already low)
            } else {
                row1.set_high().unwrap(); // Turn LED off (row high)
            }
        }

        // Update button state for next iteration
        button_was_pressed = button_pressed;
    }
}
```

## How It Works

This example demonstrates advanced embedded input processing techniques:

1. **Board Initialization**: `microbit::Board::take().unwrap()` - Gets exclusive access to the micro:bit hardware
2. **LED Setup**: Configures row 1 and column 1 of the LED matrix - an LED lights when its row is LOW and column is LOW
3. **Button Configuration**: Sets up button A as a floating input with internal pull-up resistor
4. **Polling Loop**: Continuously reads button state and implements edge detection
5. **State Management**: Tracks both LED state and previous button state for reliable toggle behavior

### GPIO Input Processing Concepts

**Active Low Logic**: The micro:bit buttons are wired as active low - when pressed, they connect to ground (logic 0), when released, the pull-up resistor pulls the pin high (logic 1).

**Polling vs Interrupts**: This example uses polling (continuously checking the button state in a loop) rather than interrupts. While less power-efficient, polling provides deterministic timing and simpler code structure for educational purposes.

**Mutability Requirements**: Even input operations like `is_low()` require mutable access because GPIO reads may involve hardware register access, interrupt flag clearing, or internal state updates that the borrow checker considers mutable operations.