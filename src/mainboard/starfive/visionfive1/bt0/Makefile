OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
SERIAL    := /dev/ttyUSB0
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive1-bt0
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive1-bt0.bin

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	# TODO: cargo make
	cargo build --release
	riscv64-unknown-elf-objcopy -O binary $(RELEASE_ELF) $(RELEASE_BIN)

run: mainboard
	jh7100-recover -D $(SERIAL) -r $(RELEASE_BIN)

objdump: mainboard
	riscv64-linux-gnu-objdump -D $(RELEASE_ELF)

hexdump: mainboard
	xxd $(RELEASE_BIN)
