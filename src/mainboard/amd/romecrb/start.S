/*
 * This software and ancillary information (herein called SOFTWARE)
 * called LinuxBIOS is made available under the terms described here.
 *
 * The SOFTWARE has been approved for release with associated
 * LA-CC Number 00-34. Unless otherwise indicated, this SOFTWARE has
 * been authored by an employee or employees of the University of
 * California, operator of the Los Alamos National Laboratory under
 * Contract No. W-7405-ENG-36 with the U.S. Department of Energy.
 *
 * The U.S. Government has rights to use, reproduce, and distribute this
 * SOFTWARE. The public may copy, distribute, prepare derivative works
 * and publicly display this SOFTWARE without charge, provided that this
 * Notice and any statement of authorship are reproduced on all copies.
 *
 * Neither the Government nor the University makes any warranty, express
 * or implied, or assumes any liability or responsibility for the use of
 * this SOFTWARE.  If SOFTWARE is modified to produce derivative works,
 * such modified SOFTWARE should be clearly marked, so as not to confuse
 * it with the version available from LANL.
 *
 */

// Zen starts with the bootblock in RAM, at an address not easily known
// to code, with a pretend segment of f000, and a pretend IP of fff0.
// Traditionally one has a jmp cs:ip instruction at said boot block;
// you can't do that because you're actually running in bigreal with
// a shadow CS set to 0x76000000 in one case.

// This is now designed to be standalone 16-bit blob, written to assume
// CS is 0xf0000, and it starts at the top 4K of "flash".
// It then jumps to 8:760000 using the classic push and ret.
// Due to the 4k alignment, we just eat the whole last page.
// This makes things easier: we can just place tables at .org 0x800,
// code at 0, and avoid coreboot's strenuous bootloader and gas games.

/* Start code to put an i386 or later processor into 32-bit protected mode.
 */

.org 0xff00
gdt:
	/* selgdt 0, unused */
	.word	0x0000, 0x0000		/* dummy */
	.byte	0x00, 0x00, 0x00, 0x00

	/* selgdt 0x08, flat code segment */
	.word	0xffff, 0x0000
	.byte	0x00, 0x9b, 0xcf, 0x00 /* G=1 and 0x0f, So we get 4Gbytes

	/* selgdt 0x10,flat data segment */
	.word	0xffff, 0x0000
	.byte	0x00, 0x93, 0xcf, 0x00
gdt_end:

// This is as far down as we can go?
.org 0xff72

.code16
/* Symbol _start16bit must be aligned to 4kB to start AP CPUs with
 * Startup IPI message without RAM.
*/
.globl _start16bit
.type _start16bit, @function

_start16bit:
	cli
movb $0xaa, %al
outb	%al, $0x80
	/* Save the BIST result */
	movl	%eax, %ebp

	/* IMMEDIATELY invalidate the translation lookaside buffer (TLB) before
	 * executing any further code. Even though paging is disabled we
	 * could still get false address translations due to the TLB if we
	 * didn't invalidate it. Thanks to kmliu@sis.com.tw for this TLB fix.
	 */

	xorl	%eax, %eax
	movl	%eax, %cr3    /* Invalidate TLB*/

	/* Invalidating the cache here seems to be a bad idea on
	 * modern processors.  Don't.
	 * If we are hyperthreaded or we have multiple cores it is bad,
	 * for SMP startup.  On Opterons it causes a 5 second delay.
	 * Invalidating the cache was pure paranoia in any event.
	 * If your CPU needs it you can write a CPU dependent version of
	 * entry16.inc.
	 */

	/* Note: gas handles memory addresses in 16 bit code very poorly.
	 * In particular it doesn't appear to have a directive allowing you
	 * associate a section or even an absolute offset with a segment register.
	 *
	 * This means that anything except cs:ip relative offsets are
	 * a real pain in 16 bit mode.  And explains why it is almost
	 * impossible to get gas to do lgdt correctly.
	 *
	 * One way to work around this is to have the linker do the
	 * math instead of the assembler.  This solves the very
	 * practical problem of being able to write code that can
	 * be relocated.
	 *
	 * An lgdt call before we have memory enabled cannot be
	 * position independent, as we cannot execute a call
	 * instruction to get our current instruction pointer.
	 * So while this code is relocatable it isn't arbitrarily
	 * relocatable.
	 *
	 * The criteria for relocation have been relaxed to their
	 * utmost, so that we can use the same code for both
	 * our initial entry point and startup of the second CPU.
	 * The code assumes when executing at _start16bit that:
	 * (((cs & 0xfff) == 0) and (ip == _start16bit & 0xffff))
	 * or
	 * ((cs == anything) and (ip == 0)).
	 *
	 * The restrictions in reset16.inc mean that _start16bit initially
	 * must be loaded at or above 0xffff0000 or below 0x100000.
	 *
	 * The linker scripts computes gdtptr16_offset by simply returning
	 * the low 16 bits.  This means that the initial segment used
	 * when start is called must be 64K aligned.  This should not
	 * restrict the address as the ip address can be anything.
	 *
	 * Also load an IDT with NULL limit to prevent the 16bit IDT being used
	 * in protected mode before c_start.S sets up a 32bit IDT when entering
	 * RAM stage. In practise: CPU will shutdown on any exception.
	 * See IA32 manual Vol 3A 19.26 Interrupts.
	 */

/*
	movw	%cs, %ax
	shlw	$4, %ax
	movw	$0xffe8, %bx
	subw	%ax, %bx
	lidt	%cs:(%bx)
*/
	movl	$0xffffffc8, %ebx
	// Leave it hand assembled. gas will NOT do the right thing.
	//lgdtl	%cs:(%bx)
	.byte 0x66, 0x2E, 0x0F, 0x01, 0x17
	movl	%cr0, %eax
	andl	$0x7FFAFFD1, %eax /* PG,AM,WP,NE,TS,EM,MP = 0 */
	orl	$0x60000001, %eax /* CD, NW, PE = 1 */
	movl	%eax, %cr0

	/* Restore BIST to %eax */
	movl	%ebp, %eax

	/* Now that we are in protected mode jump to a 32 bit code segment. */
	ljmpl	$8, $0xffffffb1
.org 0xffb1
.code32
protected:
	movb $0xac, %al
	outb	%al, $0x80
//1: jmp 1b
	movw	$0x10, %ax
	movw	%ax, %ds
	movw	%ax, %es
	movw	%ax, %ss
	movw	%ax, %fs
	movw	%ax, %gs

	/* Restore the BIST value to %eax */
	movl	%ebp, %eax
	jmp 1f

	/**
	 * The gdt is defined in entry32.inc, it has a 4 Gb code segment
	 * at 0x08, and a 4 GB data segment at 0x10;
	 */
.org 0xFFc8
9:
.globl gdtptr16
gdtptr16:
	.word	0x1f			  /* compute the table limit */
	.long	0xffffff00		 /* we know the offset */
.align	4
.org 0xfff0
	// This a hand-assembled jump back as far as possible
	.byte 0xeb, 0x80
.code32
// Make the jmp be the last bytes -- makes patching it easy
1:
//	nop
	nop
	outb %al, $0x80
	// Uses first part of gdt as stack.
	movl $0x76fe0010, %esp
	push $0x76fe0000
	ret
