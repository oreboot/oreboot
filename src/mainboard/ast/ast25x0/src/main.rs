#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![deny(warnings)]

use model::Driver;
use uart::ns16550::NS16550;

const BAUDRATE: u32 = 115200;

#[no_mangle]
pub fn _start() {
    soc::init();
    let syscon = &mut NS16550::new(soc::reg::UART5, BAUDRATE);
    syscon.init().expect("Failed to initialize system console");
    syscon.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let mut _i = 0;
    loop {
        _i += 1;
    }
}

// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

