#![no_std]
//#![feature(global_asm)]
#![deny(warnings)]
use consts::DeviceCtl;

pub fn init() {}

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

    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
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
