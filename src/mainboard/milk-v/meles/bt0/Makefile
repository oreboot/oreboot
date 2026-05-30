OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/milkv-meles-bt0
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/milkv-meles-bt0.bin

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
	fastboot flash ram $(RELEASE_BIN)
	fastboot reboot

objdump: build
	riscv64-unknown-elf-objdump -D $(RELEASE_ELF)

hexdump: build
	xxd $(RELEASE_BIN)
