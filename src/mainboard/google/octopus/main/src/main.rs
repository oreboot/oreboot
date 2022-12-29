#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(generator_trait)]
#![feature(panic_info_message)]

use buddy_system_allocator::LockedHeap;
use core::panic::PanicInfo;

mod bootblock;
mod chromeos;
mod consts;
mod ec;
mod romstage;
mod smihandler;

const HEAP_SIZE: usize = 0x8000; // 8KiB
static mut HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

extern "C" fn main() -> usize {
    0
}

/// This function is called on panic.
#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!("panic in '{}' line {}\n", location.file(), location.line(),);
        error!("{:?}", info.message());
    } else {
        error!("panic at unknown location\n");
    };
    loop {
        core::hint::spin_loop();
    }
}
