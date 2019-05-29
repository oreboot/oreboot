.section ".bootblock.vectortable", "a"

.global _vector_table

// Must be at address 0x0.
_vector_table:
    b       _start // Reset
    b       _start // Undefined instruction
    b       _start // Software Interrupt (SWI)
    b       _start // Prefetch abort
    b       _start // Data abort
    b       _start // Reserved
    b       _start // IRQ Interrupt
    b       _start // FIQ Interrupt
