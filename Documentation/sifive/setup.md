# HiFive Unleashed Setup

This tutorial gets you setup to run Oreboot on the HiFive Unleashed board.

1. Follow the main README to setup the Rust toolchain for RISC-V.

2. Confirm the build works by running Oreboot in QEMU. If it works, you should
   see the words "Welcome to oreboot" and "TESTTESTTEST" printed to your
   terminal.

```
cd src/mainboard/sifive/hifive
cargo make -p release run
```

| ⚠️ **WARNING:** QEMU and hardware use different linker scripts. Before proceeding to the next step, you must manually copy link_hw.ld to link.ld and "rm -rf" the target directory. |
| --- |

3. Create a flash image.

```
cargo make -p release
truncate -s 32M flash.rom
dd of=flash.rom if=target/riscv64imac-unknown-none-elf/release/oreboot.bin conv=notrunc
```

4. While flashing, make sure all external power is removed from the board.
   Using a 16-pin Pomona clip, connect a SF100 to the board as seen in the
   image below.

![Dediprog](dediprog.jpg)

5. Flash it. Make sure to build flashrom from source because the debian packages tend to be too old.

```
sudo flashrom -p dediprog -w flash.rom
```

6. Oreboot is designed to be run immediately after the Zeroth Stage Boot Loader
   (ZSBL), skipping the First Stage Boot Loader (FSBL) and Berkley Boot Loader
   (BBL). To perform this skip and run Oreboot directly out of the SPI0
   flashchip, set the MSEL dip switches to the value of 0b0001. Note that the
   ON label is 0 on these switches.

![MSEL](msel.jpg)

7. Now connect power to a wall outlet and micro USB to your Linux computer. Use
   the big red power switch to power the board. Use the smaller red tact switch
   to reset the processor. When the board is powered, two new /dev/ttyUSBx
   devices should appear on your computer. The device with the higher number is
   UART0. Use minicom to see "Welcome to oreboot" and TESTTESTTEST printed.

```
minicom -D /dev/ttyUSB1 -b 115200
```

![USB](usb.jpg)
