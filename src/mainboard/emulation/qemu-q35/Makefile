TARGET     = x86_64-unknown-none
QEMU       ?= qemu-system-x86_64
QEMU_FLAGS += -m 2g -machine q35 -nographic -bios $(IMAGE) -s -cpu Icelake-Server -monitor /dev/tty -serial stdio -debugcon file:debug.log
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
