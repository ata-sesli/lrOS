/*
 * lrOS boot code
 *
 * Assumes QEMU starts us in EL1.
 * Disables the MMU and caches, sets up the stack, and calls the Rust kernel.
 */

.section ".text.boot"
.globl _start

_start:
    // --- Disable MMU and Caches ---
    // Read the System Control Register for EL1.
    mrs x0, sctlr_el1

    // Clear the M bit (bit 0) to disable the MMU.
    bic x0, x0, #(1 << 0)
    // Clear the C bit (bit 2) to disable the Data Cache.
    bic x0, x0, #(1 << 2)
    // Clear the I bit (bit 12) to disable the Instruction Cache.
    bic x0, x0, #(1 << 12)

    // --- ADDED LINE ---
    // Clear the SPAN bit (bit 23). This prevents restricting memory access from EL1.
    bic x0, x0, #(1 << 23)

    // Write the modified value back.
    msr sctlr_el1, x0

    // Ensure all changes are applied before proceeding.
    isb

    // --- Setup Stack and Call Kernel ---
    ldr x0, =_boot_stack_top
    mov sp, x0
    mov fp, #0 // Clear frame pointer

    bl kmain

hang:
    wfe
    b hang

.section ".bss"
.align 16
_boot_stack_bottom:
    .space 4096 // 4 KiB stack
_boot_stack_top: