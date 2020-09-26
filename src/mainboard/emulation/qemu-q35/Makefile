TARGET     = x86_64-unknown-none
QEMU       = qemu-system-x86_64
QEMU_FLAGS = -m 1g -machine q35 -nographic -bios $(IMAGE) -cpu Icelake-Server
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
