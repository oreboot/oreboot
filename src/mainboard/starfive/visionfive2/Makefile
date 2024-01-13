OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = riscv64imac-unknown-none-elf
VERBOSE   ?= -vvvv
PORT      ?= /dev/ttyUSB0
DRAM_SIZE ?= 4G

# full image path, including multiple stages
IMAGE_BASE  = $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2
IMAGE_BT0  := $(IMAGE_BASE)-bt0
IMAGE      := $(IMAGE_BASE).bin

cibuild: mainboard

# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	cargo xtask make --release $(VERBOSE) --dram-size $(DRAM_SIZE)

objdump: mainboard
	riscv64-linux-gnu-objdump -D "$(IMAGE_BT0)"

run: mainboard
	sudo `which vf2-loader` -D ${PORT} $(IMAGE)
