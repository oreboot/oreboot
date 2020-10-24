## superiotool dump
##
## Global registers
## idx 10 11 13 14 1a 1b 1c 1d  22 24 25 26 27 28 2a 2b  2c 2d 2f
## val ff ff ff ff 38 76 90 00  ff ff 00 00 03 00 43 08  01 00 40
## def ff ff 00 00 30 50 10 00  ff 04 00 MM 03 00 c0 00  01 MM MM
## LDN 0x07 (GPIO 7, GPIO 8)
## idx 30 e0 e1 e2 e3 e4 e5 e6  e7 ec ed
## val 02 ef 06 00 00 ff ff ff  ff 10 ff
## def 00 ff NA 00 NA ff NA 00  NA 00 00
## LDN 0x09 (GPIO 2, GPIO 3, GPIO 4, GPIO 5)
## idx 30 e0 e1 e2 e3 e4 e5 e6  e7 e8 e9 ea eb ee f0 f1  f2 f4 f5 f6 f7
## val 0f fc 8c 00 00 7f 76 00  00 00 00 00 40 00 ff 4e  00 bf 1f 00 00
## def 00 ff NA 00 NA 7f NA 00  NA NA 00 00 00 00 ff NA  00 ff NA 00 NA

## from datasheet
## GRN (green) LED GPIO pin candidates: 23, 33, 53, 74, 76

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

# 2f -> 40; no clue
movb $0x2f, %al ##
outb %al, $0x2e
movb $0x40, %al ##
outb %al, $0x2f

######## LDN 0x07 ########
## not yet working, probably relies on some global registers
movb $0x07, %al ## command for device selection
outb %al, $0x2e
movb $0x07, %al ## select logical device 0x07: GPIOs 7 and 8
outb %al, $0x2f

movb $0x30, %al ## control register 0x30: GPIO activation
outb %al, $0x2e
movb $0x02, %al ## does this activate GPIO 7?
outb %al, $0x2f

movb $0xe0, %al ## control register 0xE0: GPIO 7 I/O
outb %al, $0x2e
movb $0xef, %al ## set value 1110 1111: GPIO 74 is output
outb %al, $0x2f

movb $0xec, %al ## control register 0xEC: LED function config
outb %al, $0x2e
movb $0x10, %al ## 0001 0000 - GPIO 74 is GRN LED
outb %al, $0x2f

// exit extended function mode
movb $0xaa, %al
outb %al, $0x2e

/* 123123123... */
	nop
	nop
1:
	movb $0x31, %al
	outb %al, $0x80
	movb $0x32, %al
	outb %al, $0x80
	movb $0x33, %al
	outb %al, $0x80
	jmp 1b
