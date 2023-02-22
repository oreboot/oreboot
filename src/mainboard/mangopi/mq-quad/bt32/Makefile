ORE_DIR := ../../../../..
BT0_DIR := $(ORE_DIR)/target/aarch64-unknown-none-softfloat/release
BT0_BIN := $(BT0_DIR)/oreboot-mangopi-mqquad-bt0.bin
BASE_DIR := $(ORE_DIR)/target/armv7a-none-eabi/release
ELF_FILE := $(BASE_DIR)/oreboot-mangopi-mqquad-bt32
BIN_FILE := $(BASE_DIR)/oreboot-mangopi-mqquad-bt32.bin

all:
	cargo build --release -vv
	aarch64-linux-gnu-objcopy $(ELF_FILE) --strip-all -O binary $(BIN_FILE)
	echo $(BIN_FILE)

objdump: all
	aarch64-linux-gnu-objdump -D $(ELF_FILE)

hexdump: all
	xxd $(BIN_FILE)

boot: all
	cp $(BIN_FILE) boot.bin
	truncate -s 512 boot.bin 
	cat $(BT0_BIN) >> boot.bin
	xfel write 0x20000 boot.bin
	xfel exec 0x20000

run: all
	xfel write 0x20000 $(BIN_FILE)
	xfel exec 0x20000
