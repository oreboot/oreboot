/* SPDX-License-Identifier: GPL-2.0-only */

#include "asm_defs.h"

/**
 * protected_mode_call_impl(
 *     RDI=func_ptr: u32,
 *     RSI=arg1: u32,
 *     RDX=arg2: u32
 *     RCX=info: *mut BootBlockInfo) -> u32;
 */
.text
.code64
.section ".text.protected_mode_call_impl", "ax", @progbits
.globl protected_mode_call_impl
protected_mode_call_impl:

	push	%rbp
	mov	%rsp, %rbp
	/* Preserve registers */
	push	%rbx
	push	%r12
	push	%r13
	push	%r14
	push	%r15

	/* Arguments to stack */
	push	%rdi /* Function pointer */
	push	%rsi /* Argument 1 */
	push	%rdx /* Argument 2 */

	push %rcx // Save BootBlockInfo for use in enter_long.inc


    /* Invalidate cache */
    wbinvd

	/* pass 32bit code and data segments */
	movl 4(%rcx), %edi
	movl 8(%rcx), %esi

	/* SetCodeSelector32 will drop us to protected mode on return
	 * The 32 bit code and data segments are already in rsi and rsi
	 * respectively.
	 */
	call	.SetCodeSelector32
	call	.SetupProtectedModeRegs

.code32

	/**
	 * Ok, we're in protected mode. Now we can call FSP.
	 */

	// Satisfy System-V calling convention
	movl	-56(%ebp), %edi	/* Argument 0 */
	movl	-64(%ebp), %esi	/* Argument 1 */

	// Satisfy Windows calling convention
	movl	-56(%ebp), %edx	/* Argument 0 */
	movl	-64(%ebp), %ecx	/* Argument 1 */

	/* Align the stack */
	andl	$0xFFFFFFF0, %esp

	subl	$8, %esp
	pushl	%ecx	/* Argument 1 */
	pushl	%edx	/* Argument 0 */

	movl	-48(%ebp), %ebx	/* Function to call */
	call	*%ebx
	movl	%eax, %ebx

	/**
	 * Back to long mode!
	 */

	// SetupLongModeRegs doesn't clobber edi and esi, so we can just set
	// them once here to pass to both function calls.
	movl -72(%ebp), %ecx
	movl (%ecx), %edi
	movl 12(%ecx), %esi

	call .SetupLongModeRegs
	call .SetCodeSelector64

.code64

	/* Place return value in rax */
	movl	%ebx, %eax

	/* Restore registers */
	mov	-40(%rbp), %r15
	mov	-32(%rbp), %r14
	mov	-24(%rbp), %r13
	mov	-16(%rbp), %r12
	mov	-8(%rbp), %rbx

	/* Restore stack pointer */
	mov	%rbp, %rsp
	pop	%rbp

	ret

/**
 * SetCodeSelector32(
 *     RDI=code32,
 *     RSI=data32)
 */
.align 8
.code64
.SetCodeSelector32:
	// pop the return address from stack
	pop	%rbx

	// save rsp because we need to push it after ss
	mov	%rsp, %rdx

	// use iret to jump to a 32-bit offset in a new code segment
	// iret will pop cs:rip, flags, then ss:rsp
	movw	%si, %ax	// need to push ss, but push ss instuction
	push	%rax	    // not valid in x64 mode, so use ax
	push	%rdx	// the rsp to load
	pushfq			// push rflags
	push	%rdi	// rsi is the code segment selector from caller
	push	%rbx	// push the IP for the next instruction

	// the iretq will behave like ret, with the new cs/ss value loaded
	iretq

/**
 * SetupProtectedModeRegs(
 *   EDI=code32,
 *   ESI=data32)
 */
.align 4
.code32
.SetupProtectedModeRegs:
	/* Use flat data segment */
	movl	%esi, %eax
	movl	%eax, %ds
	movl	%eax, %es
	movl	%eax, %ss
	movl	%eax, %fs
	movl	%eax, %gs

	/* Disable paging. */
	movl	%cr0, %eax
	andl	$(~CR0_PG), %eax
	movl	%eax, %cr0

	/* Disable long mode. */
	movl	$(Efer), %ecx
	rdmsr
	andl	$(~Lme), %eax
	wrmsr

	/* Disable PAE. */
	movl	%cr4, %eax
	andl	$(~Pae), %eax
	movl	%eax, %cr4

	/* Clear page table register */
	xor	%eax, %eax
	movl	%eax, %cr3

	ret

/*
 * SetupLongModeRegs(
 *   EDI=code64
 *   ESI=page_table_addr)
 *
 * Clobbers: eax, ecx, edx
 */
.align 4
.code32
.SetupLongModeRegs:
	/* Get page table address */
	movl	%esi, %eax

	/* load identity mapped page tables */
	movl	%eax, %cr3

	/* enable PAE */
	movl	%cr4, %eax
	andl    $(~Pse), %eax // Turn off PSE because... reasons?
	orl		$Pae, %eax
	movl	%eax, %cr4

	/* enable long mode */
	movl	$(Efer), %ecx
	rdmsr
	orl	    $(Lme), %eax
	wrmsr

	/* enable paging */
	movl	%cr0, %eax

	// TODO: Not convinced that these should be set here. Should probably just
	// do this once in the bootblock code.
	andl	$~(CD|NW|TS|MP), %eax

	// TODO: Coreboot doesn't set the WP bit here, but the oreboot bootblock
	// code does, so here we are.
	orl		$(PG|WP), %eax
	movl	%eax, %cr0

	ret

/**
 * SetCodeSelector64(
 *     EDI=code64)
 */
.align 4
.code32
.SetCodeSelector64:
	pop %esi // Pop the return address, needs to be pushed after code segment

	push %edi // Contains code segment selector from caller
	push %esi // Push return address

	// After lret, cs will contain new value from stack
	lret
