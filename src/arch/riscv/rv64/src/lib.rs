#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { llvm_asm!("wfi" :::: "volatile") }
    }
}

pub fn fence() {
    unsafe { llvm_asm!("fence" :::: "volatile") }
}

pub fn nop() {
    unsafe { llvm_asm!("nop" :::: "volatile") }
}

