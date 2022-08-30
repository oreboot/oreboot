/*
 * Early initialization code for RISC-V
 *
 * Copyright 2013 Google Inc.
 * Copyright 2016 Jonathan Neuschäfer <j.neuschaefer@gmx.net>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; version 2 of
 * the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.See the
 * GNU General Public License for more details.
 */

.section ".bootblock.boot", "ax", %progbits

.globl _boot
_boot:
	# The previous boot stage passes these variables:
	#   a0: hartid
	#   a1: ROM FDT

	# TODO: Support SMP.
	# Spin all harts other than 0.
	csrr a0, mhartid
spin:
	bgt a0, x0, spin

	# Set the stack pointer
	# 0xDEADBEEF is used to check stack overflow
1:     auipc t0,     %pcrel_hi(_stack_ptr)
       lwu sp, %pcrel_lo(1b)(t0)

	li t0, 0xDEADBEEF
	sw t0, 0(sp)

	# TODO: hart-local storage
	# initialize hart-local storage
	#csrr a0, mhartid
	#csrrw a1, mscratch, zero
	#call hls_init

	#li   a0, 0 # CONFIG_RISCV_WORKING_HARTID
	#call smp_pause

	# TODO: initialize entry of interrupt/exception
	#la t0, trap_entry
	#csrw mtvec, t0

	# TODO: clear any pending interrupts
	#csrwi mip, 0

	# TODO: set up the mstatus register
	#call mstatus_init

	# Pass device tree as a0 to Rust code.
	add a0, a1, x0
	call _start

	// These codes need to be implemented on a specific SoC.
//	.weak cache_as_ram
forever: tail forever
cache_as_ram:
	ret
smp_pause:
	ret
trap_entry:
	ret
hls_init:
	ret
.global abort
abort: j abort

.section ".bootblock.stack", "aw", %progbits
_stack:
.space 65536
