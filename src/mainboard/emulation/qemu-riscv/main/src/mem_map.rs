// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
pub const CLINT_BASE: usize = 0x0200_0000;
pub const PLIC_BASE: usize = 0x0c00_0000;
pub const PAYLOAD_ADDR: usize = 0x8020_0000;

const MTIME_COMPARE_OFFSET: usize = 0x4000;
pub const MTIME_COMPARE: usize = CLINT_BASE + MTIME_COMPARE_OFFSET;
