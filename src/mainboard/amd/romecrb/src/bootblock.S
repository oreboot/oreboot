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
	.quad 0x0
	.align 0x1000
pml3:
	.quad 0x00000083,0x400000083,0x800000083,0xc00000083
	.align 0x1000
pml2:
	.quad 0x40000003,0x40200003,0x40400003,0x40600003,0x40800003,0x40a00003,0x40c00003,0x40e00003
	.quad 0x41000003,0x41200003,0x41400003,0x41600003,0x41800003,0x41a00003,0x41c00003,0x41e00003
	.quad 0x42000003,0x42200003,0x42400003,0x42600003,0x42800003,0x42a00003,0x42c00003,0x42e00003
	.quad 0x43000003,0x43200003,0x43400003,0x43600003,0x43800003,0x43a00003,0x43c00003,0x43e00003
	.quad 0x44000003,0x44200003,0x44400003,0x44600003,0x44800003,0x44a00003,0x44c00003,0x44e00003
	.quad 0x45000003,0x45200003,0x45400003,0x45600003,0x45800003,0x45a00003,0x45c00003,0x45e00003
	.quad 0x46000003,0x46200003,0x46400003,0x46600003,0x46800003,0x46a00003,0x46c00003,0x46e00003
	.quad 0x47000003,0x47200003,0x47400003,0x47600003,0x47800003,0x47a00003,0x47c00003,0x47e00003
	.quad 0x48000003,0x48200003,0x48400003,0x48600003,0x48800003,0x48a00003,0x48c00003,0x48e00003
	.quad 0x49000003,0x49200003,0x49400003,0x49600003,0x49800003,0x49a00003,0x49c00003,0x49e00003
	.quad 0x4a000003,0x4a200003,0x4a400003,0x4a600003,0x4a800003,0x4aa00003,0x4ac00003,0x4ae00003
	.quad 0x4b000003,0x4b200003,0x4b400003,0x4b600003,0x4b800003,0x4ba00003,0x4bc00003,0x4be00003
	.quad 0x4c000003,0x4c200003,0x4c400003,0x4c600003,0x4c800003,0x4ca00003,0x4cc00003,0x4ce00003
	.quad 0x4d000003,0x4d200003,0x4d400003,0x4d600003,0x4d800003,0x4da00003,0x4dc00003,0x4de00003
	.quad 0x4e000003,0x4e200003,0x4e400003,0x4e600003,0x4e800003,0x4ea00003,0x4ec00003,0x4ee00003
	.quad 0x4f000003,0x4f200003,0x4f400003,0x4f600003,0x4f800003,0x4fa00003,0x4fc00003,0x4fe00003
	.quad 0x50000003,0x50200003,0x50400003,0x50600003,0x50800003,0x50a00003,0x50c00003,0x50e00003
	.quad 0x51000003,0x51200003,0x51400003,0x51600003,0x51800003,0x51a00003,0x51c00003,0x51e00003
	.quad 0x52000003,0x52200003,0x52400003,0x52600003,0x52800003,0x52a00003,0x52c00003,0x52e00003
	.quad 0x53000003,0x53200003,0x53400003,0x53600003,0x53800003,0x53a00003,0x53c00003,0x53e00003
	.quad 0x54000003,0x54200003,0x54400003,0x54600003,0x54800003,0x54a00003,0x54c00003,0x54e00003
	.quad 0x55000003,0x55200003,0x55400003,0x55600003,0x55800003,0x55a00003,0x55c00003,0x55e00003
	.quad 0x56000003,0x56200003,0x56400003,0x56600003,0x56800003,0x56a00003,0x56c00003,0x56e00003
	.quad 0x57000003,0x57200003,0x57400003,0x57600003,0x57800003,0x57a00003,0x57c00003,0x57e00003
	.quad 0x58000003,0x58200003,0x58400003,0x58600003,0x58800003,0x58a00003,0x58c00003,0x58e00003
	.quad 0x59000003,0x59200003,0x59400003,0x59600003,0x59800003,0x59a00003,0x59c00003,0x59e00003
	.quad 0x5a000003,0x5a200003,0x5a400003,0x5a600003,0x5a800003,0x5aa00003,0x5ac00003,0x5ae00003
	.quad 0x5b000003,0x5b200003,0x5b400003,0x5b600003,0x5b800003,0x5ba00003,0x5bc00003,0x5be00003
	.quad 0x5c000003,0x5c200003,0x5c400003,0x5c600003,0x5c800003,0x5ca00003,0x5cc00003,0x5ce00003
	.quad 0x5d000003,0x5d200003,0x5d400003,0x5d600003,0x5d800003,0x5da00003,0x5dc00003,0x5de00003
	.quad 0x5e000003,0x5e200003,0x5e400003,0x5e600003,0x5e800003,0x5ea00003,0x5ec00003,0x5ee00003
	.quad 0x5f000003,0x5f200003,0x5f400003,0x5f600003,0x5f800003,0x5fa00003,0x5fc00003,0x5fe00003
	.quad 0x60000003,0x60200003,0x60400003,0x60600003,0x60800003,0x60a00003,0x60c00003,0x60e00003
	.quad 0x61000003,0x61200003,0x61400003,0x61600003,0x61800003,0x61a00003,0x61c00003,0x61e00003
	.quad 0x62000003,0x62200003,0x62400003,0x62600003,0x62800003,0x62a00003,0x62c00003,0x62e00003
	.quad 0x63000003,0x63200003,0x63400003,0x63600003,0x63800003,0x63a00003,0x63c00003,0x63e00003
	.quad 0x64000003,0x64200003,0x64400003,0x64600003,0x64800003,0x64a00003,0x64c00003,0x64e00003
	.quad 0x65000003,0x65200003,0x65400003,0x65600003,0x65800003,0x65a00003,0x65c00003,0x65e00003
	.quad 0x66000003,0x66200003,0x66400003,0x66600003,0x66800003,0x66a00003,0x66c00003,0x66e00003
	.quad 0x67000003,0x67200003,0x67400003,0x67600003,0x67800003,0x67a00003,0x67c00003,0x67e00003
	.quad 0x68000003,0x68200003,0x68400003,0x68600003,0x68800003,0x68a00003,0x68c00003,0x68e00003
	.quad 0x69000003,0x69200003,0x69400003,0x69600003,0x69800003,0x69a00003,0x69c00003,0x69e00003
	.quad 0x6a000003,0x6a200003,0x6a400003,0x6a600003,0x6a800003,0x6aa00003,0x6ac00003,0x6ae00003
	.quad 0x6b000003,0x6b200003,0x6b400003,0x6b600003,0x6b800003,0x6ba00003,0x6bc00003,0x6be00003
	.quad 0x6c000003,0x6c200003,0x6c400003,0x6c600003,0x6c800003,0x6ca00003,0x6cc00003,0x6ce00003
	.quad 0x6d000003,0x6d200003,0x6d400003,0x6d600003,0x6d800003,0x6da00003,0x6dc00003,0x6de00003
	.quad 0x6e000003,0x6e200003,0x6e400003,0x6e600003,0x6e800003,0x6ea00003,0x6ec00003,0x6ee00003
	.quad 0x6f000003,0x6f200003,0x6f400003,0x6f600003,0x6f800003,0x6fa00003,0x6fc00003,0x6fe00003
	.quad 0x70000003,0x70200003,0x70400003,0x70600003,0x70800003,0x70a00003,0x70c00003,0x70e00003
	.quad 0x71000003,0x71200003,0x71400003,0x71600003,0x71800003,0x71a00003,0x71c00003,0x71e00003
	.quad 0x72000003,0x72200003,0x72400003,0x72600003,0x72800003,0x72a00003,0x72c00003,0x72e00003
	.quad 0x73000003,0x73200003,0x73400003,0x73600003,0x73800003,0x73a00003,0x73c00003,0x73e00003
	.quad 0x74000003,0x74200003,0x74400003,0x74600003,0x74800003,0x74a00003,0x74c00003,0x74e00003
	.quad 0x75000003,0x75200003,0x75400003,0x75600003,0x75800003,0x75a00003,0x75c00003,0x75e00003
	.quad 0x76000003,0x76200003,0x76400003,0x76600003,0x76800003,0x76a00003,0x76c00003,0x76e00003
	.quad 0x77000003,0x77200003,0x77400003,0x77600003,0x77800003,0x77a00003,0x77c00003,0x77e00003
	.quad 0x78000003,0x78200003,0x78400003,0x78600003,0x78800003,0x78a00003,0x78c00003,0x78e00003
	.quad 0x79000003,0x79200003,0x79400003,0x79600003,0x79800003,0x79a00003,0x79c00003,0x79e00003
	.quad 0x7a000003,0x7a200003,0x7a400003,0x7a600003,0x7a800003,0x7aa00003,0x7ac00003,0x7ae00003
	.quad 0x7b000003,0x7b200003,0x7b400003,0x7b600003,0x7b800003,0x7ba00003,0x7bc00003,0x7be00003
	.quad 0x7c000003,0x7c200003,0x7c400003,0x7c600003,0x7c800003,0x7ca00003,0x7cc00003,0x7ce00003
	.quad 0x7d000003,0x7d200003,0x7d400003,0x7d600003,0x7d800003,0x7da00003,0x7dc00003,0x7de00003
	.quad 0x7e000003,0x7e200003,0x7e400003,0x7e600003,0x7e800003,0x7ea00003,0x7ec00003,0x7ee00003
	.quad 0x7f000003,0x7f200003,0x7f400003,0x7f600003,0x7f800003,0x7fa00003,0x7fc00003,0x7fe00003
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

	movb $0x33, %al
	outb %al, $0x80
	movl	%cr0, %edx
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
	//ANDL	/*$~(CD|NW|TS|MP)*/$~0x6000000a, %edx
	//ORL	/*$(PG|WP)*/$0x80010000, %edx			/* Paging Enable */
	ORL	$0x80000000, %edx			/* Paging Enable */
	movl	%edx, %cr0
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
