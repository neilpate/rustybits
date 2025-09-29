# Example 03 - Hello World (Pure ARM Assembly)

> **🔥 ULTIMATE HARDCORE MODE! 🔥**
> 
> This example is **COMPLETELY PSYCHOTIC** - it's a **100% hardcoded ARM assembly** implementation with ZERO dependencies on linker scripts! This is embedded programming at the **most extreme bare-metal level humanly possible**.

## What it does

This is **pure ARM Thumb assembly with hardcoded addresses** - no linker dependencies whatsoever! It:

1. **8-byte hardcoded vector table** - stack pointer hardcoded to `0x20020000` (end of 128KB RAM)
2. **ZERO memory initialization** - skips `.data` and `.bss` setup completely (YOLO style!)
3. **GPIO register manipulation in assembly** - direct memory-mapped I/O with hardcoded addresses
4. **Blinks an LED using assembly loops** - CPU cycle delays with assembly instructions
5. **Pure assembly main function** - no Rust code in the execution path

## Key Achievement: 100% Hardcoded Assembly

```toml
[dependencies]
# 🔥 ABSOLUTELY NOTHING! 🔥
# Everything is hardcoded ARM Thumb assembly:
# - Vector table: 8 bytes with hardcoded stack pointer (0x20020000)
# - Reset handler: Jumps directly to main (no memory setup!)
# - Main function: GPIO manipulation with hardcoded register addresses
# - Only 1 Rust function: panic_handler (compiler requirement)
# - NO LINKER SCRIPTS: Everything hardcoded!
```

## Why This Is Absolutely Psychotic

- **🤯 100% Hardcoded**: Every address manually calculated - no linker symbols, no scripts!
- **⚡ Zero Everything**: No memory initialization, no dependencies, no safety nets!
- **🔧 Pure Chaos**: 8-byte vector table, hardcoded stack pointer, YOLO memory management!
- **🎓 Ultimate Madness**: You control every single byte that goes to the microcontroller!

## The Complete Build Process (What We Implemented)

### 1. **Source Code (`src/main.rs`)**

#### ✅ **Application Logic**
Direct hardware access without HAL abstractions:
```rust
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Configure GPIO registers directly
    // Blink LED using register manipulation
    // Use inline assembly for timing
}
```

#### ✅ **Custom Panic Handler**
Replaces the `panic-halt` crate:
```rust
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {} // Simple halt on panic
}
```

#### ✅ **Custom Reset Handler**  
Replaces `cortex-m-rt`'s startup sequence:
```rust
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // 1. Copy .data section from flash to RAM
    // 2. Zero out .bss section (uninitialized variables)
    // 3. Call main() - your code starts here!
}
```

#### ✅ **Custom Vector Table**
Replaces `cortex-m-rt`'s auto-generated vector table:
```rust
#[link_section = ".vector_table"]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    stack_pointer: 0x20020000, // End of 128K RAM
    reset: Reset,               // Our custom reset handler
    nmi: DefaultHandler,        // Exception handlers
    hard_fault: DefaultHandler,
    // ... 12 more exception handlers
};
```

### 2. **Memory Layout (`memory.x`)**
Defines the nRF52833's physical memory regions:
```
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
```

### 3. **Linking (`link.x`)**
Our custom linker script replaces `cortex-m-rt`'s `link.x`:
```
SECTIONS
{
    /* Vector table MUST be at 0x00000000 */
    .vector_table ORIGIN(FLASH) : {
        KEEP(*(.vector_table))
    } > FLASH

    /* Your code and constants */
    .text : {
        *(.text .text.*)
        *(.rodata .rodata.*)
    } > FLASH

    /* Initialized data: stored in flash, copied to RAM */
    .data : {
        _sdata = .;
        *(.data .data.*)
        _edata = .;
    } > RAM AT > FLASH
    
    _sidata = LOADADDR(.data);  /* Where .data is stored in flash */

    /* Uninitialized data: zeroed in RAM */
    .bss : {
        _sbss = .;
        *(.bss .bss.*)
        _ebss = .;
    } > RAM
}
```

#### **🔧 What the Linker Script Does**

The linker script is like a **memory blueprint** that tells the linker exactly where to place your code and data in the microcontroller's memory. Here's what each section accomplishes:

**📍 Vector Table Placement (`.vector_table`)**
- **Critical requirement**: ARM Cortex-M processors expect the vector table at address `0x00000000`
- **Contains**: Stack pointer + addresses of interrupt/exception handlers
- **Must be first**: If this isn't at address 0, the processor won't boot!

**💾 Code Section (`.text`)**  
- **Stores**: All your compiled functions and string literals in flash memory
- **Why flash**: Non-volatile storage - your program survives power loss
- **Read-only**: Code doesn't change during execution

**🔄 Initialized Data (`.data`)**
- **The clever part**: Data lives in two places!
  - **Stored** in flash (survives power loss) 
  - **Runs** from RAM (faster access during execution)
- **Symbols created**: `_sidata` (flash location), `_sdata`/`_edata` (RAM boundaries)
- **Reset handler uses these** to copy data from flash → RAM at startup

**🆕 Uninitialized Data (`.bss`)**
- **Purpose**: Variables that start with zero/default values
- **RAM only**: No need to store zeros in flash (waste of space)
- **Symbols created**: `_sbss`/`_ebss` tell reset handler what to zero out
- **Examples**: `static mut COUNTER: u32 = 0;`

**🔗 Symbol Magic**
The linker script creates these essential symbols that your `Reset()` function uses:
```rust
// Copy initialized data from flash to RAM
let src = _sidata as *const u8;
let dst = _sdata as *mut u8; 
let len = _edata as usize - _sdata as usize;

// Zero out uninitialized data
let start = _sbss as *mut u8;
let len = _ebss as usize - _sbss as usize;
```

**💡 Why This Matters**
Without this precise memory organization:
- ❌ Vector table in wrong place = processor won't boot
- ❌ No data copying = initialized variables have garbage values  
- ❌ No BSS zeroing = "zero" variables contain random data
- ❌ Wrong stack location = immediate crashes

The linker script transforms your Rust code into a **memory map** that the nRF52833 hardware can actually execute!

### 4. **Final Binary**
The linker produces an ELF file with:
- **Vector table at 0x00000000** (hardware requirement)
- **Your code in flash** (executable, persistent)
- **Data sections organized** for proper RAM initialization
- **All symbols resolved** for memory management

## Comparison with Other Examples

| Feature | Example 01 | Example 02 | **Example 03** |
|---------|------------|------------|----------------|
| **Dependencies** | 5 crates | 3 crates | **0 crates** 🎉 |
| **Abstraction Level** | High-level HAL | Direct registers | **Bare metal** |
| **Reset Handler** | cortex-m-rt | cortex-m-rt | **Custom implementation** |
| **Vector Table** | Auto-generated | Auto-generated | **Hand-crafted** |
| **Linker Script** | Built-in | Built-in | **Custom link.x** |
| **Panic Handler** | panic-halt | panic-halt | **Custom minimal** |
| **Memory Init** | Automatic | Automatic | **Manual RAM setup** |
| **Learning Value** | Board basics | Register access | **Complete system** |

## Technical Deep Dive

### Memory Initialization Process
1. **Power-on**: nRF52833 starts, PC jumps to 0x00000000
2. **Vector table**: CPU reads stack pointer and reset handler address
3. **Reset handler**: Our custom `Reset()` function runs:
   - Copies `.data` from flash to RAM (initialized globals)
   - Zeros `.bss` section (uninitialized globals)
   - Calls `main()` - your application starts!

## Running this example

```bash
cd example_03_hello_world_no_dependencies
cargo run
```

## 🔨 The Complete Compile Process

Understanding how your zero-dependency Rust code becomes a working embedded binary:

### **Step 1: Rust Compilation (`rustc`)**
```bash
rustc --target thumbv7em-none-eabihf src/main.rs
```
**What happens:**
- 🦀 **Rust compiler** translates your Rust code into LLVM IR (Intermediate Representation)
- 🎯 **Target triple** `thumbv7em-none-eabihf` specifies:
  - `thumbv7em`: ARM Cortex-M4F instruction set (Thumb-2 + DSP + FPU)
  - `none`: No operating system (bare metal)
  - `eabihf`: ARM EABI with hardware floating point
- 📝 **Attributes processed**:
  - `#[no_std]`: Don't link standard library
  - `#[no_main]`: Don't use standard main entry point
  - `#[link_section = ".vector_table"]`: Place vector table in specific section
  - `#[no_mangle]`: Keep function names for linker

**Output**: Object file (`.o`) containing ARM machine code + metadata

### **Step 2: Linking (`arm-none-eabi-ld`)**
```bash
arm-none-eabi-ld -T link.x -o target/main.elf main.o
```
**What happens:**
- 🔗 **Linker** combines your object files using your custom `link.x` script
- 📍 **Memory layout** applied from `memory.x`:
  ```
  FLASH: 0x00000000 - 0x0007FFFF (512KB)
  RAM:   0x20000000 - 0x2001FFFF (128KB)
  ```
- 📋 **Sections organized**:
  - `.vector_table` → `0x00000000` (ARM requirement)
  - `.text` → Flash memory (your code)
  - `.data` → RAM with flash backup (initialized variables)
  - `.bss` → RAM only (uninitialized variables)
- 🏷️ **Symbols created**:
  - `_sdata`, `_edata`, `_sidata` (for data copying)
  - `_sbss`, `_ebss` (for BSS zeroing)
  - `_stack_start` (stack pointer initialization)

**Output**: ELF executable with proper ARM Cortex-M memory layout

### **Step 3: Binary Generation (`objcopy`)**
```bash
arm-none-eabi-objcopy -O binary target/main.elf target/main.bin
```
**What happens:**
- 🗂️ **Strip ELF metadata**: Remove debug info, symbol tables, section headers
- 💾 **Create flash image**: Pure binary data ready for microcontroller flash
- 📏 **Memory map preserved**: Vector table first, then code, then data

**Output**: Raw binary file that can be flashed directly to nRF52833

### **Step 4: Hardware Flashing (`probe-rs`/`openocd`)**
```bash
probe-rs run --chip nRF52833_xxAA target/main.elf
```
**What happens:**
- 🔌 **Debug probe** (like J-Link) connects to nRF52833 via SWD interface
- 🧹 **Flash erase**: Clear existing program in flash memory
- 📝 **Flash programming**: Write your binary to flash starting at `0x00000000`
- ✅ **Verification**: Read back and verify flash contents match your binary

**Result**: Your program is now stored in the nRF52833's flash memory!

### **Step 5: Hardware Execution**
**Power-on sequence:**
1. 🔋 **Reset**: nRF52833 powers up, CPU starts at address `0x00000000`
2. 📋 **Vector table read**: CPU reads stack pointer (`0x20020000`) and reset handler address
3. 🏃‍♂️ **Reset handler**: Your `Reset()` function executes:
   ```rust
   // Copy .data from flash to RAM
   copy_data_section();
   // Zero .bss section in RAM  
   zero_bss_section();
   // Jump to your application
   main();
   ```
4. 💡 **Your code runs**: LED starts blinking!

### **Key Files in the Process**

| File | Purpose | Created By |
|------|---------|------------|
| `src/main.rs` | Your Rust source code | You |
| `memory.x` | Memory layout definition | You |
| `link.x` | Linker script | You |
| `Cargo.toml` | Build configuration | You |
| `main.o` | Compiled object file | `rustc` |
| `main.elf` | Linked executable | Linker |
| `main.bin` | Flash-ready binary | `objcopy` |

### **What Makes This Special**

In a typical embedded project with `cortex-m-rt`:
- ✨ **Magic happens**: Vector table, reset handler, and linker script auto-generated
- 🎁 **Convenience**: Everything "just works" but you don't understand why

In your zero-dependency implementation:
- 🔧 **Full control**: You implement every piece of the puzzle yourself
- 🎓 **Complete understanding**: You know exactly what every byte does
- 🚀 **Ultimate learning**: From Rust source to ARM machine code, no mysteries!

This is the complete journey from your Rust code to a blinking LED on real hardware! 🎉

## Additional Resources

- **[hardware.md](../hardware.md)** - Deep dive into address buses, internal memory architecture, and how your code becomes photons from the LED
- **[Example 01](../example_01_hello_world/)** - High-level HAL approach for comparison
- **[Example 02](../example_02_hello_world_minimal_dependencies/)** - Intermediate register-level programming