#![no_main]
#![no_std]

use core::arch::asm;

// ============================================================================
// ARM THUMB ASSEMBLY IMPLEMENTATION
// ============================================================================

// Minimal ARM Cortex-M vector table implementation
// Contains only the essential entries required for basic operation
core::arch::global_asm!(
    r#"
// .section directive: Creates a named section in the object file
// "a" flag = allocatable (will be loaded into memory at runtime)
.section .vector_table, "a"

// .align directive: Align to 2^n byte boundary (where n=2, so 2^2 = 4-byte alignment)
// ARM Cortex-M requires vector table aligned to its size: 8 bytes minimum
// Since our table is 8 bytes, 4-byte alignment satisfies the requirement
.align 2

// .global directive: Makes this symbol visible to the linker for external reference
// Allows linker script to reference and place this table at address 0x00000000
.global vector_table

vector_table:
    // ARM Cortex-M Vector Table Entry 0: Initial Main Stack Pointer (MSP)
    // Hardware loads this value into SP register on reset/power-up
    // 0x20020000 = nRF52833 RAM base (0x20000000) + RAM size (128KB = 0x20000)
    .long 0x20020000            

    // ARM Cortex-M Vector Table Entry 1: Reset Handler Address  
    // Hardware jumps to this address after loading stack pointer
    // +1 sets Thumb bit (LSB=1) indicating Thumb instruction set mode
    // All Cortex-M code must be Thumb mode (16/32-bit mixed instructions)
    .long Reset + 1             

    // Standard ARM Cortex-M vector table contains 48+ entries for exceptions/interrupts
    // This minimal implementation omits unused handlers to save flash space
    // Missing handlers default to HardFault if triggered (should never happen)

// .section directive: Place following code in .text section (executable code)
// .text is standard section name for program instructions
.section .text

// .align directive: Align function on 4-byte boundary for optimal ARM performance
// ARM instructions are 2 or 4 bytes, alignment prevents cross-boundary penalties
.align 2

// .thumb_func directive: Marks following symbol as Thumb mode function
// Enables proper disassembly and debugging tools recognition
// Required for all ARM Cortex-M functions (no ARM mode support)
.thumb_func

// .global directive: Export Reset symbol for linker visibility
// Linker needs to resolve "Reset + 1" reference from vector table
.global Reset

Reset:
    // Minimal reset handler with explicit stack pointer initialization
    // Note: Hardware should set SP from vector table, but explicit set ensures reliability
    
    ldr r0, =0x20020000        // Load stack pointer address (RAM top)
    mov sp, r0                 // Initialize stack pointer register
    
    // Skip .data/.bss initialization - no global variables used
    bl main                    // Branch to main application function
    
    // Infinite loop if main function returns (should never happen)
reset_loop:
    b reset_loop
"#
);

// Main application function implemented in ARM Thumb assembly
// Configures GPIO pins and implements LED blinking logic
core::arch::global_asm!(
    r#"
// Place main function in .text section with executable code
.section .text

// Align function entry point on 4-byte boundary
// Optimizes instruction fetch pipeline performance
.align 2

// Mark as Thumb mode function for proper toolchain handling
.thumb_func

// Export main symbol so Reset handler can call it via "bl main"
.global main

main:
    // .equ directive: Define assembler constants (like #define in C)
    // These create symbolic names for memory-mapped register addresses
    
    // nRF52833 GPIO Port 0 configuration register array
    // Each pin has 32-bit config register: PIN_CNF[0] through PIN_CNF[31]
    // Address calculation: PIN_CNF[n] = BASE + (n × 4 bytes)
    .equ GPIO_P0_PIN_CNF_BASE, 0x50000700  
    
    // GPIO Port 0 output set register - writing 1 to bit n sets P0.n high
    // Atomic bit-set operation: only affects bits where write data = 1
    .equ GPIO_P0_OUTSET,       0x50000508  
    
    // GPIO Port 0 output clear register - writing 1 to bit n clears P0.n low  
    // Atomic bit-clear operation: only affects bits where write data = 1
    .equ GPIO_P0_OUTCLR,       0x5000050C
    
    // BBC micro:bit v2 LED matrix pin assignments
    .equ ROW1_PIN, 21           // P0.21 - LED matrix row 1
    .equ COL1_PIN, 28           // P0.28 - LED matrix column 1
    
    // Configure P0.21 as output (LED matrix row control)
    ldr r0, =(GPIO_P0_PIN_CNF_BASE + (ROW1_PIN * 4))  // Calculate PIN_CNF[21] address
    movs r1, #1                 // Set DIR=1 (output mode), other fields default
    str r1, [r0]               // Write configuration to register
    
    // Configure P0.28 as output (LED matrix column control) 
    ldr r0, =(GPIO_P0_PIN_CNF_BASE + (COL1_PIN * 4))  // Calculate PIN_CNF[28] address
    movs r1, #1                 // Set DIR=1 (output mode), other fields default
    str r1, [r0]               // Write configuration to register
    
    // Set column pin low (enables current sink for LED matrix column)
    ldr r0, =GPIO_P0_OUTCLR     // Load output clear register address
    movs r1, #1                 // Load immediate value 1
    lsls r1, r1, #COL1_PIN      // Shift to bit 28 position (1 << 28)
    str r1, [r0]               // Clear P0.28 output (set low)

blink_loop:
    // Enable LED by setting row pin low (completes circuit through column)
    ldr r0, =GPIO_P0_OUTCLR     // Load output clear register address
    movs r1, #1                 // Load immediate value 1
    lsls r1, r1, #ROW1_PIN      // Shift to bit 21 position (1 << 21)
    str r1, [r0]               // Clear P0.21 output (LED ON state)
    
    // Software delay loop - approximately 1 second at 64MHz CPU clock
    ldr r2, =8000000           // Load delay counter value
delay1:
    subs r2, r2, #1            // Decrement counter and set flags
    bne delay1                 // Branch if not zero (continue loop)
    
    // Disable LED by setting row pin high (breaks circuit)
    ldr r0, =GPIO_P0_OUTSET     // Load output set register address
    movs r1, #1                 // Load immediate value 1
    lsls r1, r1, #ROW1_PIN      // Shift to bit 21 position (1 << 21)
    str r1, [r0]               // Set P0.21 output high (LED OFF state)
    
    // Software delay loop - approximately 0.1 second
    ldr r2, =800000            // Load shorter delay counter value
delay2:
    subs r2, r2, #1            // Decrement counter and set flags
    bne delay2                 // Branch if not zero (continue loop)
    
    // Return to start of blink cycle
    b blink_loop
"#
);

// ============================================================================
// RUST LANGUAGE REQUIREMENTS
// ============================================================================

/// Panic handler implementation required by Rust no_std environment
/// Provides behavior for unrecoverable errors (should never be called in this application)
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // Enter infinite loop on panic - minimal assembly implementation
    unsafe {
        asm!("b .", options(noreturn));
    }
}
