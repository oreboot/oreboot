ORE_DIR := ../../../../..
BASE_DIR := $(ORE_DIR)/target/aarch64-unknown-none-softfloat/release
ELF_FILE := $(BASE_DIR)/oreboot-mangopi-mqquad-bt0
BIN_FILE := $(BASE_DIR)/oreboot-mangopi-mqquad-bt0.bin

all:
	cargo build --release -vv
	aarch64-linux-gnu-objcopy $(ELF_FILE) --strip-all -O binary $(BIN_FILE)
	echo $(BIN_FILE)

objdump: all
	aarch64-linux-gnu-objdump -D $(ELF_FILE)
