/*
 * Linker script for the lrOS kernel.
 */

ENTRY(_start)

SECTIONS
{
    . = 0x40000000;
    .text :
    {
        /* This is the corrected line */
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    .data :
    {
        *(.data .data.*)
    }
    .bss :
    {
        *(.bss .bss.*)
    }
}