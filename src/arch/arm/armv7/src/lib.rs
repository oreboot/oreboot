#![no_std]
#![allow(non_snake_case)]
#![feature(global_asm)]

pub fn init() {}

use drivers::model::{Driver, Result, NOT_IMPLEMENTED};

pub struct MMU {}

impl MMU {
    pub fn new() -> MMU {
        MMU {}
    }
}

impl Driver for MMU {
    fn init(&mut self) {}

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

extern "C" {
    fn mmu_get() -> usize;
    fn mmu_set(i: usize);
}

global_asm!(include_str!("mmu.S"));
