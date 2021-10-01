# This Makefile is included by src/mainboards/VENDOR/BOARD/Makefile for qemu
# recipes.

MONITOR ?= "/dev/null"

run: mainboard $(IMAGE)
	$(QEMU) $(QEMU_FLAGS)

trace: mainboard $(IMAGE)
	$(QEMU) -d guest_errors,in_asm,int $(QEMU_FLAGS)

gdb: mainboard $(IMAGE)
	$(QEMU) -d guest_errors -s -S $(QEMU_FLAGS)
