# Rusty MicroBit - Hello World

A simple LED blinking example for the BBC micro:bit v2 written in Rust. This project demonstrates basic embedded Rust programming using the nRF52833 microcontroller and the micro:bit v2 development board.

## What This Project Does

This is a "Hello World" equivalent for embedded systems - it blinks an LED on the micro:bit's LED matrix. The program:

1. Initializes the micro:bit board
2. Configures row 1 and column 1 of the LED matrix
3. Creates a timer for delays
4. Continuously blinks the LED at the intersection of row 1 and column 1

## Hardware Requirements

- BBC micro:bit v2 (with nRF52833 microcontroller)
- USB cable for programming and power
- Computer with USB port

## Software Requirements

- Rust toolchain (1.83.0 or later recommended)
- `thumbv7em-none-eabihf` target for ARM Cortex-M4
- `probe-rs` for flashing and debugging

## Project Structure

### Cargo.toml - Project Configuration

```toml
[package]
authors = ["Bart Massey"]
edition = "2021"
name = "hello-world"
version = "0.1.0"

[[bin]]
name = "hello-world"
test = false
bench = false

[dependencies]
cortex-m-rt = "0.7.5"
embedded-hal = "1.0.0"
microbit-v2 = "0.15.1"
nrf52833-hal = "0.18.0"
panic-halt = "1.0.0"

[dependencies.cortex-m]
version = "0.7.7"
features = ["inline-asm"]
```

**Key Configuration Details:**

- **`[[bin]]` section**: Disables tests and benchmarks (`test = false`, `bench = false`) which are incompatible with `no_std` embedded environments
- **`cortex-m-rt`**: Runtime crate providing the entry point and memory layout for ARM Cortex-M processors
- **`embedded-hal`**: Hardware abstraction layer traits for embedded systems (timers, GPIO, etc.)
- **`microbit-v2`**: Board support package specifically for micro:bit v2, includes pin definitions and board initialization
- **`nrf52833-hal`**: Hardware abstraction layer for the Nordic nRF52833 System-on-Chip
- **`panic-halt`**: Simple panic handler that halts execution on panic (required for `no_std`)
- **`cortex-m`**: Core ARM Cortex-M functionality with inline assembly features enabled

### .cargo/config.toml - Build Configuration

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52833_xxAA"
rustflags = ["-C", "linker=rust-lld", "-C", "link-arg=-Tlink.x"]
```

**Configuration Explanation:**

- **`target`**: Specifies the ARM Cortex-M4F architecture target (thumbv7em = ARMv7E-M with DSP extensions, none = no OS, eabihf = embedded ABI with hardware float)
- **`runner`**: Defines how to execute the binary - uses `probe-rs` to flash and run on the nRF52833 chip
- **`linker=rust-lld`**: Uses LLVM's linker (rust-lld) instead of the system linker
- **`link-arg=-Tlink.x`**: Includes the `link.x` linker script from `cortex-m-rt` for proper memory layout

### Embed.toml - Probe Configuration

```toml
[default.general]
chip = "nrf52833_xxAA"

[default.reset]
halt_afterwards = false

[default.rtt]
enabled = false

[default.gdb]
enabled = false
```

**Probe-rs Configuration:**

- **`chip`**: Specifies the exact chip variant (nRF52833 with 512KB flash)
- **`halt_afterwards`**: Allows the program to run immediately after flashing
- **`rtt`**: Real-Time Transfer debugging disabled (not needed for simple LED blink)
- **`gdb`**: GDB debugging interface disabled (not needed for this simple example)

### rustfmt.toml - Code Formatting

```toml
max_width = 120
tab_spaces = 4
newline_style = "Windows"
use_small_heuristics = "Default"
```

**Formatting Configuration:**

- **`max_width`**: Lines wrap at 120 characters (extended from default 100 for better readability)
- **`tab_spaces`**: Uses 4 spaces for indentation
- **`newline_style`**: Windows line endings (CRLF) for consistency with development environment
- **`use_small_heuristics`**: Default formatting decisions for small code constructs

## Building and Running

### Prerequisites

1. Install Rust and the embedded target:
   ```bash
   rustup target add thumbv7em-none-eabihf
   ```

2. Install probe-rs for flashing:
   ```bash
   cargo install probe-rs --features cli
   ```

### Build Commands

```bash
# Build the project
cargo build

# Build and run on connected micro:bit
cargo run

# Format code
cargo fmt

# Check for compilation errors without building
cargo check
```

## Code Explanation

The main application (`src/main.rs`) demonstrates:

- **`#![no_main]`** and **`#![no_std]`**: Disables standard library and main function (replaced by custom entry point)
- **`#[entry]`**: Marks the custom entry point function (from `cortex-m-rt`)
- **Board initialization**: Takes ownership of the micro:bit hardware
- **GPIO configuration**: Sets up LED matrix pins (row 1 as output high, col 1 as output low)
- **Timer usage**: Creates delays using the hardware timer
- **Infinite loop**: Continuously blinks the LED with 500ms on, 100ms off pattern

## Memory Layout

The project uses auto-generated memory layout from the `microbit-v2` crate, which provides:
- Flash memory mapping for the nRF52833
- RAM allocation compatible with the micro:bit v2
- Stack and heap configuration for the Cortex-M4 processor

## Troubleshooting

### Common Issues

1. **"No loadable segments"**: Ensure all TOML files are properly configured and `cargo clean` before rebuilding
2. **"Can't find crate for test"**: Make sure `test = false` is set in the `[[bin]]` section of `Cargo.toml`
3. **Rust version conflicts**: Update Rust with `rustup update` to ensure compatibility with all dependencies
4. **Probe connection issues**: Check USB connection and try `probe-rs list` to detect connected devices

### Verification Commands

```bash
# Check Rust version
rustc --version

# List available targets
rustup target list --installed

# Check connected probes
probe-rs list

# Verify project builds
cargo check
```

## Learning Resources

- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [micro:bit v2 Documentation](https://tech.microbit.org/hardware/)
- [nRF52833 Product Specification](https://infocenter.nordicsemi.com/topic/ps_nrf52833/keyfeatures_html5.html)
- [Rust Embedded HAL Documentation](https://docs.rs/embedded-hal/)

## License

This project is intended for educational purposes and follows standard embedded Rust practices for the micro:bit platform.