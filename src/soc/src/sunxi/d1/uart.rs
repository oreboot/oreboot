//! Uart peripheral on BT0 stage
use super::ccu::{Clocks, Gating, Reset};
use super::gpio::{
    portb::{PB8, PB9},
    Function,
};
use super::time::Bps;
use core::ops::Deref;
use d1_pac::{
    uart::{
        lcr::{DLS_A, EPS_A, PEN_A, STOP_A},
        RegisterBlock,
    },
    CCU,
};

/// D1 serial peripheral
#[derive(Debug)]
pub struct Serial<UART: Instance, PINS> {
    pins: PINS,
    inner: UART,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    pub baudrate: Bps,
    pub wordlength: WordLength,
    pub parity: Parity,
    pub stopbits: StopBits,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(unused)] // should be used as exported structure in HAL crate
pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(unused)] // should be used as exported structure in HAL crate
pub enum Parity {
    None,
    Odd,
    Even,
}

/// Stop Bit configuration parameter for serial.
#[allow(unused)] // should be used as exported structure in HAL crate
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits, or 1.5 bits when WordLength is Five
    Two,
}

impl<UART: Instance, PINS: Pins<UART>> Serial<UART, PINS> {
    /// Create instance of Uart
    #[inline]
    pub fn new(uart: UART, pins: PINS, config: impl Into<Config>, clock: &Clocks) -> Self {
        // 1. unwrap parameters
        let Config {
            baudrate,
            wordlength,
            parity,
            stopbits,
        } = config.into();
        let bps = baudrate.0;
        // 2. init peripheral clocks
        // note(unsafe): async read and write using ccu registers
        let ccu = unsafe { &*CCU::ptr() };
        UART::assert_reset(ccu);
        UART::gating_mask(ccu);
        UART::deassert_reset(ccu);
        UART::gating_pass(ccu);
        // 3. set interrupt configuration
        // on BT0 stage we disable all uart interrupts
        #[rustfmt::skip]
        uart.ier().write(|w| {
            w.ptime()       .disable()
             .rs485_int_en().disable()
             .edssi()       .disable()
             .elsi()        .disable()
             .etbei()       .disable()
             .erbfi()       .disable()
        });
        // 4. calculate and set baudrate
        let uart_clk = (clock.apb1.0 + 8 * bps) / (16 * bps);
        let (dlh, dll) = ((uart_clk >> 8) as u8, (uart_clk & 0xff) as u8);
        uart.lcr.modify(|_, w| w.dlab().divisor_latch());
        uart.dlh().write(|w| unsafe { w.dlh().bits(dlh) });
        uart.dll().write(|w| unsafe { w.dll().bits(dll) });
        uart.lcr.modify(|_, w| w.dlab().rx_buffer());
        // 5. additional configurations
        let dls = match wordlength {
            WordLength::Five => DLS_A::FIVE,
            WordLength::Six => DLS_A::SIX,
            WordLength::Seven => DLS_A::SEVEN,
            WordLength::Eight => DLS_A::EIGHT,
        };
        let stop = match stopbits {
            StopBits::One => STOP_A::ONE,
            StopBits::Two => STOP_A::TWO,
        };
        let (pen, eps) = match parity {
            Parity::None => (PEN_A::DISABLED, EPS_A::ODD /* chosen randomly */),
            Parity::Odd => (PEN_A::ENABLED, EPS_A::ODD),
            Parity::Even => (PEN_A::ENABLED, EPS_A::EVEN),
        };
        #[rustfmt::skip]
        uart.lcr.modify(
            |_, w| {
                w.dls() .variant(dls)
                 .stop().variant(stop)
                 .pen() .variant(pen)
                 .eps() .variant(eps)
                 .bc()  .clear_bit()
            }, // todo: break control
        );
        // todo: pin configuration
        #[rustfmt::skip]
        uart.mcr.write(|w| {
            w.dtr()     .deasserted()
             .rts()     .deasserted()
             .loop_()   .normal()
             .afce()    .disabled()
             .function().uart()
        });
        // todo: fifo configuration
        #[rustfmt::skip]
        uart.fcr().write(|w| {
            w.fifoe() .set_bit()
             .rfifor().set_bit()
             .xfifor().set_bit()
             .dmam()  .mode_0()
             .tft()   .half_full()
             .rt()    .two_less_than_full()
        });
        // 6. return the instance
        Serial { pins, inner: uart }
    }
    // Close uart and release peripheral
    #[allow(unused)] // FIXME
    #[inline]
    pub fn free(self) -> (UART, PINS) {
        use core::ptr;
        let inner: UART = unsafe { ptr::read(&self.inner as *const _) };
        let pins: PINS = unsafe { ptr::read(&self.pins as *const _) };
        // self is closed via Drop trait
        (inner, pins)
    }
}

// Disable UART when drop; either next bootloading stage will initialize again,
// or we provide ownership of serial structure to next bootloading stage.
impl<UART: Instance, PINS> Drop for Serial<UART, PINS> {
    #[inline]
    fn drop(&mut self) {
        let ccu = unsafe { &*CCU::ptr() };
        UART::assert_reset(ccu);
        UART::gating_mask(ccu);
    }
}

pub trait Instance: Gating + Reset + Deref<Target = RegisterBlock> {}

impl Instance for d1_pac::UART0 {}

// note: if we want to assert RTS and/or CTS, implement Pins<UARTi> for them
// then users can use (tx, rx, rts, cts) in PINS type parameter
// and Serial::new function should take care of it to enable rtc and cts for peripheral

pub trait Pins<UART> {}

impl Pins<d1_pac::UART0> for (PB8<Function<6>>, PB9<Function<6>>) {}

/// Error types that may happen when serial transfer
#[derive(Debug)]
pub struct Error {
    kind: embedded_hal_nb::serial::ErrorKind,
}

impl embedded_hal_nb::serial::Error for Error {
    #[inline]
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        self.kind
    }
}

impl<UART: Instance, PINS> embedded_hal_nb::serial::ErrorType for Serial<UART, PINS> {
    type Error = Error;
}

impl<UART: Instance, PINS> embedded_hal_nb::serial::Write<u8> for Serial<UART, PINS> {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.inner.usr.read().tfnf().is_full() {
            return Err(nb::Error::WouldBlock);
        }
        self.inner.thr().write(|w| unsafe { w.thr().bits(word) });
        Ok(())
    }
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.inner.usr.read().tfe().is_empty() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
