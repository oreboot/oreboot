// The ARM1176JZ-S supports the following instructions sets:
// - the 32-bit ARM instruction set used in ARM state
// - the 16-bit Thumb instruction set used in Thumb state
// - the 8-bit Java bytecodes used in Jazelle state
//
// Note:
// "All exceptions are entered, handled, and exited in ARM state. If an exception
// occurs in Thumb state or Jazelle state, the processor reverts to ARM state.
// Exception return instructions restore the SPSR to the CPSR, that can also
// cause a transition back to Thumb state or Jazelle state."
// Because of the above, we elect to use only ARM 32-bit mode for simplicity

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
	// Configure for:
	// - Mode=Supervisor
	// - Thumb=0
	// - Imprecise abort disable=0
	// - Endianness=0 (LE)
	orr    r0, r0, #0x13
	msr    cpsr, r0

	b _asm_soc_start
