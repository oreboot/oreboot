/* SPDX-License-Identifier: GPL-2.0-or-later */

pub mod pci_def;

pub const fn bit(x: u64) -> u64 {
    1 << x
}
