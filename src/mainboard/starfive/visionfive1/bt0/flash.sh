#!/bin/sh
set -e

make
TARGET=riscv64imac-unknown-none-elf
RELEASE_BIN=../../../../../target/$TARGET/release/starfive-visionfive1-bt0.bin

# 16380 is 16K minus these 4 bytes
/usr/bin/echo -n -e '\xfc\x3f\x00\x00' | tee x.bin
cat $RELEASE_BIN >> x.bin

jh7100-reflash x.bin
