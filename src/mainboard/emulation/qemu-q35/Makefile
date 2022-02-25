OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = x86_64-unknown-none-oreboot
QEMU       ?= qemu-system-x86_64
QEMU_FLAGS += -m 2g -machine q35 -nographic -bios $(IMAGE) -cpu Icelake-Server -monitor /dev/tty -serial stdio -debugcon file:debug.log
include ../../../../Makefile.inc
include ../../../../Makefile.qemu.inc
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: skipclippy
citest: skiptest

FIXED_DTFS_SHRUNK  = fixed-dtfs-shrunk.dtb
IMAGE_SHRUNK       = $(TARGET_DIR)/image-shrunk.bin

$(IMAGE_SHRUNK): $(BOOTBLOB) $(TARGET_DIR)/$(FIXED_DTFS_SHRUNK)
	TARGET_DIR=$(TARGET_DIR) cargo run --target $(TOOLS_TARGET) --manifest-path $(TOOLS_DIR)/layoutflash/Cargo.toml -- $(TARGET_DIR)/$(FIXED_DTFS_SHRUNK) $@
	@printf "**\n** Output: $@\n**\n"

$(TARGET_DIR)/fixed-dtfs-shrunk.dtb: $(TARGET_DIR)/fixed-dtfs-shrunk.dts
	mkdir -p $(TARGET_DIR)
	dtc -W no-unit_address_vs_reg $< -O dtb -o $@

$(TARGET_DIR)/fixed-dtfs-shrunk.dts: fixed-dtfs.dts
	TARGET_DIR=$(TARGET_DIR) cargo run --target $(TOOLS_TARGET) --manifest-path $(TOOLS_DIR)/dtfs_shrinker/Cargo.toml -- fixed-dtfs.dts $@
	@printf "**\n** Output: $@\n**\n"

mainboard-shrunk: $(IMAGE_SHRUNK)
