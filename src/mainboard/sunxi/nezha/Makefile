OREBOOT=$(abspath $(CURDIR)/../../../../)
# vendor specific (external) tooling
XFEL       ?= xfel

MODE       ?= release
# config: board variant and storage memory type
VARIANT    ?= lichee
MEMORY     ?= nor

# architecture
TARGET     = riscv64imac-unknown-none-elf
DIST_DIR    = $(OREBOOT)/target/$(TARGET)/$(MODE)
# outputs: bt0 (run from SRAM), main stage (run from DRAM) and full image
# NOTE: bt0 includes the eGON header
BOOT0      ?= $(DIST_DIR)/oreboot-nezha-bt0.bin
PAYLOADER  ?= $(DIST_DIR)/oreboot-nezha-main.bin
IMAGE      ?= $(DIST_DIR)/oreboot-nezha.bin

# For the memory mapping, see manual p34
SRAMADDR   = 0x00020000
DRAMADDR   = 0x40000000

# For writing SD cards
SDCARD     ?= /dev/mmcblk0
DD         ?= sudo dd

CARGO_ARGS = --$(MODE) --variant $(VARIANT) --memory $(MEMORY)

# FIXME: Those included Makefiles do not support the xtask build setup.
# Rework or remove them. We may not even need them at all anymore.
include $(OREBOOT)/Makefile.inc
#include $(OREBOOT)/Makefile.mainboard.inc

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	cargo make $(CARGO_ARGS) --supervisor

withpayload:
	cargo make $(CARGO_ARGS) --supervisor --payload $(PAYLOAD) --dtb $(DTB)

nosbi:
	cargo make $(CARGO_ARGS)

flash:
	cargo flash $(CARGO_ARGS) --supervisor

nosbiflash:
	cargo flash $(CARGO_ARGS)

flashwithpayload:
	cargo flash $(CARGO_ARGS) --supervisor --payload $(PAYLOAD) --dtb $(DTB)

nosbiflashwithpayload:
	cargo flash $(CARGO_ARGS) --payload $(PAYLOAD)

objdump:
	cargo asm $(CARGO_ARGS) --supervisor

sdcard: mainboard
	$(DD) if=$(BOOT0) of=$(SDCARD) bs=8192 seek=1
	# redundant copy
	$(DD) if=$(BOOT0) of=$(SDCARD) bs=8192 seek=16

raminitandboot:
	$(XFEL) ddr d1
	$(XFEL) write $(DRAMADDR) $(PAYLOADER)
	$(XFEL) exec $(DRAMADDR)

felboot: mainboard raminitandboot

felbootwithpayload: withpayload raminitandboot

run: mainboard
	$(XFEL) write $(SRAMADDR) $(BOOT0)
	$(XFEL) exec $(SRAMADDR)

runnosbi: nosbi
	$(XFEL) write $(SRAMADDR) $(BOOT0)
	$(XFEL) exec $(SRAMADDR)
