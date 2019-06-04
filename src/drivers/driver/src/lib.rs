#![no_std]
#![allow(non_snake_case)]

pub trait Driver {
    /// Initialize the driver.
    fn init(&self);
    /// Returns number of bytes read.
    fn read(&self, data: &mut [u8]) -> usize;
    /// Returns number of bytes written.
    fn write(&self, data: &[u8]) -> usize;
    /// Cleanup the driver.
    fn close(&self);
}

pub struct DoD<'a> {
    Drivers: &'a [&'a mut Driver],
}

impl<'a> DoD<'a> {
    pub fn new(Drivers: &'a [&'a mut Driver]) -> DoD<'a> {
        DoD{Drivers,}
    }
}

impl<'a> Driver for DoD<'a> {
    fn init(&self) {
        for d in self.Drivers.iter() {
            d.init();
        }
    }

    fn read(&self, _data: &mut [u8]) -> usize {
        let i: usize = 0;
        // This is a bit weird but, basically, we do a read across all the drivers.
        // Later.
        i
    }

    fn write(&self, data: &[u8]) -> usize {
        self.Drivers.iter().fold(0, |sum, d| sum + d.write(data))
    }

    fn close(&self) {
        self.Drivers.iter().fold((), |_, d| d.close())
    }
}
