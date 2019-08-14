CFLAGS=-nostdinc -nostdlib -ffreestanding --static -fPIC

serialdebug.elf: serialdebug.S
	riscv64-linux-gnu-gcc ${CFLAGS} -o serialdebug.elf serialdebug.S -T link.ld

serialdebug:  serialdebug.elf
	riscv64-linux-gnu-objcopy --remove-section='.note*' -O binary serialdebug.elf serialdebug

qemu: serialdebug.elf
	qemu-system-riscv64 -machine sifive_u -bios serialdebug.elf -net none -nographic

objdump: serialdebug.elf
	riscv64-linux-gnu-objdump -D serialdebug.elf

clean:
	rm -f serialdebug serialdebug.elf

cleanrom: serialdebug
	 dd bs=32M count=1 if=/dev/zero | tr '\000' '\377' > 32m
	 cat serialdebug 32m > bigrom
	 dd of=rom bs=32M count=1 if=bigrom

writerom: rom
	flashrom -w rom -p dediprog

rom: serialdebug hifiverom
	cp hifiverom rom
	dd of=rom if=serialdebug conv=notrunc

hifiverom:
	flashrom -r hifiverom -p dediprog

