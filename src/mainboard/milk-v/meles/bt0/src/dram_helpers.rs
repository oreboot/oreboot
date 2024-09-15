// this file contains all the required helper functions required by DRAM

use core::ptr::write_volatile;

const _DDR_PHY_BADDR: usize = 0xfffd000000;
const _DDR_PHY1_BADDR: usize =   _DDR_PHY_BADDR + 0x1000000;


fn write16(reg: usize, val: u16) {
    unsafe {
        write_volatile(reg as *mut u16, val);
    }
}

// void ddr_phy0_reg_wr(unsigned long int addr,unsigned int wr_data) {
//     addr<<=1;
//     wr16(_DDR_PHY_BADDR+addr, wr_data);
// }
pub fn ddr_phy0_reg_wr(mut reg: usize, val: u16) {
    reg <<= 1;
    write16( _DDR_PHY_BADDR + reg, val);
}

// void ddr_phy1_reg_wr(unsigned long int addr,unsigned int wr_data) {
//     addr<<=1;
//     wr16(_DDR_PHY1_BADDR+addr, wr_data);
// }

pub fn ddr_phy1_reg_wr(mut reg: usize, val: u16) {
    reg <<= 1;
    write16(_DDR_PHY1_BADDR + reg, val);
}