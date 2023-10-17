# WIP Linux for oreboot on VisionFive 2

**NOTE: This is a snapshot for our development process, which works for now.**

We write the kernel to the SPI flash using U-Boot from the vendor firmware. It
occupies only the first 4 megabytes, so we have 12 more for the kernel after it.

To boot from flash, flip both DIP switches to 1 (away from the board's edge).

To boot via UART, flip the DIP switches to 0, i.e., toward to edge of the board.

## Prerequisites

- TFTP server
- [`lzss` tool](https://github.com/orangecms/compress-test-rs)
- Daniel's [Linux fork with extra patches](https://github.com/orangecms/linux/tree/vf2-upstream-6.6.rc5-unaligned_fix-oreboot) (mind the branch!)
- an initramfs; e.g. [u-root](https://u-root.org) core + boot, xz-compressed cpio;
  note that u-root needs to be built with at least Go 1.21 to have alignment fixes
- `gcc-riscv64-linux-gnu` (the RISC-V Linux GCC toolchain)

## Build the kernel

Put your initramfs in the kernel root directory, call it `init.cpio.xz`.

```sh
ARCH=riscv make vf2_cpu_defconfig
ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- make -j4
```

## Compress the kernel

We use lzss compression in oreboot and expect the kernel to follow specific
parameters. The `lzss` tool is tweaked for that purpose.

To build it, in the `compress-test-rs` repo directory, run:
`cargo build --release`.

Now run the resulting `lzss` binary in your kernel root directory:

```
lzss arch/riscv/boot/Image
```

This will create the file `arch/riscv/boot/Image.ore`.

## Network setup

For a combined TFTP + DHCP server, you can use `centre` from
https://github.com/harvey-os/go.

Run a TFTP server; put the `lzss`-compressed kernel as `vf2.ore` in its root
directory.

Run a DCHP server alongside to complement the setup, or configure IP addresses
manually.

With `centre`, assuming an entry for the VF2's MAC address in `hosts` and
`tftphome` to be the TFTP root directory, `enxa0cec863d77a` to be your laptop's
network interface name (systemd style), and `192.168.22.1` its IP address:

```sh
sudo centre -i enxa0cec863d77a -ip 192.168.22.1 -tftp-dir tftphome/ -hostfile hosts
```

Example `hosts` file:

```
192.168.22.42   vf2  u6ccf39002cfa
```

## U-Boot flash/update kernel

### One-time setup script

This handy script will let you easily reflash:

```
setenv updkernel "dhcp;tftpboot vf2.ore;sf probe;sf update 0xa0000000 0x400000 0xb00000"
saveenv
```

### Update kernel

In U-Boot, run:

```
run updkernel
```

## Boot

Follow the [README](README.md).
