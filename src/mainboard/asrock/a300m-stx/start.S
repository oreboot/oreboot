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
.code16
1:
// Super I/O config
// enter extended function mode
movb $0x87, %al
outb %al, $0x2e
outb %al, $0x2e

# 2a -> 43; pin13-pin20 are COM A
movb $0x2a, %al ##
outb %al, $0x2e
movb $0x43, %al ##
outb %al, $0x2f

# 2b -> 08; pin 38 is GP33
movb $0x2b, %al ##
outb %al, $0x2e
movb $0x08, %al ##
outb %al, $0x2f

# from stock firmware; pin 34 is GP33, pin 58 is TSIC
movb $0x2c, %al ##
outb %al, $0x2e
movb $0x21, %al ##
outb %al, $0x2f

# 2f -> 40; uncomment to switch to port 0x80 forward instead of 3f8 on UART
movb $0x2f, %al ##
outb %al, $0x2e
movb $0x40, %al ##
outb %al, $0x2f

######## LDN 0x07 ########
## not yet working, probably relies on some global registers
# movb $0x07, %al ## command for device selection
# outb %al, $0x2e
# movb $0x07, %al ## select logical device 0x07: GPIOs 7 and 8
# outb %al, $0x2f
#
# movb $0x30, %al ## control register 0x30: GPIO activation
# outb %al, $0x2e
# movb $0x02, %al ## does this activate GPIO 7?
# outb %al, $0x2f
#
# movb $0xe0, %al ## control register 0xE0: GPIO 7 I/O
# outb %al, $0x2e
# movb $0xef, %al ## set value 1110 1111: GPIO 74 is output
# outb %al, $0x2f
#
# movb $0xec, %al ## control register 0xEC: LED function config
# outb %al, $0x2e
# movb $0x10, %al ## 0001 0000 - GPIO 74 is GRN LED
# outb %al, $0x2f

// exit extended function mode
movb $0xaa, %al
outb %al, $0x2e
// 1, 2, 3.....
3:
movb $0x31, %al
outb	%al, $0x80
movb $0x32, %al
outb	%al, $0x80
movb $0x33, %al
outb	%al, $0x80
# jmp 3b ## uncomment for infinite port 0x80 loop

jmp 2f

// This is as far down as we can go?
.org 0xff72

.code16
/* Symbol _start16bit must be aligned to 4kB to start AP CPUs with
 * Startup IPI message without RAM.
*/
.globl _start16bit
.type _start16bit, @function

_start16bit:
jmp 1b
2:
#	cli
#movb $0xaa, %al
#outb	%al, $0x80
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
	jmp 1f # jump to next label named "1" forward

	/**
	 * The gdt is defined at the top of this file; it has a 4 GiB code segment
	 * at 0x08, and a 4 GiB data segment at 0x10;
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
	/* WE ARE GETTING HERE! */
	outb %al, $0x80
	// Uses first part of gdt as stack.
	// This jumps to the bootblock (bootblob.bin)
	movl $0x76fe0010, %esp
	push $0x76fe0000 # 0x76c00000 (dest) + 0x3e0000 (offset in fixed-dtfs.dts)
	# We patch the PSP dir entry to have it copy our image to 0x76c00000 in RAM
	ret

# debug the crap
# 3:
# movb $0x30, %al
# outb %al, $0x80
# movb $0x31, %al
# outb	%al, $0x80
# jmp 3b # first label named 3 backwards
