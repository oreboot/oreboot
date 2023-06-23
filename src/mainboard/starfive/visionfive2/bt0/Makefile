OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = riscv64imac-unknown-none-elf
SERIAL    ?= /dev/ttyUSB0
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2-bt0
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2-bt0.bin
RELEASE_MAIN= $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2-main.bin
RELEASE_ALL= $(OREBOOT)/target/$(TARGET)/release/starfive-visionfive2-all.bin
DRAM_SIZE := 4G
KERNEL_DIR := /home/dama/firmware/RISC-V/StarFive/VisionFive2/linux
DTB_FILE := starfive/jh7110-starfive-visionfive-2-v1.3b.dtb
DEVICE_TREE := $(KERNEL_DIR)/arch/riscv/boot/dts/$(DTB_FILE)

cibuild: mainboard
# TODO
nop:
	echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	# TODO: cargo make
	cargo build --release --config build.rustflags="'--cfg dram_size=\"$(DRAM_SIZE)\"'"
	riscv64-unknown-elf-objcopy -O binary $(RELEASE_ELF) $(RELEASE_BIN)

header:
	# add the whacko header that the mask ROM expects
	# note: this is an alias of Daniel's for `spl_tool` from
	# https://github.com/starfive-tech/Tools
	# TODO: Rewrite it in Rust (it's ackshully simple)
	# See also: https://github.com/starfive-tech/Tools/issues/1
	vf2-header -c -f $(RELEASE_BIN)

# TODO: put stuff in xtask instead...
run: mainboard header
	# use the Rust XMODEM loader, because it's written in Rust
	vf2-loader $(RELEASE_BIN).normal.out

withpayload: mainboard
	cp $(RELEASE_BIN) $(RELEASE_ALL)
	# NOTE: We hardcoded a 32K offset into `src/main.rs`.
	# Will be done more sensibly once xtask is set up.
	truncate -s 32K $(RELEASE_ALL)
	cat $(RELEASE_MAIN) >> $(RELEASE_ALL)
	truncate -s 96K $(RELEASE_ALL)
	cat $(DEVICE_TREE) >> $(RELEASE_ALL)
	vf2-header -c -f $(RELEASE_ALL)

runwithpayload: withpayload
	vf2-loader -D $(SERIAL) $(RELEASE_ALL).normal.out

runx: mainboard header
	jh7110-recover -D $(SERIAL) -r $(RELEASE_BIN).normal.out

objdump: mainboard
	riscv64-linux-gnu-objdump -D $(RELEASE_ELF)

readelf: mainboard
	riscv64-linux-gnu-readelf -elS $(RELEASE_ELF)

hexdump: mainboard
	xxd $(RELEASE_BIN)
