#![feature(llvm_asm)]
#![feature(abi_efiapi)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitfield;

pub mod fsp20;

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { llvm_asm!("hlt" :::: "volatile") }
    }
}

pub fn fence() {
    unsafe { llvm_asm!("nop" :::: "volatile") }
}

pub fn nop() {
    unsafe { llvm_asm!("nop" :::: "volatile") }
}
