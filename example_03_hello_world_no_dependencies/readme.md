# Example 03 - Hello World (Zero Dependencies)

> **🎉 ZERO DEPENDENCIES ACHIEVED! 🎉**
> 
> This example demonstrates the **ultimate minimal Rust embedded program** with **absolutely zero external dependencies**. Everything is implemented from scratch, giving you complete control over the entire embedded system.

## What it does

This is the most minimal possible embedded Rust program! It:

1. **Implements a custom ARM Cortex-M startup sequence** - complete vector table and reset handler
2. **Directly manages memory initialization** - copies .data from flash to RAM, zeros .bss section  
3. **Uses custom linker script** - defines exactly how memory is organized
4. **Blinks an LED** using direct GPIO register manipulation and CPU cycle delays
5. **Handles panics** with a custom panic handler (simple infinite loop)

## Key Achievement: Zero Dependencies

```toml
[dependencies]
# 🎉 ZERO DEPENDENCIES! 🎉
# We implement everything ourselves:
# - No cortex-m-rt (custom reset handler & vector table)
# - No cortex-m (inline assembly)  
# - No panic-halt (custom panic handler)
```

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

## The Learning Journey

This example represents the complete journey from high-level to bare metal:

1. **Example 01**: "I want to blink an LED" → Use convenient HAL crates
2. **Example 02**: "How do registers work?" → Direct hardware access with minimal deps  
3. **Example 03**: "How does it ALL work?" → **Implement the entire embedded system**

## Files Created

- **`src/main.rs`** - Application code, vector table, reset handler, panic handler
- **`link.x`** - Custom linker script defining memory sections
- **`memory.x`** - nRF52833 memory layout (512K flash, 128K RAM)
- **`Cargo.toml`** - Zero dependencies! 

## Technical Deep Dive

### Memory Initialization Process
1. **Power-on**: nRF52833 starts, PC jumps to 0x00000000
2. **Vector table**: CPU reads stack pointer and reset handler address
3. **Reset handler**: Our custom `Reset()` function runs:
   - Copies `.data` from flash to RAM (initialized globals)
   - Zeros `.bss` section (uninitialized globals)
   - Calls `main()` - your application starts!

### Assembly Output
The generated assembly is nearly identical to Example 02, proving our zero-dependency implementation maintains the same efficiency while giving complete control.

## Running this example

```bash
cd example_03_hello_world_no_dependencies
cargo run
```
