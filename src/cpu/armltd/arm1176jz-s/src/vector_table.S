.section ".cpu.vectortable", "a"

.global _vector_table

// Must be at address 0x0.
_vector_table:
	b       _asm_start           // Reset
	b       _cpu_undefined_instr // Undefined instruction
	b       _cpu_svc             // Supervisor call
	b       _cpu_prefetch_abort  // Prefetch abort
	b       _cpu_data_abort      // Data abort
	b       _cpu_reserved_vector // Reserved
	b       _cpu_irq             // IRQ Interrupt
	b       _cpu_fiq             // FIQ Interrupt

_asm_start:
	mrs    r0, cpsr
	bic    r0, r0, #0x3f
	orr    r0, r0, #0xd3
	msr    cpsr, r0

	b _asm_soc_start
