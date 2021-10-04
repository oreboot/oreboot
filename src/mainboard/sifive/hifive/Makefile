OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = riscv64imac-unknown-none-elf
QEMU       ?= qemu-system-riscv64
QEMU_FLAGS += -m 1g -machine sifive_u,start-in-flash=true -nographic -device loader,addr=0x20000000,file=${IMAGE} -bios none -smp 4
RENODE       ?= renode
RENODE_FLAGS += -e 'set bin "$(PWD)/$(ELF)" ; start @renode-release.resc'
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
include ../../../../Makefile.renode.inc

openocd:
	$(shell which openocd) -f openocd.cfg -c "flash write_image erase unlock $(IMAGE) 0x20000000; shutdown"
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: clippy
citest: skiptest
