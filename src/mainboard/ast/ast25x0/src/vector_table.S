/*
 *  armboot - Startup Code for ARM1176 CPU-core
 *
 * Copyright (c) 2007	Samsung Electronics
 *
 * Copyright (C) 2008
 * Guennadi Liakhovetki, DENX Software Engineering, <lg@denx.de>
 *
 * SPDX-License-Identifier:	GPL-2.0+
 *
 * 2007-09-21 - Restructured codes by jsgood (jsgood.yang@samsung.com)
 * 2007-09-21 - Added MoviNAND and OneNAND boot codes by
 * jsgood (jsgood.yang@samsung.com)
 * Base codes by scsuh (sc.suh)
 */

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

// Start function for the assembly code. This does:
// * initialize basic CPU registers
// * very early debug print
// * initialize stack to SRAM
// * run PLL setup
// * jump to rust code
_asm_start:
	// Put the CPU in supervisor (SVC) mode.
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
