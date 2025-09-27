# Rusty Bits - BBC micro:bit v2 Rust Examples# Rusty MicroBit - Hello World



A collection of Rust programming examples for the BBC micro:bit v2, demonstrating embedded Rust development using the nRF52833 microcontroller.

![960px-Micro-bit_v2](https://github.com/user-attachments/assets/ce0fe5b0-dc15-4ad8-a31c-e2cbbe288afc)


This repository contains multiple examples showcasing different aspects of embedded Rust programming on the micro:bit platform. Each example is self-contained in its own directory with its own `Cargo.toml` and documentation.This project is based on examples from [The Embedded Rust Book](https://docs.rust-embedded.org/book/), which provides comprehensive guidance for embedded Rust development.



## Examples## What This Project Does



### [example_01_hello_world](example_01_hello_world/)This is a "Hello World" equivalent for embedded systems - it blinks an LED on the micro:bit's LED matrix. The program:

A simple LED blinking example that demonstrates:

- Basic micro:bit board initialization1. Initializes the micro:bit board

- GPIO configuration and control2. Configures row 1 and column 1 of the LED matrix

- Timer usage for delays3. Creates a timer for delays

- Infinite loop patterns4. Continuously blinks the LED at the intersection of row 1 and column 1



## Getting Started## Hardware Requirements



### Prerequisites- BBC micro:bit v2 (with nRF52833 microcontroller)



Follow the installation guide from [The Embedded Rust Book - Installation](https://docs.rust-embedded.org/book/intro/install.html) for complete setup instructions.## Software Installation Guide

Follow the installation guide from [The Embedded Rust Book - Installation](https://docs.rust-embedded.org/book/intro/install.html) for complete setup instructions.

1. **Install Rust and the embedded target:**

   ```bash## Project Structure

   rustup target add thumbv7em-none-eabihf

   ```### Cargo.toml - Project Configuration



2. **Install probe-rs for flashing:**```toml

   ```bash[package]

   cargo install probe-rs --features cliauthors = ["Neil Pate"]

   ```edition = "2021"

name = "hello-world"

3. **Hardware Requirements:**version = "0.1.0"

   - BBC micro:bit v2 (with nRF52833 microcontroller)

   - USB cable for programming and power[[bin]]

name = "hello-world"

### Running an Exampletest = false

bench = false

1. Navigate to any example directory:

   ```bash[dependencies]

   cd example_01_hello_worldcortex-m-rt = "0.7.5"

   ```embedded-hal = "1.0.0"

microbit-v2 = "0.15.1"

2. Connect your micro:bit via USBnrf52833-hal = "0.18.0"

panic-halt = "1.0.0"

3. Build and flash:

   ```bash[dependencies.cortex-m]

   cargo runversion = "0.7.7"

   ```features = ["inline-asm"]

```

## Project Structure

**Key Configuration Details:**

```

rustymicrobit/- **`[[bin]]` section**: Disables tests and benchmarks (`test = false`, `bench = false`) which are incompatible with `no_std` embedded environments

├── .cargo/              # Build configuration (shared)- **`cortex-m-rt`**: Runtime crate providing the entry point and memory layout for ARM Cortex-M processors

├── .vscode/             # VS Code settings (shared)- **`embedded-hal`**: Hardware abstraction layer traits for embedded systems (timers, GPIO, etc.)

├── .gitignore           # Git ignore rules- **`microbit-v2`**: Board support package specifically for micro:bit v2, includes pin definitions and board initialization

├── LICENSE              # Project license- **`nrf52833-hal`**: Hardware abstraction layer for the Nordic nRF52833 System-on-Chip

├── README.md            # This file- **`panic-halt`**: Simple panic handler that halts execution on panic (required for `no_std`)

├── rustfmt.toml         # Rust formatting rules (shared)- **`cortex-m`**: Core ARM Cortex-M functionality with inline assembly features enabled

├── Embed.toml           # Probe-rs configuration (shared)

├── example_01_hello_world/### .cargo/config.toml - Build Configuration

│   ├── src/

│   │   └── main.rs      # Example source code```toml

│   ├── Cargo.toml       # Example dependencies[build]

│   ├── Cargo.lock       # Locked dependency versionstarget = "thumbv7em-none-eabihf"

│   └── README.md        # Example documentation

└── example_02_xxx/      # Future examples...[target.thumbv7em-none-eabihf]

    ├── src/runner = "probe-rs run --chip nRF52833_xxAA"

    ├── Cargo.tomlrustflags = ["-C", "linker=rust-lld", "-C", "link-arg=-Tlink.x"]

    └── README.md```

```

**Configuration Explanation:**

## Shared Configuration

- **`target`**: Specifies the ARM Cortex-M4F architecture target (thumbv7em = ARMv7E-M with DSP extensions, none = no OS, eabihf = embedded ABI with hardware float)

### .cargo/config.toml- **`runner`**: Defines how to execute the binary - uses `probe-rs` to flash and run on the nRF52833 chip

Contains build settings shared across all examples:- **`linker=rust-lld`**: Uses LLVM's linker (rust-lld) instead of the system linker

- **Target**: `thumbv7em-none-eabihf` (ARM Cortex-M4F)- **`link-arg=-Tlink.x`**: Includes the `link.x` linker script from `cortex-m-rt` for proper memory layout

- **Runner**: `probe-rs` for flashing to nRF52833

- **Linker**: Configuration for embedded linking### Embed.toml - Probe Configuration



### rustfmt.toml  ```toml

Rust code formatting rules:[default.general]

- **Line width**: 120 characterschip = "nrf52833_xxAA"

- **Tab spaces**: 4 spaces

- **Style**: Consistent across all examples[default.reset]

halt_afterwards = false

### Embed.toml

Probe-rs configuration for flashing:[default.rtt]

- **Chip**: nRF52833_xxAA specificationenabled = false

- **Reset behavior**: Run after flashing

- **Debug settings**: Optimized for development[default.gdb]

enabled = false

## Development Workflow```



### VS Code (Recommended)**Probe-rs Configuration:**

1. **Open workspace**: Open the root directory in VS Code

2. **Select example**: Navigate to any example directory- **`chip`**: Specifies the exact chip variant (nRF52833 with 512KB flash)

3. **Build and run**: Press `F5` or use `Ctrl+Shift+P` → "Rust Analyzer: Run"- **`halt_afterwards`**: Allows the program to run immediately after flashing

- **`rtt`**: Real-Time Transfer debugging disabled (not needed for simple LED blink)

### Command Line- **`gdb`**: GDB debugging interface disabled (not needed for this simple example)

```bash

# Navigate to example

cd example_01_hello_world## Memory Layout



# Build onlyThe project uses auto-generated memory layout from the `microbit-v2` crate, which provides:

cargo build- Flash memory mapping for the nRF52833

- RAM allocation compatible with the micro:bit v2

# Build and flash to micro:bit- Stack and heap configuration for the Cortex-M4 processor

cargo run

## Learning Resources

# Format code

cargo fmt- [Embedded Rust Book](https://docs.rust-embedded.org/book/)

- [micro:bit v2 Documentation](https://tech.microbit.org/hardware/)

# Check without building- [nRF52833 Product Specification](https://infocenter.nordicsemi.com/topic/ps_nrf52833/keyfeatures_html5.html)

cargo check- [Rust Embedded HAL Documentation](https://docs.rs/embedded-hal/)

```

## Learning Path

1. **Start with example_01_hello_world**: Learn basic GPIO and timing
2. **Future examples will cover**: 
   - LED matrix patterns
   - Button input handling
   - Sensor interfacing
   - Communication protocols (I2C, SPI)
   - Advanced embedded concepts

## Resources

- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [micro:bit v2 Documentation](https://tech.microbit.org/hardware/)
- [nRF52833 Product Specification](https://infocenter.nordicsemi.com/topic/ps_nrf52833/keyfeatures_html5.html)
- [Rust Embedded HAL Documentation](https://docs.rs/embedded-hal/)

## Contributing

Feel free to contribute additional examples or improvements:
1. Follow the existing project structure
2. Include comprehensive documentation
3. Test on actual micro:bit hardware
4. Use the shared configuration files

## License

This project is intended for educational purposes and follows standard embedded Rust practices for the micro:bit platform.