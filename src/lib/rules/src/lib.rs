/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

#[cfg(feature = "bootblock")]
pub const ENV_BOOTBLOCK: u8 = 1;
#[cfg(not(feature = "bootblock"))]
pub const ENV_BOOTBLOCK: u8 = 0;

#[cfg(feature = "verstage")]
pub const ENV_SEPARATE_VERSTAGE: u8 = 1;
#[cfg(not(feature = "verstage"))]
pub const ENV_SEPARATE_VERSTAGE: u8 = 0;

#[cfg(feature = "vboot_starts_before_bootblock")]
pub const ENV_INITIAL_STAGE: u8 = ENV_SEPARATE_VERSTAGE;
#[cfg(not(feature = "vboot_starts_before_bootblock"))]
pub const ENV_INITIAL_STAGE: u8 = ENV_BOOTBLOCK;
