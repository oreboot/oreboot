# This Makefile is included by src/mainboards/VENDOR/BOARD/Makefile for qemu
# recipes.
run: $(IMAGE)
	$(QEMU) $(QEMU_FLAGS)

trace: $(IMAGE)
	$(QEMU) -d guest_errors,in_asm,int $(QEMU_FLAGS)

gdb: $(IMAGE)
	$(QEMU) -d guest_errors -s -S
