#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod romstage;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let pl011 = pl011::PL011::new(0x09000000, 115200);
    let uart_driver : &driver::Driver = &pl011;
    uart_driver.init();
    uart_driver.write(b"Welcome to oreboot\r\n");

    cpu::init();
    romstage::romstage()
}
use core::panic::PanicInfo;

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

global_asm!(include_str!("vector_table.S"));
