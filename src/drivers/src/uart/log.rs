/*
 * This file is part of the oreboot project.
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

/*
 * UART that just pushes into a vec.
 */

extern crate heapless; // v0.4.x
use crate::model::*;
use consts::DeviceCtl;
use heapless::consts::*;

use heapless::Vec;

pub struct Log<'a> {
    dat: &'a mut Vec<u8, U1024>,
}

impl<'a> Log<'a> {
    pub fn new(v: &'a mut Vec<u8, U1024>) -> Log {
        Log { dat: v }
    }
}

impl<'a> Driver for Log<'a> {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        Ok(0)
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for &c in data {
            self.dat.push(c).unwrap();
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
