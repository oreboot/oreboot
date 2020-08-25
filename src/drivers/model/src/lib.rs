#![no_std]
#![deny(warnings)]

pub type Result<T> = core::result::Result<T, &'static str>;

pub const EOF: Result<usize> = Err("EOF");
pub const NOT_IMPLEMENTED: Result<usize> = Err("not implemented");

pub trait Driver {
    /// Initialize the device.
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    /// Positional read. Returns number of bytes read.
    ///
    /// If there is no more bytes to read, returns `EOF` error (note:
    /// `std::io::Read` trait would return 0 in that situation).
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize>;
    /// Positional write. Returns number of bytes written.
    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize>;
    /// Shutdown the device.
    fn shutdown(&mut self);
}
