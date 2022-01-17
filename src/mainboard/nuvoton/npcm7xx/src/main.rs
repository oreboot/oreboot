#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![deny(warnings)]

//use core::fmt;
//use core::fmt::Write;
//use model::Driver;
//use print;
//use wrappers::DoD;

// global_asm!(include_str!("../../../../../src/soc/sifive/fu540/src/bootblock.S"));
// global_asm!(include_str!("../../../../../src/soc/sifive/fu540/src/init.S"));

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    loop {}
}

// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
