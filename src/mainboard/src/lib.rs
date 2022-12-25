/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

#[cfg(feature = "romsize_kb_256")]
pub const ROM_SIZE: u32 = 0x0004_0000;
#[cfg(feature = "romsize_kb_512")]
pub const ROM_SIZE: u32 = 0x0008_0000;
#[cfg(feature = "romsize_kb_1024")]
pub const ROM_SIZE: u32 = 0x0010_0000;
#[cfg(feature = "romsize_kb_2048")]
pub const ROM_SIZE: u32 = 0x0020_0000;
#[cfg(feature = "romsize_kb_4096")]
pub const ROM_SIZE: u32 = 0x0040_0000;
#[cfg(feature = "romsize_kb_5120")]
pub const ROM_SIZE: u32 = 0x0050_0000;
#[cfg(feature = "romsize_kb_6144")]
pub const ROM_SIZE: u32 = 0x0060_0000;
#[cfg(feature = "romsize_kb_8192")]
pub const ROM_SIZE: u32 = 0x0080_0000;
#[cfg(feature = "romsize_kb_10240")]
pub const ROM_SIZE: u32 = 0x00a0_0000;
#[cfg(feature = "romsize_kb_12288")]
pub const ROM_SIZE: u32 = 0x00c0_0000;
#[cfg(feature = "romsize_kb_16384")]
pub const ROM_SIZE: u32 = 0x0100_0000;
#[cfg(feature = "romsize_kb_32768")]
pub const ROM_SIZE: u32 = 0x0200_0000;
#[cfg(feature = "romsize_kb_65536")]
pub const ROM_SIZE: u32 = 0x0400_0000;
