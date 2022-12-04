/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ptr::{read_volatile, write_volatile};

pub unsafe fn read8(addr: usize) -> u8 {
    read_volatile(addr as *const u8)
}

pub unsafe fn write8(addr: usize, value: u8) {
    write_volatile(addr as *mut u8, value);
}

pub unsafe fn read16(addr: usize) -> u16 {
    read_volatile(addr as *const u16)
}

pub unsafe fn write16(addr: usize, value: u16) {
    write_volatile(addr as *mut u16, value);
}

pub unsafe fn read32(addr: usize) -> u32 {
    read_volatile(addr as *const u32)
}

pub unsafe fn write32(addr: usize, value: u32) {
    write_volatile(addr as *mut u32, value);
}
