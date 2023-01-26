/* SPDX-License-Identifier: GPL-2.0-or-later */

pub const fn align_up(x: u64, a: u64) -> u64 {
    align(x, a)
}

pub const fn align(x: u64, a: u64) -> u64 {
    __align_mask(x, a)
}

pub const fn __align_mask(x: u64, mask: u64) -> u64 {
    (x + mask) & !mask
}
