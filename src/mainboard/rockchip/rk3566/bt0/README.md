# Rockchip RK3566

There are two variants of this SoC: RK3566 and RK3568; for a comparison, see:
<https://www.armdesigner.com/article.php?id=303>

## Set up your board

You will need to put your target system in mask ROM mode. You may need to solder
extra pins or press a button. Please look at your board's documentation.

Boards we tested on:

- <https://wiki.radxa.com/Rock3/3a> SoC: RK3568; mask ROM mode pins are located
  between SoC and GPIO header
- <https://wiki.radxa.com/Rock3/3c> SoC: RK3566; mask ROM mode pins are next to
  the top USB port

We configure UART 2 to be on the GPIO header pins 8 (TX) and 10 (RX), so you can
attach a USB serial adapter just like on many other SBCs, in a row with the GND
pin 6. See your respective board documentation for details.

**NOTE**: The UART will run at 1.5 Mbaud (`1500000` baud).

## Run oreboot

Have either <https://github.com/xboot/xrock> installed or use
<https://github.com/platform-system-interface/rk_boot>.

The mask ROM will load our code to SRAM via, respectively:

```sh
xrock extra maskrom --rc4-off --sram code.bin
```

```sh
rk_boot run --sram code.bin
```

For convenience, invoke:

```sh
make LOADER="rk_boot run" run
```
