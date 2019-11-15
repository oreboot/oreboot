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
	// These codes need to be implemented on a specific SoC.
//	.weak cache_as_ram
forever: tail forever
cache_as_ram:
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
