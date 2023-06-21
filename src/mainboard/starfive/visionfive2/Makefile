OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = riscv64imac-unknown-none-elf
# full image path, including multiple stages
IMAGE_BASE = $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2
IMAGE_BT0 := $(IMAGE_BASE)-bt0
IMAGE   := $(IMAGE_BASE).bin
VERBOSE ?= -vvvv
PORT    := /dev/ttyUSB0

cibuild: mainboard

# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	cargo xtask make --release $(VERBOSE)

objdump: mainboard
	riscv64-linux-gnu-objdump -D "$(IMAGE_BT0)"

run: mainboard
	jh7100-recover -r $(IMAGE) -D $(PORT)
