OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = riscv64imac-unknown-none-elf
# full image path, including multiple stages
IMAGE_BASE = $(OREBOOT)/target/$(TARGET)/release/emulation-qemu-riscv
IMAGE   := $(IMAGE_BASE).bin
VERBOSE ?= -vvvv

cibuild: mainboard

# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop
objdump: nop
run: nop

mainboard:
	cargo xtask make --release $(VERBOSE)
