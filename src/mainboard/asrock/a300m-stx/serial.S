/* this code is position-independent */
1: 
	movb $0x30, %al
	outb %al, $0x80
	jmp 1b
// as -o serial.o serial.S
// objcopy -O binary serial.o serial.bin
