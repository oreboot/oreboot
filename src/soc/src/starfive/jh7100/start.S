/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
  ******************************************************************************
  * @file  start.S
  * @author  StarFive Technology
  * @version  V1.0
  * @date  06/25/2020
  * @brief
  ******************************************************************************
  * @copy
  *
  * THE PRESENT SOFTWARE WHICH IS FOR GUIDANCE ONLY AIMS AT PROVIDING CUSTOMERS
  * WITH CODING INFORMATION REGARDING THEIR PRODUCTS IN ORDER FOR THEM TO SAVE
  * TIME. AS A RESULT, STARFIVE SHALL NOT BE HELD LIABLE FOR ANY
  * DIRECT, INDIRECT OR CONSEQUENTIAL DAMAGES WITH RESPECT TO ANY CLAIMS ARISING
  * FROM THE CONTENT OF SUCH SOFTWARE AND/OR THE USE MADE BY CUSTOMERS OF THE
  * CODING INFORMATION CONTAINED HEREIN IN CONNECTION WITH THEIR PRODUCTS.
  *
  * COPYRIGHT 2020 Shanghai StarFive Technology Co., Ltd.
  */

//#include "encoding.h"
//#include "platform.h"
//#include "sys.h"
	// SIFIVE:
	# The previous boot stage passes these variables:
	#   a0: hartid
	#   a1: ROM FDT
	# a0 is redundant with the mhartid register. a1 might not be valid on
	# some hardware configurations, but is always set in QEMU.
// END SIFIVE

.section ".bootblock.boot", "ax", %progbits

.globl _boot
_boot:

_start:
	csrwi 0x7c1, 0
	// This has to be four byted aligned so make our own and call trap_entry
	la t0, aligned_trap_entry
	csrw mtvec, t0
	csrwi mstatus, 0
	csrwi mie, 0

	// Allocate 4 KiB stack for each hart
	csrr t0, mhartid
	slli t0, t0, 12
	la sp, _bss_end+4096
	sub sp, sp, t0

	li	 a1, 0 // NONSMP_HART    ; // 
	csrr a0, mhartid;
	bne  a0, a1, .LbootOtherHart //other hart

	// WHAT TO DO HERE?
	// Load data section
# 	la t0, _data_lma
# 	la t1, _data
# 	beq t0, t1, 2f
# 	la t2, _edata
# 	bgeu t1, t2, 2f
# 1:
# 	ld t3, 0(t0)
# 	sd t3, 0(t1)
# 	addi t0, t0, 8
# 	addi t1, t1, 8
# 	bltu t1, t2, 1b
	call _start_boot_hart
	j .enter_uboot
2:

	// later.
	/* Clear bss section */
	la t1, _bss_start
	la t2, _bss_end
	bgeu t1, t2, 4f
3:
	sd   x0, 0(t1)
	addi t1, t1, 8
	blt  t1, t2, 3b
4:
	/*only hart 0*/
	call _start_boot_hart
	j .enter_uboot
	
.LbootOtherHart:
	li s1, 0x02000000 //CLINT_CTRL_ADDR
	csrr a0, mhartid
	slli s2, a0, 2
	add s2, s2, s1
	sw zero, 0(s2)
	fence
	csrw mip, 0

	# core 1 jumps to main_other_hart
	# Set MSIE bit to receive IPI
	li a2, (1 << 3) // MIP_MSIP
	csrw mie, a2
	
.LwaitOtherHart:
	# Wait for an IPI to signal that its safe to boot
//	call second_hart
	wfi 	
	# Only start if MIP_MSIP is set
	csrr a2, mip
	andi a2, a2, (1 << 3) // MIP_MSIP
	beqz a2, .LwaitOtherHart

	li s1, 0x20000000 // CLINT_CTRL_ADDR
	csrr a0, mhartid
	slli s2, a0, 2
	add s2, s2, s1
	sw zero, 0(s2)
	fence
	csrw mip, 0
	li a2, 2 // NUM_CORES  
	bltu a0, a2, .enter_uboot
	j .LwaitOtherHart

	# hack.
	.enter_uboot:
	j .enter_uboot
.align 4
aligned_trap_entry:
	j trap_entry
//	li t0, DEFAULT_DDR_ADDR
//	csrr a0, mhartid
//	la a1, 0
//	jr t0

