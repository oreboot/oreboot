#![no_std]

use consts::DeviceCtl;

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
    /// Control the device.
    /// The data is a DeviceCtl, an enum defined in the consts crate.
    /// To make DoD usage easier, e.g. for combined UARTs
    /// it is ok to call a ctl function with something it does not support.
    /// It will return an error only in the event
    /// of a real error. The result should include any disambiguating information
    /// so that if the Devices is part of a Device of Devices, it is possible
    /// to know which one failed.
    /// The On and Off operators should be idempotent.
    /// Leaving in the usize for now in case we want it for something.
    fn ctl(&mut self, d: DeviceCtl) -> Result<usize>;
    /// Status returns information about the device.
    /// It is allowed to return an empty result.
    /// TODO: possibly, the return ought to be a vector of Result,
    /// rather than copy to the mut [u8]?
    fn stat(&self, data: &mut [u8]) -> Result<usize>;
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
    /// multictl is useful for compound operations.
    /// E.g., one might wish to issue:
    /// Off, Baud, and On commands to a compound UART
    /// to avoid glitches.
    fn multictl(&self, _c: &[DeviceCtl]) -> Result<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests;
