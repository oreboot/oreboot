/* SPDX-License-Identifier: GPL-2.0-or-later */
#![deny(warnings)]
#![no_std]

pub mod pci_def;
pub mod pci_ids;
pub mod pci_type;

pub const fn bit(x: u64) -> u64 {
    1 << x
}
