#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;

use heapless::Vec;
use heapless::consts::*;

use core::fmt;
use core::fmt::Write;
use model::Driver;
use uart::sifive::SiFive;
use uart::log::Log;
use wrappers::DoD;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut v: Vec<u8, U1024> = Vec::new();
    let mut uarts = [
        &mut SiFive::new(/*soc::UART0*/0x10010000, 115200) as &mut dyn Driver,
        //&mut SiFive::new(/*soc::UART1*/0x10011000, 115200) as &mut dyn Driver,
        //&mut Log::new(&mut v) as &mut dyn Driver,
    ];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(console);
    fmt::write(w, format_args!("{} {}\r\n", "Formatted output:", 7)).unwrap();

    w.write_str("TESTTESTTEST\r\n").unwrap();
    architecture::halt()
}

