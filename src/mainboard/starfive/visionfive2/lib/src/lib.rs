/* SPDX-License-Identifier: GPL-2.0-only */
#![no_std]

use core::ptr::{read_volatile, write_volatile};
use core::slice;
use log::{print, println};

pub fn dump(addr: usize, length: usize) {
    let s = unsafe { slice::from_raw_parts(addr as *const u8, length) };
    println!("dump {length} bytes @{addr:x}");
    for w in s.iter() {
        print!("{:02x}", w);
    }
    println!();
}

pub fn dump_block(base: usize, size: usize, step_size: usize) {
    for b in (base..base + size).step_by(step_size) {
        dump(b, step_size);
    }
}

pub fn write32(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

pub fn read32(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

pub fn clear_bit(reg: usize, bit: u32) {
    let v = read32(reg);
    write32(reg, v & !(1 << bit));
}

pub fn set_bit(reg: usize, bit: u32) {
    let v = read32(reg);
    write32(reg, v | (1 << bit));
}

// see SiFive U74 MC core complex manual, chapter 9.5, table 123 (p 184)
const CLINT_BASE: usize = 0x0200_0000;
const HART0_MSIP: usize = CLINT_BASE + 0x0000;
const HART1_MSIP: usize = CLINT_BASE + 0x0004;
const HART2_MSIP: usize = CLINT_BASE + 0x0008;
const HART3_MSIP: usize = CLINT_BASE + 0x000c;
const HART4_MSIP: usize = CLINT_BASE + 0x0010;
const HART0_MTIMECMP: usize = CLINT_BASE + 0x4000;
const HART1_MTIMECMP: usize = CLINT_BASE + 0x4008;
const HART2_MTIMECMP: usize = CLINT_BASE + 0x4010;
const HART3_MTIMECMP: usize = CLINT_BASE + 0x4018;
const HART4_MTIMECMP: usize = CLINT_BASE + 0x4020;
const CLINT_MTIMER: usize = CLINT_BASE + 0xbff8;

// NOTE: The respective hartid is read out at runtime in most cases.
// To exhaust the match as Rust requires, we need a workaround.
// We could assert!(), but don't want to panic, just be resilient.
// Using an enum instead makes it unnecessarily complicated.
// So we just do nothing if hartid is out of range.

// FIXME: Recheck if we got the lower and higher parts in the right spot...
fn write64(addr: usize, val: u64) {
    write32(addr + 4, (val >> 32) as u32);
    write32(addr, val as u32);
}

pub fn set_mtimecmp(hartid: usize, val: u64) {
    match hartid {
        0 => write64(HART0_MTIMECMP, val),
        1 => write64(HART1_MTIMECMP, val),
        2 => write64(HART2_MTIMECMP, val),
        3 => write64(HART3_MTIMECMP, val),
        4 => write64(HART4_MTIMECMP, val),
        _ => {}
    };
}

pub fn set_ipi(hartid: usize) {
    match hartid {
        0 => write32(HART0_MSIP, 0x1),
        1 => write32(HART1_MSIP, 0x1),
        2 => write32(HART2_MSIP, 0x1),
        3 => write32(HART3_MSIP, 0x1),
        4 => write32(HART4_MSIP, 0x1),
        _ => {}
    };
}

pub fn clear_ipi(hartid: usize) {
    match hartid {
        0 => write32(HART0_MSIP, 0x0),
        1 => write32(HART1_MSIP, 0x0),
        2 => write32(HART2_MSIP, 0x0),
        3 => write32(HART3_MSIP, 0x0),
        4 => write32(HART4_MSIP, 0x0),
        _ => {}
    };
}

pub fn resume_nonboot_harts() {
    // set_ipi(0);
    // clear_ipi(0);
    set_ipi(2);
    clear_ipi(2);
    set_ipi(3);
    clear_ipi(3);
    set_ipi(4);
    clear_ipi(4);
}

pub fn udelay(t: usize) {
    let curr_time = read32(CLINT_MTIMER);
    while read32(CLINT_MTIMER) < (curr_time + 2 * t as u32) {}
}

pub fn get_mtime() -> u64 {
    let l = read32(CLINT_MTIMER) as u64;
    let h = read32(CLINT_MTIMER + 4) as u64;
    (h << 32) | l
}
