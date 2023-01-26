use core::arch::asm;

pub fn hlt() {
    unsafe { asm!("hlt"); }
}
