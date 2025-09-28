# Rusty Bits - BBC micro:bit v2 Rust Examples

![960px-Micro-bit_v2](https://github.com/user-attachments/assets/ce0fe5b0-dc15-4ad8-a31c-e2cbbe288afc)

A collection of embedded Rust programming examples for the BBC micro:bit v2. This project demonstrates various aspects of embedded development using the nRF52833 microcontroller and the micro:bit v2 development board.

This project is based on examples from [The Embedded Rust Book](https://docs.rust-embedded.org/book/), which provides comprehensive guidance for embedded Rust development.

## Documentation

- 📋 **[vscode_setup.md](vscode_setup.md)** - Complete VS Code configuration guide for embedded Rust development
- 🔍 **[deep_dive.md](deep_dive.md)** - Technical deep dive into the Rust-to-hardware compilation pipeline
- ⚡ **[hardware.md](hardware.md)** - How memory mapping physically works: address bus, Flash, RAM, and peripherals
- 📄 **[micro:bit v2 Schematic](doc/MicroBit_V2.2.1_nRF52820%20schematic.PDF)** - Official hardware schematic (PDF)

## Examples

This project follows a **progressive learning journey** from high-level convenience to bare-metal understanding. Each example strips away more abstraction layers to show you exactly what's happening under the hood.

This project uses **independent examples** rather than a Cargo workspace - each example can be copied and used standalone.

### [Example 01: Hello World](example_01_hello_world/)
**🎯 High-Level HAL Approach** - "I want to blink an LED easily"
- Board initialization using convenient HAL crates
- Hardware timer usage with high-level APIs
- Simple LED matrix control
- **5 dependencies** - Maximum convenience and safety
- **Best for**: Getting started quickly with embedded Rust

### [Example 02: Hello World (Minimal Dependencies)](example_02_hello_world_minimal_dependencies/)
**🔧 Direct Register Access** - "How do GPIO registers actually work?"
- Direct hardware register manipulation
- Reduced dependencies while maintaining essential functionality
- **3 dependencies** - Balance of control and convenience
- **Best for**: Understanding hardware interfaces and register-level programming

### [Example 03: Hello World (Zero Dependencies)](example_03_hello_world_no_dependencies/)
**⚡ Bare Metal Implementation** - "How does the ENTIRE system work?"
- **🎉 ZERO dependencies** - Everything implemented from scratch
- Custom ARM Cortex-M vector table and reset handler
- Hand-crafted linker script and memory initialization
- Direct assembly integration and complete system control
- **Best for**: Deep understanding of embedded systems architecture

> **🎓 Educational Philosophy**: Example 03 represents the **lowest level possible** understanding of embedded systems. While you wouldn't write production code this way (Example 01's approach is much more practical), seeing how everything works at the bare metal level gives you invaluable insight into what's actually happening when you use higher-level abstractions.
>
> Think of it as "embedded systems archaeology" - digging down through all the layers to understand the foundation that everything else is built upon!

## The Learning Journey

| Abstraction Level | Example 01 | Example 02 | Example 03 |
|-------------------|------------|------------|------------|
| **Dependencies** | 5 crates | 3 crates | **0 crates** 🎉 |
| **Code Style** | `led.set_high()?` | `gpio.out.set(1 << 4)` | `ptr::write_volatile(0x50000508, 1 << 4)` |
| **Startup** | Automatic | Automatic | **Manual reset handler** |
| **Memory Init** | Hidden | Hidden | **Explicit RAM setup** |
| **Vector Table** | Generated | Generated | **Hand-crafted** |
| **When to Use** | Production code | Learning registers | **Understanding systems** |

Each example builds the same functionality (blinking LED) but reveals progressively more of the underlying machinery. By the end, you'll understand embedded systems from the hardware reset vector all the way up to your application code!

> **💡 Note**: Each example directory contains its own readme.md with detailed explanations, code walkthrough, and specific instructions for that example.

## Quick Start

### Hardware Requirements
- BBC micro:bit v2 (with nRF52833 microcontroller)

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

> **💡 Tip**: See [vscode_setup.md](vscode_setup.md) for complete VS Code configuration and debugging setup.

## Additional Resources

- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [micro:bit v2 Documentation](https://tech.microbit.org/hardware/)
- [nRF52833 Product Specification](https://infocenter.nordicsemi.com/topic/ps_nrf52833/keyfeatures_html5.html)
- [Rust Embedded HAL Documentation](https://docs.rs/embedded-hal/)
