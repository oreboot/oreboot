## Connecting to the UART

- _debug_ UART using baud rate 9600; say: `picocom -b 9600 /dev/ttyUSB0`
- UART on 40 pin header with 115200 (default): `minicom -D /dev/ttyUSB1`

## Running oreboot from SRAM

Build the recovery tools:
https://github.com/xypron/JH71xx-tools/

We assume you have them in your `$PATH` now.

To run oreboot:
```sh
make && \
jh7100-recover -D /dev/ttyUSB0 -r \
../../../../target/riscv64imac-unknown-none-elf/release/seeed-beaglev-bootblob.bin
```

Or just use `run.sh`.
