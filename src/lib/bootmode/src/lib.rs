/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

use spin::rwlock::RwLock;

pub static GFX_INIT_DONE: RwLock<i32> = RwLock::new(-1);

pub fn gfx_get_init_done() -> i32 {
    if *GFX_INIT_DONE.read() < 0 {
        0
    } else {
        *GFX_INIT_DONE.read()
    }
}
