OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET      = aarch64-unknown-none-softfloat
PLATFORM    = rk3566-bt0
RELEASE_DIR = $(OREBOOT)/target/$(TARGET)/release
RELEASE_ELF = $(RELEASE_DIR)/$(PLATFORM)
RELEASE_BIN = $(RELEASE_DIR)/$(PLATFORM).bin
XROCK      ?= xrock
LOADER     ?= $(XROCK) extra maskrom --rc4 off --sram

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard: build

build:
	cargo build --release -vv
	cargo objcopy -- --strip-all -O binary $(RELEASE_BIN)

run: build
	$(LOADER) $(RELEASE_BIN)

objdump: build
	cargo objdump -- -D
