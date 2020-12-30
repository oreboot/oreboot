TARGET     = riscv64imac-unknown-none-elf
QEMU       ?= qemu-system-riscv64
QEMU_FLAGS += -m 1g -machine virt -nographic -bios none -drive format=raw,if=pflash,file=$(IMAGE)
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
