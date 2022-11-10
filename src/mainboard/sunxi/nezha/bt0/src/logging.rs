//! Log system for BT0

use core::fmt;
use embedded_hal::serial::nb::Write;
use nb::block;
use oreboot_soc::sunxi::d1::{
    gpio::{
        portb::{PB8, PB9},
        portg::{PG17, PG18},
        Function,
    },
    pac::UART0,
    uart::{self, Serial},
};
use spin::{Mutex, Once};

#[doc(hidden)]
pub(crate) static LOGGER: Once<LockedLogger> = Once::new();

type PBUART = (PB8<Function<6>>, PB9<Function<6>>);
type PGUART = (PG17<Function<7>>, PG18<Function<7>>);

type S = Wrap<Serial<UART0, PBUART>>;

// type `Serial` is declared out of this crate, avoid orphan rule
pub(crate) struct Wrap<T>(T);

#[doc(hidden)]
pub(crate) struct LockedLogger {
    pub(crate) inner: Mutex<S>,
}

impl fmt::Write for S {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.as_bytes() {
            block!(self.0.write(*byte)).unwrap();
        }
        block!(self.0.flush()).unwrap();
        Ok(())
    }
}

#[inline]
pub fn set_logger(serial: Serial<UART0, PBUART>) {
    LOGGER.call_once(|| LockedLogger {
        inner: Mutex::new(Wrap(serial)),
    });
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    LOGGER.wait().inner.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::logging::_print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::logging::_print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}
