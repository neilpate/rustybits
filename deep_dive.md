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

### Project Configuration (`Cargo.toml`)

Each example has its own `Cargo.toml` file that defines the package metadata and dependencies. Understanding this file is crucial for embedded Rust development.

#### Example Cargo.toml Structure:
```toml
[package]
name = "main"
version = "0.1.0"
edition = "2021"

[dependencies]
panic-halt = "0.2"
cortex-m = "0.7"
cortex-m-rt = "0.7"
microbit-v2 = "0.13"
```

#### Package Section (`[package]`)
- **`name`**: The package name - we standardize to `"main"` for consistency across examples
  - This determines the binary name: `target/thumbv7em-none-eabihf/debug/main`
  - VS Code launch configurations reference this binary name
- **`version`**: Semantic version following [semver](https://semver.org/) (Major.Minor.Patch)
- **`edition`**: Rust edition (2021 is current, enables latest language features)

#### Dependencies Section (`[dependencies]`)
Essential crates for embedded development:

**Core Runtime Dependencies:**
- **`panic-halt`**: Defines panic handler behavior (halts on panic - required for `#![no_std]`)
- **`cortex-m`**: Low-level ARM Cortex-M processor functionality
- **`cortex-m-rt`**: Runtime and startup code for Cortex-M processors

**Hardware Abstraction:**
- **`microbit-v2`**: Board Support Package (BSP) for BBC micro:bit v2
- **`nrf52833-hal`**: Hardware Abstraction Layer for nRF52833 chip (included via microbit-v2)



#### Binary Naming Convention
All examples use `name = "main"` to ensure consistent binary naming:
- **Build output**: `target/thumbv7em-none-eabihf/debug/main`
- **VS Code debugging**: Launch configurations reference `"main"` binary
- **Cargo run**: Executes the `main` binary via probe-rs

This standardization simplifies configuration management across the project.

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

1. **Cross-compilation** - Compile dependencies and main crate to ARM assembly (including build script execution)
2. **Memory layout configuration** - Utilize generated linker scripts
3. **Linking** with memory layout and runtime components  
4. **Binary generation** in ELF format
5. **Flashing** to the nRF52833 microcontroller via probe-rs

### Step 1: Cross-Compilation

The Rust compiler performs cross-compilation targeting the `thumbv7em-none-eabihf` architecture specification. A critical part of this process involves **build scripts** that automatically generate hardware-specific configuration files during compilation.

```bash
# Simplified build process sequence:
cargo build
├── For each dependency: Compile build script → Execute → Compile crate
├── Generated files: memory.x (memory layout), device.x (interrupts)
└── Compile main crate with generated configuration
```

**Key Insight**: Build scripts execute on your development machine (x86_64) while generating configuration for the target hardware (ARM). This enables automatic hardware configuration without manual intervention.

> **Deep Dive**: For comprehensive details about build script mechanics, execution order, and output directories, see the [Build Scripts section](#how-build-scripts-work-in-embedded-rust) in Advanced Concepts.

**Target Architecture Components**:
- **`thumbv7em`**: ARMv7E-M instruction set architecture (Cortex-M4/M7 with DSP extensions)
- **`none`**: Bare-metal execution environment (no operating system)
- **`eabihf`**: Embedded Application Binary Interface with hardware floating-point support

#### Compilation Process
```bash
rustc --target thumbv7em-none-eabihf --crate-type bin src/main.rs
```

The Rust compiler executes the following compilation phases:
1. **Lexical and Syntactic Analysis**: Transforms source code into an Abstract Syntax Tree (AST)
2. **Semantic Analysis**: Performs type checking and borrow checking validation
3. **IR Generation**: Produces LLVM Intermediate Representation (IR)
4. **Optimization**: Applies target-specific optimizations prioritizing code size for embedded constraints
5. **Code Generation**: Emits ARM Thumb-2 assembly instructions
6. **Assembly**: Produces relocatable object files (.o format)

## Step 2: Memory Layout Configuration

### Linker Script Generation
The `nrf52833-hal` crate (included as a dependency within the `microbit-v2` crate) generates a `memory.x` file during its build script execution:

```linker-script
MEMORY
{
  /* nRF52833 memory layout */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
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

The nRF52833 microcontroller follows a precise startup sequence upon power initialization:

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
- **Application code remains inactive** - the system continues hardware initialization

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

    // Transfer control to application entry point
    main();  // ← APPLICATION CODE EXECUTION BEGINS
}
```

**Where this comes from:**
- **Source**: `cortex-m-rt` crate (specifically `cortex-m-rt/src/lib.rs`)
- **When it's linked**: Automatically included when you add `cortex-m-rt` as a dependency
- **How it's used**: The `#[entry]` macro on your `main()` ensures this reset handler calls your code

#### 6. Application Execution Commences
Only after complete system initialization does `main()` receive control and application code begins execution.

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
- **Generates** a proper reset handler that invokes the renamed main function
- **Configures** the vector table to reference this reset handler
- **Ensures** proper function signatures for embedded entry points

#### 3. Automatic Linking Process
When you run `cargo build`, the linker automatically:

1. **Compiles application code** → `main.o` (contains application logic)
2. **Compiles cortex-m-rt** → `cortex_m_rt.o` (contains reset handler)
3. **Links components** using linker scripts from cortex-m-rt
4. **Positions code sections** at designated memory addresses

#### 4. What Gets Linked Into Your Binary
Your final ELF file contains code from multiple sources:

```
Your Binary Memory Layout:
┌─ Vector Table (from device.x via nrf52833-pac)
│  ├─ 0x00000000: [Stack Pointer] 
│  ├─ 0x00000004: [Reset Handler Address] ← Points to cortex-m-rt's Reset()
│  └─ 0x00000008: [Exception handlers...] ← nRF52833-specific interrupts
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
// Vector table structure comes from device.x (generated by nrf52833-pac)
// but cortex-m-rt provides the reset handler implementation:
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

> **Note**: For detailed information about SWD protocol mechanics, debug interface capabilities, and advanced debugging techniques, see the debugging reference documentation.

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

This layered approach allows you to write high-level, safe Rust code while the ecosystem handles the embedded systems complexity underneath.

## Advanced Embedded Rust Concepts

### How Build Scripts Work in Embedded Rust

Cargo's build script system enables automatic generation of hardware-specific configuration files during compilation. This section examines the detailed mechanics of this process.

#### Build Script Discovery and Execution
**1. Build Script Discovery:**
- Cargo automatically discovers `build.rs` files in dependency crates
- The nRF52833-PAC and nRF52833-HAL crates include build scripts for hardware configuration
- Build scripts compile as separate host-architecture executables

**2. Build Script Execution Order:**
```
cargo build
├── Compile build scripts (host target - your PC)
│   ├── nrf52833-pac/build.rs → target/debug/build/nrf52833-pac-*/build-script-build.exe
│   ├── nrf52833-hal/build.rs → target/debug/build/nrf52833-hal-*/build-script-build.exe  
│   └── cortex-m-rt/build.rs → target/debug/build/cortex-m-rt-*/build-script-build.exe
├── Execute build script executables (in dependency order)
│   ├── Run nrf52833-pac build-script-build.exe → generates device.x
│   ├── Run nrf52833-hal build-script-build.exe → generates memory.x
│   └── Run cortex-m-rt build-script-build.exe → sets up linker config
└── Compile main crate (ARM target - thumbv7em-none-eabihf)
    └── Links with generated files from build outputs
```

**Build Script Compilation Process:**
- Cargo compiles each `build.rs` file into a separate executable binary
- On Windows: `build-script-build.exe` (accompanied by `.pdb` debug files)
- These executables execute on the development machine, not the target hardware

**Build Script Output Directories:**
- Each build script gets its own output directory: `target/debug/build/crate-name-hash/out/`
- Generated files (like `memory.x`) are placed in these directories
- Cargo automatically adds these paths to the linker search path with `-L` flags

**Environment and Communication:**
- Build scripts run on your development machine (x86_64), not the target (ARM)
- They can read environment variables, generate code, create files
- Output via `println!("cargo:...")` instructions tells Cargo what to do
- Example: `println!("cargo:rustc-link-search={}", out_dir)` adds linker paths

**When Build Scripts Run:**
- **Every clean build**: All build scripts execute
- **Incremental builds**: Only if dependencies change or build script source changes
- **Cross-compilation**: Build scripts always run on host, regardless of target architecture

#### Memory Layout Generation (`cortex-m-rt` + `nrf52833-hal`)
**The nRF52833-HAL build script** automatically generates the `memory.x` linker script during compilation:
- **Location**: `target/debug/build/nrf52833-hal-*/out/memory.x` 
- **Contents**: nRF52833's exact memory layout (512KB Flash at 0x00000000, 128KB RAM at 0x20000000)

**The nRF52833-PAC build script** generates `device.x` with interrupt vector definitions:
- **Location**: `target/debug/build/nrf52833-pac-*/out/device.x`
- **Contents**: Complete interrupt vector table for all nRF52833 peripherals

**Integration with cortex-m-rt**:
- cortex-m-rt automatically finds and uses these generated files during linking
- Sets up proper memory sections (.text, .data, .bss, .rodata) for embedded systems
- Explains why embedded Rust projects work seamlessly without manual `memory.x` file creation

**Generated Memory Layout:**
```linker
/* Linker script for the nRF52 - WITHOUT SOFT DEVICE */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

### The HAL Ecosystem Layers

Embedded Rust uses a layered architecture for safe, portable hardware access:

#### 1. embedded-hal (Universal Traits - Interfaces Only)
```rust
use embedded_hal::{delay::DelayNs, digital::OutputPin};
```
- **Purpose**: Defines standard interfaces that work across different microcontrollers
- **What it provides**: **Only trait definitions** like `DelayNs`, `OutputPin`, `SpiDevice` - **no actual implementation**
- **Think of it as**: A contract that says "delay functions should look like this" but doesn't provide the actual delay code
- **Benefit**: Write code once, run on any chip that implements these traits

#### 2. microbit-v2 (Board Support Package + Chip HAL)
```rust
use microbit::hal::{gpio, timer};        // ← nRF52833 HAL re-exported
let board = microbit::Board::take().unwrap();  // ← Board-specific configuration
```
- **Exposes**: Complete nRF52833 chip functionality mapped to micro:bit v2 board layout
- **How it works**: Internally uses `nrf52833-hal` for chip-specific implementations and adds micro:bit board configuration
- **What you get**: 
  - `microbit::hal` - All nRF52833 peripherals (GPIO, timers, SPI, etc.) implementing `embedded-hal` traits
  - `microbit::Board` - Pre-configured pin assignments, LED matrix mapping, button setup
- **Key insight**: You never import `nrf52833-hal` directly - everything comes through the microbit crate

#### How the Re-export Works:
```rust
// Inside microbit-v2 crate source code:
pub use nrf52833_hal as hal;  // ← Makes nRF HAL available as microbit::hal

// Your imports:
use microbit::hal::{gpio, timer};        // ← Gets nrf52833_hal functionality
let board = microbit::Board::take();     // ← Gets board-specific pin mappings

// You get both chip-level control AND board-level convenience in one crate!
```

#### What `Board::take()` Actually Does:
```rust
// Inside microbit-v2 source code:
pub fn take() -> Option<Self> {
    Some(Self::new(
        pac::Peripherals::take()?,      // ← Claims ALL nRF52833 peripherals (singleton pattern)
        pac::CorePeripherals::take()?,  // ← Claims ARM Cortex-M core peripherals
    ))
}
```

### The Singleton Pattern in Embedded Systems

#### Hardware Singleton Implementation

The PAC (Peripheral Access Crate) implements a critical singleton pattern to prevent multiple access to hardware:

```rust
// Inside nrf52833-pac crate (auto-generated from chip specification):
static mut DEVICE_PERIPHERALS: bool = false;  // ← THE SINGLETON GLOBAL VARIABLE

pub fn take() -> Option<Self> {
    cortex_m::interrupt::free(|_| {  // Critical section - interrupts disabled
        if unsafe { DEVICE_PERIPHERALS } {
            None  // Already taken!
        } else {
            unsafe { DEVICE_PERIPHERALS = true; }  // Mark as claimed
            Some(Peripherals {
                TIMER0: TIMER0 { _marker: PhantomData },  // Your hardware timer
                TIMER1: TIMER1 { _marker: PhantomData },  // 4 more timers available
                P0: P0 { _marker: PhantomData },          // GPIO Port 0 (32 pins)
                P1: P1 { _marker: PhantomData },          // GPIO Port 1 (16 pins)
                GPIOTE: GPIOTE { _marker: PhantomData },  // GPIO interrupt controller
                RADIO: RADIO { _marker: PhantomData },    // 2.4GHz radio (Bluetooth)
                TEMP: TEMP { _marker: PhantomData },      // Temperature sensor
                RNG: RNG { _marker: PhantomData },        // Random number generator
                ADC: SAADC { _marker: PhantomData },      // Analog-to-digital converter
                // ... 40+ more peripherals
            })
        }
    })
}
```

#### The Global Variable Singleton Implementation

**`static mut DEVICE_PERIPHERALS: bool = false` Breakdown:**
- **`static`**: Stored in global memory, exists for the entire program lifetime
- **`mut`**: Mutable - can be changed from `false` to `true`  
- **`bool = false`**: Initially `false` (peripherals available), becomes `true` (peripherals taken)
- **Memory location**: Fixed address in RAM, typically around `0x20000000 + offset`
- **Size**: Just 1 byte - extremely efficient singleton tracking

#### Why `static mut` Is Dangerous (But Controlled Here):
```rust
// This would be unsafe in normal Rust:
static mut COUNTER: i32 = 0;
fn increment() {
    unsafe { COUNTER += 1; }  // Race condition possible!
}

// But PAC makes it safe with critical sections:
cortex_m::interrupt::free(|_| {  // Disables ALL interrupts
    unsafe { DEVICE_PERIPHERALS = true; }  // Atomic operation guaranteed
});
```

#### Memory Layout of Singleton Variables:
```rust
// These live somewhere in RAM (exact addresses determined by linker):
// Variable               | Size  | Purpose
// DEVICE_PERIPHERALS     | 1 byte| nRF52833 peripherals available?
// CORE_PERIPHERALS       | 1 byte| ARM core peripherals available?

// The linker places these in the .bss section of RAM
// (exact addresses depend on other static variables and linker script)

// After first Board::take():
// DEVICE_PERIPHERALS     = true   // nRF52833 peripherals CLAIMED
// CORE_PERIPHERALS       = true   // ARM core peripherals CLAIMED
```

#### What This Hardware Claiming Prevents:
```rust
// Without singleton pattern (dangerous):
let timer1 = unsafe { &*TIMER0::ptr() };  // Direct memory access
let timer2 = unsafe { &*TIMER0::ptr() };  // Same timer, different variable!
timer1.start();
timer2.stop();  // Conflicts with timer1! Hardware chaos!

// With singleton pattern (safe):
let peripherals1 = Peripherals::take().unwrap();  // Gets hardware
let peripherals2 = Peripherals::take();           // Returns None - prevented!
```

#### The Singleton Pattern in Action:
- **First call**: `Board::take()` returns `Some(Board)` with exclusive hardware access
- **Second call**: `Board::take()` returns `None` - hardware already claimed!
- **Why this matters**: Prevents multiple parts of code from interfering with the same hardware
- **Runtime cost**: Just a boolean check - zero performance overhead

#### Understanding PhantomData in Hardware Types

You may have noticed `PhantomData` markers in the peripheral structures above. This is a zero-cost Rust pattern used extensively in embedded code:

```rust
struct TIMER0 { _marker: PhantomData<*const ()> }  // 0 bytes at runtime
struct TIMER1 { _marker: PhantomData<*const ()> }  // 0 bytes at runtime
```

**Purpose**: `PhantomData` allows the type system to track which specific hardware peripheral you're using without any runtime cost:

```rust
let timer0 = Timer::<TIMER0>::new();   // Type: Timer<TIMER0>  
let timer1 = Timer::<TIMER1>::new();   // Type: Timer<TIMER1>

// Compiler prevents mixing them up:
fn use_timer0(t: Timer<TIMER0>) { /* ... */ }
// use_timer0(timer1);  // ← COMPILE ERROR! Wrong timer type!

// But both compile to identical assembly - zero runtime overhead
```

This pattern ensures type safety (can't mix up TIMER0 vs TIMER1) while maintaining the efficiency required for embedded systems.

This advanced knowledge helps explain why the simple high-level code in the examples works so reliably and safely, despite the complexity of embedded systems programming.
