/* SPDX-License-Identifier: GPL-2.0-only */
pub mod msr;
pub mod x86;

#[allow(non_upper_case_globals)]
mod units {
    pub const KiB: usize = 1 << 10;
    pub const MiB: usize = 1 << 20;
    pub const GiB: usize = 1 << 30;
}
pub use units::*;
