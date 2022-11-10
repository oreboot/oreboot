OREBOOT=$(abspath $(CURDIR)/../../../../)
# architecture
TARGET     = riscv64imac-unknown-none-elf
# config: board variant and storage memory type
VARIANT    ?= lichee
MEMORY		 ?= nor
# outputs: full image and main part (can be run from DRAM)
IMAGE      ?= $(OREBOOT)/target/$(TARGET)/debug/oreboot-nezha.bin
PAYLOADER  ?= $(OREBOOT)/target/$(TARGET)/debug/oreboot-nezha-main.bin
# NOTE: bt0 includes the eGON header
BOOT0		   ?= $(OREBOOT)/target/$(TARGET)/release/oreboot-nezha-bt0.bin
# DRAM space starts at 0x40000000, see manual p34
XFEL       ?= xfel
MEMADDR    ?= 0x40000000
RELEASE    ?= --release
// SDCARD     ?= /dev/disk/by-id/usb-Generic_MassStorageClass_000000001536-0:1
SDCARD     ?= /dev/disk/by-id/usb-Generic-_USB3.0_CRW_-SD_201510191731-0:1
DD         ?= dd status=progress conv=fdatasync conv=nocreat

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
	cargo make $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) --supervisor

withpayload:
	cargo make $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) \
		--supervisor --payload $(PAYLOAD) --dtb $(DTB)

nosbi:
	cargo make $(RELEASE) --variant $(VARIANT) --memory $(MEMORY)

flash:
	cargo flash $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) --supervisor

nosbiflash:
	cargo flash $(RELEASE) --variant $(VARIANT) --memory $(MEMORY)

flashwithpayload:
	cargo flash $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) \
		--supervisor --payload $(PAYLOAD) --dtb $(DTB)

nosbiflashwithpayload:
	cargo flash $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) \
		--payload $(PAYLOAD)

objdump:
	cargo asm $(RELEASE) --variant $(VARIANT) --memory $(MEMORY) \
		--supervisor

sdcardacl:
	[ -b $(SDCARD) ]
	[ -w $(SDCARD) ] || sudo -k sdcard='$(SDCARD)' sh -c 'setfacl -m u:"$${SUDO_USER:?}":rw "$${sdcard:?}"'
	[ -w $(SDCARD) ]

sdcardclean: sdcardacl
	blkdiscard -v $(SDCARD) || $(DD) conv=noerror if=/dev/zero of=$(SDCARD) iflag=fullblock bs=4K

sdcard: mainboard sdcardacl
	[ -b $(SDCARD) ]
	$(DD) if=$(BOOT0) of=$(SDCARD) bs=8192 seek=1
	# redundant copy
	$(DD) if=$(BOOT0) of=$(SDCARD) bs=8192 seek=16
	sync
	sync
	eject -v $(SDCARD)

sdcardwithpayload: withpayload sdcardacl
	[ -b $(SDCARD) ]
	$(DD) if=$(BOOT0) of=$(SDCARD) bs=8192 seek=1
	# redundant copy
	$(DD) if=$(BOOT0) of=$(SDCARD) bs=8192 seek=16
	sync
	sync
	eject -v $(SDCARD)

raminitandboot:
	$(XFEL) ddr d1
	$(XFEL) write $(MEMADDR) $(PAYLOADER)
	$(XFEL) exec $(MEMADDR)

felboot: mainboard raminitandboot

felbootwithpayload: withpayload raminitandboot
