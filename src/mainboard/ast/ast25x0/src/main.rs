#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;
mod romstage;
use crate::romstage::asmram;
use crate::romstage::chain::chain;
use core::fmt;

use device_tree::Entry::{Node, Property};
use drivers::model::{Driver, Result};
use drivers::uart::pl011::PL011;
use drivers::wrappers::{DoD, SliceReader};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut uarts = [
        // Only Uart5 is connected.
        // TODO: PL011::new(0x1E78_3000, 115200),
        // TODO: PL011::new(0x1E78_D000, 115200),
        // TODO: PL011::new(0x1E78_E000, 115200),
        // TODO: PL011::new(0x1E78_F000, 115200),
        &mut PL011::new(0x1E78_4000, 115200) as &mut dyn Driver,
    ];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    cpu::init();
    let w = &mut print::WriteTo::new(console);
    asmram::ram(w);
    chain()
    //romstage::romstage()
}
use core::panic::PanicInfo;

pub fn print_fdt(console: &mut dyn Driver) -> Result<()> {
    let mut w = print::WriteTo::new(console);
    let spi = SliceReader::new(zimage::DTB);

    for entry in device_tree::FdtReader::new(&spi)?.walk() {
        match entry {
            Node { path: p } => {
                fmt::write(&mut w, format_args!("{:depth$}{}\r\n", "", p.name(), depth = p.depth() * 2)).unwrap();
            }
            Property { path: p, value: v } => {
                let buf = &mut [0; 1024];
                let len = v.pread(buf, 0)?;
                let val = device_tree::infer_type(&buf[..len]);
                fmt::write(&mut w, format_args!("{:depth$}{} = {}\r\n", "", p.name(), val, depth = p.depth() * 2)).unwrap();
            }
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
