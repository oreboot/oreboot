/* SPDX-License-Identifier: GPL-2.0-only */
//! This is a simple logger for oreboot, built on top of Rust embedded_hal.
//! We expose the two macros `print!` and `println!` as well as the direct
//! `_print` and `_debug` methods. The former is not inteded for use, while
//! `_debug` serves as a fallback in case platform bringup gets really hard.
//! Implement the embedded_hal non-blocking (nb) serial write trait, initialize
//! your serial, and pass it along via `set_logger` to use the macros.
//!
//! ```rs
//!     // MySerial implements embedded_hal::serial::nb::Write
//!     let serial = init::MySerial::new(some_peripheral);
//!     log::set_logger(serial);
//!     log::_debug(42);
//!     println!("oreboot 🦀");
//! ```
#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use core::{
    alloc::{GlobalAlloc, Layout},
    fmt,
    ptr::null_mut,
};
use embedded_hal::serial::{nb::Write, ErrorType};
use nb::block;
use spin::{Mutex, Once};

pub trait Serial: ErrorType + Write {
    /// This is meant to be the simplest fallback for debugging:
    /// A "sign of life", possibly an LED flashing (GPIO high/low...),
    /// an unforgiving write to a UART without polling for status or anything,
    /// just to be sure that _something_ works. Can be a no-op, up to you.
    fn debug(&self, num: u8);
}

/// Set the globally available logger that enables the macros.
pub fn set_logger<S: Serial<Error = Error> + 'static>(serial: S) {
    unsafe {
        LOGGER.0.call_once(|| LockedLogger {
            l: Mutex::new(Box::new(serial)),
        });
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*));
        $crate::print!("\n");
    }
}

/// Error types that may happen with serial transfer, usually just `WouldBlock`
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

type SerialLogger = dyn Serial<Error = Error>;

#[doc(hidden)]
struct LockedLogger {
    l: Mutex<Box<SerialLogger>>,
}

// We wrap this to fulfill the orphan rule. See the Rust book for more details:
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
struct Wrap<T>(T);
type Logger = Wrap<Once<LockedLogger>>;

unsafe impl GlobalAlloc for Logger {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[doc(hidden)]
#[global_allocator]
static mut LOGGER: Logger = Wrap(Once::new());

impl fmt::Write for SerialLogger {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.as_bytes() {
            if byte == b'\n' {
                block!(self.write(b'\r')).unwrap();
            }
            block!(self.write(*byte)).unwrap();
        }
        block!(self.flush()).unwrap();
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    unsafe {
        LOGGER.0.wait().l.lock().write_fmt(args).unwrap();
    }
}

pub fn _debug(num: u8) {
    unsafe {
        LOGGER.0.wait().l.lock().debug(num);
    }
}
