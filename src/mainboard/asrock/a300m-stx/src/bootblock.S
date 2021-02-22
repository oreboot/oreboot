/*
 * This file is part of the coreboot project.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * This is the modern bootblock. It prepares the system for C environment runtime
 * setup. The actual setup is done by hardware-specific code.
 *
 * It provides a bootflow similar to other architectures, and thus is considered
 * to be the modern approach.
 *
 */
.section .bootblock.boot
.code32
.globl entryfromreset
entryfromreset:
	// The situation on amd zen vs. x86 is so different that it makes no sense to share
	// a bootblock.
	// For now, this is a partial bootblock. We will enter in 32-bit mode, with a 32-bit
	// gdt, and segment registers are set up. Our goal is to write a proper working LME.
	// We've had a hope that we can use GB PTEs but that's been failing miserably
	// and I'm not sure why.
	// We DO have a working stack. We ARE in RAM. It makes sense to take advantage
	// of that, and, further, let the 16-bit code pass us our base address in %rdi and
	// so we don't have to play all these damned linker tricks. It turns out the zen
	// will let us use a "fake" (ffxxxxxx) address of the very first lgdt (!) and we can then
	// have this code do a second lgdt with a more reasonable gdt. Or so we think.
	//
	// So, assumption going in:
	// we have a valid sp.
	// The starting address of this code block is in %rdi
	call 3f
	// Leave some roon here in case we want to, e.g.,
	// adjust the stack or something.
	.align 16
.globl gdtptr
gdtptr:
	.word	gdt_end - gdt -1 /* compute the table limit */
8:
	.long	0		// to be loaded later.

	.align	4
/* these are deliberately changed to a higher selgdt number so we can verify
 * that we're using our own new gdt.
 */
gdt:
	/* selgdt 0, unused */
	.word	0x0000, 0x0000		/* dummy */
	.byte	0x00, 0x00, 0x00, 0x00

	/* selgdt 0, unused */
	.word	0x0000, 0x0000		/* dummy */
	.byte	0x00, 0x00, 0x00, 0x00

	/* selgdt 0, unused */
	.word	0x0000, 0x0000		/* dummy */
	.byte	0x00, 0x00, 0x00, 0x00
#define CODE32 0x18
	/* selgdt 0x18, flat code segment */
	.word	0xffff, 0x0000
	.byte	0x00, 0x9b, 0xcf, 0x00 /* G=1 and 0x0f, So we get 4Gbytes
#define DAT32 0x20					  for limit */
	/* selgdt 0x20,flat data segment */
	.word	0xffff, 0x0000
	.byte	0x00, 0x93, 0xcf, 0x00
#define LM 0x28
	/* selgdt 0x28, long mode code segment. */
	.quad	0x0020980000000000		/* Long mode CS */

gdt_end:
3:
	movb $0x42, %al
	outb %al, $0x80
	movw $0x3F8, %dx
  movb $0xAA, %al
  call p4
	// TOS contains a pointer to the gdt descriptor
	popl %eax
	// The low 4 bits of %eax are 3.
	// the gdt pointer is aligned to 16.
	// Set the low 4-bits of offset to 16.
	andb $0xf0, %al
	orb $0x10, %al
	pushl %eax
	// Adjust the gdt pointer for where we are.
	addl	$8, %eax
	// Now put the adjusted pointer
	movl 	%eax, 8b
	outb %al, $0x80
	shrl $8, %eax
	outb %al, $0x80
	shrl $8, %eax
	outb %al, $0x80
	shrl $8, %eax
	outb %al, $0x80
	popl %eax
	lgdt (%eax)

  pushl %edx /* Print num */
  movw $0x3f8, %dx
  movb $'1', %al
  outb %al, %dx
  popl %edx /* Print num */

	movb $0xbf, %al
	outb %al, $0x80
	// Now fix up the page table
	// pml3 is fine, but pml4 needs the pointer set.
	// by doing this we remove dependence on the linker to
	// set pml4[0] correctly.
	movl $pml3, %eax
	orl $3, %eax
	movl %eax, pml4
	outb %al, $0x80
	shrl $8, %eax
	outb %al, $0x80
	shrl $8, %eax
	outb %al, $0x80
	shrl $8, %eax
	outb %al, $0x80

	movb $0xbe, %al
	outb %al, $0x80

	/* Now that we are in protected mode jump to a 32 bit code segment. */
	ljmpl	$0x18, $__protected_start

p4:
  push %ebx
  push %eax // Input in ax

  movw $0x3FD, %dx  // Check LCR while transmit buffer is full
  1: inb %dx, %al
  andb $0x20, %al
  cmp $0x20, %al
  jl 1b             // End check

  pop %eax
  push %eax // Restore input to ax

  movw $0x3F8, %dx  // UART port
  rorb $4, %al      // Upper nibble first
  andb $0x0f, %al
  cmp $9, %al
  jg 1f
  add $'0', %al
  jmp 2f
  1: add $0x37, %al
  2: outb %al, %dx

  movw $0x3FD, %dx  // LCR check
  1: inb %dx, %al
  andb $0x20, %al
  cmp $0x20, %al
  jl 1b             // End check

  pop %eax
  push %eax // Restore input to ax

  movw $0x3F8, %dx  // UART port
  andb $0x0f, %al
  cmp $9, %al
  jg 1f
  add $'0', %al
  jmp 2f
  1: add $0x37, %al
  2: outb %al, %dx

  movw $0x3FD, %dx  // LCR check
  1: inb %dx, %al
  andb $0x20, %al
  cmp $0x20, %al
  jl 1b             // End check

  movw $0x3F8, %dx  // UART port
  movb $0x0A, %al // \r\n after a print
  outb %al, %dx
  movb $0x0D, %al
  outb %al, %dx

  pop %eax
  pop %ebx
  ret



	// TODO: should set accessed and dirty bits in gdt entries
	// so CPU does not try to write them to ROM?
	.align 0x1000
pml4:
	.quad 0
	.align 0x1000
pml3:
	.quad 0x0000083,0x40000083,0x80000083,0xc0000083
	.align 0x1000

.code32
	// we're now in 32-bit mode.
/*
 *	When we come here we are in protected mode. We expand
 *	the stack and copies the data segment from ROM to the
 *	memory.
 *
 *	After that, we call the chipset bootstrap routine that
 *	does what is left of the chipset initialization.
 *
 *	NOTE aligned to 4 so that we are sure that the prefetch
 *	cache will be reloaded.
 *
 *	In the bootblock there is already a ljmp to __protected_start and
 *	the reset vector jumps to symbol _start16bit in entry16.inc from
 *	the reset vectors's symbol which is _start. Therefore, don't
 *	expose the _start symbol for bootblock.
 */
	.align	4
__protected_start:
	movb $0xdc, %al
	outb %al, $0x80
	// Enter here in 32-bit protected mode. Welcome to 1982.
	// First thing you have to do is get the segments to
	// sane values. Only %cs is correct when we get here.

	//post_code(POST_ENTER_PROTECTED_MODE)

	movw	$0x20, %ax
	movw	%ax, %ds
	movw	%ax, %es
	movw	%ax, %ss
	movw	%ax, %fs
	movw	%ax, %gs
__protected_start_no_load_segs:
	movb $0x30, %al
	outb %al, $0x80
	// Set a pointer to the page table pages in %cr3.
	// We can use cr3 as a scratch register here;
	// its value won't matter until we set PG in CR0 below.
	movl $pml4, %esp
	movl %esp, %cr3

	// Now for the big fun: Long Mode.
	// Once again we put the data structures inline in this
	// memory. This code is, we hope, PIC.
lme:
	movl	%cr4, %eax
	andl	/*~$Pse*/$0xffffffef, %eax			/* Page Size */
	orl	$0x60, %eax		/* Page Global, Phys. Address */
	movl	%eax, %cr4
	#define Efer  0xC0000080
	#define Lme (1<<8)
	movl	$0xc0000080, %ecx			/* Extended Feature Enable */
	RDMSR
	ORL	$(1<<8), %eax			/* Long Mode Enable */
	WRMSR

//	NOTE: I did the test below for pml4 and pml3.
//	PML4[0] is 76ff2003
//	pml3[8] is 76ff3003

//	movl $pml3, %eax
//	movl	8(%eax), %eax
//	shrl $24, %eax
//	outb    %al, $0x80
//1: jmp 1b
  movw $0x3f8, %dx /* Print num */
	movl	%cr0, %eax
	// yeah yeah repeat defines. It's ok. They've been constant for almost 40 years.
	// view screen scrape from the docs. Includes of 40-year-old constants are a PITA.
	#define PE 1       //Protected Mode Enable         If 1, system is in protected mode, else system is in real mode
	#define MP 2       //Monitor co-processor  Controls interaction of WAIT/FWAIT instructions with TS flag in CR0
	#define EM 4            //Emulation     If set, no x87 floating-point unit present, if clear, x87 FPU present
	#define TS 8            //Task switched         Allows saving x87 task context upon a task switch only after x87 instruction used
	#define ET 0x10         //Extension type        On the 386, it allowed to specify whether the external math coprocessor was an 80287 or 80387
	#define NE 0x20         //Numeric error         Enable internal x87 floating point error reporting when set, else enables PC style x87 error detection
	#define WP 0x10000      //Write protect         When set, the CPU can't write to read-only pages when privilege level is 0
	#define AM 0x40000        //Alignment mask        Alignment check enabled if AM set, AC flag (in EFLAGS register) set, and privilege level is 3
	#define NW 0x20000000     //Not-write through     Globally enables/disable write-through caching
	#define CD 0x40000000     //Cache disable         Globally enables/disable the memory cache
	#define PG 0x80000000     //Paging        If 1, enable paging and use the § CR3 register, else disable paging.
	#define CDNWTSMP 0x6000000a
	//ANDL	/*$~(CD|NW|TS|MP)*/$~0x6000000a, %eax
	//ORL	/*$(PG|WP)*/$0x80010000, %eax			/* Paging Enable */
	ORL	$0x80000000, %eax			/* Paging Enable */
	movl	%eax, %cr0
	ljmp $0x28, $_identity


	/* Long mode. Welcome to 2003.
	 * (TODO maybe): load a proper long mode GDT. */
.code64

_identity:
	movb $0x34, %al
	outb %al, $0x80
# uncomment for debugging :)
# 3:
# movb $0x30, %al
# outb %al, $0x80
# movb $0x31, %al
# outb	%al, $0x80
# jmp 3b # jump to first label named 3 backwards
	call _start
	movb $0xff, %al
	outb %al, $0x80
1:	 jmp 1b

	// SELF files are 32-bits. This is coreboot legacy. But it's also legacy legacy: lots of kernels
	// have 32-bit entry points. So to keep it simple we will support them.
	// See: https://www.codeproject.com/Articles/45788/The-Real-Protected-Long-mode-assembly-tutorial-for
	// comments in ;; are from teh original
	.globl run32
run32:
	movb $0xb0, %al
	outb %al, $0x80
//1:	jmp 1b
	xorq %rcx,%rcx
	// fuck you rust assembler.
	// it just is not able to do what is needed.
	// so we go with this shit.
	//pushw $0x18
	//pushw $0x76ff
	//pushw $0x40cb
	movl $Back32, %ecx  //; The address must be an 64-bit address,
			//; so upper 32-bits of RCX are zero.
	push %rcx
	movw $0x18, 4(%rsp)
	movb 0(%rsp), %al
	outb %al, $0x80
	movb 1(%rsp), %al
	outb %al, $0x80
	movb 2(%rsp), %al
	outb %al, $0x80
	movb 3(%rsp), %al
	outb %al, $0x80
	movb 4(%rsp), %al
	outb %al, $0x80
	movb 5(%rsp), %al
	outb %al, $0x80
	movb 6(%rsp), %al
	outb %al, $0x80
	movb 7(%rsp), %al
	outb %al, $0x80
	movb 8(%rsp), %al
	outb %al, $0x80
	movb 9(%rsp), %al
	outb %al, $0x80
	movb 10(%rsp), %al
	outb %al, $0x80
	movb 11(%rsp), %al
	outb %al, $0x80
//	movq %rsp, %rsi
//	call _asdebug
	movb $0xee, %al
	outb %al, $0x80
	lret
.code32
Back32:
				//; We are now in Compatibility mode again
	movb $0xb1, %al
	outb %al, $0x80
	movw	$0x20, %ax
	movw	%ax, %ds
	movw	%ax, %es
	movw	%ax, %ss
	//  this seems unlikely to be needed
	// mov ax,data16_idx
	movw	%ax, %fs
	movw	%ax, %gs

	mov $0x2000, %esp
				// ; Disable Paging to get out of Long Mode
	movb $0xb2, %al
	outb %al, $0x80
	movl %cr0, %eax // ; Read CR0.
	andl $0x7fffffff, %eax // ; Set PE=0.
	movl %eax, %cr0 // ; Write CR0.
	movb $0xb3, %al
	outb %al, $0x80

				// ; Deactivate Long Mode
	movl $0xc0000080, %ecx // ; EFER MSR number.
	rdmsr // ; Read EFER.
	btc $8, %eax // ; Set LME=0.
	wrmsr // ; Write EFER.
	movb $0xb4, %al
	outb %al, $0x80

	// The address to jump to was in %rdx
	//; Back to the dirty, old, protected mode :(
	// The dtb pointer is in %esi
	//call _asdebug
	movb $0xb5, %al
	outb %al, $0x80
	movl %esi, %eax
	movl $0x1BADB002, %edi
	outb %al, $0x80
	shr $8, %eax
	outb %al, $0x80
	shr $8, %eax
	outb %al, $0x80
	shr $8, %eax
	outb %al, $0x80
	movl %esi, %eax
	movl %edx, %esi
	jmp *%eax
	movb $0xdb, %al
	outb %al, $0x80
1: jmp 1b
	jmp *%eax
#if 0
	.section ".reset", "ax", %progbits
	// TODO: get rid of this shit and stop using the linker to assemble rom images.
	// It did not end well.
//	.code16
.globl _boot
_boot:
.globl	_resetvector
_resetvector:
	.byte  0xe9
//	.int   _start16bit - ( . + 2 )
	/* Note: The above jump is hand coded to work around bugs in binutils.
	 * 5 byte are used for a 3 byte instruction.  This works because x86
	 * is little endian and allows us to use supported 32bit relocations
	 * instead of the weird 16 bit relocations that binutils does not
	 * handle consistently between versions because they are used so rarely.
	*/
	// This id padding to get us properly sized. That way we don't care that
	// our tools tend to load us, ARM-style, at the front of a region, rather
	// than the back, x86-style (ARM roms are a 0; x86 at the top of 4 GiB).
	.byte 0,0,0,0,0,0,0,0,0,0,0
	.previous
#endif
