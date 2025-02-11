OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET := riscv64imac-unknown-none-elf
RELEASE := release
IMAGE_NAME := sipeed-m1s-dock-c906
IMAGE_ELF := "$(OREBOOT)/target/$(TARGET)/$(RELEASE)/$(IMAGE_NAME)"
IMAGE_BIN := "$(IMAGE_ELF).bin"
PORT ?= /dev/ttyUSB1

cibuild: mainboard

ciclippy: lint

# TODO
nop:
	echo nope...

lint:
	cargo clippy -- -D warnings -A clippy::write_with_newline

citest: nop
checkformat: nop

mainboard:
	cargo build --release
	cargo objcopy -- -O binary "$(IMAGE_BIN)"

run: mainboard
	bl_boot run -p $(PORT) --d0-binary "$(IMAGE_BIN)"

objdump: mainboard
	riscv64-unknown-elf-objdump -D "$(IMAGE_ELF)"
