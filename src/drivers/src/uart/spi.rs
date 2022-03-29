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

use crate::model::{Driver, Result, NOT_IMPLEMENTED};
use consts::DeviceCtl;
use core::ptr;

// The "spi uart" is just a base address, which we offset
// with data. The one spi specific element is the dance we
// play to make sure SPI block caching does not hide an IO.
// The dance requires a read from 0, and offseting the
// character in the address 0xc00000 + char.
pub struct SPI {
    base: usize,
}

impl SPI {
    pub fn new(base: usize) -> SPI {
        Self { base }
    }
}

impl Driver for SPI {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (_idx, &c) in data.iter().enumerate() {
            {
                let y = self.base as *const u8;
                let o = (self.base + 0xc0_0000 + (c << 5) as usize) as *const u8;
                // SPI is block oriented and cached by the block. A block
                // is 32 bytes.
                // The bits we wish to display must hence be shifted left 5
                // bits to make them visible.
                // As to caching: a read to 0, followed by a read to 1,
                // will not show the read to 1; it
                // is part of the block read starting at zero. The result is that
                // a print of, e.g., 01234, will result in an apparent print
                // of 0. Since only one block is cached, we can 'flush' the
                // cache by reading a different block.
                // To defeat block caching, reference block 0,
                // then the "address" for the character, or'ed with 0xc00000
                // This results in a block read to 0, and a block read to the
                // address containing the character in bits 5-12.
                unsafe {
                    ptr::read_volatile(y);
                    ptr::read_volatile(o);
                }
            }
        }
        Ok(data.len())
    }

    fn ctl(&mut self, __d: DeviceCtl) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn stat(&self, _data: &mut [u8]) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn shutdown(&mut self) {}
}
