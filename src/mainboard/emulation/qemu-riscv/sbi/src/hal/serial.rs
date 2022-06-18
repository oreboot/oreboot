use super::{
    pac_encoding::{
        UART_FCR,
        UART_IER,
        UART_LCR,
        UART_LSR,
        UART_MCR,
        UART_RBR,
        UART_THR, // UART_USR,
    },
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
        let x_self = Self { uart: base };
        /*
        unsafe {
            write_reg::<u8>(base, UART_IER, 0x0);
            // TODO
            //write_reg::<u8>(base, UART_LCR, 0x0);
            // TODO
            // write_reg::<u8>(base, UART_LCR, 0x0);
            write_reg::<u8>(base, UART_FCR, 0xc7);
            write_reg::<u8>(base, UART_MCR, 0x0b);
            // TODO
            // write_reg::<u8>(base, UART_LCR, 0x0);
        }
        */
        // FIXME: This works; why doesn't println! work?
        while unsafe { read_reg::<u8>(base, UART_LSR) & 1 << 6 } == 0 {}
        unsafe { write_reg::<u8>(base, UART_THR, 0x41) }
        while unsafe { read_reg::<u8>(base, UART_LSR) & 1 << 6 } == 0 {}
        unsafe { write_reg::<u8>(base, UART_THR, 0x42) }
        while unsafe { read_reg::<u8>(base, UART_LSR) & 1 << 6 } == 0 {}
        unsafe { write_reg::<u8>(base, UART_THR, 0x43) }
        x_self
    }
}
impl Read<u8> for Serial {
    type Error = Infallible;
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if unsafe { (read_reg::<u8>(self.uart, UART_LSR) & (1 << 0)) != 0 } {
            Ok(unsafe { read_reg::<u8>(self.uart, UART_RBR) & 0xff })
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
impl Write<u8> for Serial {
    type Error = Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // FIXME: which bit is correct?
        while unsafe { read_reg::<u8>(self.uart, UART_LSR) & 1 << 6 } == 0 {}
        unsafe { write_reg::<u8>(self.uart, UART_THR, word) }
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // TODO
        while unsafe { read_reg::<u8>(self.uart, UART_LSR) & 1 << 5 } == 0 {}
        Ok(())
    }
}
