#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    halt()
}
use core::panic::PanicInfo;
use soc::sifive::fu540;

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("" :::: "volatile") }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

