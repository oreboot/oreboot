#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

pub fn init() {}

use consts::DeviceCtl;
use model::{Driver, Result, NOT_IMPLEMENTED};

pub struct MMU {}

impl MMU {
    pub fn new() -> MMU {
        MMU {}
    }
}

impl Default for MMU {
    fn default() -> Self {
        Self::new()
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

    fn ctl(&mut self, __d: DeviceCtl) -> Result<usize> {
        NOT_IMPLEMENTED
    }
    fn stat(&self, _data: &mut [u8]) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    // Shutdown. Hmm.
    fn shutdown(&mut self) {}
}

extern "C" {
    fn mmu_get() -> usize;
    fn mmu_set(i: usize);
}

pub fn nop() {
    unsafe { asm!("nop") }
}

global_asm!(include_str!("mmu.S"));
