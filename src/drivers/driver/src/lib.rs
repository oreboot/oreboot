#![no_std]
#![allow(non_snake_case)]

use core::slice::{from_raw_parts, from_raw_parts_mut};

pub type Result<T> = core::result::Result<T, &'static str>;

const EOF : Result<usize> = Err("EOF");
const NOT_IMPLEMENTED : Result<usize> = Err("not implemented");

pub trait Driver {
    /// Initialize the driver.
    fn init(&mut self);
    /// Returns number of bytes read.
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize>;
    /// Returns number of bytes written.
    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize>;
    /// Cleanup the driver.
    fn close(&mut self);
}

pub struct DoD<'a> {
    drivers: &'a mut [&'a mut Driver],
}

impl<'a> DoD<'a> {
    pub fn new(drivers: &'a mut [&'a mut Driver]) -> DoD<'a> {
        DoD{drivers,}
    }
}

impl<'a> Driver for DoD<'a> {
    fn init(&mut self) {
        self.drivers.iter_mut().fold((), |_, d| d.init())
    }

    fn pread(&self, _data: &mut [u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    // If there are multiple errors, the last one is returned.
    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize> {
        self.drivers.iter_mut().fold(Ok(0), |ret, d|
            match (ret, d.pwrite(data, pos)) {
                (Ok(sum), Ok(count)) => Ok(sum + count),
                (_, err @ Err(_)) => err,
                (err, _) => err,
            }
        )
    }

    fn close(&mut self) {
        self.drivers.iter_mut().fold((), |_, d| d.close())
    }
}

/// The driver reads directly from memory.
pub struct Memory {
}

impl Memory {
    pub fn new() -> Memory {
        return Memory{}
    }
}

impl Driver for Memory {
    fn init(&mut self) {
    }

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

    fn close(&mut self) {
    }
}

/// The driver reads from a slice.
pub struct SliceReader<'a> {
    data: &'a [u8],
}

impl<'a> SliceReader<'a> {
    pub fn new(data: &'a [u8]) -> SliceReader {
        return SliceReader{
            data: data,
        }
    }
}

impl<'a> Driver for SliceReader<'a> {
    fn init(&mut self) {
    }

    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if pos > self.data.len() {
            return EOF
        }
        let count = core::cmp::min(data.len(), self.data.len() - pos);
        data[..count].copy_from_slice(&self.data[pos..pos+count]);
        Ok(data.len())
    }

    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn close(&mut self) {
    }
}

/// The driver reads from a section (offset+size) of another driver.
pub struct SectionReader<'a> {
    driver: &'a Driver,
    offset: usize,
    size: usize,
}

impl<'a> SectionReader<'a> {
    pub fn new(driver: &'a Driver, offset: usize, size: usize) -> SectionReader {
        return SectionReader{
            driver: driver,
            offset: offset,
            size: size,
        }
    }
}

impl<'a> Driver for SectionReader<'a> {
    fn init(&mut self) {
    }

    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if pos > self.size {
            return EOF
        }
        let count = core::cmp::min(data.len(), self.size - pos);
        self.driver.pread(&mut data[..count], pos + self.offset)
    }

    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn close(&mut self) {
    }
}
