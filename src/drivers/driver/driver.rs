#![no_std]

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
