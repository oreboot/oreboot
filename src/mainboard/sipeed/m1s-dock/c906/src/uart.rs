use log::{Error, Serial};

#[derive(Debug)]
pub struct BSerial {
    u0: bl808_pac::UART0,
}

impl BSerial {
    #[inline]
    pub fn new(u0: bl808_pac::UART0) -> Self {
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
        /* baud rate configuration */
        let period = u0.bit_period.read();
        let rxp = period.receive().bits();
        let txp = period.transmit().bits();
        u0.bit_period
            .write(|w| w.transmit().variant(txp).receive().variant(rxp));
        Self { u0 }
    }
}

impl Serial for BSerial {}

impl embedded_hal_nb::serial::ErrorType for BSerial {
    type Error = Error;
}

impl embedded_hal_nb::serial::Write<u8> for BSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        if self.u0.bus_state.read().transmit_busy().is_busy() {
            return Err(nb::Error::WouldBlock);
        }
        self.u0.data_write.write(|w| w.value().variant(c));
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
