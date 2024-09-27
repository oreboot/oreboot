// this file contains all the required helper functions required by DRAM

use core::ptr::{read_volatile, write_volatile};

const _DDR_PHY_BADDR: usize = 0xfffd000000;
const _DDR_PHY1_BADDR: usize = _DDR_PHY_BADDR + 0x1000000;

fn write16(reg: usize, val: u16) {
    unsafe {
        write_volatile(reg as *mut u16, val);
    }
}

pub fn read16(reg: usize) -> u16 {
    unsafe { read_volatile(reg as *mut u16) }
}

// void ddr_phy0_reg_wr(unsigned long int addr,unsigned int wr_data) {
//     addr<<=1;
//     wr16(_DDR_PHY_BADDR+addr, wr_data);
// }
pub fn ddr_phy0_reg_wr(mut reg: usize, val: u16) {
    reg <<= 1;
    write16(_DDR_PHY_BADDR + reg, val);
}

// void ddr_phy1_reg_wr(unsigned long int addr,unsigned int wr_data) {
//     addr<<=1;
//     wr16(_DDR_PHY1_BADDR+addr, wr_data);
// }

pub fn ddr_phy1_reg_wr(mut reg: usize, val: u16) {
    reg <<= 1;
    write16(_DDR_PHY1_BADDR + reg, val);
}

// void ddr_phy_reg_wr(unsigned long int addr,unsigned int wr_data)
pub fn ddr_phy_reg_wr(mut reg: usize, val: u16) {
    reg <<= 1;
    write16(_DDR_PHY_BADDR + reg, val);
    write16(_DDR_PHY1_BADDR + reg, val);
}

// unsigned int ddr_phy_reg_rd(unsigned long int addr) {
//     //unsigned long int ddr_phy_sel,addr_low,rd_data;
//     unsigned int rd_data;
//     addr<<=1;
//     rd_data=rd16(_DDR_PHY_BADDR+addr);
//     return rd_data;
// }

pub fn ddr_phy_reg_rd(mut reg: usize) -> u32 {
    reg <<= 1;
    let rd_data: u32 = read16(_DDR_PHY_BADDR + reg) as u32;
    rd_data
}
pub fn ddr_phy1_reg_rd(mut reg: usize) -> u16 {
    reg <<= 1;
    let rd_data = read16(_DDR_PHY_BADDR + reg);
    rd_data
}

// board/thead/light-c910/lpddr4/src/waitfwdone.c
// void dwc_ddrphy_phyinit_userCustom_G_waitFwDone(unsigned char train2d)
pub fn dwc_ddrphy_phyinit_user_custom_g_wait_fw_done(train2d: u8) {
    let mut train_result: u32 = 0x1;
    let i: u32;
    let mut stream_msg = [0; 32];

    while (((train_result & 0xffff) != 0x7) & ((train_result & 0xffff) != 0xff)) {
        train_result = get_mails();

        if ((train_result & 0xff) == 0xff) {
            println!(
                "[+] PHY0 {} DDR_INIT_ERR",
                if train2d != 0 { "train2d" } else { "" }
            );
        }

        //Steam MSG
        if ((train_result & 0xffff) == 0x8) {
            stream_msg[0] = get_mails(); //msg first byte

            for i in 1..=(stream_msg[0] & 0xffff) {
                stream_msg[i as usize] = get_mails();
            }
        }
    }
}

fn get_mails() -> u32 {
    let mut read;
    let mut msg0;
    let mut msg1;

    read = 0x1;
    loop {
        //read = (unsigned int)(*(volatile unsigned short*)(0xfe7a0008));
        read = ddr_phy_reg_rd(0xd0004);
        if ((read & 0x1) == 1) {
            break;
        }
    }
    //read msg
    //msg0 = (unsigned int)(*(volatile unsigned short*)(0xfe7a0064));
    msg0 = ddr_phy_reg_rd(0xd0032);
    //msg1 = (unsigned int)(*(volatile unsigned short*)(0xfe7a0068));
    msg1 = ddr_phy_reg_rd(0xd0034);

    //write-back
    //*(volatile unsigned short*)(0xfe7a0062) = 0;
    ddr_phy0_reg_wr(0xd0031, 0);

    //wait ack end
    read = 0x0;
    loop {
        //read = (unsigned int)(*(volatile unsigned short*)(0xfe7a0008));
        read = ddr_phy_reg_rd(0xd0004);
        if ((read & 0x1) == 1) {
            break;
        }
    }

    //re-enable
    //*(volatile unsigned short*)(0xfe7a0062) = 1;
    ddr_phy0_reg_wr(0xd0031, 1);
    (msg0 as u32 + ((msg1 as u32) << 16))
}

// board/thead/light-c910/lpddr4/src/waitfwdone.c
// void dwc_ddrphy1_phyinit_userCustom_G_waitFwDone(unsigned char train2d)
pub fn dwc_ddrphy1_phyinit_user_custom_g_wait_fw_done(train2d: u8) {
    let mut train_result: u32 = 0x1;
    let i: u32;
    let mut stream_msg = [0; 32];

    while (((train_result & 0xffff) != 0x7) & ((train_result & 0xffff) != 0xff)) {
        train_result = get_phy1_mails();

        if ((train_result & 0xff) == 0xff) {
            println!(
                "[+] PHY0 {} DDR_INIT_ERR",
                if train2d != 0 { "train2d" } else { "" }
            );
        }

        //Steam MSG
        if ((train_result & 0xffff) == 0x8) {
            stream_msg[0] = get_phy1_mails(); //msg first byte

            for i in 1..=(stream_msg[0] & 0xffff) {
                stream_msg[i as usize] = get_phy1_mails();
            }
        }
    }
}

fn get_phy1_mails() -> u32 {
    let mut read;
    let mut msg0;
    let mut msg1;

    read = 0x1;
    loop {
        //read = (unsigned int)(*(volatile unsigned short*)(0xfe7a0008));
        read = ddr_phy1_reg_rd(0xd0004);
        if ((read & 0x1) == 1) {
            break;
        }
    }
    //read msg
    //msg0 = (unsigned int)(*(volatile unsigned short*)(0xfe7a0064));
    msg0 = ddr_phy1_reg_rd(0xd0032);
    //msg1 = (unsigned int)(*(volatile unsigned short*)(0xfe7a0068));
    msg1 = ddr_phy1_reg_rd(0xd0034);

    //write-back
    //*(volatile unsigned short*)(0xfe7a0062) = 0;
    ddr_phy1_reg_wr(0xd0031, 0);

    //wait ack end
    read = 0x0;
    loop {
        //read = (unsigned int)(*(volatile unsigned short*)(0xfe7a0008));
        read = ddr_phy1_reg_rd(0xd0004);
        if ((read & 0x1) == 1) {
            break;
        }
    }

    //re-enable
    //*(volatile unsigned short*)(0xfe7a0062) = 1;
    ddr_phy1_reg_wr(0xd0031, 1);
    (msg0 as u32 + ((msg1 as u32) << 16))
}
