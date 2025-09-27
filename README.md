# Rusty Bits - BBC micro:bit v2 Rust Examples

![960px-Micro-bit_v2](https://github.com/user-attachments/assets/ce0fe5b0-dc15-4ad8-a31c-e2cbbe288afc)

A collection of embedded Rust programming examples for the BBC micro:bit v2. This project demonstrates various aspects of embedded development using the nRF52833 microcontroller and the micro:bit v2 development board.

This project is based on examples from [The Embedded Rust Book](https://docs.rust-embedded.org/book/), which provides comprehensive guidance for embedded Rust development.

## Examples

### [Example 01: Hello World](example_01_hello_world/)
**Basic LED blinking** - The "Hello World" of embedded systems
- Board initialization and GPIO configuration  
- Hardware timer usage for delays
- Simple LED matrix control

**Run with:** `cargo run --bin example_01_hello_world`

<!-- Future examples will be added here -->
<!-- ### Example 02: Button Input
**Reading button presses** - Handling user input
- GPIO input configuration
- Interrupt handling
- Button debouncing

**Run with:** `cargo run --bin example_02_buttons`
-->

## Hardware Requirements

- BBC micro:bit v2 (with nRF52833 microcontroller)

## Software Installation Guide
Follow the installation guide from [The Embedded Rust Book - Installation](https://docs.rust-embedded.org/book/intro/install.html) for complete setup instructions.

## Project Structure

## Running Examples

Each example is a separate binary in the same Cargo project:

```bash
# Run a specific example
cargo run --bin example_01_hello_world

# List all available examples  
cargo run --bin <TAB>

# Build all examples
cargo build
```

Make sure your micro:bit v2 is connected via USB before running examples.

## Project Configuration

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


## Memory Layout

The project uses auto-generated memory layout from the `microbit-v2` crate, which provides:
- Flash memory mapping for the nRF52833
- RAM allocation compatible with the micro:bit v2
- Stack and heap configuration for the Cortex-M4 processor

## Learning Resources

- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [micro:bit v2 Documentation](https://tech.microbit.org/hardware/)
- [nRF52833 Product Specification](https://infocenter.nordicsemi.com/topic/ps_nrf52833/keyfeatures_html5.html)
- [Rust Embedded HAL Documentation](https://docs.rs/embedded-hal/)
