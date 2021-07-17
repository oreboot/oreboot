#![no_std]

pub mod clock;
pub mod iopad;
pub mod iopadctl;
pub mod reg;
pub mod rstgen;
pub mod syscon;

use core::ptr;

// TODO: There might be a better way to detect whether we are running in QEMU.
pub fn is_qemu() -> bool {
    // On hardware, the instruction at 0x1008 is 'lw t1, -4(t0)'.
    //    unsafe { ptr::read_volatile(reg::QEMU_FLAG as *mut u32) == 0x02828613 }
    false
}
