/* SPDX-License-Identifier: GPL-2.0-only */
#![no_std]

#[allow(non_upper_case_globals)]
pub mod units {
    pub const KiB: usize = 1 << 10;
    pub const MiB: usize = 1 << 20;
    pub const GiB: usize = 1 << 30;
}
pub use units::*;

// Baud is baud rate.
// Since it is not exclusive to UARTs we have it here.
pub enum Baud {
    B115200,
}
