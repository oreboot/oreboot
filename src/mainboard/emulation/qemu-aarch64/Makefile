TARGET     = aarch64-unknown-none-softfloat
QEMU       ?= qemu-system-aarch64
QEMU_FLAGS += -machine virt -cpu cortex-a53 -nographic -m 2g -bios $(IMAGE)
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
