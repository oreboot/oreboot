use core::ptr::{read_volatile, write_volatile};
use log::{Error, Serial};

const UART0_BASE: usize = 0x0414_0000;
const UART0_THR: usize = UART0_BASE + 0x0000; /* Transmitter holding reg. */
const UART0_LSR: usize = UART0_BASE + 0x0014; /* Line status reg.         */
const LSR_THRE: u8 = 0x20; /* transmit holding register empty */

fn read_8(reg: usize) -> u8 {
    unsafe { read_volatile(reg as *mut u8) }
}

fn write_8(reg: usize, val: u8) {
    unsafe {
        write_volatile(reg as *mut u8, val);
    }
}

#[derive(Debug)]
pub struct SGSerial();

impl SGSerial {
    pub fn new() -> Self {
        Self()
    }
}

impl Serial for SGSerial {}

impl embedded_hal_nb::serial::ErrorType for SGSerial {
    type Error = Error;
}

impl embedded_hal_nb::serial::Write<u8> for SGSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        if read_8(UART0_LSR) & LSR_THRE == 0 {
            return Err(nb::Error::WouldBlock);
        }
        write_8(UART0_THR, c);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), self::Error> {
        let tfe_empty = true;
        if tfe_empty {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
