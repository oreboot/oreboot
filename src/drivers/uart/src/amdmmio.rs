/*
 * This file is part of the oreboot project.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

use core::ops;
use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::register_bitfields;

const RETRY_COUNT: u32 = 100_000;
const COM1: usize = 0xfedc9000;
const COM2: usize = 0xfedca000;

// We fill out as little of this as possible.
// We're firmware and should never plan to use it
// with all its features.
#[repr(C)]
pub struct RegisterBlock {
    // pathetically, this is also the DLL.
    d: ReadWrite<u32, D::Register>, /* data register */
    // This is the IER when we are not in DLAB mode.
    // We never intend to set inerrupts, so we just set
    // it up as thing we write to.
    dlm: ReadWrite<u32, D::Register>, /* data register */
    _8: u32,
    lcr: ReadWrite<u32, LCR::Register>,
    _10: u32,
    stat: ReadOnly<u32, STAT::Register>, /* status */
}

pub struct AMDMMIO {
    base: usize,
    // TODO: implement baudrate
    //baudrate: u32,
}

impl ops::Deref for AMDMMIO {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! {
    u32,
    D [
        Data OFFSET(0) NUMBITS(8) []
    ],
    STAT[
        DR OFFSET(0) NUMBITS(1) [],
        THRE OFFSET(5) NUMBITS(1) [],
        TEMT OFFSET(6) NUMBITS(1) []
    ],
    FCR[
        FIFOENABLE OFFSET(0) NUMBITS(1) []
    ],
    LCR[
        BITSPARITY OFFSET(0) NUMBITS(3) [
            EIGHTN1 = 3
        ],
        DLAB OFFSET(7) NUMBITS(1)[
            Data = 0,
            BaudRate = 1
        ]
    ]
}

impl AMDMMIO {
    pub fn new(base: usize) -> AMDMMIO {
        AMDMMIO { base: base }
    }

    pub fn com1() -> AMDMMIO {
        AMDMMIO { base: COM1 }
    }

    pub fn com2() -> AMDMMIO {
        AMDMMIO { base: COM2 }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for AMDMMIO {
    fn init(&mut self) -> Result<()> {
        self.lcr.write(LCR::BITSPARITY::EIGHTN1 + LCR::DLAB::BaudRate);
        self.dlm.set(0);
        self.d.set(1); // approx. 115200
        self.lcr.write(LCR::BITSPARITY::EIGHTN1 + LCR::DLAB::Data);
        self.dlm.set(0); // Just clear the IER to be safe.
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        'outer: for (read_count, c) in data.iter_mut().enumerate() {
            for _ in 0..RETRY_COUNT {
                let stat = self.stat.extract();
                if stat.is_set(STAT::DR) {
                    let d = self.d.extract();
                    *c = d.read(D::Data) as u8;
                    continue 'outer;
                }
            }
            return Ok(read_count);
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        'outer: for (sent_count, &c) in data.iter().enumerate() {
            for _ in 0..RETRY_COUNT {
                if self.stat.is_set(STAT::THRE) {
                    self.d.set(c.into());
                    continue 'outer;
                }
            }
            return Ok(sent_count);
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}
