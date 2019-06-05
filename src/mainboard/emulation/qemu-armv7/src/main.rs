#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;
mod romstage;
use core::fmt;

use device_tree::Entry::{Node, Property};
use driver::Driver;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut pl011 = pl011::PL011::new(0x09000000, 115200);
    let uart_driver: &mut driver::Driver = &mut pl011;
    uart_driver.init();
    uart_driver.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    let s = &mut [uart_driver];
    let console = &mut driver::DoD::new(s);

    if let Err(err) = print_fdt(console) {
        let mut w = print::WriteTo::new(console);
        fmt::write(&mut w, format_args!("error: {}\n", err)).expect(err);
    }

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

pub fn print_fdt(console: &mut driver::Driver) -> driver::Result<()> {
    let mut w = print::WriteTo::new(console);
    let spi = driver::SliceReader::new(zimage::DTB);

    for entry in device_tree::FdtReader::new(&spi)?.walk() {
        match entry {
            Node{path: p} => {
                fmt::write(&mut w, format_args!(
                        "{:depth$}{}\r\n",
                        "", p.name(), depth=p.depth()*2)).unwrap();
            },
            Property{path: p, value: v} => {
                let buf = &mut [0; 1024];
                let len = v.pread(buf, 0)?;
                let val = device_tree::infer_type(&buf[..len]);
                fmt::write(&mut w, format_args!(
                        "{:depth$}{} = {}\r\n",
                        "", p.name(), val, depth=p.depth()*2)).unwrap();
            },
        }
    }
    Ok(())
}

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
