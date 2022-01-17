#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![deny(warnings)]

mod romstage;
use core::arch::asm;
use core::arch::global_asm;
use core::fmt::Write;
use core::mem::zeroed;
use core::ptr::write_volatile;

use model::Driver;
use wrappers::DoD;

#[no_mangle]
pub fn _init() -> ! {
    extern "C" {
        static mut _bss: u32;
        static mut _ebss: u32;
        static mut _stack: u32;
        static mut _estack: u32;
    }

    unsafe {
        let mut bss: *mut u32 = &mut _bss;
        let ebss: *mut u32 = &mut _ebss;
        while bss < ebss {
            write_volatile(bss, zeroed());
            bss = bss.offset(1);
        }
    }

    unsafe {
        let stack: *mut u32 = &mut _stack;
        let mut estack: *mut u32 = &mut _estack;
        while estack < stack {
            write_volatile(estack, 0xdeadbeef);
            estack = estack.offset(1);
        }
    }
    main()
}

fn main() -> ! {
    let mut pl011 = uart::pl011::PL011::new(0x09000000, 115200);
    let uart_driver: &mut dyn Driver = &mut pl011;
    // TODO: Handle error here and quit, rather than unwrapping.
    uart_driver.init().unwrap();
    uart_driver.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    let s = &mut [uart_driver];
    let console = &mut DoD::new(s);

    cpu::init();
    let mut w = print::WriteTo::new(console);
    write!(w, "hi").expect("blame ryan");
    write!(w, "1").expect("blame ryan");
    write!(w, "2").expect("blame ryan");
    write!(w, "3").expect("blame ryan");
    write!(w, "4").expect("blame ryan");
    write!(w, "5").expect("blame ryan");
    write!(w, "6").expect("blame ryan");
    write!(w, "7").expect("blame ryan");
    romstage::romstage(&mut w)
}
use core::panic::PanicInfo;

pub fn halt() -> ! {
    loop {
        unsafe { asm!("wfe") }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

global_asm!(include_str!("vector_table.S"));
