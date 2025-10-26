# Rusty Bits - BBC micro:bit v2 Rust Examples

![960px-Micro-bit_v2](https://github.com/user-attachments/assets/ce0fe5b0-dc15-4ad8-a31c-e2cbbe288afc)

A collection of embedded Rust programming examples for the BBC micro:bit v2. This project demonstrates various aspects of embedded development using the nRF52833 microcontroller and the micro:bit v2 development board.

This project took inspiration and the initial Hello World example from [The Embedded Rust Book](https://docs.rust-embedded.org/book/) and has evolved to include progressively advanced embedded programming concepts and techniques. The Embedded Rust Book is an excellent reference and readers are encouraged to refer to it alongside these examples.

> **Note on the name**: This repository shares its name with the excellent [The Rusty Bits YouTube channel](https://www.youtube.com/@therustybits/videos) by pure coincidence. They had the name first! The channel provides comprehensive embedded Rust programming tutorials and is definitely worth checking out, though it is unaffiliated with this project.

## Documentation

- 📋 **[vscode_setup.md](vscode_setup.md)** - Complete VS Code configuration guide for embedded Rust development
- 🦀 **[embedded_rust_primer.md](embedded_rust_primer.md)** - Beginner's guide to embedded Rust: `#![no_std]`, `#[entry]`, HAL patterns, and memory management
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
- **Zero dependencies** - Everything implemented from scratch
- Custom ARM Cortex-M vector table and reset handler
- Hand-crafted linker script and memory initialization
- Direct assembly integration and complete system control
- **Best for**: Deep understanding of embedded systems architecture

### [Example 04: Hello World (Pure ARM Assembly)](example_03_hello_world_asm/)
**🔥 Advanced Bare-Metal Implementation** - "Complete hardware control"
- **Pure ARM Thumb assembly** implementation with minimal Rust scaffolding
- Hardcoded memory addresses and stack pointer configuration
- 8-byte minimal vector table with no runtime initialization
- **Best for**: Silicon-level understanding and maximum performance optimization

### [Example 05: Button-Controlled LED (Polling)](example_05_buttons_polled/)
**🎮 Interactive Input Processing** - "How do I handle user input with polling?"
- Button polling with edge detection and software debouncing
- GPIO input configuration with pull-up resistors
- State management for toggle functionality
- Active-low input logic and mutability requirements
- **Best for**: Learning basic input processing techniques and interactive embedded applications

### [Example 06: Button-Controlled LED (Interrupts)](example_06_buttons_interrupts/)
**⚡ Interrupt-Driven Processing** - "How do I use hardware interrupts for efficient input handling?"
- GPIOTE peripheral configuration for hardware interrupt generation
- Atomic state management between interrupt handler and main loop
- Power-efficient operation with Wait-For-Interrupt (WFI) instruction
- Direct register access and minimal interrupt service routines
- **Best for**: Learning interrupt-driven architecture and power-efficient embedded design

### [Example 07: RTT Debug Output](example_07_rtt_MCU_to_PC/)
**🔍 Real-Time Debug Logging** - "How do I debug embedded applications without UART?"
- RTT (Real-Time Transfer) for high-speed debug output
- Printf-style debugging via the debug probe connection
- Zero GPIO pins required, no UART configuration needed
- Automatic timestamping and formatted output support
- **Best for**: Learning modern embedded debugging techniques and real-time logging

### [Example 08: RTT Bidirectional Communication](example_08_rtt_bidirectional/)
**💬 Interactive Debug Terminal** - "How do I send commands to my embedded device?"
- Bidirectional RTT communication for interactive debugging
- Receive input from host PC and send responses
- Non-blocking I/O for real-time interactivity
- Direct channel control with UpChannel and DownChannel
- **Best for**: Building interactive debug interfaces and command interpreters

### [Example 09: Onboard Accelerometer](example_09_onboard_triax/)
**📡 I²C Sensor Communication** - "How do I read data from I²C sensors?"
- I²C communication using TWIM (Two-Wire Interface Master) peripheral with DMA
- LSM303AGR accelerometer sensor driver integration
- Reading acceleration data in milligravities across three axes
- High-level driver abstractions over low-level register access
- Multiple abstraction layers from Rust code to hardware I²C signals
- **Best for**: Learning I²C protocol implementation and sensor interfacing

> **Note**: Examples 07, 08, and 09 require `cargo embed` instead of `cargo run` to access the interactive RTT terminal.

> **Educational Philosophy**: Examples 03 and 04 represent progressively lower levels of embedded systems programming. While production code typically uses higher-level abstractions (Example 01 approach), understanding bare-metal implementation provides valuable insight into the underlying hardware behavior and system architecture.
>
> Example 04 specifically demonstrates the absolute minimum required for ARM Cortex-M execution, showing exactly what instructions run on the processor without any runtime overhead or abstraction layers.

## The Learning Journey

| Abstraction Level | Example 01 | Example 02 | Example 03 | Example 04 |
|-------------------|------------|------------|------------|------------|
| **Dependencies** | 5 crates | 3 crates | 0 crates | **0 crates** |
| **Implementation** | High-level Rust | Register access | Bare metal Rust | **99% Assembly** |
| **Code Style** | `led.set_high()?` | `gpio.out.set(1 << 4)` | `ptr::write_volatile(0x50000508, 1 << 4)` | **`str r1, [r0]`** |
| **Startup** | Automatic | Automatic | Manual reset handler | **Assembly reset handler** |
| **Memory Init** | Hidden | Hidden | Explicit RAM setup | **No initialization** |
| **Vector Table** | Generated | Generated | Hand-crafted | **8-byte minimal** |
| **Binary Size** | ~4KB+ | ~2KB+ | ~1KB+ | **~100 bytes** |
| **When to Use** | Production code | Learning registers | Understanding systems | **Performance optimization** |

Each example builds the same functionality (blinking LED) but reveals progressively more of the underlying machinery. The progression moves from high-level abstractions through register manipulation to complete bare-metal assembly implementation, providing comprehensive understanding of embedded systems from hardware reset vector to application logic.

**Examples 5+**: While Examples 01-04 focus on different abstraction levels of the same functionality, Examples 5 onwards explore different embedded programming concepts and peripherals (input processing, sensors, communication protocols, etc.) using practical implementation approaches.

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
