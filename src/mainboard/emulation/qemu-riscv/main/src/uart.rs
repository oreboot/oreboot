use log::{Error, Serial};
use ns16550a::Uart;

const UART_BASE: usize = 0x1000_0000;

pub struct QEMUSerial {
    uart: Uart,
}

impl QEMUSerial {
    pub fn new() -> Self {
        let uart = Uart::new(UART_BASE);
        uart.init(
            ns16550a::WordLength::EIGHT,
            ns16550a::StopBits::ONE,
            ns16550a::ParityBit::DISABLE,
            ns16550a::ParitySelect::EVEN,
            ns16550a::StickParity::DISABLE,
            ns16550a::Break::DISABLE,
            ns16550a::DMAMode::MODE0,
            100,
        );
        Self { uart }
    }
}

impl Serial for QEMUSerial {}

impl embedded_hal_nb::serial::ErrorType for QEMUSerial {
    type Error = Error;
}

impl embedded_hal_nb::serial::Write<u8> for QEMUSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        self.uart.put(c as u8);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), self::Error> {
        Ok(())
    }
}
