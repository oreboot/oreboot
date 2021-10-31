#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use soc::ccu::CCU;
use soc::gpio::GPIO;
use uart::sunxi::Sunxi;
use uart::sunxi::UART0;

global_asm!(include_str!("../start.S"));

// hart = hardware thread (something like core)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // clock
    let mut ccu = CCU::new();
    ccu.init().unwrap();
    let mut gpio = GPIO::new();
    gpio.init().unwrap();
    let mut uart = Sunxi::new(UART0 as usize, 115200);
    uart.init().unwrap();
    uart.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    arch::halt()
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut Sunxi::new(UART0 as usize, 115200);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = writeln!(w, "PANIC: {}\r", info);
    arch::halt()
}
