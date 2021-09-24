OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = arm-none-eabihf
QEMU       ?= qemu-system-arm
QEMU_FLAGS += -machine virt -nographic -m 2g -bios $(IMAGE)
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: skipclippy
citest: skiptest
