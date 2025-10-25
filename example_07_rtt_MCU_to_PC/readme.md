# Example 07 - RTT (Real-Time Transfer) Debug Output

Demonstrates real-time debug logging from the micro:bit to your development PC using RTT (Real-Time Transfer), a high-speed debug communication protocol.

## What it does

This program prints timestamped debug messages to your PC terminal at 1-second intervals. RTT provides printf-style debugging without requiring any GPIO pins or UART configuration.

**Key Features:**
- Real-time debug output via the debug probe (no UART pins needed)
- High-speed communication (~1 MB/s vs. ~14 KB/s for UART)
- Zero-overhead logging suitable for timing-critical code
- Automatic timestamping by the probe-rs host tool

## Running this example

### Important: Using `cargo embed` for RTT

**Unlike previous examples**, this example requires `cargo embed` instead of `cargo run`.

**Why `cargo embed`?**
- `cargo run` (via probe-rs) only flashes the binary and exits
- `cargo embed` flashes the binary **and** opens an interactive RTT terminal
- The RTT terminal stays connected, displaying output in real-time

### Quick Start

1. Connect your micro:bit via USB
2. Open a terminal and run:
```bash
cd example_07_rtt_MCU_to_PC
cargo embed
```

3. The terminal will display timestamped output from your micro:bit:
```
21:27:10.723: RTT Example Started!
21:27:11.723: Count: 0
21:27:12.723: Count: 1
21:27:13.723: Count: 2
...
```

Press `Ctrl+C` to exit the RTT terminal.

## The Code

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::hal::timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer0 = timer::Timer::new(board.TIMER0);

    let mut loop_count: u32 = 0;

    rprintln!("RTT Example Started!");

    loop {
        timer0.delay_ms(1000);
        rprintln!("Count: {}", loop_count);
        loop_count += 1;
    }
}
```

## How It Works

### RTT Overview

RTT (Real-Time Transfer) is a high-performance debug communication protocol developed by SEGGER. It leverages the debug probe connection (SWD) already present on the micro:bit, enabling bidirectional communication without consuming any GPIO pins.

**Architecture:**
```
Microcontroller (nRF52833)    Debug Probe (nRF52820)    Development PC
┌──────────────────────┐      ┌──────────────────┐      ┌────────────┐
│  RTT Buffer in RAM   │◄────►│  SWD Interface   │◄────►│  probe-rs  │
│  (1KB circular)      │      │                  │      │            │
└──────────────────────┘      └──────────────────┘      └────────────┘
```

### Code Breakdown

#### Initialization
```rust
rtt_init_print!();
```
- Allocates a 1KB circular buffer in RAM
- Sets up the RTT control block at a known memory location
- Configures RTT channel 0 for output
- Enables the `rprintln!()` macro for formatted printing

#### Printing Debug Messages
```rust
rprintln!("Count: {}", loop_count);
```
- Formats the string using standard Rust formatting syntax
- Writes to the RTT buffer in RAM (non-blocking, microseconds)
- probe-rs polls the buffer and displays output on the PC terminal
- Timestamps are added by probe-rs, not by the microcontroller

### Output Format

The timestamps you see are added by probe-rs on your development PC:

```
21:27:11.723: Count: 1
^^^^^^^^^^^^^  ← Added by probe-rs (host timestamp)
              Count: 1  ← Your message from the microcontroller
```

This design is efficient because:
- The microcontroller doesn't waste cycles formatting timestamps
- No Real-Time Clock (RTC) required on the device
- Accurate timing reference from the development PC

## Advanced Usage

### Formatted Output

RTT supports all standard Rust formatting options:

```rust
let value = 42;
let voltage = 3.3;

// Basic formatting:
rprintln!("Value: {}", value);

// Multiple variables:
rprintln!("Value: {}, Voltage: {}V", value, voltage);

// Number formatting:
rprintln!("Hex: 0x{:04X}", value);        // Hex: 0x002A
rprintln!("Binary: {:08b}", value);       // Binary: 00101010
rprintln!("Float: {:.2}", voltage);       // Float: 3.30

// Debug formatting:
let array = [1, 2, 3];
rprintln!("Array: {:?}", array);          // Array: [1, 2, 3]
```

### Performance Considerations

RTT is designed for minimal impact on timing-critical code:

- **RAM Usage**: ~1KB buffer + ~100 bytes control structure
- **Flash Usage**: ~2-3KB for RTT implementation
- **CPU Overhead**: Single buffer write (microseconds)
- **Non-blocking**: If buffer is full, messages are dropped (configurable)

## Additional Resources

- **[RTT Target Documentation](https://docs.rs/rtt-target/)** - Complete API reference
- **[probe-rs Documentation](https://probe.rs/)** - probe-rs tool and debugging guide