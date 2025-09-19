# Allwinner H616

## Developing oreboot

TL;DR `make run`

We currently use [`xfel`](https://github.com/xboot/xfel) to load our code over
USB. [FEL](https://linux-sunxi.org/FEL) is Allwinner's mask ROM loader.

Goal: initialize the [DRAM controller](https://linux-sunxi.org/DRAM_Controller).

## Boards

There are many development boards.

The oreboot contributors are currently using the following:

### [MangoPi MQ-Quad](https://mangopi.org/mangopi_mqquad)

We assign UART0 to the GPIO pins that a commonly used for RPi-like headers.
I.e., pin 8 is TX, pin 10 is RX.

See also: <https://linux-sunxi.org/MangoPi_MQ-Quad>

### [KickPI K2B](https://www.kickpi.com/product/k2b/)

We assign UART0 to the GPIO pins that are labeled for it on the 2x10-pin header.
I.e., pin 17 is TX, pin 15 is RX.

See also: <https://linux-sunxi.org/Kickpi_K2B_H618>
