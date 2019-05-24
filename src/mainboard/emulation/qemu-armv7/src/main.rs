// main.rs

#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
use core::intrinsics;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	for &i in b"Welcome to oreboot" {
		putc(i);
	}
	halt()
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
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("" :::: "volatile") }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
