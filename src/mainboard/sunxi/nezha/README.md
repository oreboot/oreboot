# Allwinner Nezha / D1

The Allwinner (aka sunxi) Nezha board features a D1 SoC, based on the XuanTie
C906 core. It has a 256 MB NAND flash, large enough for oreboot and LinuxBoot.

More comprehensive information can be found in the [linux-sunxi wiki](
https://linux-sunxi.org/index.php?title=Allwinner_Nezha).

## Running oreboot

For now, there is no tooling to build a full image that could run off flash, an
SD card or a USB drive. Fortunately, [xfel](https://github.com/xboot/xfel) can
initialize DRAM over the FEL BROM loader, then transfer code to memory and
execute it. Writing to NAND flash is not supported as of the time writing this.

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

The first approach is trivial: Set the environment variable `PAYLOAD_A` and let
it point to the payload, as usual. Useful examples:

- [D1 UART C example](
  https://github.com/bigmagic123/d1-nezha-baremeta/tree/main/src/3.uart)
- [xv6-d1](https://github.com/michaelengel/xv6-d1)

Both of these examples would need to be compiled using a C toolchain such as the
[GNU toolchain for RISC-V](https://github.com/riscv/riscv-gnu-toolchain). When
installing it, make sure to also have the accompanying binutils. Adjust the
respective linker scripts and memory setups to start at the offset after oreboot
in the flash layout file `fixed-dtfs.dts`.

For the second approach, and implementation of SBI, the RISC-V Supervisor Binary
Interface, is necessary. That may be its own payload or implemented within
oreboot statically. Either way, the flash layout needs to be adjusted for it to
fit and the code in order to load it and the next payload to memory.
