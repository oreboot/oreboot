#define REGBYTES        4

// NOTE: `mxstatus` is a custom T-Head register. Do not confuse with `mstatus`.
// It allows for configuring special eXtensions. See further below for details.
/*mxstatus (0x7c0)*/
// MAEE (Memory Attribute Extension Enable)
#define    MAEE             (0x1 << 21)
// EN_THEADISAEE (T-Head ISA Extension Enable)
#define    EN_THEADISAEE    (0x1 << 22)

/*
see https://linux-sunxi.org/D1 for the C906 manual
original document is in Chinese, translated by Google here for reference

p19:

2. In all cases where virtual address and physical address conversion are
performed: when the authority is not in machine mode and the MMU is turned on,
there are two ways to configure the page properties of the address.
Type: sysmap.h file and page attributes expanded by C906 in PTE.
Which configuration method is used depends on the C906 extension register
MXSTATUS.
The value of the MAEE field in. If the value of the MAEE field is 1, the page
attribute of the address is determined by the extended page attribute in the
corresponding pte. If the value of the MAEE field is 0, the page attribute of
the address is determined by sysmap.h.

p52 (on mxstatus):

Flags – 63:59 bit page attributes

C906 extended page attribute, exists when the MAEE bit of the MXSTATUS register
is 1, and the function is as described in the MMU EntryLo register (SMEL).

p582:

MAEE - extended MMU address attributes:
• When MAEE is 0, the MMU address attribute is not extended.
• When MAEE is 1, the address attribute bit in the pte of the MMU is extended,
  and the user can configure the address attribute of the page.
This bit will be reset to 1’b0.

THEADISAEE - enable extended instruction set:
• When THEADISAEE is 0, an illegal instruction exception will be triggered when
  the C906 extended instruction is executed.
• When THEADISAEE is 1, the C906 extended instruction can be executed normally.
This bit will be reset to 1’b0.

*/

.section ".bootblock.boot", "ax", %progbits

.globl _boot
_boot:
    /*disable interrupt*/
    csrw mie, zero

    /*enable theadisaee and DO NOT SET maee (bit 21) - that'd bork Linux */
    li t1, (0x1 << 22 | 0x0 << 21)
    csrs 0x7c0, t1     // mxstatus

    /*invaild ICACHE/DCACHE/BTB/BHT*/
    li t2, 0x30013
    csrs 0x7c2, t2     // mcor

		li sp, (0x40010000 - 8)
		# 0xDEADBEEF is used to check stack underflow.
		li t0, 0xDEADBEEF
		sw t0, 0(sp)

    # la sp, __stack_end
    jal clear_bss
    jal _start
    j .

clear_bss:
		la    t0, __bss_start
		la    t1, __bss_end

clbss_1:
    sw zero, 0(t0)
    addi t0, t0, 4
    blt t0, t1, clbss_1
    ret

_debug:
    li t0, 0x02500000
    li t2, 0x0
    sw t2, 4(t0)
_loop:
    li t1, 0x42424242
    sw t1, 0(t0)
    j _loop

.section ".bootblock.stack", "aw", %progbits
_stack:
.space 65536
