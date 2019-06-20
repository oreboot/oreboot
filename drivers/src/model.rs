pub type Result<T> = core::result::Result<T, &'static str>;

pub const EOF: Result<usize> = Err("EOF");
pub const NOT_IMPLEMENTED: Result<usize> = Err("not implemented");

pub trait Driver {
    /// Initialize the driver.
    fn init(&mut self) {}
    /// Returns number of bytes read.
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize>;
    /// Returns number of bytes written.
    fn pwrite(&mut self, data: &[u8], pos: usize) -> Result<usize>;
    /// Cleanup the driver.
    fn close(&mut self);
}
