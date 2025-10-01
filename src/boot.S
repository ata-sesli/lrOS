/*
 * lrOS boot code
 *
 * This code is the first to run. It sets up a stack and then calls the Rust
 * kernel's kmain() function.
 */

.section ".text.boot"

.globl _start

_start:
    // Set the stack pointer to the top of the boot stack
    ldr x0, =_boot_stack_top
    mov sp, x0

    // Call the Rust kernel's main function
    bl kmain

    // If kmain returns (which it shouldn't), loop forever
hang:
    wfe
    b hang

.section ".bss"
.align 16
_boot_stack_bottom:
    .space 4096 // 4 KiB stack
_boot_stack_top: