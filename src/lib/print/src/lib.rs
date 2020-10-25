#![no_std]

use core::fmt;
use model::Driver;

pub struct WriteTo<'a, D: Driver> {
    drv: &'a mut D,
}

impl<'a, D: Driver> WriteTo<'a, D> {
    pub fn new(drv: &'a mut D) -> Self {
        WriteTo { drv }
    }
}

impl<'a, D: Driver> fmt::Write for WriteTo<'a, D> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.drv.pwrite(s.as_bytes(), 0) {
            Err(_) => Err(fmt::Error),
            _ => Ok(()),
        }
    }
}
