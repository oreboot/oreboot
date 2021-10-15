# This Makefile is included by src/mainboards/VENDOR/BOARD/Makefile for qemu
# recipes.

MONITOR ?= "/dev/null"

run: mainboard
	$(QEMU) $(QEMU_FLAGS)

trace: mainboard
	$(QEMU) -d guest_errors,in_asm,int $(QEMU_FLAGS)

gdb: mainboard
	$(QEMU) -d guest_errors -s -S $(QEMU_FLAGS)
