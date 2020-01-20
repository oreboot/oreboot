#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut _i = 0;
    loop {
        _i += 1;
    }
}

// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

global_asm!(include_str!("vector_table.S"));
