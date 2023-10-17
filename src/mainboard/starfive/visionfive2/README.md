# JH7110

The JH7110 SoC support booting from various sources: SPI flash, UART, eMMC, SD.

To boot from flash, flip both DIP switches to 1 (away from the board's edge).

To boot via UART, flip the DIP switches to 0, i.e., toward to edge of the board.

## Sources

This code was mostly translated from [StarFive's U-Boot fork](https://github.com/starfive-tech/u-boot/tree/JH7110_VisionFive2_devel).

## Manual

https://doc-en.rvspace.org/JH7110/PDF/JH7110_TRM_StarFive_Preliminary.pdf

NOTE: The manual does not cover DRAM nor graphics.

## Prerequisites

You will need StarFive's [`spl_tool` to add a header to the binary](https://github.com/starfive-tech/Tools) plus [`vf2-loader`](https://github.com/orangecms/vf2-loader).
Note that we assume `spl_tool` to be renamed into `vf2-header` in our Makefiles.
Put the board in UART loader mode.

## Running without payload

Run `make -C bt0 run`. An xtask is to be done / work in progress.
Set the serial to use via the `PORT` environment variable.
To see the serial output, connect to it right after, e.g.:

```sh
make -C bt0 run PORT=/dev/ttyUSB0 && picocom -b 115200 /dev/ttyUSB0
```

## Running with kernel

See <kernel.md> for preparation. We now assume that your Linux kernel root
directory is within this oreboot mainboard directory here.
We take the DTB from it to pass to the kernel:

```sh
make -C main
make -C bt0 PORT=/dev/ttyUSB0 KERNEL_DIR=$(PWD)/linux runwithpayload && \
    picocom -b 115200 /dev/ttyUSB0
```
