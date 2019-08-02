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
	# The boot ROM may pass the following arguments:
	#   a0: the value of mhartid
	#   a1: a pointer to the flattened devicetree
	#
	# Preserve only the FDT pointer. We can query mhartid ourselves at any
	# time.
	#
	csrw mscratch, a1

	# initialize cache as ram
	call cache_as_ram

	# initialize stack point for each hart
	# and the stack must be page-aligned.
	# 0xDEADBEEF used to check stack overflow
	csrr a0, mhartid
	la   t0, _stack
	slli t1, a0, 12 # PageShift
	add  t0, t0, t1
	li   t1, 0xDEADBEEF
	//store t1, 0(t0)
	li   t1, 4096 - 88 # RISCV_PGSIZE - HLS_SIZE
	add  sp, t0, t1

	# initialize hart-local storage
	csrr a0, mhartid
	csrrw a1, mscratch, zero
	call hls_init

	li   a0, 0 # CONFIG_RISCV_WORKING_HARTID
	call smp_pause

	# initialize entry of interrupt/exception
	la t0, trap_entry
	csrw mtvec, t0

	# clear any pending interrupts
	csrwi mip, 0

	# set up the mstatus register
	//call mstatus_init
	tail _start

	// These codes need to be implemented on a specific SoC.
//	.weak cache_as_ram
cache_as_ram:
	ret
smp_pause:
	ret
trap_entry:
	ret
hls_init:
	ret
_stack:
	.space 4096

