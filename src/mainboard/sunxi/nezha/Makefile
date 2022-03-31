OREBOOT=$(abspath $(CURDIR)/../../../../)
# RV64GCV, ackshully
TARGET     = riscv64gc-unknown-none-elf
QEMU       ?= qemu-system-riscv64
QEMU_FLAGS += -m 1g -machine sifive_u,start-in-flash=true -nographic -device loader,addr=0x20000000,file=${IMAGE} -bios none -smp 4
XFEL       ?= xfel
IMAGE      ?= target/$(TARGET)/release/image.bin
# DRAM space starts at 0x40000000, see manual p34
MEMADDR    ?= 0x40000000
include $(OREBOOT)/Makefile.inc
include $(OREBOOT)/Makefile.qemu.inc
include $(OREBOOT)/Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: clippy
citest: skiptest

boot: raminit
	$(XFEL) write $(MEMADDR) target/$(TARGET)/release/bootblob.bin
	$(XFEL) exec $(MEMADDR)

raminit:
	$(XFEL) ddr d1

objdump:
	rust-objdump -d target/$(TARGET)/release/nezha

run: raminit
	$(XFEL) write $(MEMADDR) $(IMAGE)
	$(XFEL) exec $(MEMADDR)
