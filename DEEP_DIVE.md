# Deep Dive: From Rust Code to Running micro:bit

This document explains the detailed process of how Rust code becomes a running program on the BBC micro:bit v2, covering compilation, linking, memory layout, and flashing.

## VS Code Integration and rust-analyzer

### Code Lens Feature
When you open any example's `src/main.rs` file in VS Code, you'll notice small interactive buttons above the `#[entry]` function:

- **▶️ Run**: Executes `cargo run --bin example_name`
- **🐛 Debug**: Launches debugging session (if configured)

### How Code Lens Works
The **rust-analyzer** Language Server analyzes your code and provides these "Code Lens" actions by:

1. **Detecting entry points**: Recognizes `#[entry]` as a runnable binary target
2. **Reading Cargo.toml**: Understands your project's binary configuration
3. **Generating commands**: Creates the appropriate `cargo run --bin` command
4. **Integration with runner**: Uses your `.cargo/config.toml` runner configuration

### What Happens When You Click Run
When you click the ▶️ Run button, rust-analyzer executes:

```bash
cargo run --bin example_01_hello_world
```

Which triggers the build process and uses your configured runner:
```toml
# From .cargo/config.toml
runner = "probe-rs run --chip nRF52833_xxAA"
```

This seamlessly integrates the entire compilation → linking → flashing pipeline into a single click.

### What Happens When You Click Debug

The **🐛 Debug** text next to the run arrow launches an interactive debugging session. When you click "Debug", rust-analyzer essentially runs:

```bash
cargo build  # Build with debug symbols
probe-rs gdb --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/example_name
```

#### Debug Mode Capabilities

Debug mode allows you to:

1. **Set breakpoints**: Click in the left margin of VS Code to pause execution at specific lines
2. **Step-through execution**: Execute your code line by line to see what's happening
3. **Variable inspection**: View the values of variables at any point during execution
4. **Memory examination**: Look at register values and memory contents
5. **Call stack viewing**: See the function call hierarchy

#### The Debug Process

When debugging starts:
1. **Builds with debug info** (symbols, line number mappings)
2. **Launches probe-rs in GDB mode** 
3. **Connects to the micro:bit** via the interface chip's SWD bridge
4. **Loads the program** into flash and halts at the entry point
5. **Opens VS Code's debug interface**

#### VS Code Debug Interface

Once debugging is active, you get access to:
- **Debug console**: For GDB commands and program output
- **Variables panel**: Shows local variables and their values  
- **Watch panel**: Monitor specific expressions
- **Call stack**: See which functions called which
- **Breakpoints panel**: Manage your breakpoints

#### Requirements for Debugging

For debugging to work properly, you need:
- **Debug configuration** in `.vscode/launch.json` (optional for basic debugging)
- **Debug symbols enabled** in your build (automatic in debug builds)
- **Connected micro:bit** with proper probe-rs support

This is extremely powerful for embedded development because you can debug **on the actual hardware** while your program runs on the micro:bit, inspecting real GPIO states, timer values, and memory contents!

## VS Code Workspace Configuration

Setting up VS Code for embedded Rust development requires several configuration files that work together to provide seamless building, running, and debugging capabilities.

### Project Structure
```
rustymicrobit/
├── .vscode/
│   ├── launch.json      # Debug configurations
│   ├── tasks.json       # Build and run tasks
│   └── settings.json    # Workspace settings (optional)
├── Embed.toml           # probe-rs configuration (shared)
├── example_01_hello_world/
│   ├── .cargo/
│   │   └── config.toml  # Example-specific cargo configuration
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml       # Example dependencies and metadata
│   ├── Cargo.lock       # Locked dependency versions
│   └── Embed.toml       # probe-rs configuration (per-example)
├── example_02_hello_world_minimal_dependencies/
│   ├── .cargo/
│   │   └── config.toml  # Example-specific cargo configuration
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml       # Example dependencies and metadata
│   ├── Cargo.lock       # Locked dependency versions
│   └── Embed.toml       # probe-rs configuration (per-example)
└── README.md            # Project overview
```

**Note**: This project uses **independent examples** rather than a Cargo workspace. Each example is a complete, standalone Rust project that can be copied and used independently.

### Tasks Configuration (`.vscode/tasks.json`)

Tasks define how VS Code executes build and run operations:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Build Example 01",
            "type": "shell",
            "command": "cargo",
            "args": [
                "build"
            ],
            "options": {
                "cwd": "${workspaceFolder}/example_01_hello_world"
            },
            "group": "build",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            },
            "problemMatcher": [
                "$rustc"
            ]
        },
        {
            "label": "Run Example 01",
            "type": "shell",
            "command": "cargo",
            "args": [
                "run"
            ],
            "options": {
                "cwd": "${workspaceFolder}/example_01_hello_world"
            },
            "group": "build",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            },
            "problemMatcher": [
                "$rustc"
            ]
        }
    ]
}
```

#### Key Task Properties:
- **`label`**: Name displayed in VS Code's task menu and referenced by other configurations
- **`type: "shell"`**: Executes commands in the system shell
- **`command` & `args`**: The actual command to run (equivalent to `cargo build` and `cargo run`)
- **`options.cwd`**: Working directory - crucial for finding local `.cargo/config.toml` which specifies the target
- **`group: "build"`**: Groups related tasks together
- **`problemMatcher: ["$rustc"]`**: Parses Rust compiler output to show errors in VS Code's Problems panel
- **`presentation`**: Controls how the terminal output is displayed

#### Target Configuration:
Notice that the tasks don't explicitly specify `--target thumbv7em-none-eabihf`. This is because the target is configured in each example's `.cargo/config.toml` file, making the tasks simpler and ensuring consistency.

### Launch Configuration (`.vscode/launch.json`)

Launch configurations define debugging sessions:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Debug Example 01",
            "cwd": "${workspaceFolder}/example_01_hello_world",
            "connectUnderReset": false,
            "chip": "nRF52833_xxAA",
            "flashingConfig": {
                "flashingEnabled": true,
                "haltAfterReset": false
            },
            "coreConfigs": [
                {
                    "coreIndex": 0,
                    "programBinary": "../target/thumbv7em-none-eabihf/debug/example_01_hello_world"
                }
            ],
            "preLaunchTask": "Build Example 01"
        }
    ]
}
```

#### Key Launch Properties:
- **`type: "probe-rs-debug"`**: Uses the probe-rs VS Code extension for ARM debugging
- **`request: "launch"`**: Starts a new debugging session (vs "attach" to existing)
- **`cwd`**: Working directory for the debug session
- **`chip: "nRF52833_xxAA"`**: Specific microcontroller target for probe-rs
- **`flashingConfig`**: Controls how the program is loaded onto the microcontroller
- **`programBinary`**: Path to the compiled ELF file to debug
- **`preLaunchTask`**: Task to run before debugging (builds the project)

### Cargo Configuration (`.cargo/config.toml`)

Each example has its own cargo configuration in `example_XX/.cargo/config.toml`, making each example completely self-contained:

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52833_xxAA"
rustflags = ["-C", "linker=rust-lld", "-C", "link-arg=-Tlink.x"]
```

#### Configuration Breakdown:
- **`[build] target`**: Default compilation target for this directory
- **`runner`**: Command executed when you run `cargo run`
- **`rustflags`**: Additional flags passed to the Rust compiler:
  - **`-C linker=rust-lld`**: Use LLVM's linker (works better for embedded)
  - **`-C link-arg=-Tlink.x`**: Use cortex-m-rt's linker script

### VS Code Settings (`.vscode/settings.json`) - Optional

Workspace-specific settings can enhance the development experience:

```json
{
    "rust-analyzer.cargo.features": "all"
}
```

#### Setting Explanations:
- **`rust-analyzer.cargo.features: "all"`**: Enables all available Cargo features during code analysis, providing complete IntelliSense and error checking for all feature-gated code

### Independent Examples Architecture

This project uses **independent examples** rather than a Cargo workspace:

- **Each example is self-contained**: Complete with its own `Cargo.toml`, `Cargo.lock`, and `.cargo/config.toml`
- **No workspace dependencies**: Examples can be copied and used independently
- **Reproducible builds**: Each `Cargo.lock` ensures identical dependency versions
- **Shared configuration**: VS Code settings and `Embed.toml` are shared across examples

### Integration Flow

When you click the ▶️ Run button in VS Code:

1. **rust-analyzer** detects the `#[entry]` function in the current file
2. **Looks up** the `"rust: cargo run"` task in `tasks.json`
3. **Executes** `cargo run --target thumbv7em-none-eabihf` from the example directory
4. **Cargo** uses the local `.cargo/config.toml` for runner and build settings
5. **Builds** the project with the ARM target using example-specific dependencies
6. **Runs** the configured runner: `probe-rs run --chip nRF52833_xxAA`
7. **probe-rs** uses `Embed.toml` configuration and flashes the program to micro:bit

For debugging (🐛 button):

1. **VS Code** finds the matching launch configuration
2. **Runs** the `preLaunchTask` to build the project
3. **Launches** probe-rs in debug mode with `Embed.toml` settings
4. **Connects** to the micro:bit and loads the program
5. **Starts** the debug session with breakpoint support

This configuration provides a seamless embedded development experience where each example is completely independent while maintaining single-click build/flash/run capabilities!

## Overview

The journey from Rust source to running embedded code involves several critical steps:

1. **Cross-compilation** to ARM Cortex-M4 assembly
2. **Linking** with memory layout and runtime components  
3. **Binary generation** in ELF format
4. **Flashing** to the nRF52833 microcontroller via probe-rs

## Step 1: Cross-Compilation

### Target Architecture
```toml
# .cargo/config.toml
target = "thumbv7em-none-eabihf"
```

This specifies:
- **`thumbv7em`**: ARMv7E-M architecture (Cortex-M4/M7 with DSP extensions)
- **`none`**: No operating system (bare metal)
- **`eabihf`**: Embedded Application Binary Interface with Hardware Floating Point

### Compilation Process
```bash
rustc --target thumbv7em-none-eabihf --crate-type bin src/main.rs
```

The Rust compiler:
1. **Parses** Rust source code into an Abstract Syntax Tree (AST)
2. **Type checks** and performs borrow checking
3. **Generates** LLVM Intermediate Representation (IR)
4. **Optimizes** the IR for size and speed (embedded targets prefer size)
5. **Outputs** ARM Thumb-2 assembly code
6. **Assembles** to object files (.o)

## Step 2: Memory Layout Configuration

### Linker Script Generation
The `microbit-v2` crate generates a `memory.x` file during build:

```linker-script
MEMORY
{
  /* nRF52833 memory layout */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```

### Runtime Components
The `cortex-m-rt` crate provides:

```rust
// Startup sequence (simplified)
#[no_mangle]
pub unsafe extern "C" fn Reset() {
    // 1. Initialize RAM
    // 2. Set up stack pointer
    // 3. Copy .data section from flash to RAM
    // 4. Zero .bss section
    // 5. Call main()
}
```

## Step 3: Linking Process

### Linker Invocation
```bash
rust-lld \
  -flavor gnu \
  -Tlink.x \           # Cortex-M runtime linker script
  -Tmemory.x \         # Memory layout
  --gc-sections \      # Remove unused code
  -o target.elf \
  main.o cortex_m_rt.o microbit_v2.o ...
```

### Memory Sections
The linker organizes code into sections:

```
Flash (0x00000000 - 0x0007FFFF):
├── .vector_table    # Interrupt vectors (@ 0x00000000)
├── .text           # Program code
├── .rodata         # Read-only data (strings, constants)
└── .data (initial) # Initial values for RAM variables

RAM (0x20000000 - 0x2001FFFF):
├── .data           # Initialized variables (copied from flash)
├── .bss            # Zero-initialized variables
├── .uninit         # Uninitialized variables
└── Stack           # Grows downward from top of RAM
```

### Critical Linking Elements

**Vector Table** (first 256 bytes of flash):
```rust
// Generated by cortex-m-rt
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
```

**Entry Point**:
```rust
#[entry] // Macro from cortex-m-rt
fn main() -> ! {
    // Your code here
}
// Expands to proper reset handler setup
```

## Step 4: Binary Generation

### ELF Output
The linker produces an ELF (Executable and Linkable Format) file containing:
- **Headers**: Architecture, entry point, section locations
- **Program sections**: Code and data with load addresses  
- **Symbol table**: Function and variable names for debugging
- **Debug info**: Source line mappings (if enabled)

### Verification
```bash
# Check the binary structure
arm-none-eabi-objdump -h target.elf
arm-none-eabi-size target.elf

# Typical output:
# text    data     bss     dec     hex filename
# 8432     108    2048   10588    295c target.elf
```

## Step 5: Flashing with probe-rs

### micro:bit Hardware Architecture

The BBC micro:bit v2 contains **two separate microcontrollers**:

1. **Target MCU**: nRF52833 (Nordic ARM Cortex-M4) - runs your Rust code
2. **Interface MCU**: nRF52820 (also Nordic ARM Cortex-M4) - acts as debug probe

```
PC ←→ USB ←→ nRF52820 (Interface) ←→ SWD ←→ nRF52833 (Target)
                │                              │
         [Debug Firmware]              [Your Rust Code]
```

### Interface MCU Functions
The nRF52820 interface chip provides:
- **USB-to-SWD bridge**: Converts USB commands to Serial Wire Debug protocol
- **Virtual COM port**: For serial communication with your program
- **Mass storage device**: Drag-and-drop .hex file programming (alternative to probe-rs)
- **WebUSB interface**: Browser-based programming support

### probe-rs Architecture
```
PC (probe-rs) ←→ USB ←→ nRF52820 Interface ←→ SWD ←→ nRF52833 Target
```

When you run `cargo run`, probe-rs:
1. **Connects via USB** to the nRF52820 interface chip
2. **Sends debug commands** over a proprietary protocol  
3. **Interface chip translates** these to SWD signals
4. **Target chip responds** via SWD back through the interface

This is different from external debug probes (J-Link, ST-Link) where the probe is separate hardware.

### Connection Process
1. **USB enumeration**: PC detects micro:bit as USB device (VID: 0x0d28, PID: 0x0204)
2. **Interface identification**: probe-rs recognizes the nRF52820 debug firmware
3. **Target discovery**: Interface chip scans for connected nRF52833 via SWD
4. **Debug session**: Establishes communication channel to target processor


### Flash Memory Layout After Programming
```
nRF52833 Flash (512KB):
0x00000000: ┌─ Vector Table ────────────┐
0x00000100: ├─ Reset Handler ──────────┤  
0x00000200: ├─ Main Program Code ──────┤
0x00002000: ├─ Constant Data ──────────┤
0x00002100: ├─ Initial RAM Values ─────┤
0x00002200: ├─ (unused) ───────────────┤
            │                          │
0x0007FFFF: └──────────────────────────┘
```

## Debug Interface Details

### SWD (Serial Wire Debug) Protocol

SWD is ARM's proprietary debugging protocol, designed as a more efficient alternative to JTAG for ARM Cortex-M processors.

#### Physical Interface
- **SWCLK**: Serial Wire Clock - provides timing for data transfers
- **SWDIO**: Serial Wire Data I/O - bidirectional data line
- **Ground**: Common reference
- **VCC**: Power reference (3.3V on micro:bit)

#### Protocol Stack
```
Application (probe-rs) 
       ↕
Debug Port (DP) - Controls the debug interface
       ↕  
Access Port (AP) - Provides memory access
       ↕
Target Memory Bus - CPU's internal buses
```

#### How SWD Works


#### Debug Features Enabled by SWD
- **Flash Programming**: Direct write access to flash memory controllers
- **Memory Inspection**: Read any RAM/peripheral register in real-time
- **CPU Control**: Halt, reset, single-step execution
- **Breakpoints**: Set hardware breakpoints (nRF52833 has 6 hardware breakpoints)
- **Watchpoints**: Monitor memory locations for read/write access
- **Real-time Trace**: Extract execution trace data (if supported by target)

#### SWD vs JTAG Comparison
| Feature | SWD | JTAG |
|---------|-----|------|
| Pins | 2 (+ power/ground) | 4 (+ power/ground) |
| Speed | Up to 50MHz | Up to 25MHz typical |
| Complexity | Simpler protocol | More complex state machine |
| ARM Support | Native ARM protocol | Generic standard |
| Trace | Supports SWO trace | Supports ETM trace |

#### Security and SWD
Modern ARM processors (including nRF52833) support:
- **Debug Authentication**: Cryptographic authentication before debug access
- **Secure Debug**: Different privilege levels for debug access  
- **Access Port Protection**: Fine-grained control over memory regions
- **Debug Disable**: Complete disabling of debug interface for production

### probe-rs Capabilities
```bash
# List connected probes
probe-rs list

# Flash and run
probe-rs run --chip nRF52833_xxAA target.elf

# Interactive debugging
probe-rs gdb --chip nRF52833_xxAA target.elf

# Real-time tracing (if supported)
probe-rs rtt --chip nRF52833_xxAA target.elf
```

## Performance Considerations

### Code Size Optimization
```toml
# Cargo.toml optimizations for embedded
[profile.release]
opt-level = "s"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
panic = "abort"     # Smaller panic handler
```

### Memory Usage
- **Flash**: Typically 4-20KB for simple examples
- **RAM**: Static allocation preferred (no heap allocator)
- **Stack**: Usually 2-8KB depending on recursion depth
