// TODO: rearrange and move to src/console

use core::fmt;
use embedded_hal::serial::nb::Write;
use nb::block;
use oreboot_soc::sunxi::d1::{
    gpio::{
        portb::{PB8, PB9},
        Function,
    },
    pac::UART0,
    uart::Serial,
};
use spin::{Mutex, Once};

#[doc(hidden)]
pub(crate) static LOGGER: Once<LockedLogger> = Once::new();

type S = Wrap<Serial<UART0, (PB8<Function<6>>, PB9<Function<6>>)>>;

// type `Serial` is declared out of this crate, avoid orphan rule
pub(crate) struct Wrap<T>(T);

#[doc(hidden)]
pub(crate) struct LockedLogger {
    pub(crate) inner: Mutex<S>,
}

impl fmt::Write for S {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            block!(self.0.write(*byte)).ok();
        }
        block!(self.0.flush()).ok();
        Ok(())
    }
}

#[inline]
pub fn set_logger(serial: Serial<UART0, (PB8<Function<6>>, PB9<Function<6>>)>) {
    LOGGER.call_once(|| LockedLogger {
        inner: Mutex::new(Wrap(serial)),
    });
}

#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut logger = $crate::logging::LOGGER.wait().inner.lock();
        core::write!(logger, $($arg)*).ok();
        core::write!(logger, "\r\n").ok();
    });
}

#[macro_export(local_inner_macros)]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($fmt: literal $(, $($arg: tt)+)?) => ({
        use core::fmt::Write;
        let mut logger = $crate::logging::LOGGER.wait().inner.lock();
        core::write!(logger, $fmt $(, $($arg)+)?).ok();
        $crate::print!("\r\n");
    });
}
