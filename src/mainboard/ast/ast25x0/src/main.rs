#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![deny(warnings)]

use model::Driver;
use uart::ns16550x32::NS16550x32;

const BAUDRATE: u32 = 115200;
const UART_CLK: u32 = 24_000_000;

#[no_mangle]
pub fn _start() {
    soc::init();
    // TODO: Set UART routing to UART5=IO5 (default, but should still set it)
    let syscon = &mut NS16550x32::new(soc::reg::UART5, BAUDRATE, UART_CLK);
    syscon.init().expect("Failed to initialize system console");
    syscon.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let mut _i = 0;
    loop {
        syscon.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
        _i += 1;
    }
}

// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

