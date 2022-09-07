.section ".debug", "ax", %progbits

// write to UART3 (debug)
_debug:
  li t5, 0x12440000
  li t4, 0x42
  sw t4, 0(t5)
  li t4, 0x41
  sw t4, 0(t5)
  li t4, 0x44
  sw t4, 0(t5)
  li t4, '\r'
  sw t4, 0(t5)
  li t4, '\n'
  sw t4, 0(t5)

boot:
	la t0, aligned_trap_entry
	csrw mtvec, t0
  j start

trap_entry:
        ret
hls_init:
        ret
.align 4
aligned_trap_entry:
	j trap_entry
