# gives the `run` target
include ../../../../../Makefile.qemu.inc
OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
# full image path, including multiple stages
IMAGE_BASE = $(OREBOOT)/target/$(TARGET)/release/emulation-qemu-riscv
IMAGE      := $(IMAGE_BASE).bin
QEMU       ?= qemu-system-riscv64
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
	cargo build --release
	cargo objcopy -- -O binary $(IMAGE)
	truncate -s 32M $(IMAGE)
	# cargo xtask make --release $(VERBOSE)

run: mainboard
	$(QEMU) -m 1g -machine virt -nographic -bios $(IMAGE) \
		-device loader,addr=0x80200000,file=$(PAYLOAD)
