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

// Check that the slice [offset:offset+len] is in [0:upper_bound].
fn in_bounds(offset: usize, len: usize, upper_bound: usize) -> bool {
    // Protects against overflows.
    len < upper_bound && offset < upper_bound - len
}

impl<'a> Driver for SliceReader<'a> {
    fn init(&mut self) {
    }

    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if !in_bounds(pos, data.len(), self.data.len()) {
            return EOF
        }
        data.copy_from_slice(&self.data[pos..pos+data.len()]);
        Ok(data.len())
    }

    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn close(&mut self) {
    }
}

/// The driver reads/writes from/to a section (offset+size) of another driver.
pub struct Section<'a> {
    driver: &'a mut Driver,
    offset: usize,
    size: usize,
}

impl<'a> Section<'a> {
    pub fn new(driver: &'a mut Driver, offset: usize, size: usize) -> Section {
        return Section{
            driver: driver,
            offset: offset,
            size: size,
        }
    }
}

impl<'a> Driver for Section<'a> {
    fn init(&mut self) {
        self.driver.init()
    }

    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if !in_bounds(pos, data.len(), self.size) {
            return EOF
        }
        self.driver.pread(data, pos + self.offset)
    }

    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize> {
        if !in_bounds(pos, data.len(), self.size) {
            return EOF
        }
        self.driver.pwrite(data, pos + self.offset)
    }

    fn close(&mut self) {
        self.driver.close()
    }
}
