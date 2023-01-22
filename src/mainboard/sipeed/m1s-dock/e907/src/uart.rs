use log::{Error, Serial};

#[derive(Debug)]
pub struct BSerial {
    u0: bl808_pac::UART0,
    u1: bl808_pac::UART1,
}

impl BSerial {
    #[inline]
    pub fn new(u0: bl808_pac::UART0, u1: bl808_pac::UART1) -> Self {
        // TX config
        u0.transmit_config.write(|w| {
            w.word_length()
                .eight()
                .stop_bits()
                .one()
                .freerun()
                .enable()
                .function()
                .enable()
        });
        u1.transmit_config.write(|w| {
            w.word_length()
                .eight()
                .stop_bits()
                .one()
                .freerun()
                .enable()
                .function()
                .enable()
        });
        /* baud rate configuration */
        let period = u0.bit_period.read();
        let rxp = period.receive().bits();
        let txp = period.transmit().bits();
        u1.bit_period
            .write(|w| w.transmit().variant(txp).receive().variant(rxp));
        Self { u0, u1 }
    }
}

impl Serial for BSerial {
    fn debug(&self, num: u8) {
        self.u0.data_write.write(|w| w.value().variant(num));
    }
}

impl embedded_hal::serial::ErrorType for BSerial {
    type Error = Error;
}

impl embedded_hal::serial::nb::Write<u8> for BSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        if self.u1.bus_state.read().transmit_busy().is_busy() {
            return Err(nb::Error::WouldBlock);
        }
        self.u1.data_write.write(|w| w.value().variant(c));
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), self::Error> {
        // TODO
        if true {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
