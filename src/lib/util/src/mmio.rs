use core::ptr::{read_volatile, write_volatile};

pub fn read8(address: usize) -> u8 {
    unsafe { read_volatile(address as *mut u8) }
}

pub fn write8(address: usize, value: u8) {
    unsafe { write_volatile(address as *mut u8, value) }
}

pub fn read16(address: usize) -> u16 {
    unsafe { read_volatile(address as *mut u16) }
}

pub fn write16(address: usize, value: u16) {
    unsafe { write_volatile(address as *mut u16, value) }
}

pub fn read32(address: usize) -> u32 {
    unsafe { read_volatile(address as *mut u32) }
}

pub fn write32(address: usize, value: u32) {
    unsafe { write_volatile(address as *mut u32, value) }
}

// NOTE: Some hardware does not like direct u64 MMIO access.
// So read low and high word (little endian) one at a time, then merge them.
pub fn read64le(reg: usize) -> u64 {
    let l = read32(reg) as u64;
    let h = read32(reg + 4) as u64;
    (h << 32) | l
}

// NOTE: Some hardware does not like direct u64 MMIO access.
// So write low and high word separately, little endian.
pub fn write64le(address: usize, value: u64) {
    write32(address, value as u32);
    write32(address + 4, (value >> 32) as u32);
}
