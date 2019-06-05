#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;
mod romstage;
use core::fmt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut pl011 = pl011::PL011::new(0x09000000, 115200);
    let uart_driver: &mut driver::Driver = &mut pl011;
    uart_driver.init();
    uart_driver.pwrite(b"Welcome to oreboot\r\n", 0);
    let s = &mut [uart_driver];
    let console = &mut driver::DoD::new(s);

    cpu::init();
    let mut w = print::WriteTo::new(console);
    fmt::write(&mut w, format_args!("hi")).expect("blame ryan");
    fmt::write(&mut w, format_args!("1")).expect("blame ryan");
    fmt::write(&mut w, format_args!("2")).expect("blame ryan");
    fmt::write(&mut w, format_args!("3")).expect("blame ryan");
    fmt::write(&mut w, format_args!("4")).expect("blame ryan");
    fmt::write(&mut w, format_args!("5")).expect("blame ryan");
    fmt::write(&mut w, format_args!("6")).expect("blame ryan");
    fmt::write(&mut w, format_args!("7")).expect("blame ryan");
    fmt::write(&mut w, format_args!("{}{}\r\n", 3, "7")).expect("blame ryan");
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
