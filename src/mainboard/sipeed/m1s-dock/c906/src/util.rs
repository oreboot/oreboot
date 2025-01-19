use core::ptr::{read_volatile, write_volatile};

pub fn write32(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

pub fn read32(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

pub fn clear_bit(reg: usize, bit: u32) {
    unsafe {
        let v = read32(reg);
        write32(reg, v & !(1 << bit));
    }
}

pub fn set_bit(reg: usize, bit: u32) {
    unsafe {
        let v = read32(reg);
        write32(reg, v | (1 << bit));
    }
}

pub fn dump(addr: usize, length: usize) {
    let s = unsafe { core::slice::from_raw_parts(addr as *const u8, length) };
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

pub fn udelay(t: usize) {
    unsafe {
        for _ in 0..t * 480 {
            riscv::asm::nop();
        }
    }
}

pub fn sleep() {
    udelay(1000);
}
