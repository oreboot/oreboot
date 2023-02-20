ORE_DIR := ../../../../..
BASE_DIR := $(ORE_DIR)/target/armv7a-none-eabi/release
ELF_FILE := $(BASE_DIR)/oreboot-allwinner-h616-bt32
BIN_FILE := $(BASE_DIR)/oreboot-allwinner-h616-bt32.bin

OBJDUMP ?= arm-linux-gnueabi-objdump
# aarch64-linux-gnu-objdump

all:
	cargo build --release -vv
	cargo objcopy -- --strip-all -O binary $(BIN_FILE)
	ls -l $(BIN_FILE)

clean:
	cargo clean

objdump: all
	cargo objdump -- -D

objdumpgcc: all
	$(OBJDUMP) -D $(ELF_FILE)

hexdump: all
	xxd $(BIN_FILE)

run: all
	xfel write 0x20000 $(BIN_FILE)
	xfel exec 0x20000

runx: all
	sunxi-fel spl $(BIN_FILE)
