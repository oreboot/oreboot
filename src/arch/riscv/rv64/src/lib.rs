#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

use model::*;
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("wfi" :::: "volatile") }
    }
}

pub fn fence() {
    unsafe { asm!("fence" :::: "volatile") }
}

pub fn nop() {
    unsafe { asm!("nop" :::: "volatile") }
}

/// The driver reads directly from memory, but there may be restrictions
/// around word alignment and operation size.
pub struct IOPort;

impl Driver for IOPort {
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        let src = unsafe { from_raw_parts(pos as *const u8, data.len()) };
        data.copy_from_slice(src);
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize> {
        let dst = unsafe { from_raw_parts_mut(pos as *mut u8, data.len()) };
        dst.copy_from_slice(data);
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

