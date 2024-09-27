// this file contains all the required helper functions required by DRAM

use crate::dram::{DDR_CFG0, DDR_SYSREG_BADDR, _DDR_PHY1_BADDR, _DDR_PHY_BADDR};
use core::ptr::{read_volatile, write_volatile};

fn write16(reg: usize, val: u16) {
    unsafe { write_volatile(reg as *mut u16, val) }
}

pub fn read16(reg: usize) -> u16 {
    unsafe { read_volatile(reg as *mut u16) }
}

pub fn ddr_phy0_reg_wr(reg: usize, val: u16) {
    write16(_DDR_PHY_BADDR + (reg << 1), val);
}

pub fn ddr_phy1_reg_wr(reg: usize, val: u16) {
    write16(_DDR_PHY1_BADDR + (reg << 1), val);
}

pub fn ddr_phy_reg_wr(reg: usize, val: u16) {
    write16(_DDR_PHY_BADDR + (reg << 1), val);
    write16(_DDR_PHY1_BADDR + (reg << 1), val);
}

pub fn ddr_phy_reg_rd(reg: usize) -> u16 {
    read16(_DDR_PHY_BADDR + (reg << 1))
}

pub fn ddr_phy1_reg_rd(reg: usize) -> u16 {
    read16(_DDR_PHY_BADDR + (reg << 1))
}

pub fn ddr_phy_broadcast_en(_: u32) {
    crate::util::read32(DDR_SYSREG_BADDR + DDR_CFG0);
    crate::util::read32(DDR_SYSREG_BADDR + DDR_CFG0);
}

// board/thead/light-c910/lpddr4/src/waitfwdone.c
pub fn dwc_ddrphy_phyinit_user_custom_g_wait_fw_done(train2d: u8) {
    loop {
        let train_result = get_phy0_mails() as u8;
        match train_result {
            0x7 => {
                println!("PHY0 DDR_INIT_OK");
                return;
            }
            0x8 => {
                let mut stream_msg = [0; 32];
                stream_msg[0] = get_phy0_mails(); //msg first byte
                for i in 1..=(stream_msg[0] & 0xffff) {
                    stream_msg[i as usize] = get_phy0_mails();
                }
            }
            0xff => {
                let step = if train2d != 0 { "train2d" } else { "" };
                panic!("[+] PHY0 {step} DDR_INIT_ERR");
            }
            _ => {
                println!("PHY0 DDR_INIT_STAGE is {train_result:x}");
            }
        }
    }
}

fn get_phy0_mails() -> u32 {
    while ddr_phy_reg_rd(0xd0004) & 0x1 == 1 {}
    //read msg
    let msg0 = ddr_phy_reg_rd(0xd0032);
    let msg1 = ddr_phy_reg_rd(0xd0034);
    //write-back
    ddr_phy0_reg_wr(0xd0031, 0);
    //wait ack end
    while ddr_phy_reg_rd(0xd0004) & 0x1 == 0 {}
    //re-enable
    ddr_phy0_reg_wr(0xd0031, 1);
    msg0 as u32 + ((msg1 as u32) << 16)
}

// board/thead/light-c910/lpddr4/src/waitfwdone.c
pub fn dwc_ddrphy1_phyinit_user_custom_g_wait_fw_done(train2d: u8) {
    loop {
        let train_result = get_phy1_mails() as u8;
        match train_result {
            0x7 => {
                println!("PHY0 DDR_INIT_OK");
                return;
            }
            0x8 => {
                let mut stream_msg = [0; 32];
                stream_msg[0] = get_phy1_mails(); //msg first byte
                for i in 1..=(stream_msg[0] & 0xffff) {
                    stream_msg[i as usize] = get_phy1_mails();
                }
            }
            0xff => {
                let step = if train2d != 0 { "train2d" } else { "" };
                panic!("[+] PHY0 {step} DDR_INIT_ERR");
            }
            _ => {
                println!("PHY0 DDR_INIT_STAGE is {train_result:x}");
            }
        }
    }
}

fn get_phy1_mails() -> u32 {
    while ddr_phy1_reg_rd(0xd0004) & 0x1 == 1 {}
    // read msg
    let msg0 = ddr_phy1_reg_rd(0xd0032);
    let msg1 = ddr_phy1_reg_rd(0xd0034);
    // write-back
    ddr_phy1_reg_wr(0xd0031, 0);
    // wait ack end
    while ddr_phy1_reg_rd(0xd0004) & 0x1 == 0 {}
    // re-enable
    ddr_phy1_reg_wr(0xd0031, 1);
    msg0 as u32 + ((msg1 as u32) << 16)
}
