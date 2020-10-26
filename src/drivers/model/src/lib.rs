#![no_std]

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
    /// Reads the exact number of bytes to fill in the `data`.
    /// Returns ok if `data` is empty.
    fn pread_exact(&self, mut data: &mut [u8], mut pos: usize) -> Result<()> {
        while !data.is_empty() {
            match self.pread(&mut data, pos) {
                Ok(0) => return Err("unexpected eof"),
                Ok(x) => {
                    data = &mut data[x..];
                    pos += x;
                }
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
