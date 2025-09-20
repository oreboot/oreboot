use log::{Error, Serial};
use util::mmio::{read32, write32};

use crate::mem_map::{CCU_BASE, UART0_BASE};

const IER: usize = UART0_BASE + 0x0004;
const FCR: usize = UART0_BASE + 0x0008;
const LCR: usize = UART0_BASE + 0x000C;
const MCR: usize = UART0_BASE + 0x0010;
const USR: usize = UART0_BASE + 0x007C;

pub struct SunxiSerial;

impl SunxiSerial {
    pub fn new() -> Self {
        // disable interrupts
        write32(IER, 0);

        let hz = 24_000_000;
        let bps = 115_200;
        let clk = (hz + 8 * bps) / (16 * bps);
        let (dlh, dll) = ((clk >> 8) as u8, clk as u8);

        const DLAB: u32 = 1 << 7;
        let v = read32(LCR) | DLAB;
        write32(LCR, v);

        let v = read32(LCR) & !(0b00 << 4) & !(1 << 2) | (0b11 << 0);
        write32(LCR, v);

        write32(UART0_BASE + 0x0000, dll as u32);
        write32(UART0_BASE + 0x0004, dlh as u32);

        let v = read32(LCR) & !DLAB;
        write32(LCR, v);

        let v = read32(MCR) & !(0b11 << 6) & !(1 << 5) & !(1 << 4) & !(1 << 1) & !(1 << 0);
        write32(MCR, v);

        // let v = read32(FCR);
        // disable FIFO
        write32(FCR, 0);

        SunxiSerial {}
    }
}

impl Serial for SunxiSerial {}

impl embedded_hal_nb::serial::ErrorType for SunxiSerial {
    type Error = Error;
}

const TFE: u32 = 1 << 2;
const TFNF: u32 = 1 << 1;

impl embedded_hal_nb::serial::Write<u8> for SunxiSerial {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // FIXME: which register/bit do we need to check?
        for _ in 0..400 {
            core::hint::spin_loop();
        }
        // if read32(USR) & TFNF == 0 {
        if read32(USR) & TFE == 0 {
            return Err(nb::Error::WouldBlock);
        }
        write32(UART0_BASE, word as u32);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        /*
        if self.inner.usr.read().tfe().is_empty() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
        */
        Ok(())
    }
}
