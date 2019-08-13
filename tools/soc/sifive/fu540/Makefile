serialdebug:
	riscv64-linux-gnu-gcc -nostdinc -nostdlib -ffreestanding --static serialdebug.S
	riscv64-linux-gnu-objcopy --remove-section='.note*' -O binary a.out serialdebug
