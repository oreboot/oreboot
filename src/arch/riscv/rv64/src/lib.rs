#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]

use core::panic::PanicInfo;

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("wfi" :::: "volatile") }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

pub fn fence() {
    unsafe { asm!("fence" :::: "volatile") }
}

pub fn nop() {
    unsafe { asm!("nop" :::: "volatile") }
}

