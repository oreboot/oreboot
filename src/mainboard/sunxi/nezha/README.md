# Allwinner Nezha / D1

The Allwinner (aka sunxi) Nezha board features a D1 SoC, based on the XuanTie
C906 core. It has a 256 MB NAND flash, large enough for oreboot and LinuxBoot.

More comprehensive information can be found in the [linux-sunxi wiki](
https://linux-sunxi.org/index.php?title=Allwinner_Nezha).

There is a [multitude of other boards based on the D1](
https://linux-sunxi.org/Category:D1_Boards).

## Running oreboot

### Prepare

The [xfel](https://github.com/xboot/xfel) tool (clone and build it first) can
initialize DRAM over the FEL BROM loader, then transfer code to memory and
execute it. It can also read from and write to NAND and NOR SPI flash.

### From flash

To flash an oreboot image including a LinuxBoot payload and device tree blob:

```sh
_LINUX_BOOT=/path/to/linux/arch/riscv/boot
_BOARD=lichee-rv-dock

make flashwithpayload \
  PAYLOAD="$_LINUX_BOOT/Image" \
  DTB="$_LINUX_BOOT/dts/allwinner/sun20i-d1-$_BOARD.dtb"
```

**Note**

The DTB (device tree blob) is built together with Linux for the respective board
and to be found within Linux in the directory `arch/riscv/boot/dts/allwinner/`.
We currently do not patch it at runtime, so you need to hardcode your board's
memory size into it.

For a work-in-progress Linux 5.19 (not all patches are upstream yet), see:
https://github.com/orangecms/linux/tree/5.19-smaeul-plus-dts

### From DRAM

**FIXME: running from DRAM needs rework**

The DRAM space starts at `0x40000000`. So the Makefile is set up to run oreboot
from there, using two variants:

- `make boot`: just run the bootblob with no payload; useful to work on oreboot
- `make run`: run the full image; mind this takes a minute for the USB transfer

## BROM loader

In order to enter FEL mode, press the `FEL` button on the board when powering it
on. To run code again, you need to reset and reenter FEL mode. The easiest way
is to power the board and on again. Another option is to solder a reset button
to the test pin next to the flash. For convenience, you can route a wire from it
to one of the N/C pins on the GPIO header.

## Payload Support

With oreboot, there are two possibilities to run a payload:

1. Remain in M-mode, load and execute the payload directly
2. Execute an SBI which drops to S-mode and executes the actual payload

**Note: the first variant is currently hard-disabled and needs adjustments**

Useful example M-mode payloads:

- [D1 UART C example](
  https://github.com/bigmagic123/d1-nezha-baremeta/tree/main/src/3.uart)
- [xv6-d1](https://github.com/michaelengel/xv6-d1)

Both of these examples would need to be compiled using a C toolchain such as the
[GNU toolchain for RISC-V](https://github.com/riscv/riscv-gnu-toolchain). When
installing it, make sure to also have the accompanying binutils. Adjust the
respective linker scripts and memory setups to start at the offset after oreboot
main in the flash layout.

For the second approach, an implementation of SBI, the RISC-V Supervisor Binary
Interface, is necessary. That may be its own payload, or using RustSBI, which is
implemented here within oreboot statically.
