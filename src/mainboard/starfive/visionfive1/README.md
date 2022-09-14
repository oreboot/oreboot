# JH7100 / StarFive VisionFive1 / BeagleV Starlight

This port mainly targets the [StarFive VisionFive 1 board](https://doc-en.rvspace.org/Doc_Center/visionfive.html) board, which is based on
the [JH7100 SoC](https://doc-en.rvspace.org/Doc_Center/jh7100.html), featuring
two [U74 cores from SiFive](https://www.sifive.com/cores/u74). It may also work
for the [BeagleV Starlight](https://github.com/beagleboard/beaglev-starlight),
which had been a protoype and dropped. The [original plan was to release a fully
open RISC-V board](https://beagleboard.org/static/beagleV/beagleV.html).

Aside: There is also a [SoM designed by Antmicro named ARVSOM](https://antmicro.com/blog/2021/04/arv-som-announcement/), featuring a SPI flash
and a connector that appears to mimic the Raspberry Pi CM4. In the [repository](https://github.com/antmicro/arvsom) are KiCAD schematics and board design, but
so far there appears to be no publicly buyable product.

## Documentation and Licenses

**NOTE**: The [official documentation is very sparse](https://github.com/starfive-tech/JH7100_docs).
However, there are sources available under the GPLv2 license for multiple
components, which have been translated and merged in this implementation:

- [DRAM init](https://github.com/starfive-tech/JH7100_ddrinit)
- ["second" boot](https://github.com/starfive-tech/JH7100_secondBoot)

## Run and Flash oreboot

To run oreboot from SRAM or flash it to the on-board SPI flash, you can use its
mask ROM's protocol based on X-modem transfer. Clone the repo and build the
`jh7100-recover` tool [from Heinrich Schuchardt's JH71xx-tools fork](https://github.com/xypron/JH71xx-tools/). Add it to your `$PATH`.

**NOTE**: You will need to connect to _two_ UARTs for the following procedure;
one is on the debug header, which is where the protocol is running, and the
other one is on the [40 pin header, pins `8` and `10`](https://doc-en.rvspace.org/VisionFive/Quick_Start_Guide/VisionFive_QSG/pinout_diagram.html).

On the debug header, the mask ROM's protocol runs at `9600` baud, which the
`jh7100-recover` handles. It will run oreboot when loaded to SRAM, and then
oreboot moves the UART (it is the same, UART0) to pins `8` and `10`, running at
`115200` baud. We retain this behavior from the vendor's firmware. This allows
for easy development, i.e., connect to the header on pins `8` and `10` first,
using `minicom`, `picocom`, `screen` or whatever you prefer, and now you can
load oreboot, as well as rebuild and rerun when you make changes.

To load oreboot to SRAM, run `make run` in this directory. Then hold the _BOOT_
button on the board, press the _RESET_ button once, and release the _BOOT_
button again. The transfer should then start, and take about 15 seconds.

TODO: setup and instructions for flashing
