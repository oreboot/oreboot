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
	/* format:
	 *   limit: word,
	 *   base_lo: word,
	 *   base_hi: byte,
	 *   access: byte, // bit 7: present; bit 6...5: privilege ring; bit 4: descriptor type (1 for code and data); bit 3: executable; bit 2: direction (0 is up--so base <= limit); bit 1: writable; bit 0: accessed flag
	 *   limit_hi: nibble,
	 *   flags: nibble, // bit 3: unit of base and limit (1: 4 KiB page; 0: 1 Byte); bit 2: protected mode kind (0: 16 bit protected mode; 1: 32 bit protected mode); bit 1: 0; bit 0: 0
	 *   base_highest: byte
	 */

	/* selgdt 0, unused */
	.word	0x0000     /* limit */
	.word	0x0000     /* base_lo */
	.byte	0x00       /* base_hi */
	.byte	0x00       /* access; so present=0 */
	.byte	0x00       /* high nibble: flags; low nibble: limit_hi; ignored since present=0 */
	.byte	0x00       /* base_highest */

	/* selgdt 0x08, flat code segment */
	.word	0xffff     /* limit */
	.word	0x0000     /* base_lo */
	.byte	0x00       /* base_hi */
	.byte	0b10011011 /* access; so present, code and data, executable, writable, accessed */
	.byte	0xcf       /* high nibble: flags; low nibble: limit_hi; so 32 bit protected mode, base and limit in units of pages, limit = 0xf_ffff--so we get 4 GiB */
	.byte	0x00       /* base_highest */

	/* selgdt 0x10,flat data segment */
	.word	0xffff     /* limit */
	.word	0x0000     /* base_lo */
	.byte	0x00       /* base_hi */
	.byte	0b10010011 /* access: so present, code and data, not executable, writable, accessed */
	.byte	0xcf       /* high nibble: flags; low nibble: limit_hi; so 32 bit protected mode, base and limit in units of pages, limit = 0xf_ffff--so we get 4 GiB */
	.byte	0x00       /* base_highest */
gdt_end:

// This is as far down as we can go?
.org 0xff72

.code16
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

	/*
	 * Also load an IDT with NULL limit to prevent the 16bit IDT being used
	 * in protected mode before c_start.S sets up a 32bit IDT when entering
	 * RAM stage. In practise: CPU will shutdown on any exception.
	 * See IA32 manual Vol 3A 19.26 Interrupts.
	 *
	 * movw	%cs, %ax
	 * shlw	$4, %ax
	 * movw	$0xffe8, %bx
	 * subw	%ax, %bx
	 * lidt	%cs:(%bx)
	 */

	movl	$0xffffffe8, %ebx
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
	ljmpl	$8, $0xffffffa8
.org 0xffa8
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

	// Give bootstrap core and thread a bigger stack (32 KiB)
	// Added 0x500 is in order to avoid the BIOS data area, then an extra 0x500 in order to page-align the stack again
	mov $0x9000, %esp

	// Enable APIC (in xAPIC mode) so we can read out the APIC_ID from MMIO at $0xFEE0_0020
	movl   $0x1b, %ecx
	rdmsr
	orl $0x800, %eax
	wrmsr

	// Check whether we are running on the bootstrap core (and thread)
	test $0x100, %eax
	jnz 3f

	// Note: Non-bootstrap cores reach this place

	// See coreboot:src/cpu/x86/smm/smihandler.c function "nodeid".
	// APICx020: bits 31...24 (8 bits): APIC_ID; APIC defaults to 0xFEE0_0000
	movl ($0xFEE0_0020), %ebx
	shr $24, %ebx
	inc %ebx

	// Give other cores and threads a smaller stack (2 KiB)
	shl $11, %ebx
	add %ebx, %esp

	// Note: All the stacks are below address 0x8_9000
	// Video memory via MMIO is eventually mapped at 0xA_0000--avoid area
3:

	/* Restore the BIST value to %eax */
	movl	%ebp, %eax
	jmp 1f

	/**
	 * The gdt is defined in entry32.inc, it has a 4 Gb code segment
	 * at 0x08, and a 4 GB data segment at 0x10;
	 */
.org 0xFFe8
9:
.globl gdtptr16
gdtptr16:
	.word	0x1f			 /* gdt limit (size - 1) */
	.long	0xffffff00		 /* offset */
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

	push $0x76f08000
	ret
