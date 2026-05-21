use core::ptr::{read_volatile, write_volatile};
use core::slice;

use log::{print, println};

pub fn udelay(time: usize) {
    for _ in 0..time {
        unsafe { core::arch::asm!("nop") }
    }
}

pub fn dump(addr: usize, length: usize) {
    let s = unsafe { slice::from_raw_parts(addr as *const u8, length) };
    for w in s.iter() {
        print!("{:02x}", w);
    }
    println!();
}

pub fn dump_block(base: usize, size: usize, step_size: usize) {
    println!("dump {size} bytes @{base:08x}");
    for b in (base..base + size).step_by(step_size) {
        dump(b, step_size);
    }
}

pub fn read32(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

pub fn write32(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

pub fn read64(reg: usize) -> u64 {
    let l = read32(reg) as u64;
    let h = read32(reg + 4) as u64;
    (h << 32) | l
}

pub fn write64(reg: usize, val: u64) {
    write32(reg, val as u32);
    write32(reg + 4, (val >> 32) as u32);
}
