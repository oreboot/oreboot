OREBOOT=$(abspath $(CURDIR)/../../../../)
# architecture
TARGET     = riscv64imac-unknown-none-elf
MODE       ?= release
# config: board variant and storage memory type
VARIANT    ?= lichee
MEMORY     ?= nor
# outputs: full image and main part (can be run from DRAM)
IMAGE      ?= $(OREBOOT)/target/$(TARGET)/$(MODE)/oreboot-nezha.bin
PAYLOADER  ?= $(OREBOOT)/target/$(TARGET)/$(MODE)/oreboot-nezha-main.bin
# NOTE: bt0 includes the eGON header
BOOT0      ?= $(OREBOOT)/target/$(TARGET)/$(MODE)/oreboot-nezha-bt0.bin
XFEL       ?= xfel
# For the memory mapping, see manual p34
SRAMADDR   = 0x00020000
DRAMADDR   = 0x40000000
SDCARD     ?= /dev/mmcblk0
DD         ?= sudo dd

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
	cargo make --$(MODE) --variant $(VARIANT) --memory $(MEMORY) --supervisor

withpayload:
	cargo make --$(MODE) --variant $(VARIANT) --memory $(MEMORY) --supervisor \
		--payload $(PAYLOAD) --dtb $(DTB)

nosbi:
	cargo make --$(MODE) --variant $(VARIANT) --memory $(MEMORY)

flash:
	cargo flash --$(MODE) --variant $(VARIANT) --memory $(MEMORY) --supervisor

nosbiflash:
	cargo flash --$(MODE) --variant $(VARIANT) --memory $(MEMORY)

flashwithpayload:
	cargo flash --$(MODE) --variant $(VARIANT) --memory $(MEMORY) --supervisor \
		--payload $(PAYLOAD) --dtb $(DTB)

nosbiflashwithpayload:
	cargo flash --$(MODE) --variant $(VARIANT) --memory $(MEMORY) \
		--payload $(PAYLOAD)

objdump:
	cargo asm --$(MODE) --variant $(VARIANT) --memory $(MEMORY) --supervisor

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
