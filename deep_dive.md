# Deep Dive: From Rust Code to Running micro:bit

This document explains the detailed process of how Rust code becomes a running program on the BBC micro:bit v2, covering compilation, linking, memory layout, and flashing.

> **Note**: For VS Code setup and configuration details, see [vscode_setup.md](vscode_setup.md).  
> **Hardware Focus**: For detailed explanations of address buses, internal memory architecture, and physical hardware operation, see [hardware.md](hardware.md).

## Project Architecture

This project uses **independent examples** rather than a Cargo workspace:

- **Each example is self-contained**: Complete with its own `Cargo.toml`, `Cargo.lock`, and `.cargo/config.toml`
- **No workspace dependencies**: Examples can be copied and used independently
- **Reproducible builds**: Each `Cargo.lock` ensures identical dependency versions
- **Shared configuration**: `Embed.toml` configuration is shared across examples

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

### Target Architecture Explained
```toml
# .cargo/config.toml
target = "thumbv7em-none-eabihf"
```

**Target Breakdown:**
- **`thumbv7em`**: ARM Cortex-M4F architecture with DSP extensions
- **`none`**: No operating system (bare metal)
- **`eabihf`**: Embedded Application Binary Interface with Hardware Float

This tells Rust to generate ARM assembly code specifically for the nRF52833's Cortex-M4F processor.

### Probe-rs Configuration (`Embed.toml`)

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
- **`rtt`**: Real-Time Transfer debugging disabled (not needed for simple examples)
- **`gdb`**: GDB debugging interface disabled (not needed for simple examples)

## The Compilation Pipeline

The journey from Rust source to running embedded code involves several critical steps:

1. **Cross-compilation** to ARM Cortex-M4 assembly
2. **Linking** with memory layout and runtime components  
3. **Binary generation** in ELF format
4. **Flashing** to the nRF52833 microcontroller via probe-rs

### Step 1: Cross-Compilation
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
nRF52833 Memory Layout (after linking):

FLASH (512K)                    RAM (128K)
0x00000000                      0x20000000
┌─────────────────────┐         ┌─────────────────────┐
│ Vector Table        │         │                     │
│ ├─ Stack Pointer    │         │                     │
│ ├─ Reset Handler    │         │      Stack          │
│ └─ Exception Vec... │         │        ↓            │
├─────────────────────┤         │                     │
│ .text (Your Code)   │         ├─────────────────────┤
│ ├─ main()           │         │ .data (Init Vars)   │
│ ├─ functions        │         │ ├─ global vars      │
│ └─ compiled code    │         │ └─ static vars      │
├─────────────────────┤         ├─────────────────────┤
│ .rodata (Constants) │         │ .bss (Zero Vars)    │
│ ├─ string literals  │         │ ├─ uninit globals   │
│ └─ const arrays     │         │ └─ zeroed memory    │
└─────────────────────┘         └─────────────────────┘
0x0007FFFF                      0x2001FFFF

Flash: Non-volatile storage      RAM: Fast volatile memory
```

### Power-On Startup Sequence

When the nRF52833 microcontroller powers on, here's exactly what happens:

#### 1. Hardware Reset
- **Power-on Reset (POR)**: CPU starts in a known state with all registers zeroed
- **Clock Initialization**: Internal RC oscillator starts (64MHz HFCLK from 64MHz RC)
- **Program Counter (PC)**: Set to `0x00000000` (start of flash memory)

#### 2. Vector Table Lookup
The CPU immediately reads the first 8 bytes of flash memory:

```
Address     Content                    Purpose
0x00000000: [Initial Stack Pointer]   → Loaded into CPU's SP register
0x00000004: [Reset Handler Address]   → Loaded into CPU's PC register
```

This is why the vector table **must** be at the very start of flash - the CPU hardware expects it there.

#### 3. Stack Pointer Setup
- CPU loads the initial stack pointer from `0x00000000`
- For nRF52833: typically `0x20020000` (end of 128K RAM)
- Stack grows downward from this address

#### 4. Jump to Reset Handler
- CPU loads the reset handler address from `0x00000004`
- Jumps to that address (usually `cortex-m-rt`'s `Reset()` function)
- **Your code hasn't run yet** - this is still initialization!

#### 5. Runtime Initialization (cortex-m-rt)
The reset handler is provided by the `cortex-m-rt` crate and performs critical setup:

```rust
// This code comes from the cortex-m-rt crate source:
// https://github.com/rust-embedded/cortex-m-rt/blob/master/src/lib.rs

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Use symbols provided by the linker script
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;
        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;
    }

    // Copy initialized data from flash to RAM (.data section)
    let mut src = &_sidata as *const u32;
    let mut dest = &mut _sdata as *mut u32;
    while dest < &mut _edata as *mut u32 {
        *dest = *src;
        dest = dest.offset(1);
        src = src.offset(1);
    }

    // Zero out uninitialized variables (.bss section)
    let mut dest = &mut _sbss as *mut u32;
    while dest < &mut _ebss as *mut u32 {
        *dest = 0;
        dest = dest.offset(1);
    }

    // Call any pre-init hooks
    #[cfg(feature = "device")]
    extern "Rust" { fn __pre_init(); }
    #[cfg(feature = "device")]
    __pre_init();

    // Finally, call your main() function!
    main();  // ← YOUR CODE STARTS HERE
}
```

**Where this comes from:**
- **Source**: `cortex-m-rt` crate (specifically `cortex-m-rt/src/lib.rs`)
- **When it's linked**: Automatically included when you add `cortex-m-rt` as a dependency
- **How it's used**: The `#[entry]` macro on your `main()` ensures this reset handler calls your code

#### 6. Your Code Begins
Only after all this initialization does `main()` get called and your Rust code starts executing.

### How the Reset Code Gets Into Your Binary

The reset handler code doesn't appear in your source files, but it gets included in your final binary through Rust's dependency and linking system:

#### 1. Dependency Declaration
In your `Cargo.toml`:
```toml
[dependencies]
cortex-m-rt = "0.7.0"
```
This pulls in the `cortex-m-rt` crate containing the reset handler implementation.

#### 2. The `#[entry]` Macro Magic
When you write:
```rust
#[entry]
fn main() -> ! {
    // your code
}
```

The `#[entry]` macro (from `cortex-m-rt`) transforms your code:
- **Renames** your `main()` function internally (to avoid conflicts)
- **Creates** a proper reset handler that calls your renamed main
- **Sets up** the vector table to point to this reset handler
- **Ensures** proper function signatures for embedded entry points

#### 3. Automatic Linking Process
When you run `cargo build`, the linker automatically:

1. **Compiles your code** → `main.o` (contains your LED logic)
2. **Compiles cortex-m-rt** → `cortex_m_rt.o` (contains reset handler)
3. **Links them together** using linker scripts from cortex-m-rt
4. **Places everything** at the correct memory addresses

#### 4. What Gets Linked Into Your Binary
Your final ELF file contains code from multiple sources:

```
Your Binary Memory Layout:
┌─ Vector Table (from cortex-m-rt)
│  ├─ 0x00000000: [Stack Pointer] 
│  ├─ 0x00000004: [Reset Handler Address] ← Points to cortex-m-rt's Reset()
│  └─ 0x00000008: [Exception handlers...]
├─ Reset Handler Code (from cortex-m-rt crate)
│  ├─ RAM initialization loops
│  ├─ .data/.bss setup code  
│  └─ Call to your main() ← Handoff to your code
├─ Your Application Code (from main.rs)
│  ├─ LED register manipulation
│  ├─ Delay loops with asm::nop()
│  └─ Infinite loop logic
└─ Panic Handler (from panic-halt crate)
   └─ Simple infinite loop on panic
```

#### 5. Linker Script Coordination
The cortex-m-rt crate provides `link.x` which:
- **Defines** where each section goes in memory
- **Ensures** the vector table is at address `0x00000000`
- **Connects** the reset handler to your `#[entry]` function
- **Handles** memory region definitions and stack setup

#### 6. Build-Time Integration
The linking happens transparently during `cargo build`:
```bash
# What cargo does internally:
rustc --target thumbv7em-none-eabihf src/main.rs     # Compile your code
rustc --target thumbv7em-none-eabihf cortex-m-rt...  # Compile runtime
rust-lld -Tlink.x -Tmemory.x main.o cortex_m_rt.o   # Link everything
```

The reset handler is **real compiled ARM assembly code** that gets included in your binary - you just don't write it yourself. This is a key benefit of Rust's embedded ecosystem: critical startup code is provided by well-tested crates.

### Why This Matters

This startup sequence explains several important concepts:

- **Why `memory.x` is critical**: The linker must place the vector table at exactly `0x00000000`
- **Why `#[entry]` works**: It ensures your `main()` gets called by the reset handler
- **Why globals work**: They're initialized before your code runs
- **Why the stack works**: It's set up by hardware before any function calls

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

#### SWD vs JTAG
SWD is ARM's more efficient alternative to JTAG, using only 2 pins instead of 4 and offering higher speeds with a simpler protocol specifically designed for ARM Cortex-M processors.

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

## The Embedded Rust Ecosystem

These examples leverage the power of Rust's embedded ecosystem to hide much of the low-level complexity:

### Hardware Abstraction Layers
- **`microbit-v2` crate**: Provides a high-level board abstraction - you get simple functions like `board.display_pins.row1` instead of manually configuring GPIO registers
- **`nrf52833-hal` crate**: Handles the Nordic chip specifics - timers, GPIO, and peripherals are wrapped in safe, easy-to-use APIs
- **`cortex-m-rt` crate**: Takes care of the startup sequence, memory layout, and low-level ARM Cortex-M details

### What's Hidden Away
Behind the simple Rust code are hundreds of lines of:
- **Register manipulation**: Direct hardware register reads/writes for GPIO, timers, clocks
- **Memory layout configuration**: Linker scripts defining where code and data live in flash/RAM  
- **Startup code**: Assembly routines that run before your `main()` function
- **Interrupt vectors**: Hardware interrupt handling and vector tables
- **Clock configuration**: Setting up the chip's various clock sources and frequencies

### Memory Layout
The project uses auto-generated memory layout from the `microbit-v2` crate, which provides:
- Flash memory mapping for the nRF52833 (typically starts at 0x00000000)
- RAM allocation compatible with the micro:bit v2 (typically starts at 0x20000000) 
- Stack and heap configuration for the Cortex-M4 processor
- Bootloader compatibility (leaves space for the micro:bit's built-in bootloader)

This layered approach allows you to write high-level, safe Rust code while the ecosystem handles the embedded systems complexity underneath.
