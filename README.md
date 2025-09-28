# Rusty Bits - BBC micro:bit v2 Rust Examples

![960px-Micro-bit_v2](https://github.com/user-attachments/assets/ce0fe5b0-dc15-4ad8-a31c-e2cbbe288afc)

A collection of embedded Rust programming examples for the BBC micro:bit v2. This project demonstrates various aspects of embedded development using the nRF52833 microcontroller and the micro:bit v2 development board.

This project is based on examples from [The Embedded Rust Book](https://docs.rust-embedded.org/book/), which provides comprehensive guidance for embedded Rust development.

## Documentation

- 📋 **[VSCODE_SETUP.md](VSCODE_SETUP.md)** - Complete VS Code configuration guide for embedded Rust development
- 🔍 **[DEEP_DIVE.md](DEEP_DIVE.md)** - Technical deep dive into the Rust-to-hardware compilation pipeline
- 📄 **[micro:bit v2 Schematic](doc/MicroBit_V2.2.1_nRF52820%20schematic.PDF)** - Official hardware schematic (PDF)

## Examples

### [Example 01: Hello World](example_01_hello_world/)
**Basic LED blinking** - The "Hello World" of embedded systems
- Board initialization and GPIO configuration  
- Hardware timer usage for delays
- Simple LED matrix control

> **💡 Note**: Each example directory contains its own README.md with detailed explanations, code walkthrough, and specific instructions for that example.


<!-- Future examples will be added here -->
<!-- ### Example 02: Button Input
**Reading button presses** - Handling user input
- GPIO input configuration
- Interrupt handling
- Button debouncing

**Run with:** `cargo run --bin example_02_buttons`
-->

## Quick Start

### Hardware Requirements
- BBC micro:bit v2 (with nRF52833 microcontroller)
- USB cable for programming

### Software Installation
Follow the installation guide from [The Embedded Rust Book - Installation](https://docs.rust-embedded.org/book/intro/install.html) for complete setup instructions.

### VS Code Extensions
Required extensions for the best development experience:

1. **rust-analyzer** (`rust-lang.rust-analyzer`)
   - Provides Rust language support and Code Lens features

2. **probe-rs-debugger** (`probe-rs.probe-rs-debugger`)  
   - Required for embedded debugging support

### Running Examples
Each example is a complete, independent Rust project:

**In VS Code (Recommended):**
1. Open an example's `src/main.rs` file in VS Code
2. Click the ▶️ **Run** button above the `#[entry]` function
3. The program builds and flashes automatically to your micro:bit

**From Command Line:**
```bash
# Navigate to an example and run it
cd example_01_hello_world
cargo run
```

> **💡 Tip**: See [VSCODE_SETUP.md](VSCODE_SETUP.md) for complete VS Code configuration and debugging setup.

## Project Architecture

This project uses **independent examples** rather than a Cargo workspace - each example can be copied and used standalone.

## Learning Resources

- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [micro:bit v2 Documentation](https://tech.microbit.org/hardware/)
- [nRF52833 Product Specification](https://infocenter.nordicsemi.com/topic/ps_nrf52833/keyfeatures_html5.html)
- [Rust Embedded HAL Documentation](https://docs.rs/embedded-hal/)
