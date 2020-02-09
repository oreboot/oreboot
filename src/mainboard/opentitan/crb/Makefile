TARGET = riscv32imc-unknown-none-elf
ROM    = $(TARGET_DIR)/rom.bin
FLASH  = $(TARGET_DIR)/flash.bin
include ../../../../Makefile.inc

.DEFAULT_GOAL = $(IMAGE).vmem

$(ROM): $(BOOTBLOB)
	dd if=$< of=$@ ibs=2048 count=1

$(FLASH): $(BOOTBLOB)
	dd if=$< of=$@ ibs=32768 skip=16383
	dd if=$< of=$@ ibs=32768 count=1 skip=16383

$(IMAGE).vmem: $(FLASH)
	cargo run --target $(TOOLS_TARGET) --manifest-path $(TOOLS_DIR)/bin2vmem/Cargo.toml -- -o 0 <$< >$@
	@printf "**\n** Output: $@\n**\n"

run: $(IMAGE).vmem
	Vtop_earlgrey_verilator --rominit=rom.vmem --flashinit=$<

trace: $(IMAGE).vmem
	Vtop_earlgrey_verilator -t --rominit=rom.vmem --flashinit=$<
