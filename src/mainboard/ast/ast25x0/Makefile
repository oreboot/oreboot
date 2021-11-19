OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = armv7a-none-eabihf
QEMU       ?= qemu-system-arm
QEMU_FLAGS += -m 512m -machine ast2500-evb -nographic -drive file=${IMAGE},format=raw,if=mtd
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: skipclippy
citest: skiptest
