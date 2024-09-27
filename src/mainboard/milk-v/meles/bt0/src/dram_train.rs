// board/thead/light-c910/lpddr4/src/ddr_phy_fw/lp4x_3733_phy_train1d2d_dualrank.c

use crate::dram_helpers::{
    ddr_phy_broadcast_en, ddr_phy_reg_rd, ddr_phy_reg_wr,
    dwc_ddrphy1_phyinit_user_custom_g_wait_fw_done, dwc_ddrphy_phyinit_user_custom_g_wait_fw_done,
};
use crate::dram_training_data::{DCCM_ARRAY, DCCM_ARRAY1, ICCM_ARRAY, ICCM_ARRAY1};

// void lp4_phy_train1d2d(enum DDR_TYPE type, int speed, enum DDR_BITWIDTH bits)
pub fn lp4_phy_train1d2d(freq: u16, bits: u8) {
    lp4x_3733_phy_train1d2d();
}

fn lp4x_3733_phy_train1d2d() {
    println!("lp4x_3733_phy_train1d2d...");
    ddr_phy_reg_wr(0x1005f, 0x55f);
    ddr_phy_reg_wr(0x1015f, 0x55f);
    ddr_phy_reg_wr(0x1105f, 0x55f);
    ddr_phy_reg_wr(0x1115f, 0x55f);
    ddr_phy_reg_wr(0x1205f, 0x55f);
    ddr_phy_reg_wr(0x1215f, 0x55f);
    ddr_phy_reg_wr(0x1305f, 0x55f);
    ddr_phy_reg_wr(0x1315f, 0x55f);
    ddr_phy_reg_wr(0x55, 0x19f);
    ddr_phy_reg_wr(0x1055, 0x19f);
    ddr_phy_reg_wr(0x2055, 0x19f);
    ddr_phy_reg_wr(0x3055, 0x19f);
    ddr_phy_reg_wr(0x4055, 0x19f);
    ddr_phy_reg_wr(0x5055, 0x19f);
    ddr_phy_reg_wr(0x200c5, 0x19);
    ddr_phy_reg_wr(0x2002e, 0x5);
    ddr_phy_reg_wr(0x90204, 0x0);
    ddr_phy_reg_wr(0x20024, 0x1e3);
    ddr_phy_reg_wr(0x2003a, 0x2);
    ddr_phy_reg_wr(0x2007d, 0x212);
    ddr_phy_reg_wr(0x2007c, 0x61);
    ddr_phy_reg_wr(0x20056, 0x3);
    ddr_phy_reg_wr(0x1004d, 0x600);
    ddr_phy_reg_wr(0x1014d, 0x600);
    ddr_phy_reg_wr(0x1104d, 0x600);
    ddr_phy_reg_wr(0x1114d, 0x600);
    ddr_phy_reg_wr(0x1204d, 0x600);
    ddr_phy_reg_wr(0x1214d, 0x600);
    ddr_phy_reg_wr(0x1304d, 0x600);
    ddr_phy_reg_wr(0x1314d, 0x600);
    ddr_phy_reg_wr(0x10049, 0xe00);
    ddr_phy_reg_wr(0x10149, 0xe00);
    ddr_phy_reg_wr(0x11049, 0xe00);
    ddr_phy_reg_wr(0x11149, 0xe00);
    ddr_phy_reg_wr(0x12049, 0xe00);
    ddr_phy_reg_wr(0x12149, 0xe00);
    ddr_phy_reg_wr(0x13049, 0xe00);
    ddr_phy_reg_wr(0x13149, 0xe00);
    ddr_phy_reg_wr(0x43, 0x60);
    ddr_phy_reg_wr(0x1043, 0x60);
    ddr_phy_reg_wr(0x2043, 0x60);
    ddr_phy_reg_wr(0x3043, 0x60);
    ddr_phy_reg_wr(0x4043, 0x60);
    ddr_phy_reg_wr(0x5043, 0x60);
    ddr_phy_reg_wr(0x20018, 0x3);
    ddr_phy_reg_wr(0x20075, 0x4);
    ddr_phy_reg_wr(0x20050, 0x0);
    ddr_phy_reg_wr(0x2009b, 0x2);
    ddr_phy_reg_wr(0x20008, 0x3a5);
    ddr_phy_reg_wr(0x20088, 0x9);
    ddr_phy_reg_wr(0x200b2, 0x104);
    ddr_phy_reg_wr(0x10043, 0x5a1);
    ddr_phy_reg_wr(0x10143, 0x5a1);
    ddr_phy_reg_wr(0x11043, 0x5a1);
    ddr_phy_reg_wr(0x11143, 0x5a1);
    ddr_phy_reg_wr(0x12043, 0x5a1);
    ddr_phy_reg_wr(0x12143, 0x5a1);
    ddr_phy_reg_wr(0x13043, 0x5a1);
    ddr_phy_reg_wr(0x13143, 0x5a1);
    ddr_phy_reg_wr(0x200fa, 0x1);
    ddr_phy_reg_wr(0x20019, 0x1);
    ddr_phy_reg_wr(0x200f0, 0x0);
    ddr_phy_reg_wr(0x200f1, 0x0);
    ddr_phy_reg_wr(0x200f2, 0x4444);
    ddr_phy_reg_wr(0x200f3, 0x8888);
    ddr_phy_reg_wr(0x200f4, 0x5555);
    ddr_phy_reg_wr(0x200f5, 0x0);
    ddr_phy_reg_wr(0x200f6, 0x0);
    ddr_phy_reg_wr(0x200f7, 0xf000);
    ddr_phy_reg_wr(0x20025, 0x0);
    ddr_phy_reg_wr(0x2002d, 0x1);
    ddr_phy_reg_wr(0x20021, 0x0);
    ddr_phy_reg_wr(0x2002c, 0x0);
    ddr_phy_reg_wr(0x20060, 0x2);
    ddr_phy_reg_wr(0xd0000, 0x0);
    println!("lp4x_3733_phy_train1d2d: copy over ICCM blob");
    for i in 0..16384 {
        ddr_phy_reg_wr(0x50000 + i, ICCM_ARRAY[i]);
    }
    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x0);
    println!("lp4x_3733_phy_train1d2d: copy over DCCM blob");
    for i in 0..830 {
        ddr_phy_reg_wr(0x54000 + i, DCCM_ARRAY[i]);
    }
    // NOTE: With this, we get data from the PHY mailbox; see dram_helpers
    // functions get_phyX_mails.
    ddr_phy_reg_wr(0x54009, 0x4);
    // ddr_phy_reg_wr(0x54007, 0x18);

    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0099, 0x9);
    ddr_phy_reg_wr(0xd0099, 0x1);
    ddr_phy_reg_wr(0xd0099, 0x0);
    println!("lp4x_3733_phy_train1d2d: ddr_phy_broadcast_en 0");
    ddr_phy_broadcast_en(0);
    println!("dwc_ddrphy_phyinit_user_custom_g_wait_fw_done");
    dwc_ddrphy_phyinit_user_custom_g_wait_fw_done(0);
    // FIXME: this gets stuck in the first message ready poll loop
    // println!("dwc_ddrphy1_phyinit_user_custom_g_wait_fw_done");
    // dwc_ddrphy1_phyinit_user_custom_g_wait_fw_done(0);
    println!("lp4x_3733_phy_train1d2d: ddr_phy_broadcast_en 1");
    ddr_phy_broadcast_en(1);
    ddr_phy_reg_wr(0xd0099, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x0);
    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x0);

    println!(
        "CHA CDD RR01 {}, RR10 {}",
        0xff & ddr_phy_reg_rd(0x54013),
        0xff & (ddr_phy_reg_rd(0x54013) >> 8)
    );
    println!(
        "CHA CDD RW11 {}, RW10 {}",
        0xff & ddr_phy_reg_rd(0x54014),
        0xff & (ddr_phy_reg_rd(0x54014) >> 8)
    );
    println!(
        "CHA CDD RW01 {}, RW00 {}",
        0xff & ddr_phy_reg_rd(0x54015),
        0xff & (ddr_phy_reg_rd(0x54015) >> 8)
    );
    println!(
        "CHA CDD WR11 {}, WR10 {}",
        0xff & ddr_phy_reg_rd(0x54016),
        0xff & (ddr_phy_reg_rd(0x54016) >> 8)
    );
    println!(
        "CHA CDD WR01 {}, WR00 {}",
        0xff & ddr_phy_reg_rd(0x54017),
        0xff & (ddr_phy_reg_rd(0x54017) >> 8)
    );
    println!(
        "CHA CDD WW01 {}, WW10 {}",
        0xff & ddr_phy_reg_rd(0x54018),
        0xff & (ddr_phy_reg_rd(0x54018) >> 8)
    );
    println!(
        "CHB CDD RR01 {}, RR10 {}",
        0xff & ddr_phy_reg_rd(0x5402d),
        0xff & (ddr_phy_reg_rd(0x5402c) >> 8)
    );
    println!(
        "CHB CDD RW11 {}, RW10 {}",
        0xff & ddr_phy_reg_rd(0x5402e),
        0xff & (ddr_phy_reg_rd(0x5402d) >> 8)
    );
    println!(
        "CHB CDD RW01 {}, RW00 {}",
        0xff & ddr_phy_reg_rd(0x5402f),
        0xff & (ddr_phy_reg_rd(0x5402e) >> 8)
    );
    println!(
        "CHB CDD WR11 {}, WR10 {}",
        0xff & ddr_phy_reg_rd(0x54030),
        0xff & (ddr_phy_reg_rd(0x5402f) >> 8)
    );
    println!(
        "CHB CDD WR01 {}, WR00 {}",
        0xff & ddr_phy_reg_rd(0x54031),
        0xff & (ddr_phy_reg_rd(0x54030) >> 8)
    );
    println!(
        "CHB CDD WW01 {}, WW10 {}",
        0xff & ddr_phy_reg_rd(0x54032),
        0xff & (ddr_phy_reg_rd(0x54031) >> 8)
    );

    println!("lp4x_3733_phy_train1d2d: copy over ICCM1 blob");
    for i in 0..16384 {
        ddr_phy_reg_wr(0x50000 + i, ICCM_ARRAY1[i]);
    }
    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x0);
    println!("lp4x_3733_phy_train1d2d: copy over DCCM1 blob");
    for i in 0..702 {
        ddr_phy_reg_wr(0x54000 + i, DCCM_ARRAY1[i]);
    }

    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0099, 0x9);
    ddr_phy_reg_wr(0xd0099, 0x1);
    ddr_phy_reg_wr(0xd0099, 0x0);
    ddr_phy_broadcast_en(0);
    dwc_ddrphy_phyinit_user_custom_g_wait_fw_done(1);
    // FIXME: this gets stuck in the first message ready poll loop
    // dwc_ddrphy1_phyinit_user_custom_g_wait_fw_done(1);
    ddr_phy_broadcast_en(1);
    ddr_phy_reg_wr(0xd0099, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x0);
    ddr_phy_reg_wr(0xd0000, 0x1);
    ddr_phy_reg_wr(0xd0000, 0x0);

    let v = ddr_phy_reg_rd(0x54026);
    println!("TrainedVREFDQ_RANK0  : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x54027);
    println!("TrainedVREFDQ_RANK1  : {:02x}", v as u8);
    println!("RxClkDly_Margin_A0   : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x54028);
    println!("VrefDac_Margin_A0    : {:02x}", v as u8);
    println!("TxDqDly_Margin_A0    : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x54029);
    println!("DeviceVref_Margin_A0 : {:02x}", v as u8);
    println!("RxClkDly_Margin_A1   : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x5402a);
    println!("VrefDac_Margin_A1    : {:02x}", v as u8);
    println!("TxDqDly_Margin_A1    : {:02x}", 0xff & (v >> 8));
    let v = ddr_phy_reg_rd(0x5402b);
    println!("DeviceVref_Margin_A1 : {:02x}", v as u8);
    let v = ddr_phy_reg_rd(0x54040);
    println!("TrainedVREFDQ_RANK0  : {:02x}", v as u8);
    println!("TrainedVREFDQ_RANK1  : {:02x}", 0xff & (v >> 8));
    let v = ddr_phy_reg_rd(0x54041);
    println!("RxClkDly_Margin_A0   : {:02x}", v as u8);
    println!("VrefDac_Margin_A0    : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x54042);
    println!("TxDqDly_Margin_A0    : {:02x}", v as u8);
    println!("DeviceVref_Margin_A0 : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x54043);
    println!("RxClkDly_Margin_A1   : {:02x}", v as u8);
    println!("VrefDac_Margin_A1    : {:02x}", (v >> 8) as u8);
    let v = ddr_phy_reg_rd(0x54044);
    println!("TxDqDly_Margin_A1    : {:02x}", v as u8);
    println!("DeviceVref_Margin_A1 : {:02x}", (v >> 8) as u8);
}
