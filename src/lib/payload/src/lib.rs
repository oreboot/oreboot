/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

extern crate alloc;

pub mod arch;
pub mod drivers;
pub mod oreboot_tables;
pub mod pci;
pub mod sysinfo;
