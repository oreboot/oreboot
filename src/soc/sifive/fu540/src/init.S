// This is a GROSS HACK until rust is fixed to not expect
// atomic load store on 16 bit quantities, which riscv
// can not do.
.globl __atomic_store_16
__atomic_store_16:
	ret
.globl __atomic_load_16
__atomic_load_16:
	ret
// END GROSS HACK

.globl _stack_ptr
.section .rodata
_stack_ptr: .word 0x08010000
