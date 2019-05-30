#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
use core::intrinsics;

mod print;
mod romstage;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for &i in b"Welcome to oreboot\r\n" {
        putc(i);
    }
    cpu::init();
    print::print("hi");
    romstage::romstage()
}
use core::panic::PanicInfo;

fn putc(data: u8) {
    // UART address is specific to QEMU's virt machine.
    let uart_rx = 0x09000000 as *mut u32;
    unsafe {
        intrinsics::volatile_store(uart_rx, data as u32);
    }
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
