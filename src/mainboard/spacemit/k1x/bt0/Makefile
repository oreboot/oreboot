OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
SERIAL    := /dev/ttyUSB0
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-bt0
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-bt0.bin
FINAL_BIN= $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-bt0_h.bin

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
	cd $(OREBOOT) && ./tools/spacemit/build.py -c tools/spacemit/oreboot.json -o $(FINAL_BIN)

run: build
	fastboot stage $(FINAL_BIN)
	fastboot continue

objdump: build
	riscv64-unknown-elf-objdump -D $(RELEASE_ELF)

hexdump: build
	xxd $(RELEASE_BIN)
