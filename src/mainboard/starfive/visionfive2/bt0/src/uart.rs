use embedded_hal_nb::serial::{Error as _, ErrorType, Read, Write};
use log::{Error, Serial};

use crate::pac;

pub use jh71xx_hal::uart::Config;
use jh71xx_hal::uart::Uart;

/// Wrapper around the [`jh71xx_hal::uart::Uart`] UART peripheral type.
pub struct JH71XXSerial(jh71xx_hal::uart::Uart<pac::UART0>);

impl JH71XXSerial {
    /// Creates a new [JH71XXSerial] with a custom configuration.
    pub fn new_with_config(mut uart: pac::UART0, timeout: u64, config: Config) -> Self {
        Self(Uart::new_with_config(uart, timeout, config))
    }
}

impl Serial for JH71XXSerial {}

impl ErrorType for JH71XXSerial {
    type Error = Error;
}

impl Read for JH71XXSerial {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.0.read().map_err(|err| match err {
            nb::Error::Other(e) => nb::Error::Other(Error { kind: e.kind() }),
            nb::Error::WouldBlock => nb::Error::WouldBlock,
        })
    }
}

impl Write for JH71XXSerial {
    fn write(&mut self, val: u8) -> nb::Result<(), Self::Error> {
        self.0.write(val).map_err(|err| match err {
            nb::Error::Other(e) => nb::Error::Other(Error { kind: e.kind() }),
            nb::Error::WouldBlock => nb::Error::WouldBlock,
        })
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.0.flush().map_err(|err| match err {
            nb::Error::Other(e) => nb::Error::Other(Error { kind: e.kind() }),
            nb::Error::WouldBlock => nb::Error::WouldBlock,
        })
    }
}

// UART0 Clock = clk_osc (24Mhz)
pub const UART_CLK: usize = 24_000_000;

pub(crate) fn uart0_divisor() -> u16 {
    let uart0 = pac::uart0_reg();

    // Clear FIFOs to set UART0 to idle
    uart0
        .fcr()
        .modify(|_, w| w.rfifor().set_bit().xfifor().set_bit());
    while uart0.usr().read().busy().bit_is_set() {}

    uart0.lcr().modify(|_, w| w.dlab().set_bit());
    let div = uart0.dll().read().bits() | (uart0.dlh().read().bits() << 8);
    uart0.lcr().modify(|_, w| w.dlab().clear_bit());

    div as u16
}
