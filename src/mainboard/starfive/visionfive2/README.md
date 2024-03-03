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

**NOTE**: You will need [`vf2-loader`](https://github.com/orangecms/vf2-loader).

Put the board in UART loader mode.

Run `make run` in this directory. This will call xtask.

### Configuration

Set the make variable `DRAM_SIZE` to `2G` or `8G` as needed; default is `4G`.
Set the serial port to use via the `PORT` variable.
Set the verbosity level with `VERBOSE`, e.g., `VERBOSE=-v`.

For more notes, look at the `Makefile`.

### Example

The following will build oreboot for 8GB of DRAM, run the code over `ttyUSB0`,
and connect to the serial port via `picocom` for monitoring its output:

```sh
make run DRAM_SIZE=8G PORT=/dev/ttyUSB0 && picocom -b 115200 /dev/ttyUSB0
```

## Run without payload

**TODO**

## Run with kernel

See <kernel.md> for preparation. We now assume that your Linux kernel root
directory is within this oreboot mainboard directory here.
We take the DTB from it to pass it on at runtime.
Add both kernel and dtb to the build system. Edit `board.dts`.

**TODO**
