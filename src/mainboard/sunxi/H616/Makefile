FEL := xfel
LOAD_ADDR := 0x20000
ALL_BIN := boot.bin

ORE_DIR := ../../../..

BT32_DIR := $(ORE_DIR)/target/armv7a-none-eabi/release
BT32_BIN := $(BT32_DIR)/oreboot-allwinner-h616-bt32.bin

BT0_DIR := $(ORE_DIR)/target/aarch64-unknown-none-softfloat/release
BT0_BIN := $(BT0_DIR)/oreboot-allwinner-h616-bt0.bin

mainboard:
	$(MAKE) -C bt32
	$(MAKE) -C bt0
	cp $(BT32_BIN) $(ALL_BIN)
	truncate -s 2048 $(ALL_BIN) 
	cat $(BT0_BIN) >> $(ALL_BIN)

run: mainboard
	cargo xtask run
