use core::arch::asm;

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("wfi") }
    }
}

pub fn fence() {
    unsafe { asm!("fence") }
}

pub fn nop() {
    unsafe { asm!("nop") }
}
