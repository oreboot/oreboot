pub fn init() {
    oreboot_arch::riscv32::init()
}

// There is no ibex-specific way of halting yet.
pub use oreboot_arch::riscv32::halt;
