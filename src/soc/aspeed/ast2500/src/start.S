.global _asm_soc_start

_asm_soc_start:
	// Set the inital stack pointer to the end of SRAM.
	ldr sp, =(0x1E720000 + 0x9000)
	b _cpu_start
