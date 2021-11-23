OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = x86_64-unknown-none
DPCMD      ?= dpcmd
include ../../../../Makefile.inc
include ../../../../Makefile.mainboard.inc
cibuild: $(IMAGE)
ciclippy: clippy
citest: skiptest

# 32MiB chip. The BIOS is the last 8MiB.
# Flashrom is buggy for this chip.
flash: target/x86_64-unknown-none/release/image.bin
	$(DPCMD) -v -u $< -a0x1800000
