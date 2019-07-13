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
use core::fmt::Write;
use drivers::model::Driver;
use drivers::uart::ns16550::NS16550;
use drivers::wrappers::DoD;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut uarts = [
        // Only Uart5 is connected.
        // TODO: PL011::new(0x1E78_3000, 115200),
        // TODO: PL011::new(0x1E78_D000, 115200),
        // TODO: PL011::new(0x1E78_E000, 115200),
        // TODO: PL011::new(0x1E78_F000, 115200),
        &mut NS16550::new(0x1E78_4000, 115200) as &mut dyn Driver,
    ];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(console);
    fmt::write(w, format_args!("{} {}\r\n", "Formatted output:", 7)).unwrap();

    w.write_str("Starting CPU init\r\n").unwrap();
    cpu::init(); // TODO: does this do anything yet?
    w.write_str("Completed CPU init\r\n").unwrap();

    w.write_str("Starting RAM init\r\n").unwrap();
    asmram::ram(w);
    w.write_str("Completed RAM init\r\n").unwrap();

    w.write_str("Starting chain\r\n").unwrap();
    chain(); // TODO: What is chain supposed to do? It doesn't return.
    w.write_str("Completed chain\r\n").unwrap();

    w.write_str("Starting romstage\r\n").unwrap();
    // TODO: romstage::romstage();
    w.write_str("Romstage exited -- halting\r\n").unwrap();
    halt()
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
