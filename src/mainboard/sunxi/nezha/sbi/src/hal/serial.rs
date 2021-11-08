use super::{
    pac_encoding::{UART_LSR, UART_RBR, UART_THR, UART_USR},
    read_reg, write_reg,
};
use core::convert::Infallible;
use embedded_hal::serial::{Read, Write};
// const SUNXI_UART_USR_NF: u32 = 0x02;
const SUNXI_UART_USR_RFNE: u32 = 0x04;
pub struct Serial {
    uart: usize,
}
impl Serial {
    pub fn new(base: usize) -> Self {
        Self { uart: base }
    }
}
impl Read<u8> for Serial {
    type Error = Infallible;
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if unsafe { (read_reg::<u32>(self.uart, UART_LSR) & (1 << 0)) != 0 } {
            Ok(unsafe { (read_reg::<u32>(self.uart, UART_RBR) & 0xff) as u8 })
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
impl Write<u8> for Serial {
    type Error = Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        while unsafe { read_reg::<u32>(self.uart, UART_USR) & SUNXI_UART_USR_RFNE } == 0 {}
        unsafe { write_reg::<u32>(self.uart, UART_THR, word as u32) }
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        while unsafe { read_reg::<u32>(self.uart, UART_USR) & SUNXI_UART_USR_RFNE } == 0 {}
        Ok(())
    }
}
