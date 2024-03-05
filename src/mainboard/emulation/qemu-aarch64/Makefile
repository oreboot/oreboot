OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = aarch64-unknown-none-softfloat
# full image path, including multiple stages
IMAGE_BASE = $(OREBOOT)/target/$(TARGET)/release/emulation-qemu-aarch64
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
