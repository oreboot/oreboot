#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;

// use heapless::consts::*;
// use heapless::Vec;

use core::fmt;
use core::fmt::Write;
use model::Driver;
use soc::clock::Clock;
use soc::ddr::DDR;
// use uart::log::Log;
use uart::sifive::SiFive;
use wrappers::DoD;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let v: Vec<u8, U1024> = Vec::new();
    let mut uarts = [&mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200) as &mut dyn Driver];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(console);
    fmt::write(w, format_args!("{} {}\r\n", "Formatted output:", 7)).unwrap();

    w.write_str("Initializing clocks.\r\n").unwrap();
    let mut clk = Clock::new();
    clk.pwrite(b"on", 0).unwrap();

    w.write_str("Clocks initialized!\r\n").unwrap();
    let mut ddr = DDR::new();
    ddr.pwrite(b"on", 0).unwrap();

    architecture::halt()
}
