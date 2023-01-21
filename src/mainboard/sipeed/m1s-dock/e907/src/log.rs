//! Log system for BT0
// essentially copied from sunxi/nezha

use core::fmt;
use embedded_hal::serial::{ErrorType,nb::Write};
use nb::block;

pub trait Serial: ErrorType + Write {
    fn debug(&self, num: u8);
}

struct Wrap<T>(T);

/// Error types that may happen when serial transfer
#[derive(Debug)]
pub struct Error {
    pub kind: embedded_hal::serial::ErrorKind,
}

impl embedded_hal::serial::Error for Error {
    #[inline]
    fn kind(&self) -> embedded_hal::serial::ErrorKind {
        self.kind
    }
}

pub type SerialLogger = dyn Serial<Error=Error>;

extern crate alloc;
use alloc::boxed::Box;
type Logger = Wrap<Option<Box<SerialLogger>>>;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
unsafe impl GlobalAlloc for Logger {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[doc(hidden)]
#[global_allocator]
static mut LOGGER: Logger = Wrap(None);

impl fmt::Write for SerialLogger {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.as_bytes() {
            block!(self.write(*byte)).unwrap();
        }
        block!(self.flush()).unwrap();
        Ok(())
    }
}

#[inline]
pub fn set_logger<S: Serial<Error=Error> + 'static>(serial: S) {
    unsafe {
        LOGGER = Wrap(Some(Box::new(serial)));
    }
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    unsafe {
        if let Some(l) = &mut LOGGER.0 {
             l.write_fmt(args).unwrap();
        }
    }
}

#[inline]
#[doc(hidden)]
pub fn _debug(num: u8) {
    unsafe {
        if let Some(l) = &mut LOGGER.0 {
            l.debug(num);
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
