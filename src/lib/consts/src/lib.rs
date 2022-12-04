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

#[cfg(__SMM__)]
pub const ENV_SMM: bool = true;
#[cfg(not(__SMM__))]
pub const ENV_SMM: bool = false;

#[cfg(__DECOMPRESSOR__)]
pub const ENV_DECOMPRESSOR: bool = true;
#[cfg(not(__DECOMPRESSOR__))]
pub const ENV_DECOMPRESSOR: bool = false;

#[cfg(__BOOTBLOCK__)]
pub const ENV_BOOTBLOCK: bool = true;
#[cfg(not(__BOOTBLOCK__))]
pub const ENV_BOOTBLOCK: bool = false;

#[cfg(__ROMSTAGE__)]
pub const ENV_ROMSTAGE: bool = true;
#[cfg(not(__ROMSTAGE__))]
pub const ENV_ROMSTAGE: bool = false;

#[cfg(__RAMSTAGE__)]
pub const ENV_RAMSTAGE: bool = true;
#[cfg(not(__RAMSTAGE__))]
pub const ENV_RAMSTAGE: bool = false;

#[cfg(__SEPARATE_VERSTAGE__)]
pub const ENV_SEPARATE_VERSTAGE: bool = true;
#[cfg(not(__SEPARATE_VERSTAGE__))]
pub const ENV_SEPARATE_VERSTAGE: bool = false;

#[cfg(__RMODULE__)]
pub const ENV_RMODULE: bool = true;
#[cfg(not(__RMODULE__))]
pub const ENV_RMODULE: bool = false;

#[cfg(__POSTCAR__)]
pub const ENV_POSTCAR: bool = true;
#[cfg(not(__POSTCAR__))]
pub const ENV_POSTCAR: bool = false;

pub const ENV_ROMSTAGE_OR_BEFORE: bool = ENV_DECOMPRESSOR
    || ENV_BOOTBLOCK
    || ENV_ROMSTAGE
    || (ENV_SEPARATE_VERSTAGE && !cfg!(VBOOT_STARTS_IN_ROMSTAGE));
