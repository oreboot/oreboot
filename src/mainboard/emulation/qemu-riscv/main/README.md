# QEMU RISC-V

## SBI test

Clone and build `sbitest` from <https://github.com/orangecms/sbitest>.

## Running oreboot

In this directory, run:

```sh
make PAYLOAD="path/to/sbitest_qemu.bin"
```

NOTE: You can also run other payloads, such as [barebox](https://barebox.org).

```sh
make PAYLOAD="path/to/barebox-2024.01.0/images/barebox-dt-2nd.img" run
```
