use core::arch::global_asm;

global_asm!(include_str!("bootblock.S"));

// There is no earlgrey-specific way of halting yet.
pub use oreboot_cpu::lowrisc::ibex::halt;
