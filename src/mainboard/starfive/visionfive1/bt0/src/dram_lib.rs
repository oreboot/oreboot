use core::ptr::{read_volatile, write_volatile};

/// ## OMC = Orbit Memory Controller
/// https://www.openedges.com/memorycontroller
pub const OMC_BASE_ADDR: usize = 0x1100_0000;
/// The configuration has two base addresses
pub const CFG0_BASE_ADDR: usize = OMC_BASE_ADDR + 0x0000_0000;
pub const CFG1_BASE_ADDR: usize = OMC_BASE_ADDR + 0x0001_0000;
/// The respective "security" parts (TODO: what is that?) are `0x1000` off.
pub const SEC0_BASE_ADDR: usize = CFG0_BASE_ADDR + 0x0000_1000;
pub const SEC1_BASE_ADDR: usize = CFG1_BASE_ADDR + 0x0000_1000;

/// The PHY has two base addresses
pub const PHY0_BASE_ADDR: usize = OMC_BASE_ADDR + 0x0082_0000;
pub const PHY1_BASE_ADDR: usize = OMC_BASE_ADDR + 0x0083_0000;

/// From those, the actual PI and PHY parts are `0x2000` and `0x4000` off,
/// respectively:
/// | offset | length  | name |
/// | ------ | ------- | ---- |
/// | 0x2000 | 0x2000  | PI   |
/// | 0x4000 | 0x2000  | PHY  |

pub fn read(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

pub fn write(dest: usize, val: u32) {
    unsafe {
        write_volatile(dest as *mut u32, val);
    }
}

/**
 * The PHY registers are off base by 4096 << 2, i.e., 16384, or 0x4000.
 * They are 32 bits (4 bytes) long, so shift by 2.
 */
pub fn read_phy(base: usize, reg: usize) -> u32 {
    read(base + 0x4000 + (reg << 2))
}

/**
 * The PHY registers are off base by 4096 << 2, i.e., 16384, or 0x4000.
 * They are 32 bits (4 bytes) long, so shift by 2.
 */
pub fn write_phy(base: usize, reg: usize, val: u32) {
    write(base + 0x4000 + (reg << 2), val);
}

/**
 * The PI registers are off base by 2048 << 2, i.e., 8192, or 0x2000.
 * They are 32 bits (4 bytes) long, so shift by 2.
 */
pub fn read_pi(base: usize, reg: usize) -> u32 {
    read(base + 0x2000 + (reg << 2))
}

/**
 * The PI registers are off base by 2048 << 2, i.e., 8192, or 0x2000.
 * They are 32 bits (4 bytes) long, so shift by 2.
 */
pub fn write_pi(base: usize, reg: usize, val: u32) {
    write(base + 0x2000 + (reg << 2), val);
}
