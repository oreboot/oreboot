CFLAGS=-nostdinc -nostdlib -ffreestanding --static -fPIC

serialdebug serialdebug.elf: serialdebug.S
	riscv64-linux-gnu-gcc ${CFLAGS} -o serialdebug.elf serialdebug.S
	riscv64-linux-gnu-objcopy --remove-section='.note*' -O binary serialdebug.elf serialdebug

qemu: serialdebug.S
	riscv64-linux-gnu-gcc ${CFLAGS} -DQEMU -o serialdebug_qemu.elf serialdebug.S -T link.ld
	qemu-system-riscv64 -machine sifive_u -bios serialdebug_qemu.elf -net none -nographic

objdump: serialdebug.elf
	riscv64-linux-gnu-objdump -D serialdebug.elf

clean:
	rm -f serialdebug serialdebug.elf serialdebug_qemu serialdebug_qemu.elf
