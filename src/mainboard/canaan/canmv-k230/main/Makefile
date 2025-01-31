OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/oreboot-k230-main
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/oreboot-k230-main.bin
KENDRYTE_BOOT ?= kendryte_boot

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
	cargo objcopy -- -O binary $(RELEASE_BIN)

run: build
	$(KENDRYTE_BOOT) run -a 0x1000 $(RELEASE_BIN)

objdump:
	cargo objdump -D

hexdump: build
	xxd $(RELEASE_BIN)
