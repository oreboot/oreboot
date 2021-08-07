#define REGBYTES        4
/*mxstatus*/
#define    EN_THEADISAEE    (0x1 << 22)

.section ".bootblock.boot", "ax", %progbits

.globl _boot
_boot:
    /*disable interrupt*/
    csrw mie, zero

    /*enable theadisaee*/
    li t1, (0x1 << 22) // EN_THEADISAEE
    csrs 0x7c0, t1     // mxstatus

    /*invaild ICACHE/DCACHE/BTB/BHT*/
    li t2, 0x30013
    csrs 0x7c2, t2     // mcor

		li sp, (0x40002000 - 8)
		# 0xDEADBEEF is used to check stack underflow.
		li t0, 0xDEADBEEF
		sw t0, 0(sp)

    # la sp, __stack_end
    jal clear_bss
    jal _start_here
    j .

clear_bss:
		la    t0, __bss_start
		la    t1, __bss_end

clbss_1:
    sw zero, 0(t0)
    addi t0, t0, 4
    blt t0, t1, clbss_1
    ret

.section ".bootblock.stack", "aw", %progbits
_stack:
.space 65536

.globl _stack_ptr
.section .rodata
_stack_ptr: .long 0x40001000
