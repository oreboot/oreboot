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
	mrs    r0, cpsr
	bic    r0, r0, #0x3f
	orr    r0, r0, #0xd3
	msr    cpsr, r0

	// Debug print to UART5.
	ldr r0, =0x1E784000
	mov r1, 'H'
	str r1, [r0]
	mov r1, 'i'
	str r1, [r0]
	mov r1, '\n'
	str r1, [r0]

	// TODO: Setup PLL, MUX and maybe memory.
	//bl    lowlevel_init

	// Set the inital stack pointer to the end of SRAM.
	ldr sp, =(0x1E720000 + 0x9000)
	b _start


// TODO: core::fmt depends on real implementations of these functions
.global __sync_val_compare_and_swap_1
.global __sync_val_compare_and_swap_2
.global __sync_val_compare_and_swap_4
__sync_val_compare_and_swap_1:
__sync_val_compare_and_swap_2:
__sync_val_compare_and_swap_4:
	b       _asm_start
