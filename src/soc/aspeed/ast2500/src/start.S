.global _asm_soc_start

_asm_soc_start:
	// Set the inital stack pointer to the end of SRAM.
	// This is written to be both Thumb and ARM-32 compatible (hopefully)
	ldr r0, =(0x1E720000 + 0x9000)
	mov sp, r0
	b   _cpu_start
