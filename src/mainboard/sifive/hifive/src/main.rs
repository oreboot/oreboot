#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;

use core::fmt;
use core::fmt::Write;
use model::Driver;
use soc::is_qemu;
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

    let w = &mut print::WriteTo::new(uart0);

    // The memtest is broken on QEMU because QEMU loads the firmware into memory at address
    // 0x80000000. When we try to test this memory, we hose our own program. This works on hardware
    // because hardware is running off of SPI which is mapped at some other address.
    if !is_qemu() {
        w.write_str("Testing DDR...\r\n").unwrap();
        match test_ddr(0x80000000 as *mut u32, 4) {
            Err((a, v)) => fmt::write(w,format_args!(
                    "Unexpected read 0x{:x} at address 0x{:x}\r\n", v, a as usize)).unwrap(),
            _ => w.write_str("Passed\r\n").unwrap(),
        }
    }

    w.write_str("TESTTESTTEST\r\n").unwrap();
    architecture::halt()
}

// Returns Err((address, got)) or OK(()).
fn test_ddr(addr: *mut u32, size: usize) -> Result<(), (*const u32, u32)> {
    // Fill with data.
    for i in 0..(size/4) {
        unsafe { ptr::write(addr.add(i), (i+1) as u32) };
    }

    // Read back data.
    for i in 0..(size/4) {
        let v = unsafe {ptr::read(addr.add(i))};
        if v != i as u32 + 1 {
            return Err((unsafe {addr.add(i)}, v))
        }
    }
    Ok(())
}
