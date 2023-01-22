OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET := riscv64imac-unknown-none-elf
RELEASE := release
IMAGE_NAME := sipeed-m1s-dock-c906
IMAGE_ELF := "$(OREBOOT)/target/$(TARGET)/$(RELEASE)/$(IMAGE_NAME)"

all: build

build:
	cargo build --release

objdump: build
	riscv64-unknown-elf-objdump -D "$(IMAGE_ELF)"
