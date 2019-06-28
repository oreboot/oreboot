#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod print;
mod romstage;
use crate::romstage::asmram;
use crate::romstage::chain::chain;

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

    cpu::init();
    let w = &mut print::WriteTo::new(console);
    asmram::ram(w);
    chain()
    //romstage::romstage()
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
