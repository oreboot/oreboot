OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET    = riscv64imac-unknown-none-elf
BT0_BIN   = $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-bt0.bin
MAIN_BIN   = $(OREBOOT)/target/$(TARGET)/release/spacemit-k1x-main.bin
FINAL_BIN = $(abspath $(CURDIR)/spacemit-k1x.bin)

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard: build

build:
	cargo make --release -vv

run: build
	fastboot stage $(BT0_BIN)
	fastboot continue
	# TODO
	# fastboot stage $(MAIN_BIN)
	# fastboot continue

objdump: build
	cargo objdump -- -D

hexdump: build
	xxd $(BT0_BIN)

flowchart:
	mmcd -i flow.chart -w 1920 -H 1080 -o flow.png
