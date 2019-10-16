OpenSBI
-------

OpenSBI is used in the `FW_JUMP` mode. This jumps to 0x80200000 with a device
tree at 0x88000000.

```
git clone https://github.com/riscv/opensbi
export CROSS_COMPILE=riscv64-linux-gnu-
export PLATFORM_RISCV_XLEN=64
make PLATFORM=sifive/fu540
ls -l buildplatform/sifive/fu540/firmware/fw_jump.bin
```
