pub mod clint;
pub mod pac_encoding;
pub mod serial;
pub use clint::msip;
use core::ptr::{read_volatile, write_volatile};
pub use serial::Serial;
#[inline]
pub unsafe fn write_reg<T>(addr: usize, offset: usize, val: T) {
    write_volatile((addr + offset) as *mut T, val);
}

#[inline]
pub unsafe fn read_reg<T>(addr: usize, offset: usize) -> T {
    read_volatile((addr + offset) as *const T)
}
