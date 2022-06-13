pub const UART0_BASE: usize = 0x0250_0000;
pub const UART_THR: usize = 0;
pub const UART_RBR: usize = 0;
pub const UART_LSR: usize = 0x14;
pub const UART_USR: usize = 0x7c;

pub const CLINT_BASE: usize = 0x0400_0000;
pub const MSIP0: usize = 0;
pub const MTIMECMPL: usize = 0x4000;

pub mod mtimecmp {
    use super::{
        CLINT_BASE, MTIMECMPL,
        write_reg,
    };
    pub fn write(word: u64) {
        unsafe {
            let mask = u64::MAX;
            write_reg(CLINT_BASE, MTIMECMPL, (word & mask) as u32);
            write_reg(CLINT_BASE, MTIMECMPL + 4, (word >> 32) as u32);
        }
    }
}
pub mod msip {
    use super::{
        CLINT_BASE, MSIP0,
        write_reg,
    };

    pub fn set_ipi(_word: usize) {
        unsafe { write_reg(CLINT_BASE, MSIP0, 1u64) }
    }
    pub fn clear_ipi(_word: usize) {
        unsafe { write_reg(CLINT_BASE, MSIP0, 0) }
    }
}

use core::ptr::write_volatile;

#[inline]
unsafe fn write_reg<T>(addr: usize, offset: usize, val: T) {
    write_volatile((addr + offset) as *mut T, val);
}
