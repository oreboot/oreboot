.code32
	call 1f
1: 
	popl %eax
1: 
//	movb $0xaa, %al
	outb %al, $0x80
	jmp 1b
