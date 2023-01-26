/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

pub const CBMEM_ID_ROMSTAGE_INFO: u32 = 0x47545352;

// FIXME: unimplemented, requires full cbmem implementation
pub fn cbmem_find<T>(_id: u32) -> Option<T> {
    None
}
