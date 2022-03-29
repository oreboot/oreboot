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
	# a0 is redundant with the mhartid register. a1 might not be valid on
	# some hardware configurations, but is always set in QEMU.
	csrr a0, mhartid

	# Reset the mtimecmp to some absurdly large number.
	# The mtimecmp register is memory mapped at 0x2004000 + 0x8 * mhartid
	# TODO: Move to Rust
	li t0, 0x2004000
	slli t1, a0, 3
	add t1, t1, t0
	li t0, 0xffffffffffffffff
	sd t0, (t1)

	# Clear all interrupt enable and pending bits.
	# TODO: Move to Rust
	csrwi mie, 0x0
	csrwi mip, 0x0

	# as a guess, start bss at 256k ?
	# Layout of L2-LIM (address 0x18040000):
	#   0x00000-0x0ffff: .bss segment (64K bytes)
	#   0x10000-0x1ffff: stack of boot hart (64K bytes)
	#   0x20000-0x20fff: stack of hart 0 (4K bytes)
	#   0x21000-0x21fff: stack of hart 1 (4K bytes)
	#   ...
	#   0x2n000-0x2nfff: stack of hart n (4K bytes)

	# Clear the .bss segment. Every hart does this, which should be safe.
BAD DO NOT USE
	li t0, 0x18040000
	li t1, 0x18050000
clear_bss:
	sd zero, (t0)
	addi t0, t0, 8
	bne t0, t1, clear_bss

	# Decide if this is the boot hart.
	li t0, 1
	beq a0, t0, setup_boot_hart_stack

setup_nonboot_hart_stack:
	# sp <- 0x08021000 + (0x1000 * mhartid) - 8
	li sp, (0x18050000 - 8)
	slli t0, a0, 12
	add sp, sp, t0
	
	# 0xDEADBEEF is used to check stack underflow.
	li t0, 0xDEADBEEF
	sw t0, 0(sp)

	call _start_nonboot_hart

setup_boot_hart_stack:
	li sp, (0x18050000 - 8)
	# 0xDEADBEEF is used to check stack underflow.
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

	call _start_boot_hart

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
