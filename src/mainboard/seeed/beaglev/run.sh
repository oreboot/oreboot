#!/bin/sh

_RECOV=../../../../target/riscv64imac-unknown-none-elf/release/seeed-beaglev-bootblob.bin

make && jh7100-recover -D /dev/ttyUSB0 -r $_RECOV
