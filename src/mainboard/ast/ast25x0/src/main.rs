#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

use soc::asmram;

use arch::nop;

use core::fmt::Write;
use model::Driver;
use print;
use wrappers::DoD;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mmu = &mut arch::MMU::new();
    // not a lot to do if this fails.
    mmu.pwrite(b"off", 0).unwrap();
    let mut uarts = [
        // Only Uart5 is connected.
        // TODO: PL011::new(0x1E78_3000, 115200),
        // TODO: PL011::new(0x1E78_D000, 115200),
        // TODO: PL011::new(0x1E78_E000, 115200),
        // TODO: PL011::new(0x1E78_F000, 115200),
        //&mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200) as &mut dyn Driver,
    ];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init().ok();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(console);
    write!(w, "{} {}\r\n", "Formatted output:", 7).unwrap();

    write!(w, "Starting CPU init\r\n").unwrap();
    cpu::init(); // TODO: does this do anything yet?

    write!(w, "Completed CPU init\r\n").unwrap();

    write!(w, "Starting RAM init\r\n").unwrap();
    asmram::ram(w);
    write!(w, "Completed RAM init\r\n").unwrap();

    write!(w, "Starting chain\r\n").unwrap();
    //chain(); // TODO: What is chain supposed to do? It doesn't return.
    // TODO, chain also now doesn't compile.
    write!(w, "Completed chain\r\n").unwrap();

    write!(w, "Starting romstage\r\n").unwrap();
    // TODO: romstage::romstage();
    write!(w, "Romstage exited -- halting\r\n").unwrap();
    halt()
}
use core::panic::PanicInfo;

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        nop();
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

global_asm!(include_str!(
    "../../../../../src/soc/aspeed/ast25x0/src/vector_table.S"
));
