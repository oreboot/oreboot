OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
SERIAL    := /dev/ttyUSB0
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/milk_v-duo_s-bt0
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/milk_v-duo_s-bt0.bin

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

hexdump: mainboard
	xxd $(RELEASE_BIN)
