#![no_std]
#![deny(warnings)]

pub mod clock;
pub mod ctl;
pub mod ddr;
pub mod ddrregs;
pub mod phy;
pub mod reg;
pub mod ux00;

use core::ptr;

// TODO: There might be a better way to detect whether we are running in QEMU.
pub fn is_qemu() -> bool {
    // On hardware, the MSEL is only 4 bits, so it is impossible for it to reach this value.
    unsafe { ptr::read_volatile(reg::MSEL as *mut u32) == 0x297 }
}
