# Example 03 - Hello World (Pure ARM Assembly)

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


### **Build System Minimized:**
- **4 essential files total** (vs typical 6-8+ files)
- **20-line minimal linker script** (vs typical 100+ lines)
- **Hardcoded build configuration** in `.cargo/config.toml`

## Implementation Details

### 1. **Pure Assembly Vector Table**
```assembly
.section .vector_table, "a"
vector_table:
    .long 0x20020000            // HARDCODED stack pointer (end of 128KB RAM)
    .long Reset + 1             // Reset Handler (Thumb bit set)
    // Only 8 bytes total! No other interrupts/exceptions!
```

### 2. **Pure Assembly Reset Handler**
```assembly
Reset:
    // Set up stack pointer explicitly 
    ldr r0, =0x20020000        // Load hardcoded stack address
    mov sp, r0                 // Set stack pointer
    
    bl main                    // Jump directly to main (YOLO - no memory init!)
    
reset_loop:
    b reset_loop               // Loop forever if main returns
```

### 3. **Pure Assembly Main Function**
```assembly
main:
    // GPIO register addresses - ALL HARDCODED!
    .equ GPIO_P0_PIN_CNF_BASE, 0x50000700  // GPIO configuration base
    .equ GPIO_P0_OUTSET,       0x50000508  // Set pins high
    .equ GPIO_P0_OUTCLR,       0x5000050C  // Set pins low
    
    // Configure P0.21 (Row 1) as output
    ldr r0, =(GPIO_P0_PIN_CNF_BASE + (21 * 4))  // PIN_CNF[21] = 0x50000754
    movs r1, #1                 // DIR=1 (output)
    str r1, [r0]               // Direct register write!
    
    // Blink loop with assembly timing
    blink_loop:
        // Turn LED ON - assembly register manipulation
        ldr r0, =GPIO_P0_OUTCLR
        movs r1, #1
        lsls r1, r1, #21        // 1 << 21 (bit manipulation in assembly!)
        str r1, [r0]
        
        // Assembly delay loop - ~1 second
        ldr r2, =800000         // Hardcoded timing constant
    delay1:
        subs r2, r2, #1         // Decrement counter
        bne delay1              // Branch if not zero
        
        b blink_loop            // Infinite loop
```

### 4. **Minimal Panic Handler (Only Rust Code)**
```rust
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { asm!("b .", options(noreturn)); }  // Pure assembly panic!
}
```

### 5. **Minimal Linker Script (`minimal.ld`) - Only 20 Lines!**
```ld
/* 🔥 MINIMAL HARDCORE LINKER SCRIPT 🔥
 * Only defines the absolute essentials - no memory layout, no initialization!
 */

ENTRY(Reset)

SECTIONS
{
    /* Vector table MUST be at address 0x00000000 */
    .vector_table 0x00000000 : {
        KEEP(*(.vector_table))
    }
    
    /* Code can go anywhere after - we don't care! */
    .text : {
        *(.text .text.*)
    }
    
    /* Discard everything else - we don't use it! */
    /DISCARD/ : {
        *(.data .data.*)
        *(.bss .bss.*)
        *(.rodata .rodata.*)
    }
}
```

**What this achieves:**
- ✅ **Vector table at 0x00000000** (ARM hardware requirement)
- ✅ **Code placement** (linker decides where, we don't care)
- ✅ **Discard unused sections** (no .data, .bss, .rodata - we don't use globals!)



### 4. **Final Binary**
The linker produces an ELF file with:
- **Vector table at 0x00000000** (hardware requirement)
- **Your code in flash** (executable, persistent)
- **Data sections organized** for proper RAM initialization
- **All symbols resolved** for memory management

## Comparison with Other Examples

| Feature | Example 01 | Example 02 | **Example 03 ASM** |
|---------|------------|------------|-------------------|
| **Dependencies** | 5 crates | 3 crates | **0 crates** |
| **Implementation** | High-level Rust | Rust with registers | **99% ARM assembly** |
| **Reset Handler** | cortex-m-rt | cortex-m-rt | **Custom assembly** |
| **Vector Table** | Auto-generated (1024+ bytes) | Auto-generated (1024+ bytes) | **8 bytes minimal** |
| **Linker Script** | Built-in (complex) | Built-in (complex) | **20 lines optimized** |
| **Memory Init** | Automatic .data/.bss | Automatic .data/.bss | **No initialization** |
| **Stack Pointer** | Linker symbol | Linker symbol | **Hardcoded 0x20020000** |
| **GPIO Access** | HAL abstractions | Direct registers | **Direct register addresses** |
| **Binary Size** | ~4KB+ | ~2KB+ | **~100 bytes** |
| **Complexity Level** | Beginner | Intermediate | **Advanced** |
| **Learning Value** | Board basics | Register access | **Complete system understanding** |

### **Learning Progression:**
1. **Example 01**: High-level embedded programming with HAL abstractions
2. **Example 02**: Direct register manipulation and hardware understanding  
3. **Example 03**: Complete bare-metal implementation and system-level programming

## Technical Deep Dive
