/* Minimal ARM Cortex-M Linker Script
 * Defines only essential section placement for bare-metal operation
 * Memory layout and initialization handled directly in assembly code
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