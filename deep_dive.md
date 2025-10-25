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
- **`name`**: Package name - standardized to `"main"` for consistency
  - Determines binary name: `target/thumbv7em-none-eabihf/debug/main`
  - VS Code launch configurations reference this binary
- **`version`**: Semantic version following [semver](https://semver.org/)
- **`edition`**: Rust edition (2021 is current)

#### Dependencies Section (`[dependencies]`)
Essential crates for embedded development:

**Core Runtime Dependencies:**
- **`panic-halt`**: Defines panic handler behavior (halts on panic - required for `#![no_std]`)
- **`cortex-m`**: Low-level ARM Cortex-M processor functionality
- **`cortex-m-rt`**: Runtime and startup code for Cortex-M processors

**Hardware Abstraction:**
- **`microbit-v2`**: Board Support Package (BSP) for BBC micro:bit v2
- **`nrf52833-hal`**: Hardware Abstraction Layer for nRF52833 chip (included via microbit-v2)

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

**Key Insight**: Build scripts execute on the development host (x86_64) while generating configuration for the target hardware (ARM). This cross-compilation approach enables automatic hardware-specific configuration without manual intervention.

> **Deep Dive**: For comprehensive details about build script mechanics, execution order, and output directories, see the [Build Scripts section](#how-build-scripts-work-in-embedded-rust) in Advanced Concepts.

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
The `nrf52833-hal` crate generates the `memory.x` linker script during its build script execution, defining the nRF52833's memory regions:

```linker-script
MEMORY
{
  /* nRF52833 memory layout */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```

> **Note**: For detailed information about build script mechanics and memory layout generation, see the [Build Scripts section](#how-build-scripts-work-in-embedded-rust) in Advanced Concepts.

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

When you run `cargo run`, probe-rs executes the following sequence:
1. **Connects via USB** to the nRF52820 interface chip
2. **Sends debug commands** using a proprietary protocol  
3. **Interface chip translates** commands to SWD signals
4. **Target chip responds** via SWD back through the interface

Unlike external debug probes (J-Link, ST-Link), the micro:bit integrates the debug interface directly on the board.

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

The Rust embedded ecosystem provides multiple abstraction layers that simplify bare-metal development while maintaining safety and efficiency.

### Hardware Abstraction Layers
- **`microbit-v2` crate**: Board Support Package providing high-level abstractions for display pins, buttons, and peripherals
- **`nrf52833-hal` crate**: Hardware Abstraction Layer wrapping Nordic chip specifics (timers, GPIO, peripherals) in safe APIs
- **`cortex-m-rt` crate**: Runtime providing startup sequence, memory layout, and low-level ARM Cortex-M functionality

### Implementation Details Hidden by Abstraction
The simple high-level Rust code conceals substantial low-level complexity:
- **Register manipulation**: Direct hardware register operations for GPIO, timers, and clocks
- **Memory layout configuration**: Linker scripts defining code and data placement in flash and RAM  
- **Startup code**: Assembly routines executing before application code
- **Interrupt vectors**: Hardware interrupt handling and vector table configuration
- **Clock configuration**: Microcontroller clock source and frequency management

This layered architecture enables high-level, safe Rust development while the ecosystem manages embedded systems complexity.

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
The nRF52833-HAL build script automatically generates the `memory.x` linker script:
- **Location**: `target/debug/build/nrf52833-hal-*/out/memory.x` 
- **Contents**: nRF52833 memory layout specification (512KB Flash at 0x00000000, 128KB RAM at 0x20000000)

The nRF52833-PAC build script generates `device.x` with interrupt vector definitions:
- **Location**: `target/debug/build/nrf52833-pac-*/out/device.x`
- **Contents**: Complete interrupt vector table for all nRF52833 peripherals

**Integration with cortex-m-rt**:
- cortex-m-rt automatically locates and utilizes these generated files during linking
- Configures proper memory sections (.text, .data, .bss, .rodata) for embedded systems
- Enables seamless embedded Rust development without manual memory configuration

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

Embedded Rust employs a layered architecture for safe, portable hardware access:

#### 1. embedded-hal (Universal Traits)
```rust
use embedded_hal::{delay::DelayNs, digital::OutputPin};
```
- **Purpose**: Defines standard interfaces compatible across different microcontrollers
- **Provides**: Trait definitions (DelayNs, OutputPin, SpiDevice) without concrete implementations
- **Function**: Establishes contracts specifying interface requirements without implementation details
- **Benefit**: Enables portable code execution across different hardware platforms

#### 2. nrf52833-hal (Chip-Specific Implementation)

**Note**: This HAL is accessed through the microbit-v2 BSP's re-export (`microbit::hal`) rather than as a direct dependency. The microbit-v2 crate includes and re-exports the nrf52833-hal internally.

```rust
use microbit::hal::{gpio, timer, pac};  // Accesses nrf52833-hal via BSP
```
- **Purpose**: Provides safe, type-safe abstractions for nRF52833 chip peripherals
- **Implementation**: Wraps PAC (Peripheral Access Crate) register operations with embedded-hal trait implementations
- **Functionality**:
  - **GPIO**: Configures pins as inputs/outputs with pull-up/pull-down resistors
  - **Timers**: Provides delay functionality and timer configuration
  - **Peripherals**: SPI, I2C, UART, ADC, PWM with safe APIs
- **Type Safety**: Uses type-state pattern to prevent invalid configurations at compile time
- **Zero-cost**: Abstractions compile to identical assembly as direct register access

#### 3. microbit-v2 (Board Support Package)
```rust
use microbit::hal::{gpio, timer};
let board = microbit::Board::take().unwrap();
```
- **Exposes**: Complete nRF52833 functionality mapped to micro:bit v2 board layout
- **Implementation**: Utilizes `nrf52833-hal` for chip-specific implementations with board-specific configuration
- **Components**: 
  - `microbit::hal` - nRF52833 peripherals (GPIO, timers, SPI) implementing embedded-hal traits
  - `microbit::Board` - Pre-configured pin assignments, LED matrix mapping, button configuration
- **Design**: Direct re-export from microbit crate eliminates need for separate nrf52833-hal imports

#### Re-export Mechanism:
```rust
// microbit-v2 crate internal implementation:
pub use nrf52833_hal as hal;

// Application code:
use microbit::hal::{gpio, timer};  // Accesses nrf52833_hal functionality
let board = microbit::Board::take();  // Accesses board-specific pin mappings
```

This design provides both chip-level peripheral control and board-level convenience through a unified interface.

#### Board::take() Implementation:
```rust
// Inside microbit-v2 source code:
pub fn take() -> Option<Self> {
    Some(Self::new(
        pac::Peripherals::take()?,      // ← Claims ALL nRF52833 peripherals (singleton pattern)
        pac::CorePeripherals::take()?,  // ← Claims ARM Cortex-M core peripherals
    ))
}
```

This method ensures exclusive hardware access through the singleton pattern described below.

#### Accessing Board Pins

After acquiring the board with `Board::take()`, you can access pre-configured pins:

```rust
let board = microbit::Board::take().unwrap();
let mut row1 = board.display_pins.row1;  // Takes ownership of the pin
```

**What happens in this line:**

1. **Field Access**: `board.display_pins` returns a structure containing all LED matrix pins
2. **Move Semantics**: `row1` is **moved** from the `board.display_pins` structure to the new variable
3. **Ownership Transfer**: The `board.display_pins.row1` field is now uninitialized (moved out)
4. **Type**: `row1` is a `Pin<Output<PushPull>>` - already configured as an output

**Under the hood:**
```rust
// Simplified microbit-v2 Board structure:
pub struct Board {
    pub display_pins: DisplayPins,
    // ... other fields
}

pub struct DisplayPins {
    pub row1: Pin<Output<PushPull>>,  // Pre-configured output pin
    pub col1: Pin<Output<PushPull>>,  // Pre-configured output pin
    // ... other pins
}
```

**Why this design works:**

- **Pre-configuration**: The `Board::new()` constructor already configured these pins as outputs
- **No allocation**: Moving a pin from the structure to a variable is a zero-cost operation
- **Type safety**: You can't accidentally use the same pin twice (it's been moved)
- **Trait implementation**: The pin implements `embedded_hal::digital::OutputPin` trait, providing `set_high()` and `set_low()` methods

**Move semantics in action:**
```rust
let board = microbit::Board::take().unwrap();
let mut row1 = board.display_pins.row1;  // row1 moved out
// board.display_pins.row1  // ← Compile error: value has been moved
row1.set_high().unwrap();  // ✓ Works - we own row1
```

This is standard Rust ownership - the pin can only exist in one place at a time, preventing accidental duplicate access to the same hardware resource.

### The Singleton Pattern in Embedded Systems

#### Hardware Singleton Implementation

The PAC (Peripheral Access Crate) implements a singleton pattern preventing concurrent hardware access:

```rust
// nrf52833-pac crate (auto-generated from chip specification):
static mut DEVICE_PERIPHERALS: bool = false;

pub fn take() -> Option<Self> {
    cortex_m::interrupt::free(|_| {  // Critical section - interrupts disabled
        if unsafe { DEVICE_PERIPHERALS } {
            None  // Already claimed
        } else {
            unsafe { DEVICE_PERIPHERALS = true; }  // Mark as claimed
            Some(Peripherals {
                TIMER0: TIMER0 { _marker: PhantomData },
                TIMER1: TIMER1 { _marker: PhantomData },
                P0: P0 { _marker: PhantomData },
                P1: P1 { _marker: PhantomData },
                GPIOTE: GPIOTE { _marker: PhantomData },
                RADIO: RADIO { _marker: PhantomData },
                TEMP: TEMP { _marker: PhantomData },
                RNG: RNG { _marker: PhantomData },
                ADC: SAADC { _marker: PhantomData },
                // ... additional peripherals
            })
        }
    })
}
```

#### Singleton Implementation Details

**`static mut DEVICE_PERIPHERALS: bool` Analysis:**
- **`static`**: Global memory storage with program lifetime
- **`mut`**: Mutable state tracking (false → true)  
- **`bool`**: Single-byte efficiency for peripheral availability tracking
- **Memory location**: .bss section in RAM (approximately 0x20000000 + offset)

#### Thread Safety Through Critical Sections

The `cortex_m::interrupt::free` function provides atomic access by disabling interrupts during singleton operations:

```rust
// Critical section implementation ensures atomicity:
cortex_m::interrupt::free(|_| {
    // All interrupts disabled - no preemption possible
    unsafe { DEVICE_PERIPHERALS = true; }  // Atomic state modification
});
```

This mechanism prevents race conditions that would occur with unsynchronized mutable static access:

```rust
// Without critical sections (unsafe):
static mut COUNTER: i32 = 0;
fn increment() {
    unsafe { COUNTER += 1; }  // Potential race condition
}
```

#### Memory Layout of Singleton Variables:

Linker places singleton state variables in RAM's .bss section:

```rust
// Typical memory layout in RAM:
// Address        Variable                 Size    Purpose
// 0x20000000+x   DEVICE_PERIPHERALS      1 byte  nRF52833 peripheral availability
// 0x20000001+y   CORE_PERIPHERALS        1 byte  ARM core peripheral availability
```

After first `Board::take()` call, both flags transition from false to true, preventing subsequent hardware access attempts.

#### Singleton Safety Benefits

The singleton pattern enforces exclusive hardware access at runtime:

```rust
// Unsafe: Direct hardware access without ownership tracking
let timer1 = unsafe { &*TIMER0::ptr() };
let timer2 = unsafe { &*TIMER0::ptr() };  // Same hardware, different references
timer1.start();
timer2.stop();  // Conflicting operations cause hardware malfunction

// Safe: Singleton pattern prevents duplicate access
let peripherals1 = Peripherals::take().unwrap();  // Acquires exclusive hardware ownership
let peripherals2 = Peripherals::take();           // Returns None - already claimed
```

**Singleton Pattern Characteristics:**
- **First call**: Returns `Some(Board)` with exclusive hardware ownership
- **Subsequent calls**: Return `None` - hardware access already granted
- **Runtime overhead**: Single boolean check per acquisition attempt
- **Enforcement mechanism**: Static state variable tracks ownership in critical section

#### Understanding PhantomData in Hardware Types

Hardware peripheral structures use `PhantomData` markers for zero-cost type safety:

```rust
struct TIMER0 { _marker: PhantomData<*const ()> }  // Zero runtime size
struct TIMER1 { _marker: PhantomData<*const ()> }  // Zero runtime size
```

`PhantomData` enables compile-time peripheral type tracking without runtime overhead:

```rust
let timer0 = Timer::<TIMER0>::new();  // Type: Timer<TIMER0>  
let timer1 = Timer::<TIMER1>::new();  // Type: Timer<TIMER1>

// Type system prevents peripheral confusion:
fn use_timer0(t: Timer<TIMER0>) { /* ... */ }
use_timer0(timer1);  // Compile error: type mismatch

// Generated assembly identical for both types - zero overhead
```

This mechanism provides type safety (prevents TIMER0/TIMER1 confusion) while maintaining efficiency requirements for embedded systems.

The singleton pattern and zero-cost abstractions demonstrated above explain how high-level embedded Rust code achieves reliability and safety without sacrificing the performance critical for resource-constrained microcontroller environments.

