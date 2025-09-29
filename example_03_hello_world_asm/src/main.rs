#![no_main]
#![no_std]

use core::arch::asm;

// ============================================================================
// PURE ARM ASSEMBLY IMPLEMENTATION - No Rust code in critical paths!
// ============================================================================

// Minimal vector table - only what we actually need!
core::arch::global_asm!(
    r#"
.section .vector_table, "a"
.align 2
.global vector_table
vector_table:
    .long 0x20020000            // 0x00: HARDCODED stack pointer = end of 128KB RAM (grows down from here)
    .long Reset + 1             // 0x04: Reset Handler (Thumb bit set)
    // 🔥 HARDCORE: Only 8 bytes, completely self-contained! 🔥

.section .text
.align 2
.thumb_func
.global Reset
Reset:
    // 🔥 PSYCHO MODE WITH SAFETY: Minimal setup but ensure we don't crash! 🔥
    
    // Set up stack pointer explicitly (even though vector table should do this)
    ldr r0, =0x20020000        // Load stack top address
    mov sp, r0                 // Set stack pointer
    
    // Jump to main - no memory initialization needed since we use no globals!
    bl main                    // Call main immediately - YOLO style!
    
    // If main somehow returns, loop forever
reset_loop:
    b reset_loop
"#
);

// Main function implemented in pure assembly
core::arch::global_asm!(
    r#"
.section .text
.align 2
.thumb_func
.global main
main:
    // GPIO register addresses
    .equ GPIO_P0_PIN_CNF_BASE, 0x50000700
    .equ GPIO_P0_OUTSET,       0x50000508
    .equ GPIO_P0_OUTCLR,       0x5000050C
    
    // micro:bit v2 LED matrix pins
    .equ ROW1_PIN, 21           // P0.21
    .equ COL1_PIN, 28           // P0.28
    
    // Configure P0.21 (Row 1) as output
    ldr r0, =(GPIO_P0_PIN_CNF_BASE + (ROW1_PIN * 4))  // PIN_CNF[21]
    movs r1, #1                 // DIR=1 (output), input buffer disconnected
    str r1, [r0]
    
    // Configure P0.28 (Col 1) as output and set low (column active)
    ldr r0, =(GPIO_P0_PIN_CNF_BASE + (COL1_PIN * 4))  // PIN_CNF[28]
    movs r1, #1                 // DIR=1 (output), input buffer disconnected
    str r1, [r0]
    
    // Set column low (active) - never changes
    ldr r0, =GPIO_P0_OUTCLR
    movs r1, #1
    lsls r1, r1, #COL1_PIN      // 1 << 28
    str r1, [r0]

blink_loop:
    // Turn LED ON (row low)
    ldr r0, =GPIO_P0_OUTCLR
    movs r1, #1
    lsls r1, r1, #ROW1_PIN      // 1 << 21
    str r1, [r0]
    
    // Delay ~1 second
    ldr r2, =800000
delay1:
    subs r2, r2, #1
    bne delay1
    
    // Turn LED OFF (row high)
    ldr r0, =GPIO_P0_OUTSET  
    movs r1, #1
    lsls r1, r1, #ROW1_PIN      // 1 << 21
    str r1, [r0]
    
    // Delay ~1 second
    ldr r2, =800000
delay2:
    subs r2, r2, #1
    bne delay2
    
    // Loop forever
    b blink_loop
"#
);

// ============================================================================
// MINIMAL RUST SCAFFOLDING - Only what's absolutely required
// ============================================================================

// Custom panic handler - the only Rust function we need!
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // Pure assembly panic - just loop forever
    unsafe {
        asm!("b .", options(noreturn));
    }
}
