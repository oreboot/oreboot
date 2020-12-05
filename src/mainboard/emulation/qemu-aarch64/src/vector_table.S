.section ".bootblock.vectortable", "a"

.global _vector_table

/* TODO: update vector table */
// Must be at address 0x0.
_vector_table:
    b       _asm_start // Reset
    b       _asm_start // Undefined instruction
    b       _asm_start // Software Interrupt (SWI)
    b       _asm_start // Prefetch abort
    b       _asm_start // Data abort
    b       _asm_start // Reserved
    b       _asm_start // IRQ Interrupt
    b       _asm_start // FIQ Interrupt

_asm_start:
    ldr x0, =_bss
    ldr x1, =_ebss
    cmp x0, x1
    beq _clear_bss_end
    ldr x2, =0x0
_clear_bss:
    stp x2, x2, [x0], #16
    cmp x0, x1
    bne _clear_bss
_clear_bss_end:

    /* Initialize stack with sentinel value to later check overflow. */
    ldr x0, =_estack
    ldr x1, =_stack
    cmp x0, x1
    beq _set_stack_end
    ldr x2, =0xdeadbeefdeadbeef
_set_stack:
    stp x2, x2, [x0], #16
    cmp x0, x1
    bne _set_stack
_set_stack_end:
    sub sp, x0, #16

    adr x0, _start
    blr x0

// TODO: core::fmt depends on real implementations of these functions
.global __sync_val_compare_and_swap_1
.global __sync_val_compare_and_swap_2
.global __sync_val_compare_and_swap_4
__sync_val_compare_and_swap_1:
__sync_val_compare_and_swap_2:
__sync_val_compare_and_swap_4:
    b       _asm_start


