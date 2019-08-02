#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]


#[no_mangle]
pub extern "C" fn _start() -> ! {
    architecture::halt()
}


