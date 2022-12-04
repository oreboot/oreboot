// From cpuio crate by Eric Kidd <git@randomhacks.net>
// homepage: https://github.com/emk/toyos-rs/tree/master/crates/cpuio
// License: Apache-2.0/MIT
//! Rust wrappers around the x86-family I/O instructions.

use core::arch::asm;

/// Read a `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    asm!("inb dx", out("al") result, in("dx") port, options(nomem, nostack));
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outb(value: u8, mut _port: u16) -> u8 {
    asm!("outb dx", out("dx") _port, in("al") value, options(nomem, nostack));
    _port as u8
}

/// Read a `u16`-sized value from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    let mut result: u16;
    asm!("inw dx", out("ax") result, in("dx") port, options(nomem, nostack));
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outw(value: u16, mut _port: u16) -> u16 {
    asm!("outw dx", out("dx") _port, in("ax") value, options(nomem, nostack));
    _port
}

/// Read a `u32`-sized value from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!("inl dx", out("eax") result, in("dx") port, options(nomem, nostack));
    result
}

/// Write a `u32`-sized `value` to `port`.
pub unsafe fn outl(value: u32, mut _port: u16) -> u32 {
    asm!("outl dx", out("dx") _port, in("eax") value, options(nomem, nostack));
    _port as u32
}
