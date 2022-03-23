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
            match self.pread(data, pos) {
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
mod tests {
    use super::*;

    pub struct OneByteRead<'a> {
        buf: &'a [u8],
    }

    impl<'a> Driver for OneByteRead<'a> {
        fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
            if pos >= self.buf.len() {
                return EOF;
            }
            if data.len() == 0 {
                return Ok(0);
            }
            data[0] = self.buf[pos];
            Ok(1)
        }
        fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
            Err("not implemented")
        }
        fn ctl(&mut self, __d: DeviceCtl) -> Result<usize> {
            NOT_IMPLEMENTED
        }
        fn stat(&self, _data: &mut [u8]) -> Result<usize> {
            NOT_IMPLEMENTED
        }
        fn shutdown(&mut self) {}
    }

    #[test]
    fn pread_exact_reads_all_bytes() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let drv = OneByteRead { buf: &data };

        let mut got: [u8; 3] = [0, 0, 0];
        drv.pread_exact(&mut got, 0).unwrap();
        assert_eq!(data[0..3], got);
    }

    #[test]
    fn pread_exact_reads_all_bytes_from_custom_position() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let drv = OneByteRead { buf: &data };

        let mut got: [u8; 3] = [0, 0, 0];
        drv.pread_exact(&mut got, 1).unwrap();
        assert_eq!(data[1..4], got);
    }

    #[test]
    fn pread_exact_returns_ok_for_empty_data_buf() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let drv = OneByteRead { buf: &data };

        let mut got: [u8; 0] = [];
        drv.pread_exact(&mut got, 10).unwrap();
    }

    #[test]
    fn pread_exact_returns_eof_when_not_enough_bytes_available() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let drv = OneByteRead { buf: &data };

        let mut got: [u8; 5] = [0, 0, 0, 0, 0];
        assert_eq!(drv.pread_exact(&mut got, 0).err(), Some("EOF"));
    }
}
