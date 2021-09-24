VENDOR      = $(notdir $(shell dirname $(CURDIR)))
BOARD       = $(notdir $(CURDIR))
FIXED_DTFS  = fixed-dtfs.dtb
FLASHROM_PROGRAMMER ?= dediprog
OBJCOPY    ?= rust-objcopy
# ELF and BOOTBLOB are intermediate files while IMAGE is the final firmware
# image ready to be flashed onto the machine.
ELF      = $(TARGET_DIR)/$(BOARD)
BOOTBLOB = $(TARGET_DIR)/bootblob.bin
IMAGE    = $(TARGET_DIR)/image.bin

$(shell echo "VENDOR       = $(VENDOR)" 1>&2)
$(shell echo "BOARD        = $(BOARD)" 1>&2)

$(IMAGE): $(BOOTBLOB) $(TARGET_DIR)/$(FIXED_DTFS)
	TARGET_DIR=$(TARGET_DIR) cargo run --target $(TOOLS_TARGET) --manifest-path $(TOOLS_DIR)/layoutflash/Cargo.toml -- $(TARGET_DIR)/$(FIXED_DTFS) $@
	@printf "**\n** Output: $@\n**\n"

$(TARGET_DIR)/bootblob.bin: $(ELF)
	$(OBJCOPY) -O binary -R .bss $< $@

$(TARGET_DIR)/fixed-dtfs.dtb: fixed-dtfs.dts
	mkdir -p $(TARGET_DIR)
	dtc -W no-unit_address_vs_reg $< -O dtb -o $@

# Re-run cargo every time.
.PHONY: $(ELF)
$(ELF):
	RUST_TARGET_PATH=$(TOP)/src/custom_targets cargo build --target "$(TARGET)" -Z build-std=core,alloc $(CARGO_FLAGS)

# The default target: build the board.
mainboard: $(IMAGE)

# The rest of this file are conveniences for debugging.
objdump: $(ELF)
	rust-objdump -d $(ELF)

stack-sizes: $(ELF)
	stack-sizes $(ELF)

flash: $(IMAGE)
	sudo $(which flashrom) -V -w $(IMAGE) -p $(FLASHROM_PROGRAMMER)
