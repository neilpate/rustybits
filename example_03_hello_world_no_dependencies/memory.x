/* 
 * Custom Memory Layout for nRF52833 (BBC micro:bit v2)
 * Replacing cortex-m-rt's automatic memory layout
 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These values correspond to the nRF52833 with 512K flash and 128K RAM */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

/* The initial stack pointer - points to the end of RAM */
/* Stack grows downward, so we start at the top */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);