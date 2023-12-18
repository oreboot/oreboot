use log::{Error, Serial};

use crate::pac;

// UART0 Clock = clk_osc (24Mhz)
const UART_CLK: u32 = 24_000_000;
const UART_BAUDRATE_32MCLK_115200: u32 = 115200;
const DIVISOR: u32 = UART_CLK.saturating_div(16).saturating_div(UART_BAUDRATE_32MCLK_115200);

pub(crate) fn uart0_divisor() -> u16 {
    let uart0 = pac::uart0_reg();

    // Clear FIFOs to set UART0 to idle
    uart0.fcr().modify(|_, w| w.rfifor().set_bit().xfifor().set_bit());
    while uart0.usr().read().busy().bit_is_set() {}

    uart0.lcr().modify(|_, w| w.dlab().set_bit());
    let div = uart0.dll().read().bits() | (uart0.dlh().read().bits() << 8);
    uart0.lcr().modify(|_, w| w.dlab().clear_bit());

    div as u16
}

#[derive(Debug)]
pub struct JH71XXSerial();

impl JH71XXSerial {
    #[inline]
    pub fn new() -> Self {
        let uart0 = pac::uart0_reg();

        /* wair for UART0 to stop being busy */
        while uart0.usr().read().busy().bit_is_set() {}

        /* set DLAB to access DLL/DLH registers */
        uart0.lcr().modify(|_, w| w.dlab().set_bit());
        /* NOTE: Setting the divisor requires knowing the clock. */
        uart0.dll().write(|w| w.dll().variant(DIVISOR as u8));
        uart0.dlh().write(|w| w.dlh().variant((DIVISOR >> 8) as u8));
        /* clear the DLAB to access the other UART0 registers */
        uart0.lcr().modify(|_, w| w.dlab().clear_bit());

        /* 8 data bits, 1 stop bit, no parity */
        uart0.lcr().modify(|_, w| {
            w.dls().variant(0b11);
            w.stop().clear_bit();
            w.pen().clear_bit()
        });

        /* disable flow control */
        uart0.mcr().modify(|_, w| w.afce().clear_bit());

        /*
         * Program FIFO: enabled, mode 0 (set for compatibility with quark),
         * generate the interrupt at 8th byte
         * Clear TX and RX FIFO
         */
        uart0.fcr().modify(|_, w| {
            w.fifoe().set_bit();
            w.dmam().clear_bit();
            // Trigger on the 8th byte
            w.rt().variant(0b10);
            w.rfifor().set_bit();
            w.xfifor().set_bit()
        });

        uart0.ier().modify(|_, w| w.ptime().clear_bit()); // disable the serial interrupt

        Self()
    }
}

impl Serial for JH71XXSerial {}

impl embedded_hal_nb::serial::ErrorType for JH71XXSerial {
    type Error = Error;
}

impl embedded_hal_nb::serial::Write<u8> for JH71XXSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        let uart0 = pac::uart0_reg();
        if uart0.lsr().read().thre().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        uart0.thr().write(|w| w.thr().variant(c));
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
