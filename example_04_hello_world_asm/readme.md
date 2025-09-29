# Example 04 - Hello World (Pure ARM Assembly)

> **Advanced Bare-Metal Implementation**
> 
> This example demonstrates a **pure ARM Thumb assembly** implementation with **zero runtime dependencies** and **minimal build infrastructure**. It represents the most direct approach to embedded programming on the nRF52833, providing complete control over every aspect of program execution.

## Technical Overview

This implementation uses pure ARM Thumb assembly with direct hardware register access:

1. **Minimal vector table** - 8-byte table with stack pointer at `0x20020000` (128KB RAM boundary)
2. **No runtime initialization** - eliminates `.data` and `.bss` section setup for reduced complexity
3. **Direct GPIO manipulation** - memory-mapped I/O using hardcoded register addresses (`0x50000700` base)
4. **Assembly-based timing** - CPU cycle counting for LED blink intervals
5. **Assembly main function** - complete program logic implemented in ARM Thumb assembly
6. **Optimized linker script** - 20-line minimal script handling only essential section placement

## Technical Optimizations

### **Build System Simplification:**
- **Linker script reduction**: `link.x` (69 lines) → `minimal.ld` (20 lines, 71% reduction)
- **Memory layout**: Eliminated `memory.x` in favor of hardcoded address constants
- **Configuration consolidation**: Removed `Embed.toml`, consolidated into `.cargo/config.toml`
- **File count**: Reduced to 4 essential files (vs typical 6-8+ files)
- **Build configuration**: Centralized in `.cargo/config.toml`

## Implementation Details

### 1. **ARM Cortex-M Vector Table**
```assembly
.section .vector_table, "a"
vector_table:
    .long 0x20020000            // Initial stack pointer (end of 128KB RAM)
    .long Reset + 1             // Reset handler address with Thumb bit set
    // Minimal 8-byte implementation - unused handlers omitted
```

### 2. **Reset Handler Implementation**
```assembly
Reset:
    // Initialize stack pointer explicitly
    ldr r0, =0x20020000        // Load stack pointer address
    mov sp, r0                 // Set stack pointer register
    
    bl main                    // Branch to main application function
    
reset_loop:
    b reset_loop               // Infinite loop if main returns
```

### 3. **Main Application Function**
```assembly
main:
    // nRF52833 GPIO register addresses
    .equ GPIO_P0_PIN_CNF_BASE, 0x50000700  // Pin configuration register base
    .equ GPIO_P0_OUTSET,       0x50000508  // Output set register
    .equ GPIO_P0_OUTCLR,       0x5000050C  // Output clear register
    
    // Configure P0.21 as output for LED control
    ldr r0, =(GPIO_P0_PIN_CNF_BASE + (21 * 4))  // Calculate PIN_CNF[21] address
    movs r1, #1                 // Set DIR=1 (output mode)
    str r1, [r0]               // Write configuration
    
    // LED blink loop with software timing
    blink_loop:
        // Enable LED (set pin low)
        ldr r0, =GPIO_P0_OUTCLR
        movs r1, #1
        lsls r1, r1, #21        // Shift to bit 21 position
        str r1, [r0]
        
        // Software delay loop
        ldr r2, =800000         // Delay counter value
    delay1:
        subs r2, r2, #1         // Decrement counter
        bne delay1              // Continue until zero
        
        b blink_loop            // Repeat cycle
```

### 4. **Panic Handler Implementation**
```rust
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { asm!("b .", options(noreturn)); }
}
```

### 5. **Minimal Linker Script (`minimal.ld`)**
```ld
/* Minimal ARM Cortex-M Linker Script
 * Defines only essential section placement for bare-metal operation
 */

ENTRY(Reset)

SECTIONS
{
    /* ARM Cortex-M vector table placement at reset address */
    .vector_table 0x00000000 : {
        KEEP(*(.vector_table))
    }
    
    /* Executable code section - linker determines placement */
    .text : {
        *(.text .text.*)
    }
    
    /* Discard unused sections to minimize binary size */
    /DISCARD/ : {
        *(.data .data.*)
        *(.bss .bss.*)
        *(.rodata .rodata.*)
    }
}
```

**Technical achievements:**
- **Vector table placement** at address 0x00000000 (ARM hardware requirement)
- **Flexible code placement** with linker-determined addresses
- **Binary size optimization** through unused section elimination

## Comparison with Other Examples

| Feature | Example 01 | Example 02 | Example 03 | **Example 04 ASM** |
|---------|------------|------------|------------|-------------------|
| **Dependencies** | 5 crates | 3 crates | 0 crates | **0 crates** |
| **Implementation** | High-level Rust | Rust with registers | Bare metal Rust | **99% ARM assembly** |
| **Reset Handler** | cortex-m-rt | cortex-m-rt | Custom Rust | **Custom assembly** |
| **Vector Table** | Auto-generated (1024+ bytes) | Auto-generated (1024+ bytes) | Hand-crafted Rust | **8 bytes minimal** |
| **Linker Script** | Built-in (complex) | Built-in (complex) | Custom linker script | **20 lines optimized** |
| **Memory Init** | Automatic .data/.bss | Automatic .data/.bss | Explicit RAM setup | **No initialization** |
| **Stack Pointer** | Linker symbol | Linker symbol | Linker symbol | **Hardcoded 0x20020000** |
| **GPIO Access** | HAL abstractions | Direct registers | Direct register access | **Direct register addresses** |
| **Binary Size** | ~4KB+ | ~2KB+ | ~1KB+ | **~100 bytes** |
| **Complexity Level** | Beginner | Intermediate | Advanced | **Expert** |
| **Learning Value** | Board basics | Register access | System understanding | **Complete hardware control** |

### **Learning Progression:**
1. **Example 01**: High-level embedded programming with HAL abstractions
2. **Example 02**: Direct register manipulation and hardware understanding  
3. **Example 03**: Complete bare-metal implementation and system-level programming
4. **Example 04**: Pure assembly implementation with maximum hardware control

## Building and Running

```bash
cd example_04_hello_world_asm
cargo build    # Compiles with minimal build system
cargo run      # Programs and executes on BBC micro:bit v2
```

**Expected results:**
- **Minimal build output** - no external dependency compilation required
- **Fast compilation** - assembly code compiles efficiently
- **Small binary size** - approximately 100 bytes vs typical 4KB+ embedded binaries
- **LED operation** - LED matrix displays blinking pattern with assembly-controlled timing

**Memory Map Generated:**
```
Address    Size  Section       Content
0x00000000   8   .vector_table  Stack pointer + reset handler
0x00000008  ~90  .text          ARM Thumb assembly code
```

## Additional Resources

- **[hardware.md](../hardware.md)** - Deep dive into address buses, internal memory architecture, and silicon-level operation
- **[Example 01](../example_01_hello_world/)** - High-level HAL approach for comparison
- **[Example 02](../example_02_hello_world_minimal_dependencies/)** - Intermediate register-level programming
- **[Example 03](../example_03_hello_world_no_dependencies/)** - Complete bare-metal Rust implementation
