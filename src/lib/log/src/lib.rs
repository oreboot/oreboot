/* SPDX-License-Identifier: GPL-2.0-only */
//! This is a simple logger for oreboot, built on top of Rust `embedded_hal`.
//!
//! We expose the two macros `print!` and `println!` as well as the direct
//! `print` and `debug` methods. The former is not inteded for direct use, while
//! `debug` serves as a fallback in case platform bringup gets really hard.
//!
//! Implement the embedded_hal non-blocking (nb) serial write trait, initialize
//! your serial, and pass it along via `log::init` to use the macros.
//!
//! You will need to create a `&'static mut`, which you can achieve  by wrapping
//! your serial in a `static mut` holding an `Option` for your serial's type and
//! extracting it again in `.as_mut().unwrap()`. Consider putting that into a
//! function and adding either a `spin::Once` or some other gating mechanism to
//! avoid dupliate initialization and mutable aliasing.
//! It may simply `panic!()` if you choose so.
//!
//! Here is an example for a `spin::Once` that results in a no-op for further
//! calls to `init_logger()`, being tolerant at runtime:
//!
//! ```rs
//! // MySerial implements embedded_hal::serial::nb::Write
//! fn init_logger(s: MySerial) {
//!     static ONCE: spin::Once<()> = spin::Once::new();
//!
//!     ONCE.call_once(|| unsafe {
//!         static mut SERIAL: Option<MySerial> = None;
//!         SERIAL.replace(s);
//!         log::init(SERIAL.as_mut().unwrap());
//!     });
//! }
//!
//! fn main() {
//!     /* ... */
//!     let serial = init::MySerial::new(some_peripheral);
//!     init_logger(serial);
//!     log::debug(42);
//!     println!("oreboot 🦀");
//!     /* ... */
//! }
//! ```
#![no_std]

use core::fmt;
use embedded_hal::serial::{nb::Write, ErrorType};
use nb::block;
use spin::Mutex;

pub trait Serial: ErrorType + Write + Send {
    /// This is meant to be the simplest fallback for debugging:
    /// A "sign of life", possibly an LED flashing (GPIO high/low...),
    /// an unforgiving write to a UART without polling for status or anything,
    /// just to be sure that _something_ works. Can be a no-op, up to you.
    fn debug(&self, num: u8);
}

/// Set the globally available logger that enables the macros.
pub fn init(serial: &'static mut SerialLogger) {
    LOGGER.lock().replace(serial);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print(core::format_args!($($arg)*));
        $crate::print!("\n");
    }
}

/// Error types that may happen with serial transfer, usually just `WouldBlock`
#[derive(Debug)]
pub struct Error {
    pub kind: embedded_hal::serial::ErrorKind,
}

impl embedded_hal::serial::Error for Error {
    fn kind(&self) -> embedded_hal::serial::ErrorKind {
        self.kind
    }
}

type SerialLogger = dyn Serial<Error = Error>;

static LOGGER: Mutex<Option<&'static mut SerialLogger>> = Mutex::new(None);

impl fmt::Write for SerialLogger {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for &byte in s.as_bytes() {
            // Inject a carriage return before a newline
            if byte == b'\n' {
                block!(self.write(b'\r')).unwrap();
            }
            block!(self.write(byte)).unwrap();
        }
        block!(self.flush()).unwrap();
        Ok(())
    }
}

#[doc(hidden)]
pub fn print(args: fmt::Arguments) {
    use fmt::Write;
    LOGGER.lock().as_mut().unwrap().write_fmt(args).unwrap();
}

pub fn debug(num: u8) {
    LOGGER.lock().as_mut().unwrap().debug(num);
}
