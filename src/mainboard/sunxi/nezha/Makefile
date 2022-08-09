OREBOOT=$(abspath $(CURDIR)/../../../../)
# architecture
TARGET     = riscv64imac-unknown-none-elf
# config: board variant and storage memory type
VARIANT    ?= lichee
MEMORY		 ?= nor
# outputs: full image and main part (can be run from DRAM)
IMAGE      ?= $(OREBOOT)/target/$(TARGET)/debug/oreboot-nezha.bin
PAYLOADER  ?= $(OREBOOT)/target/$(TARGET)/debug/oreboot-nezha-main.bin
# DRAM space starts at 0x40000000, see manual p34
XFEL       ?= xfel
MEMADDR    ?= 0x40000000
RELEASE    ?= --release

# FIXME: Those included Makefiles do not support the xtask build setup.
# Rework or remove them. We may not even need them at all anymore.
#include $(OREBOOT)/Makefile.inc
#include $(OREBOOT)/Makefile.mainboard.inc

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	cargo make $(RELEASE) --variant $(VARIANT) --memory $(MEMORY)

withpayload:
	cargo make $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) \
		--payload $(PAYLOAD) --dtb $(DTB)

flash:
	cargo flash $(RELEASE) --variant $(VARIANT) --memory $(MEMORY)

flashwithpayload:
	cargo flash $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) \
		--payload $(PAYLOAD) --dtb $(DTB)

objdump:
	cargo asm --variant $(VARIANT) --memory $(MEMORY)

raminitandboot:
	$(XFEL) ddr d1
	$(XFEL) write $(MEMADDR) $(PAYLOADER)
	$(XFEL) exec $(MEMADDR)

felboot: mainboard raminitandboot

felbootwithpayload: withpayload raminitandboot
