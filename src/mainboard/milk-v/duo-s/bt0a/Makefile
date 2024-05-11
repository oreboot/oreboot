OREBOOT=$(abspath $(CURDIR)/../../../../../)
TARGET     = aarch64-unknown-none-softfloat
RELEASE_ELF= $(OREBOOT)/target/$(TARGET)/release/milk_v-duo_s-bt0a
RELEASE_BIN= $(OREBOOT)/target/$(TARGET)/release/milk_v-duo_s-bt0a.bin

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
	aarch64-linux-gnu-objcopy $(RELEASE_ELF) --strip-all -O binary $(RELEASE_BIN)

run: build
	sg_boot run $(RELEASE_BIN)

objdump: build
	aarch64-linux-gnu-objdump -D $(RELEASE_ELF)
