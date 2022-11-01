#![doc = include_str!("README.md")]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    loop{
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop{
    }
}
