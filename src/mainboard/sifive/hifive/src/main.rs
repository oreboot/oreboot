#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;

use clock::ClockNode;
use core::fmt::Write;
use core::{fmt, ptr};
use device_tree::{FdtReader, Entry, infer_type};
use model::Driver;
use soc::clock::Clock;
use soc::ddr::DDR;
use spi::SiFiveSpi;
use uart::sifive::SiFive;
use wrappers::{Memory, SectionReader};

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
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

    let w = &mut print::WriteTo::new(uart0);

    w.write_str("## ROM Device Tree\r\n").unwrap();
    fmt::write(w, format_args!("ROM FDT address: 0x{:x}\n", fdt_address));
    // We have no idea how long the FDT really is. This caps it to 1MiB.
    let rom_fdt = &mut SectionReader::new(&Memory{}, fdt_address, 1024*1024);
    // TODO: For some reason, the following function call hangs on hardware (but passes in QEMU).
    // It looks like the fdt_address is garbage.
    //if let Err(err) = print_fdt(rom_fdt, w) {
    //    fmt::write(w, format_args!("error: {}\n", err)).unwrap();
    //}

    w.write_str("## Oreboot Fixed Device Tree\r\n").unwrap();
    // Fixed DTFS is at offset 512KiB in flash. Max size 512Kib.
    let fixed_fdt = &mut SectionReader::new(&Memory{}, 0x20000000 + 512*1024, 512*1024);
    if let Err(err) = print_fdt(fixed_fdt, w) {
        fmt::write(w, format_args!("error: {}\n", err)).unwrap();
    }

    w.write_str("Initializing DDR...\r\n").unwrap();
    let mut ddr = DDR::new();
    let m = match ddr.pwrite(b"on", 0) {
        Ok(size) => size,
        Err(error) => {
            panic!("problem initalizing DDR: {:?}", error);
        },
    };
    w.write_str("Done\r\n").unwrap();

    fmt::write(w,format_args!("Memory size is: {:x}\r\n", m)).unwrap();

    w.write_str("Testing DDR...\r\n").unwrap();
    match test_ddr(0x80000000 as *mut u32, m, w) {
        Err((a, v)) => fmt::write(w,format_args!(
                "Unexpected read 0x{:x} at address 0x{:x}\r\n", v, a as usize)).unwrap(),
        _ => w.write_str("Passed\r\n").unwrap(),
    }

    w.write_str("TESTTESTTEST\r\n").unwrap();
    architecture::halt()
}

// Returns Err((address, got)) or OK(()).
fn test_ddr(addr: *mut u32, size: usize, w: &mut print::WriteTo<>) -> Result<(), (*const u32, u32)> {
    w.write_str("Starting to fill with data\r\n").unwrap();
    // Fill with data.
    for i in 0..(size/4) {
        unsafe { ptr::write(addr.add(i), (i+1) as u32) };
    }

    w.write_str("Starting to read back data\r\n").unwrap();
    // Read back data.
    for i in 0..(size/4) {
        let v = unsafe {ptr::read(addr.add(i))};
        if v != i as u32 + 1 {
            return Err((unsafe {addr.add(i)}, v))
        }
    }
    Ok(())
}

// TODO: move out of mainboard
pub fn print_fdt(fdt: &mut dyn Driver, w: &mut print::WriteTo<>) -> Result<(), &'static str> {
    for entry in FdtReader::new(fdt)?.walk() {
        match entry {
            Entry::Node { path: p } => {
                fmt::write(w, format_args!("{:depth$}{}\r\n", "", p.name(), depth = p.depth() * 2)).unwrap();
            }
            Entry::Property { path: p, value: v } => {
                let buf = &mut [0; 1024];
                let len = v.pread(buf, 0)?;
                let val = infer_type(&buf[..len]);
                fmt::write(w, format_args!("{:depth$}{} = {}\r\n", "", p.name(), val, depth = p.depth() * 2)).unwrap();
            }
        }
    }
    Ok(())
}
