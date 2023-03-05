/* SPDX-License-Identifier: GPL-2.0-only */
//! This is a simple logger for oreboot, built on top of Rust `embedded_hal`.
//!
//! We expose the two macros `print!` and `println!` for global use.
//! Implement the embedded_hal non-blocking (nb) serial write trait, initialize
//! your serial, and pass it along via `log::init` to use the macros.
//!
//! If you want to use a mutex in this crate or gain extra debug print helpers,
//! activate the `"mutex"` and/or `"debug"` features, respectively in your
//! board's `Cargo.toml`, e.g.:
//! ```toml
//! log = { path = "../../../../lib/log", features = ["debug"] }
//! ```
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
//! ```rs
//! // MySerial implements embedded_hal_nb::serial::Write
//! fn init_logger(s: MySerial) {
//!     static ONCE: spin::Once<()> = spin::Once::new();
//!
//!     ONCE.call_once(|| unsafe {
//!         static mut SERIAL: Option<MySerial> = None;
//!         SERIAL.replace(s);
//!         log::init(SERIAL.as_mut().unwrap());
//!     });
//! }
//! ```
//!
//! Note: Some platforms do not support atomics in SRAM, e.g., StarFive JH71x0.
//! In this case or to keep things simple, initalize without further fencing:
//! ```rs
//! // MySerial implements embedded_hal_nb::serial::Write
//! fn init_logger(s: MySerial) {
//!     unsafe {
//!         static mut SERIAL: Option<MySerial> = None;
//!         SERIAL.replace(s);
//!         log::init(SERIAL.as_mut().unwrap());
//!     }
//! }
//! ```
//!
//! Either way, invoke your init function from main early on:
//! ```rs
//! fn main() {
//!     /* ... */
//!     let serial = init::MySerial::new(some_peripheral);
//!     init_logger(serial);
//!     println!("oreboot 🦀");
//!     /* ... */
//! }
//! ```
#![no_std]

use core::fmt;
use embedded_hal_nb::serial::{ErrorType, Write};
use nb::block;

pub trait Serial: ErrorType + Write + Send {}

/// Set the globally available logger that enables the macros.
pub fn init(serial: &'static mut SerialLogger) {
    #[cfg(feature = "mutex")]
    LOGGER.lock().replace(serial);
    #[cfg(not(feature = "mutex"))]
    unsafe {
        LOGGER.replace(serial);
    }
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

#[cfg(feature = "mutex")]
static LOGGER: spin::Mutex<Option<&'static mut SerialLogger>> = spin::Mutex::new(None);

#[cfg(not(feature = "mutex"))]
static mut LOGGER: Option<&mut SerialLogger> = None;

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
    #[cfg(feature = "mutex")]
    LOGGER.lock().as_mut().unwrap().write_fmt(args).unwrap();
    #[cfg(not(feature = "mutex"))]
    unsafe {
        if let Some(l) = &mut LOGGER {
            l.write_fmt(args).unwrap();
        }
    }
}

// WHEN THINGS GO AWFULLY AWRY, ENTER HERE - use with `features = ["debug"]`

#[cfg(feature = "debug")]
#[inline(always)]
// shift n by s and convert to what represents its hex digit in ASCII
fn shift_and_hex(n: u32, s: u8) -> u8 {
    // drop to a single nibble (4 bits), i.e., what a hex digit can hold
    let x = (n >> s) as u8 & 0x0f;
    // digits are in the range 0x30..0x39
    // letters start at 0x40, i.e., off by 7 from 0x3a
    if x > 9 {
        x + 0x37
    } else {
        x + 0x30
    }
}

#[cfg(feature = "debug")]
#[inline(always)]
pub fn print_hex(i: u32) {
    unsafe {
        if let Some(l) = &mut LOGGER {
            nb::block!(l.write(b'0')).unwrap();
            nb::block!(l.write(b'x')).unwrap();
            // nibble by nibble... keep it simple
            nb::block!(l.write(shift_and_hex(i, 28))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 24))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 20))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 16))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 12))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 8))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 4))).unwrap();
            nb::block!(l.write(shift_and_hex(i, 0))).unwrap();
            nb::block!(l.write(b'\r')).unwrap();
            nb::block!(l.write(b'\n')).unwrap();
        }
    }
}

#[cfg(feature = "debug")]
#[inline(always)]
pub fn print_mem<T>(s: *const T) {
    let p = s as u32;
    let m = unsafe { core::ptr::read_volatile(p as *mut u32) };
    print_hex(m);
}

#[cfg(feature = "debug")]
#[inline(always)]
pub fn print_ptr<T>(s: *const T) {
    let p = s as u32;
    print_hex(p);
}

#[cfg(feature = "debug")]
#[inline(always)]
pub fn print_strptr(s: &str) {
    print_ptr(s.as_ptr());
}

#[cfg(feature = "debug")]
#[inline(always)]
pub fn print_strmem(s: &str) {
    let p = s.as_ptr();
    let m = unsafe { core::ptr::read_volatile(p as *mut u32) };
    print_hex(m);
}

#[cfg(feature = "debug")]
#[no_mangle]
pub fn print_str(s: &str) {
    unsafe {
        #[cfg(not(feature = "mutex"))]
        if let Some(l) = &mut LOGGER {
            for byte in s.bytes() {
                // Inject a carriage return before a newline
                if byte == b'\n' {
                    nb::block!(l.write(b'\r')).unwrap();
                }
                nb::block!(l.write(byte)).unwrap();
            }
        }
    }
}
