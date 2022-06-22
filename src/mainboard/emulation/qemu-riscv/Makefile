OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = riscv64imac-unknown-none-elf
QEMU       ?= qemu-system-riscv64
QEMU_FLAGS += -m 1g -machine virt -nographic -bios $(IMAGE)
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: clippy
citest: skiptest
