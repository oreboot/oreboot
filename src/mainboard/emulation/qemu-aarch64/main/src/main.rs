#![no_std]
#![no_main]
#![deny(warnings)]

mod romstage;

use core::{
    arch::global_asm,
    mem::zeroed,
    panic::PanicInfo,
    ptr::{addr_of_mut, write_volatile},
};

#[no_mangle]
pub fn _init() -> ! {
    extern "C" {
        static mut _bss: u32;
        static mut _ebss: u32;
        static mut _stack: u32;
        static mut _estack: u32;
    }

    unsafe {
        let mut bss: *mut u32 = addr_of_mut!(_bss);
        let ebss: *mut u32 = addr_of_mut!(_ebss);
        while bss < ebss {
            write_volatile(bss, zeroed());
            bss = bss.offset(1);
        }
    }

    unsafe {
        let stack: *mut u32 = addr_of_mut!(_stack);
        let mut estack: *mut u32 = addr_of_mut!(_estack);
        while estack < stack {
            write_volatile(estack, 0xdeadbeef);
            estack = estack.offset(1);
        }
    }

    main()
}

fn main() -> ! {
    romstage::romstage()
}

pub fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

global_asm!(include_str!("vector_table.S"));
