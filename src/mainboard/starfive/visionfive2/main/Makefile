OREBOOT     = $(abspath $(CURDIR)/../../../../../)
TARGET      = riscv64imac-unknown-none-elf
RELEASE_ELF = $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2-main
RELEASE_BIN = $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2-main.bin

all:
	cargo build --release
	riscv64-unknown-elf-objcopy -O binary "$(RELEASE_ELF)" "$(RELEASE_BIN)"

objdump: all
	riscv64-unknown-elf-objdump -D "$(RELEASE_ELF)"
