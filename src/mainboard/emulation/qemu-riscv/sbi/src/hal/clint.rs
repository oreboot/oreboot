pub mod mtimecmp {
    use crate::hal::{
        pac_encoding::{CLINT_BASE, MTIMECMPL},
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
    use crate::hal::{
        pac_encoding::{CLINT_BASE, MSIP0},
        write_reg,
    };

    pub fn set_ipi(_word: usize) {
        unsafe { write_reg(CLINT_BASE, MSIP0, 1u64) }
    }
    pub fn clear_ipi(_word: usize) {
        unsafe { write_reg(CLINT_BASE, MSIP0, 0) }
    }
}
