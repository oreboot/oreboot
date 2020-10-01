#![no_std]
#![deny(warnings)]

use core::slice::{from_raw_parts, from_raw_parts_mut};
use model::*;

pub struct DoD<'a> {
    drivers: &'a mut [&'a mut dyn Driver],
}

impl<'a> DoD<'a> {
    pub fn new(drivers: &'a mut [&'a mut dyn Driver]) -> DoD<'a> {
        DoD { drivers }
    }
}

impl<'a> Driver for DoD<'a> {
    fn init(&mut self) -> Result<()> {
        self.drivers.iter_mut().fold(Ok(()), |ret, d| {
            match (ret, d.init()) {
                (Ok(()), Ok(())) => Ok(()),
                (_, Err(err)) => Err(err),
                (err, _) => err,
            }
        })
    }

    fn pread(&self, _data: &mut [u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    // If there are multiple errors, the last one is returned.
    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize> {
        self.drivers.iter_mut().fold(Ok(0), |ret, d| {
            match (ret, d.pwrite(data, pos)) {
                (Ok(sum), Ok(count)) => Ok(sum + count),
                (_, err @ Err(_)) => err,
                (err, _) => err,
            }
        })
    }

    fn shutdown(&mut self) {
        self.drivers.iter_mut().for_each(|d| d.shutdown())
    }
}

/// The driver reads directly from memory.
pub struct Memory;

impl Driver for Memory {
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

/// The driver reads from a slice.
pub struct SliceReader<'a> {
    data: &'a [u8],
}

impl<'a> SliceReader<'a> {
    pub fn new(data: &'a [u8]) -> SliceReader {
        SliceReader { data }
    }
}

impl<'a> Driver for SliceReader<'a> {
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if pos >= self.data.len() {
            return EOF;
        }
        let count = data.len().min(self.data.len() - pos);
        data[..count].copy_from_slice(&self.data[pos..pos + count]);
        Ok(count)
    }

    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn shutdown(&mut self) {}
}

/// The driver reads from a section (offset+size) of another driver.
pub struct SectionReader<'a, D: Driver> {
    driver: &'a D,
    offset: usize,
    size: usize,
}

impl<'a, D: Driver> SectionReader<'a, D> {
    pub fn new(driver: &'a D, offset: usize, size: usize) -> SectionReader<D> {
        SectionReader { driver, offset, size }
    }
}

impl<'a, D: Driver> Driver for SectionReader<'a, D> {
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if pos >= self.size {
            return EOF;
        }
        let count = core::cmp::min(data.len(), self.size - pos);
        self.driver.pread(&mut data[..count], pos + self.offset)
    }

    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn shutdown(&mut self) {}
}
