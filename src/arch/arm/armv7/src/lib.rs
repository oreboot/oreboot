#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

pub fn init() {}

use model::{Driver, Result, NOT_IMPLEMENTED};
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub struct MMU {}

impl MMU {
    pub fn new() -> MMU {
        MMU {}
    }
}

impl Driver for MMU {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn pwrite(&mut self, data: &[u8], _pos: usize) -> Result<usize> {
        let r: usize;
        match data {
            b"off" => {
                let mut r0 = unsafe { mmu_get() };
                // TODO: make this a register type
                r0 &= !0x00002300; // clear bits 13, 9:8 (--V- --RS)
                r0 &= !0x00000087; // clear bits 7, 2:0 (B--- -CAM)
                r0 |= 0x00000002; // set bit 1 (A) Align
                r0 |= 0x00001000; // set bit 12 (I) I-Cache
                unsafe {
                    mmu_set(r0);
                }
                r = 1;
            }
            b"on" => r = 1,
            _ => r = 0,
        }
        Ok(r)
    }

    // Shutdown. Hmm.
    fn shutdown(&mut self) {}
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


extern "C" {
    fn mmu_get() -> usize;
    fn mmu_set(i: usize);
}

global_asm!(include_str!("mmu.S"));
