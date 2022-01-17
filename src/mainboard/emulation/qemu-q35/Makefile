OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = x86_64-unknown-none-oreboot
QEMU       ?= qemu-system-x86_64
QEMU_FLAGS += -m 2g -machine q35 -nographic -bios $(IMAGE) -cpu Icelake-Server -monitor /dev/tty -serial stdio -debugcon file:debug.log
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: skipclippy
citest: skiptest
