# Example 02 - Hello World (Minimal Dependencies)

A minimal LED blinking example that demonstrates direct hardware register access with as few dependencies as possible.

## What it does

This is a bare-metal "Hello World" that shows what's happening under the hood! The program:

1. Directly configures GPIO registers (no HAL abstractions)
2. Sets up row 1 and column 1 of the LED matrix using raw register writes
3. Continuously blinks the LED at the intersection (1s on, 1s off) using CPU cycle delays

## Key Difference from Example 01

- **Example 01**: Uses high-level HAL crates with convenient abstractions
- **Example 02**: Direct register manipulation with minimal dependencies

## Dependencies

This example uses only the essential crates:

```toml
[dependencies]
panic-halt = "1.0.0"    # Panic handler for no_std environment
cortex-m-rt = "0.7.0"   # Cortex-M runtime (reset handler & linker script)
cortex-m = "0.7.0"      # Cortex-M core functionality (for asm::nop)
```

**No HAL crates** - we write directly to hardware registers!

## Running this example

```bash
cd example_02_hello_world_minimal_dependencies
cargo run --bin main
```
## Code Overview

```rust
#[entry]
fn main() -> ! {
    unsafe {
        // Configure P0.21 (Row 1) as output - direct register access
        let pin_cnf_21 = GPIO_P0_PIN_CNF.add(ROW1_PIN as usize);
        core::ptr::write_volatile(pin_cnf_21, 1); // DIR=1 (output)

        // Configure P0.28 (Col 1) as output and set low (column active)
        let pin_cnf_28 = GPIO_P0_PIN_CNF.add(COL1_PIN as usize);
        core::ptr::write_volatile(pin_cnf_28, 1); // DIR=1 (output)
        core::ptr::write_volatile(GPIO_P0_OUTCLR, 1 << COL1_PIN);
    }

    loop {
        // Turn LED on - direct GPIO register write
        unsafe {
            core::ptr::write_volatile(GPIO_P0_OUTCLR, 1 << ROW1_PIN);
        }
        
        // Delay ~1s using CPU cycles (no hardware timer)
        for _ in 0..800_000 {
            asm::nop();
        }

        // Turn LED off - direct GPIO register write
        unsafe {
            core::ptr::write_volatile(GPIO_P0_OUTSET, 1 << ROW1_PIN);
        }
        
        // Delay ~1s using CPU cycles
        for _ in 0..800_000 {
            asm::nop();
        }
    }
}
```

## Key Concepts

- **Direct Register Access**: Writing directly to nRF52833 GPIO registers at hardcoded addresses
- **Raw Pointers**: Using `*mut u32` pointers to access memory-mapped hardware
- **Volatile Operations**: `write_volatile()` ensures the compiler doesn't optimize away hardware access
- **CPU Cycle Delays**: Simple loop with `nop` instructions instead of hardware timers
- **Memory Layout**: Custom `memory.x` file defines flash/RAM layout for the linker

## Hardware Register Details

```rust
// nRF52833 GPIO Port 0 register addresses (from reference manual)
const GPIO_P0_OUTSET: *mut u32 = 0x5000_0508 as *mut u32;  // Set pins high
const GPIO_P0_OUTCLR: *mut u32 = 0x5000_050C as *mut u32;  // Set pins low  
const GPIO_P0_PIN_CNF: *mut u32 = 0x5000_0700 as *mut u32; // Pin configuration

// micro:bit v2 LED matrix connections
const ROW1_PIN: u32 = 21; // P0.21
const COL1_PIN: u32 = 28; // P0.28
```

### �️ Memory Mapping Basics

The nRF52833 uses a **unified memory architecture** where everything looks like memory to your Rust code, but different addresses go to different physical hardware:

```
nRF52833 Memory Map:
┌─────────────────┬─────────────────────┬─────────────────────┐
│ Address Range   │ What Lives There    │ Your Code Example   │
├─────────────────┼─────────────────────┼─────────────────────┤
│ 0x0000_0000     │ Flash Memory        │ const MSG = "Hi!";  │
│ to 0x0007_FFFF  │ (Program code,      │ (read-only)         │
│ (512KB)         │  constants)         │                     │
├─────────────────┼─────────────────────┼─────────────────────┤
│ 0x2000_0000     │ SRAM Memory         │ let mut count = 0;  │
│ to 0x2001_FFFF  │ (Variables, stack)  │ (read/write)        │
│ (128KB)         │                     │                     │
├─────────────────┼─────────────────────┼─────────────────────┤
│ 0x5000_0508     │ GPIO OUTSET         │ Turn pins HIGH      │
│ 0x5000_050C     │ GPIO OUTCLR         │ Turn pins LOW       │
│ 0x5000_0700+    │ GPIO PIN_CNF        │ Configure pins      │
└─────────────────┴─────────────────────┴─────────────────────┘
```

**Why This Works:**
- **Same syntax**: `*0x5000_0508` and `*0x2000_1000` use identical Rust code
- **Different hardware**: Address decoder routes to GPIO vs SRAM automatically  
- **No special I/O**: Unlike x86, no separate `in`/`out` instructions needed

**Memory-Mapped I/O Example:**
```rust
// All of these use the same Rust syntax but hit different hardware:
unsafe {
    let flash_data = core::ptr::read_volatile(0x0000_2000 as *const u32);    // Flash
    let ram_data = core::ptr::read_volatile(0x2000_1000 as *const u32);      // SRAM  
    core::ptr::write_volatile(0x5000_0508 as *mut u32, 1 << 21);            // GPIO
}
```

> **� Want the Full Hardware Story?** See [hardware.md](../hardware.md) for complete details on address buses, SRAM cells, flash programming, and how your Rust code becomes photons from the LED!

## Memory Layout and Build Process

### The `memory.x` File

The `memory.x` file in the project root defines the memory layout for the nRF52833 microcontroller:

```linker-script
/* nRF52833 memory layout */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These values correspond to the nRF52833 with 512K flash and 128K RAM */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

### How `cortex-m-rt` Uses `memory.x`

During the build process, `cortex-m-rt` automatically finds and processes this file:

1. **Discovery**: `cortex-m-rt` searches for `memory.x` in the project root
2. **Linker Script Generation**: It generates a complete `link.x` linker script that includes your memory layout
3. **Section Placement**: The generated script places code and data sections according to your memory map:

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

### Build Process Details

1. **Compilation**: Rust compiles your code to ARM assembly
2. **Object Files**: Assembler creates `.o` object files with relocatable addresses
3. **Linker Invocation**: `rust-lld` (the linker) is called with:
   - Your object files
   - The generated `link.x` script (which includes your `memory.x`)
   - Target specification: `thumbv7em-none-eabihf`

4. **Address Resolution**: The linker places sections according to `memory.x`:
   - **Code** → Flash starting at `0x00000000`
   - **Constants** → Flash after code
   - **Variables** → RAM starting at `0x20000000`
   - **Stack** → RAM top (`0x20020000`) growing downward

5. **Symbol Generation**: Creates memory layout symbols that `cortex-m-rt` uses:
   - `_sdata`, `_edata`: Data section boundaries
   - `_sbss`, `_ebss`: BSS section boundaries
   - `_sidata`: Location of data initialization values in flash

### Reset Handler Integration

`cortex-m-rt` generates a reset handler that uses the memory layout symbols to:
1. Copy initialized data from flash to RAM
2. Zero out uninitialized variables (BSS section)  
3. Call your `main()` function

This happens automatically before your code runs, setting up the memory environment your program expects.

---

> **🚀 Ready for the Final Challenge?** Check out [Example 03](../example_03_hello_world_no_dependencies/) which achieves **ZERO dependencies** by implementing everything from scratch - including the reset handler, vector table, and linker script! You'll see exactly how `cortex-m-rt` works by building it yourself. It's the ultimate deep dive into embedded systems architecture!

> **🔍 Want to Understand the Complete Hardware Story?** See [hardware.md](../hardware.md) for detailed explanations of address buses, SRAM cells, flash programming, and how the nRF52833's internal memory architecture works at the silicon level!