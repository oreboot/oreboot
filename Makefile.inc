# This Makefile is included by src/mainboards/VENDOR/BOARD/Makefile
TOP         = $(abspath $(CURDIR)/../../../..)
VENDOR      = $(notdir $(shell dirname $(CURDIR)))
BOARD       = $(notdir $(CURDIR))
MODE       ?= release
FIXED_DTFS  = fixed-dtfs.dtb
TOOLS_DIR   = $(TOP)/tools
FLASHROM_PROGRAMMER ?= dediprog
OBJCOPY    ?= rust-objcopy
# Cargo will build the tools using the target from the .cargo directory which
# does not work so well if you are cross-compiling. This shell one-liner gets
# the target of the current system which can be passed via --target for
# building tools
TOOLS_TARGET = $(shell rustc -Z unstable-options --print target-spec-json | sed -n -e 's/.*llvm-target": "\([^"]\+\).*/\1/p')
CARGO_FLAGS = --verbose

# Override variables depending on the build.
include $(TOP)/Makefile.mode.$(MODE).inc

# ELF and BOOTBLOB are intermediate files while IMAGE is the final firmware
# image ready to be flashed onto the machine.
ELF      = $(TARGET_DIR)/$(BOARD)
BOOTBLOB = $(TARGET_DIR)/bootblob.bin
IMAGE    = $(TARGET_DIR)/image.bin

$(shell echo "***** Oreboot Build Config *****" 1>2)
$(shell echo "TOP          = $(TOP)" 1>2)
$(shell echo "VENDOR       = $(VENDOR)" 1>2)
$(shell echo "BOARD        = $(BOARD)" 1>2)
$(shell echo "MODE         = $(MODE)" 1>2)
$(shell echo "TARGET       = $(TARGET)" 1>2)
$(shell echo "TARGET_DIR   = $(TARGET_DIR)" 1>2)
$(shell echo "TOOLS_TARGET = $(TOOLS_TARGET)" 1>2)
$(shell echo "cargo --version  = $(shell cargo --version)" 1>2)
$(shell echo "rustc --version  = $(shell rustc --version)" 1>2)
$(shell echo "rustup --version = $(shell rustup --version)" 1>2)
$(shell echo "**********" 1>2)

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

clean:
	rm -rf target/

# The rest of this file are conveniences for debugging.
objdump: $(ELF)
	rust-objdump -d $(ELF)

stack-sizes: $(ELF)
	stack-sizes $(ELF)

flash: $(IMAGE)
	sudo $(which flashrom) -V -w $(IMAGE) -p $(FLASHROM_PROGRAMMER)
