OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET := riscv32imac-unknown-none-elf
RELEASE := release
IMAGE_NAME := sipeed-m1s-dock-e907
IMAGE_ELF := "$(OREBOOT)/target/$(TARGET)/$(RELEASE)/$(IMAGE_NAME)"
# https://github.com/smaeul/bouffalo-loader
BLL_DIR := "/home/dama/firmware/Bouffalo Lab/bouffalo-loader"
BL_LOADER := "python3 $(BLL_DIR)/loader.py"
BL_LOADER_CFG := "$(BLL_DIR)/bl808_header_cfg.conf"

all: build run

build:
	cargo build --release

objdump: build
	riscv64-unknown-elf-objdump -D "$(IMAGE_ELF)"

run:
	"$(BL_LOADER)" -c bl808 -p /dev/ttyUSB1 -C "$(BL_LOADER_CFG)" "$(IMAGE_ELF)"
