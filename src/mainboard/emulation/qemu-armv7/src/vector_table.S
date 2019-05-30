.section ".bootblock.vectortable", "a"

.global _vector_table

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
    // TODO: initialize the stack to something better
    ldr sp, =0x40100000
    b _start


// TODO: core::fmt depends on real implementations of these functions
.global __sync_val_compare_and_swap_1
.global __sync_val_compare_and_swap_2
.global __sync_val_compare_and_swap_4
__sync_val_compare_and_swap_1:
__sync_val_compare_and_swap_2:
__sync_val_compare_and_swap_4:
    b       _asm_start
