use embedded_hal_nb::nb;
use embedded_hal_nb::serial::{ErrorType, Write};
use log::{Error, Serial};
use ns16550a::Uart;
pub struct VirtSerial {
    inner: Uart,
}

impl From<Uart> for VirtSerial {
    fn from(u: Uart) -> VirtSerial {
        VirtSerial { inner: u }
    }
}

impl ErrorType for VirtSerial {
    type Error = Error;
}

impl Write<u8> for VirtSerial {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        self.inner.put(word);
        Ok(())
    }
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        Ok(())
    }
}

impl Serial for VirtSerial {}
