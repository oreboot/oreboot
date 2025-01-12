# Bouffalo Lab BL808

This SoC features two XuanTie cores:

- [E907](https://www.xrvm.com/product/xuantie/E907), with _no_ MMU, suitable for an embedded/real-time OS, e.g., Zephyr
- [C906](https://www.xrvm.com/product/xuantie/C906), with an MMU, suitable for a virtual memory OS such as Linux

Execution starts on the core described in the binary header.

## reference code

### smaeul's OpenSBI fork

https://github.com/smaeul/opensbi/commits/bl808

## DRAM / PSRAM init

### smaeul's U-Boot driver port

https://github.com/smaeul/u-boot/commit/47a847d44fe9c7dae03e9a3e840bb6016e9edd9f

## Running oreboot

NOTE: Until we have rewritten bouffalo-loader in Rust, this is a preliminary
workflow.

First, clone https://github.com/orangecms/bouffalo-loader, and check out the
`extend` branch. Install its dependencies, `pyserial` and `pyelftools`.

1. in the `c906` directory, run `make`
2. in the `e906` directory, run `make`
3. in the `e906` directory, run `make BLL_DIR=path/to/bouffalo-loader run`
