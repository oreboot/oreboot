// Prints A to serial forever.

#define BASE 0x10010000
#define TD (BASE + 0x0)
#define TXC (BASE + 0x8)

.global _start
_start:
	li t0, TXC
	lw t1, 0(t0)
	ori t1, t1, 0x1
	sw t1, 0(t0)
	li t0, TD

	call check_full
	li t1, 'T'
	sw t1, 0(t0)
	call check_full
	li t1, 'E'
	sw t1, 0(t0)
	call check_full
	li t1, 'S'
	sw t1, 0(t0)
	call check_full
	li t1, 'T'
	sw t1, 0(t0)
	call check_full
	li t1, 'T'
	sw t1, 0(t0)
	call check_full
	li t1, 'E'
	sw t1, 0(t0)
	call check_full
	li t1, 'S'
	sw t1, 0(t0)
	call check_full
	li t1, 'T'
	sw t1, 0(t0)
	call check_full
	li t1, 'T'
	sw t1, 0(t0)
	call check_full
	li t1, 'E'
	sw t1, 0(t0)
	call check_full
	li t1, 'S'
	sw t1, 0(t0)
	call check_full
	li t1, 'T'
	sw t1, 0(t0)
	call check_full
	li t1, '\r'
	sw t1, 0(t0)
	call check_full
	li t1, '\n'
	sw t1, 0(t0)

loop:
	jal loop

check_full:
	li t2, TD
	lw t2, 0(t2)
	srli t2, t2, 31
	bgtz t2, check_full
	ret
