/*
 * Custom Linker Script for nRF52833 (BBC micro:bit v2)
 * Replaces cortex-m-rt's link.x
 * 
 * This script defines how code and data are organized in memory
 */

/* Include memory layout */
INCLUDE memory.x

/* Entry point - where execution begins */
ENTRY(Reset)

/* Define sections */
SECTIONS
{
    /* Vector table must be at the very start of flash (0x00000000) */
    .vector_table ORIGIN(FLASH) : {
        /* Keep vector table */
        KEEP(*(.vector_table))
    } > FLASH

    /* Program code and constants */
    .text : {
        /* Code */
        *(.text .text.*)
        
        /* Read-only data */
        *(.rodata .rodata.*)
        
        /* Keep important sections */
        KEEP(*(.text.Reset))
        KEEP(*(.text.DefaultHandler))
    } > FLASH

    /* Initialized data - stored in flash, copied to RAM at startup */
    .data : {
        . = ALIGN(4);
        _sdata = .;  /* Start of .data in RAM */
        *(.data .data.*)
        . = ALIGN(4);
        _edata = .;  /* End of .data in RAM */
    } > RAM AT > FLASH
    
    /* Store the load address of .data */
    _sidata = LOADADDR(.data);

    /* Uninitialized data - zeroed at startup */
    .bss : {
        . = ALIGN(4);
        _sbss = .;   /* Start of .bss */
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;   /* End of .bss */
    } > RAM

    /* Discard debug info to save space */
    /DISCARD/ : {
        *(.ARM.exidx .ARM.exidx.*)
        *(.ARM.attributes)
    }
}

/* Stack grows down from end of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Provide default handlers */
PROVIDE(DefaultHandler = DefaultHandler);