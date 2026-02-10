/* Architecture and Entry Point */
OUTPUT_ARCH( "riscv" )
ENTRY( _start )

/* * Memory Configuration
 * Adjust ORIGIN and LENGTH to match your specific hardware implementation.
 * Current: 16 MiB starting at address 0x00000000
 */
MEMORY
{
  RAM (rwx) : ORIGIN = 0x00000000, LENGTH = 16M
}

SECTIONS
{
  /* Code Section */
  .text :
  {
    KEEP(*(.text.init))   /* Ensure startup code is placed first */
    *(.text*)       /* All other code */
    *(.rodata*)     /* Read-only data (strings, constants) */
  } > RAM

  /* Initialized Data Section */
  .data :
  {
    . = ALIGN(4);
    _data_start = .;
    *(.data*)
    *(.sdata*)
    . = ALIGN(4);
    _data_end = .;
  } > RAM

  /* Uninitialized Data Section (BSS) */
  .bss :
  {
    . = ALIGN(4);
    _bss_start = .;
    *(.bss*)
    *(.sbss*)
    *(COMMON)
    . = ALIGN(4);
    _bss_end = .;
  } > RAM

  /* * Stack Pointer Definition
   * We place the stack at the very end of the RAM memory block.
   * This allows the stack to grow downwards towards the heap/data.
   */
  _stack_top = ORIGIN(RAM) + LENGTH(RAM);
}