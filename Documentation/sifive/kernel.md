
```
git clone https://github.com/torvalds/linux
git checkout v5.2
export ARCH=riscv
export CROSS_COMPILE=riscv64-linux-gnu-
make tinyconfig
make menuconfig
# Set the following:
#     CONFIG_CLK_SIFIVE=y
#     CONFIG_CLK_SIFIVE_FU540_PRCI=y
#     CONFIG_SERIAL_SIFIVE=y
#     CONFIG_SERIAL_SIFIVE_CONSOLE=y
#     CONFIG_SERIAL_SIFIVE_PLIC=y
#     CONFIG_SPI_SIFIVE=y
#     CONFIG_PRINTK=y
#     CONFIG_CMDLINE=console=ttySIF0
make -j12
qemu-system-riscv64 -machine sifive_u -m 2G -nographic -device loader,addr=0x80000000,file=arch/riscv/boot/Image
```

Crashses on this line:

```
0x00000000800000dc:  12000073          sfence.vma      zero,zero
0x00000000800000e0:  8082              ret
```
