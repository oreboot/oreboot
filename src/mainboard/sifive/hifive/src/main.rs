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
use soc::ddr::DDR;
use clock::ClockNode;
use uart::sifive::SiFive;
use spi::SiFiveSpi;
use core::ptr;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    uart0.init();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    // Set SPIs to 50MHZ clock rate.
    let spi0 = &mut SiFiveSpi::new(0x10040000, 50_000_000);
    let spi1 = &mut SiFiveSpi::new(0x10041000, 50_000_000);
    let spi2 = &mut SiFiveSpi::new(0x10050000, 50_000_000);

    uart0.pwrite(b"Initializing clocks...\r\n", 0).unwrap();

    // Peripheral clocks get their dividers updated when the PLL initializes.
    let mut clks = [
        spi0 as &mut dyn ClockNode,
        spi1 as &mut dyn ClockNode,
        spi2 as &mut dyn ClockNode,
        uart0 as &mut dyn ClockNode,
    ];
    let mut clk = Clock::new(&mut clks);
    clk.pwrite(b"on", 0).unwrap();

    uart0.pwrite(b"Done\r\n", 0).unwrap();
    uart0.pwrite(b"Initializing DDR...\r\n", 0).unwrap();

    let mut ddr = DDR::new();
    ddr.pwrite(b"on", 0).unwrap();

    uart0.pwrite(b"Done\r\n", 0).unwrap();

    // Test DDR.
    let data = 0x12345678 as u32;
    let addr = 0x80000000 as *mut u32;
    unsafe { ptr::write(addr, data) };
    match unsafe { ptr::read(addr) } {
        data => uart0.pwrite(b"DDR init passed\r\n", 0).unwrap(),
        _ => uart0.pwrite(b"DDR init failed\r\n", 0).unwrap(),
    };

    uart0.pwrite(b"TESTTESTTEST\r\n", 0).unwrap();
    architecture::halt()
}
