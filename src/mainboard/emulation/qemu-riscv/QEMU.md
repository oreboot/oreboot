Note:

qemu is not completely back yet.

To use qemu:

```
qemu-system-riscv64 -machine virt -bios ../../../..///target/riscv64imac-unknown-none-elf/release/emulation-qemu-riscv-bootblob.bin -nographic -s -monitor /dev/null -serial stdio
```

or if you want the monitor:

```
qemu-system-riscv64 -machine virt \
-bios ../../../..//oreboot/target/riscv64imac-unknown-none-elf/release/emulation-qemu-riscv-bootblob.bin \
-nographic -s -monitor /dev/tty -serial stdio
```

add -S to wait for gdb.

Remove this file when layoutflash or a similar tool is working.
