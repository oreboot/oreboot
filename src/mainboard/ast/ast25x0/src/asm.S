// Copyright 2019 the u-root Authors. All rights reserved
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
.section ".bootblock.vectortable", "a"
	.global xmit
xmit:
	// r0: uart to write to
	// r1: character to xmit
1:
	ldr  r12, [r0, #0x14]
	tst  r12, #0x20
	beq  1b
	strb r1, [r0]
	mov  pc, lr

xfer:
	// r0: uart to write to
	// r7: flash addr
	// r8: mem addr
	// r9: size
	// r10: FMC
	// r11: string to print
	mov r13, lr

	// Start DMA load of kernel
	str r7, [r10, #0x84]
	str r8, [r10, #0x88]
	// The DMA size is the actual size -4 according to the datasheet
	ldr r1, [r9]
	sub r1, r1, #4
	str r1, [r10, #0x8C]
	mov r1, #0x1
	str r1, [r10, #0x80]

	// Print loading message
	mov  r1, #0
1:
	ldrb r1, [r11], #1
	bl   xmit
	cmp  r1, #':'
	bne  1b
	// Wait for DMA to complete
2:
	ldr r1, [r10, #0x8C]
	cmp r1, #0x0
	bne 2b

	// Print checksum
	mov  r1, #0x20 // (space)
	bl   xmit
	mov  r1, #'['
	bl   xmit

	ldr  r2, [r10, #0x90]
	ldr  r9, =hex

	ldr  r4, =0xf0000000
	mov  r5, #0
	mov  r6, #28
3:
	// Mask everything but the current digit to print
	and  r3, r2, r4, lsr r5
	// Calculate the address to the hex character
	add  r1, r9, r3, lsr r6
	ldrb r1, [r1]
	bl   xmit
	add  r5, r5, #4
	sub  r6, r6, #4
	cmp  r5, #32
	bne  3b

	mov  r1, #']'
	bl   xmit
	mov  r1, #'\r'
	bl   xmit
	mov  r1, #'\n'
	bl   xmit

	mov r1, #0x0
	str r1, [r10, #0x80]
	mov pc, r13

	.globl _main
_main:
	ldr r0,  =0x1e784000  // Output uart
	ldr r10, =0x1e620000  // Static memory controller (FMC)

	mov  r1, #'\n'
	bl   xmit

	// Start DMA load of kernel
	// ldr r7,  =kernel_start
	// Load the kernel higher than 0x80008000 + size to avoid an extra relocation
	//ldr r8,  =0x81000000
	//ldr r9,  =kernel_size
	ldr r11, =kernel_str
	bl xfer

	// Start DMA load of DTB
	//ldr r7,  =dtb_start
	//ldr r8,  =CONFIG_DTB_START
	//ldr r9,  =dtb_size
	ldr r11, =dtb_str
	bl xfer

	// Start DMA load of initrd
	//ldr r7,  =initrd_start
	//ldr r8,  =CONFIG_INITRD_START
	//ldr r9,  =initrd_size
	ldr r11, =initrd_str
	bl xfer

	ldr r11, =starting_str
	mov  r9, #0
1:
	ldrb r1, [r11],#1
	bl   xmit
	cmp  r1, #'\n'
	bne  1b

	mov r0, #0x0
	sub r1, r0, #0x1
	//ldr r2,  =CONFIG_DTB_START
	mov r3, #0x0
	mov r4, #0x0
	mov r5, #0x0
	mov r6, #0x0
	mov r7, #0x0
	mov r8, #0x0
	mov r9, #0x0
	mov r10, #0x0
	mov r11, #0x0
	mov r12, #0x0
	mov r13, #0x0
	mov r14, #0x0
	ldr pc, =0x81000000
1:	 b 1b
	.align 4
kernel_str:
	.ascii "Loading kernel:"
	.align 4
dtb_str:
	.ascii "Loading device tree:"
	.align 4
initrd_str:
	.ascii "Loading initrd:"
	.align 4
starting_str:
	.ascii "Starting kernel ...\r\n"
hex:
	.ascii "0123456789abcdef"
