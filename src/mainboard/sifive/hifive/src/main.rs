#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::riscv::rv64::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    halt()
}
//use soc::sifive::fu540;
//use arch::riscv::rv64;


