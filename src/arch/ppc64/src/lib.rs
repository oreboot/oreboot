#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

pub fn halt() -> ! {
    loop {}
}
