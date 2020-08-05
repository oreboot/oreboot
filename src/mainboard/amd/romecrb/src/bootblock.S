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
.globl gdtptr
gdtptr:
	.word	gdt_end - gdt -1 /* compute the table limit */
	.long	gdt		 /* we know the offset */

	.align	4
gdt:
	/* selgdt 0, unused */
	.word	0x0000, 0x0000		/* dummy */
	.byte	0x00, 0x00, 0x00, 0x00

	/* selgdt 0x08, flat code segment */
	.word	0xffff, 0x0000
	.byte	0x00, 0x9b, 0xcf, 0x00 /* G=1 and 0x0f, So we get 4Gbytes
					  for limit */
	/* selgdt 0x10,flat data segment */
	.word	0xffff, 0x0000
	.byte	0x00, 0x93, 0xcf, 0x00

	/* long mode code segment. */
	.quad	0x0020980000000000		/* Long mode CS */
	/* selgdt 0x18, flat code segment (64-bit) */
	.word   0xffff, 0x0000
	.byte   0x00, 0x9b, 0xaf, 0x00

gdt_end:
.globl gdtptr16
gdtptr16:
	.word	gdt_end - gdt -1 /* compute the table limit */
	.long	gdt		 /* we know the offset */
3:
	// TOS contains a pointer to the gdt descriptor
	popl %eax
	lgdt (%eax)
	// Now fix up the page table
	movl $pml3, %eax
	orl $3, %eax
	movl %eax, pml4
	outb %al, $0x80
	movl $pml3, %eax
	movl $pml2, %ebx
	orl $3, %ebx
	movl  %ebx, 8(%eax)
	movl	%cr0, %eax
	andl	$0x7FFAFFD1, %eax /* PG,AM,WP,NE,TS,EM,MP = 0 */
	orl	$0x60000001, %eax /* CD, NW, PE = 1 */
	movl	%eax, %cr0

	movb $0xbe, %al
	outb %al, $0x80
	/* Now that we are in protected mode jump to a 32 bit code segment. */
	ljmpl	$8, $__protected_start

	// TODO: should set accessed and dirty bits in gdt entries
	// so CPU does not try to write them to ROM?
	.align 0x1000
pml4:
	.quad 0
	.align 0x1000
pml3:
	.quad 0x0000083,0x40000083,0x80000083,0xc0000083
	.align 0x1000
pml2:
	.quad 0x40000083,0x40200083,0x40400083,0x40600083,0x40800083,0x40a00083,0x40c00083,0x40e00083
	.quad 0x41000083,0x41200083,0x41400083,0x41600083,0x41800083,0x41a00083,0x41c00083,0x41e00083
	.quad 0x42000083,0x42200083,0x42400083,0x42600083,0x42800083,0x42a00083,0x42c00083,0x42e00083
	.quad 0x43000083,0x43200083,0x43400083,0x43600083,0x43800083,0x43a00083,0x43c00083,0x43e00083
	.quad 0x44000083,0x44200083,0x44400083,0x44600083,0x44800083,0x44a00083,0x44c00083,0x44e00083
	.quad 0x45000083,0x45200083,0x45400083,0x45600083,0x45800083,0x45a00083,0x45c00083,0x45e00083
	.quad 0x46000083,0x46200083,0x46400083,0x46600083,0x46800083,0x46a00083,0x46c00083,0x46e00083
	.quad 0x47000083,0x47200083,0x47400083,0x47600083,0x47800083,0x47a00083,0x47c00083,0x47e00083
	.quad 0x48000083,0x48200083,0x48400083,0x48600083,0x48800083,0x48a00083,0x48c00083,0x48e00083
	.quad 0x49000083,0x49200083,0x49400083,0x49600083,0x49800083,0x49a00083,0x49c00083,0x49e00083
	.quad 0x4a000083,0x4a200083,0x4a400083,0x4a600083,0x4a800083,0x4aa00083,0x4ac00083,0x4ae00083
	.quad 0x4b000083,0x4b200083,0x4b400083,0x4b600083,0x4b800083,0x4ba00083,0x4bc00083,0x4be00083
	.quad 0x4c000083,0x4c200083,0x4c400083,0x4c600083,0x4c800083,0x4ca00083,0x4cc00083,0x4ce00083
	.quad 0x4d000083,0x4d200083,0x4d400083,0x4d600083,0x4d800083,0x4da00083,0x4dc00083,0x4de00083
	.quad 0x4e000083,0x4e200083,0x4e400083,0x4e600083,0x4e800083,0x4ea00083,0x4ec00083,0x4ee00083
	.quad 0x4f000083,0x4f200083,0x4f400083,0x4f600083,0x4f800083,0x4fa00083,0x4fc00083,0x4fe00083
	.quad 0x50000083,0x50200083,0x50400083,0x50600083,0x50800083,0x50a00083,0x50c00083,0x50e00083
	.quad 0x51000083,0x51200083,0x51400083,0x51600083,0x51800083,0x51a00083,0x51c00083,0x51e00083
	.quad 0x52000083,0x52200083,0x52400083,0x52600083,0x52800083,0x52a00083,0x52c00083,0x52e00083
	.quad 0x53000083,0x53200083,0x53400083,0x53600083,0x53800083,0x53a00083,0x53c00083,0x53e00083
	.quad 0x54000083,0x54200083,0x54400083,0x54600083,0x54800083,0x54a00083,0x54c00083,0x54e00083
	.quad 0x55000083,0x55200083,0x55400083,0x55600083,0x55800083,0x55a00083,0x55c00083,0x55e00083
	.quad 0x56000083,0x56200083,0x56400083,0x56600083,0x56800083,0x56a00083,0x56c00083,0x56e00083
	.quad 0x57000083,0x57200083,0x57400083,0x57600083,0x57800083,0x57a00083,0x57c00083,0x57e00083
	.quad 0x58000083,0x58200083,0x58400083,0x58600083,0x58800083,0x58a00083,0x58c00083,0x58e00083
	.quad 0x59000083,0x59200083,0x59400083,0x59600083,0x59800083,0x59a00083,0x59c00083,0x59e00083
	.quad 0x5a000083,0x5a200083,0x5a400083,0x5a600083,0x5a800083,0x5aa00083,0x5ac00083,0x5ae00083
	.quad 0x5b000083,0x5b200083,0x5b400083,0x5b600083,0x5b800083,0x5ba00083,0x5bc00083,0x5be00083
	.quad 0x5c000083,0x5c200083,0x5c400083,0x5c600083,0x5c800083,0x5ca00083,0x5cc00083,0x5ce00083
	.quad 0x5d000083,0x5d200083,0x5d400083,0x5d600083,0x5d800083,0x5da00083,0x5dc00083,0x5de00083
	.quad 0x5e000083,0x5e200083,0x5e400083,0x5e600083,0x5e800083,0x5ea00083,0x5ec00083,0x5ee00083
	.quad 0x5f000083,0x5f200083,0x5f400083,0x5f600083,0x5f800083,0x5fa00083,0x5fc00083,0x5fe00083
	.quad 0x60000083,0x60200083,0x60400083,0x60600083,0x60800083,0x60a00083,0x60c00083,0x60e00083
	.quad 0x61000083,0x61200083,0x61400083,0x61600083,0x61800083,0x61a00083,0x61c00083,0x61e00083
	.quad 0x62000083,0x62200083,0x62400083,0x62600083,0x62800083,0x62a00083,0x62c00083,0x62e00083
	.quad 0x63000083,0x63200083,0x63400083,0x63600083,0x63800083,0x63a00083,0x63c00083,0x63e00083
	.quad 0x64000083,0x64200083,0x64400083,0x64600083,0x64800083,0x64a00083,0x64c00083,0x64e00083
	.quad 0x65000083,0x65200083,0x65400083,0x65600083,0x65800083,0x65a00083,0x65c00083,0x65e00083
	.quad 0x66000083,0x66200083,0x66400083,0x66600083,0x66800083,0x66a00083,0x66c00083,0x66e00083
	.quad 0x67000083,0x67200083,0x67400083,0x67600083,0x67800083,0x67a00083,0x67c00083,0x67e00083
	.quad 0x68000083,0x68200083,0x68400083,0x68600083,0x68800083,0x68a00083,0x68c00083,0x68e00083
	.quad 0x69000083,0x69200083,0x69400083,0x69600083,0x69800083,0x69a00083,0x69c00083,0x69e00083
	.quad 0x6a000083,0x6a200083,0x6a400083,0x6a600083,0x6a800083,0x6aa00083,0x6ac00083,0x6ae00083
	.quad 0x6b000083,0x6b200083,0x6b400083,0x6b600083,0x6b800083,0x6ba00083,0x6bc00083,0x6be00083
	.quad 0x6c000083,0x6c200083,0x6c400083,0x6c600083,0x6c800083,0x6ca00083,0x6cc00083,0x6ce00083
	.quad 0x6d000083,0x6d200083,0x6d400083,0x6d600083,0x6d800083,0x6da00083,0x6dc00083,0x6de00083
	.quad 0x6e000083,0x6e200083,0x6e400083,0x6e600083,0x6e800083,0x6ea00083,0x6ec00083,0x6ee00083
	.quad 0x6f000083,0x6f200083,0x6f400083,0x6f600083,0x6f800083,0x6fa00083,0x6fc00083,0x6fe00083
	.quad 0x70000083,0x70200083,0x70400083,0x70600083,0x70800083,0x70a00083,0x70c00083,0x70e00083
	.quad 0x71000083,0x71200083,0x71400083,0x71600083,0x71800083,0x71a00083,0x71c00083,0x71e00083
	.quad 0x72000083,0x72200083,0x72400083,0x72600083,0x72800083,0x72a00083,0x72c00083,0x72e00083
	.quad 0x73000083,0x73200083,0x73400083,0x73600083,0x73800083,0x73a00083,0x73c00083,0x73e00083
	.quad 0x74000083,0x74200083,0x74400083,0x74600083,0x74800083,0x74a00083,0x74c00083,0x74e00083
	.quad 0x75000083,0x75200083,0x75400083,0x75600083,0x75800083,0x75a00083,0x75c00083,0x75e00083
	.quad 0x76000083,0x76200083,0x76400083,0x76600083,0x76800083,0x76a00083,0x76c00083,0x76e00083
	.quad 0x77000083,0x77200083,0x77400083,0x77600083,0x77800083,0x77a00083,0x77c00083,0x77e00083
	.quad 0x78000083,0x78200083,0x78400083,0x78600083,0x78800083,0x78a00083,0x78c00083,0x78e00083
	.quad 0x79000083,0x79200083,0x79400083,0x79600083,0x79800083,0x79a00083,0x79c00083,0x79e00083
	.quad 0x7a000083,0x7a200083,0x7a400083,0x7a600083,0x7a800083,0x7aa00083,0x7ac00083,0x7ae00083
	.quad 0x7b000083,0x7b200083,0x7b400083,0x7b600083,0x7b800083,0x7ba00083,0x7bc00083,0x7be00083
	.quad 0x7c000083,0x7c200083,0x7c400083,0x7c600083,0x7c800083,0x7ca00083,0x7cc00083,0x7ce00083
	.quad 0x7d000083,0x7d200083,0x7d400083,0x7d600083,0x7d800083,0x7da00083,0x7dc00083,0x7de00083
	.quad 0x7e000083,0x7e200083,0x7e400083,0x7e600083,0x7e800083,0x7ea00083,0x7ec00083,0x7ee00083
	.quad 0x7f000083,0x7f200083,0x7f400083,0x7f600083,0x7f800083,0x7fa00083,0x7fc00083,0x7fe00083
.code32
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

	movw	$16, %ax
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
	movb $0x31, %al
	outb %al, $0x80
	movl	%cr4, %eax
	andl	/*~$Pse*/$0xffffffef, %eax			/* Page Size */
	orl	$0x60, %eax		/* Page Global, Phys. Address */
	movl	%eax, %cr4
	movb $0x32, %al
	outb %al, $0x80
	#define Efer  0xC0000080
	#define Lme (1<<8)
	movl	$0xc0000080, %ecx			/* Extended Feature Enable */
	RDMSR
	ORL	$(1<<8), %eax			/* Long Mode Enable */
	WRMSR

	movl	pml4, %eax
	outb    %al, $0x80
1: jmp 1b
	movb $0x33, %al
	outb %al, $0x80
	movl	%cr0, %eax
	// yeah yeah repeat defines. It's ok. They've been constant for almost 40 years.
	// view screen scrape from the docs. Includes of 40-year-old constants are a PITA.
	#define PE 1       //Protected Mode Enable         If 1, system is in protected mode, else system is in real mode
	#define MP 2       //Monitor co-processor  Controls interaction of WAIT/FWAIT instructions with TS flag in CR0
	#define EM 4       //Emulation     If set, no x87 floating-point unit present, if clear, x87 FPU present
	#define TS 8       //Task switched         Allows saving x87 task context upon a task switch only after x87 instruction used
	#define ET 0x10       //Extension type        On the 386, it allowed to specify whether the external math coprocessor was an 80287 or 80387
	#define NE 0x20       //Numeric error         Enable internal x87 floating point error reporting when set, else enables PC style x87 error detection
	#define WP 0x10000      //Write protect         When set, the CPU can't write to read-only pages when privilege level is 0
	#define AM 0x40000      //Alignment mask        Alignment check enabled if AM set, AC flag (in EFLAGS register) set, and privilege level is 3
	#define NW 0x20000000     //Not-write through     Globally enables/disable write-through caching
	#define CD 0x40000000      //Cache disable         Globally enables/disable the memory cache
	#define PG 0x80000000      //Paging        If 1, enable paging and use the § CR3 register, else disable paging.
	#define CDNWTSMP 0x6000000a
	//ANDL	/*$~(CD|NW|TS|MP)*/$~0x6000000a, %eax
	//ORL	/*$(PG|WP)*/$0x80010000, %eax			/* Paging Enable */
	ORL	$0x80000000, %eax			/* Paging Enable */
	movl	%eax, %cr0
	ljmp $0x18, $_identity


	/* Long mode. Welcome to 2003.
	 * (TODO maybe): load a proper long mode GDT. */
.code64

_identity:
	movb $0x34, %al
	outb %al, $0x80
1:	jmp 1b
	call _start
1:	 jmp 1b

	
#if 0
	.section ".reset", "ax", %progbits
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
