OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
SERIAL    := /dev/ttyUSB0
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-bt0
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-bt0.bin

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard: build

build:
	# TODO: cargo make
	cargo build --release
	riscv64-unknown-elf-objcopy -O binary $(RELEASE_ELF) $(RELEASE_BIN)

run: build
	echo TODO

objdump: build
	riscv64-unknown-elf-objdump -D $(RELEASE_ELF)

hexdump: build
	xxd $(RELEASE_BIN)
