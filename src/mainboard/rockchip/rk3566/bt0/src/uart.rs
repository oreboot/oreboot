use log::{Error, Serial};
use util::mmio::{read32, write32};

use crate::mem_map::UART2_BASE;

const UART2_THR: usize = UART2_BASE + 0x0000; /* Transmitter holding reg. */
const UART2_LCR: usize = UART2_BASE + 0x000C;
const UART2_LSR: usize = UART2_BASE + 0x0014; /* Line status reg.         */
const UART2_SHADOW_FIFO: usize = UART2_BASE + 0x0098;
const LSR_THRE: u32 = 1 << 6; /* transmit holding register empty */

#[derive(Debug)]
pub struct RKSerial();

impl RKSerial {
    pub fn new() -> Self {
        // LCR: DLAB is bit 7 for config, 0x3 for 8 bits
        write32(UART2_LCR, (1 << 7) | 0x3);
        // DLL: divisor latch low
        write32(UART2_THR, 0x1);
        // LCR: DLAB off, config done, keep 0x3 for 8 bits
        write32(UART2_LCR, 0x3);
        // shadow FIFO enable
        write32(UART2_SHADOW_FIFO, 0x1);
        Self()
    }
}

impl Serial for RKSerial {}

impl embedded_hal_nb::serial::ErrorType for RKSerial {
    type Error = Error;
}

impl embedded_hal_nb::serial::Write<u8> for RKSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        if read32(UART2_LSR) & LSR_THRE == 0 {
            return Err(nb::Error::WouldBlock);
        }
        write32(UART2_THR, c.into());
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
