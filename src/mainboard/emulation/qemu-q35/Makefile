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

xxx.rom: coreboot.rom
	cp $< $@

coreboot: mainboard-shrunk xxx.rom
	cbfstool xxx.rom remove -n fallback/ramstage
	cbfstool xxx.rom remove -n fallback/romstage
	objcopy  --set-start 0xffff1000 target/x86_64-unknown-none/release/qemu-q35
	cbfstool xxx.rom add-stage -a 0x1000 -n fallback/romstage -f target/x86_64-unknown-none/release/qemu-q35
	cbfstool xxx.rom print
	echo 'to verify: ndisasm -s 4096 -b 32 xxx.rom|less'
	echo 'look for this: 00001000  BC0010FFFF        mov esp,0xffff1000'
	ndisasm -s 4096 -b 32 xxx.rom | head -8192 | grep 00001000 | head -1
