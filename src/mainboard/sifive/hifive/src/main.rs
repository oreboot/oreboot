#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;

use core::fmt;
use core::fmt::Write;
use model::Driver;
use soc::clock::Clock;
use clock::ClockNode;
use uart::sifive::SiFive;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    uart0.init();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let mut clks = [uart0 as &mut dyn ClockNode];
    let mut clk = Clock::new(&mut clks);
    clk.pwrite(b"on", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);
    fmt::write(w, format_args!("{} {}\r\n", "Formatted output:", 7)).unwrap();

    w.write_str("TESTTESTTEST\r\n").unwrap();
    architecture::halt()
}
