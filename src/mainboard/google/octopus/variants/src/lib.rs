/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

pub mod baseboard;
#[cfg(feature = "bobba")]
pub mod bobba;

/// From https://github.com/coreboot/coreboot/master/blob/src/mainboard/google/octopus/Kconfig#L181
///
/// Add entries for other variants as needed
#[cfg(feature = "bobba")]
pub const DRAM_PART_IN_CBI_BOARD_ID_MIN: i32 = 3;
#[cfg(not(feature = "bobba"))]
pub const DRAM_PART_IN_CBI_BOARD_ID_MIN: i32 = 255;

