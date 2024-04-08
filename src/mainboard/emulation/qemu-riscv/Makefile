# gives the `run` target
include ../../../../Makefile.qemu.inc
OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = riscv64imac-unknown-none-elf
# full image path, including multiple stages
IMAGE_BASE = $(OREBOOT)/target/$(TARGET)/release/emulation-qemu-riscv
IMAGE   := $(IMAGE_BASE).bin
QEMU       ?= qemu-system-riscv64
QEMU_FLAGS += -m 1g -machine virt -nographic -bios none -drive format=raw,if=pflash,file=$(IMAGE)
VERBOSE ?= -vvvv

cibuild: mainboard

# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop
objdump: nop

mainboard:
	cargo xtask make --release $(VERBOSE)
