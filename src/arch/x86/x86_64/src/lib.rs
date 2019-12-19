#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("hlt" :::: "volatile") }
    }
}

pub fn fence() {
    unsafe { asm!("nop" :::: "volatile") }
}

pub fn nop() {
    unsafe { asm!("nop" :::: "volatile") }
}

