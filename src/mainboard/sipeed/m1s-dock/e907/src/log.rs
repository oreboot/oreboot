//! Log system for BT0
// essentially copied from sunxi/nezha

use crate::init::Serial;
use core::fmt;
use embedded_hal::serial::blocking::Write;
// use embedded_hal::serial::nb::Write;
// use nb::block;

type S = Wrap<Serial>;

#[doc(hidden)]
pub(crate) static mut LOGGER: Option<Logger> = None;

// type `Serial` is declared outside this crate, avoid orphan rule
pub(crate) struct Wrap<T>(T);

#[doc(hidden)]
pub(crate) struct Logger {
    pub(crate) inner: S,
}

impl fmt::Write for S {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.as_bytes() {
            self.0.write(&[*byte]).ok();
            // self.0.write(*byte).unwrap();
        }
        // block!(self.0.flush()).unwrap();
        Ok(())
    }
}

#[inline]
pub fn set_logger(serial: Serial) {
    unsafe {
        LOGGER = Some(Logger {
            inner: Wrap(serial),
        });
    }
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    unsafe {
        if let Some(l) = &mut LOGGER {
            l.inner.write_fmt(args).unwrap();
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::log::_print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::log::_print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}
