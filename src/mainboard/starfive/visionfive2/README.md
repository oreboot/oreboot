# JH7110

## Sources

This code was mostly translated from [StarFive's U-Boot fork](https://github.com/starfive-tech/u-boot/tree/JH7110_VisionFive2_devel).

## Manual

https://doc-en.rvspace.org/JH7110/PDF/JH7110_TRM_StarFive_Preliminary.pdf

NOTE: The manual does not cover DRAM nor graphics.

## Running

You will need StarFive's [`spl_tool` to add a header to the binary](https://github.com/starfive-tech/Tools) plus [`vf2-loader`](https://github.com/orangecms/vf2-loader).
Put the board in UART loader mode.

Run `make run` in the `bt0/` directory. An xtask is to be done.
You can set the serial to use via the `PORT` environment variable.
To see the serial output, connect to it right after, e.g.:

```sh
make run PORT=/dev/ttyUSB0 && picocom -b 115200 /dev/ttyUSB0
```
