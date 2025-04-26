// board/thead/light-c910/lpddr4/src/ddr_phy_fw/lp4x_3733_phy_train1d2d_dualrank.c

use crate::dram::Bits;
use crate::dram_helpers::{
    ddr_phy0_reg_wr, ddr_phy1_reg_rd, ddr_phy1_reg_wr, ddr_phy_broadcast_en, ddr_phy_reg_rd,
    ddr_phy_reg_wr, dwc_ddrphy_phyinit_user_custom_g_wait_fw_done,
};
use crate::dram_training_data::{DCCM_ARRAY, DCCM_ARRAY1, ICCM_ARRAY, ICCM_ARRAY1};

pub fn lp4_phy_train1d2d(freq: u16, bits: Bits) {
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
    ddr_phy_reg_wr(0x2002e, 0x2);
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
    ddr_phy_reg_wr(0x1004a, 0x500);
    ddr_phy_reg_wr(0x1104a, 0x500);
    ddr_phy_reg_wr(0x1204a, 0x500);
    ddr_phy_reg_wr(0x1304a, 0x500);
    ddr_phy_reg_wr(0x20025, 0x0);
    ddr_phy_reg_wr(0x2002d, 0x0);
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

    ddr_phy_reg_wr(0x54009, 0x4);
    // ddr_phy_reg_wr(0x54007,0x18);

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

    ddr_phy_reg_wr(0x90000, 0x10);
    ddr_phy_reg_wr(0x90001, 0x400);
    ddr_phy_reg_wr(0x90002, 0x10e);
    ddr_phy_reg_wr(0x90003, 0x0);
    ddr_phy_reg_wr(0x90004, 0x0);
    ddr_phy_reg_wr(0x90005, 0x8);
    ddr_phy_reg_wr(0x90029, 0xb);
    ddr_phy_reg_wr(0x9002a, 0x480);
    ddr_phy_reg_wr(0x9002b, 0x109);
    ddr_phy_reg_wr(0x9002c, 0x8);
    ddr_phy_reg_wr(0x9002d, 0x448);
    ddr_phy_reg_wr(0x9002e, 0x139);
    ddr_phy_reg_wr(0x9002f, 0x8);
    ddr_phy_reg_wr(0x90030, 0x478);
    ddr_phy_reg_wr(0x90031, 0x109);
    ddr_phy_reg_wr(0x90032, 0x0);
    ddr_phy_reg_wr(0x90033, 0xe8);
    ddr_phy_reg_wr(0x90034, 0x109);
    ddr_phy_reg_wr(0x90035, 0x2);
    ddr_phy_reg_wr(0x90036, 0x10);
    ddr_phy_reg_wr(0x90037, 0x139);
    ddr_phy_reg_wr(0x90038, 0xb);
    ddr_phy_reg_wr(0x90039, 0x7c0);
    ddr_phy_reg_wr(0x9003a, 0x139);
    ddr_phy_reg_wr(0x9003b, 0x44);
    ddr_phy_reg_wr(0x9003c, 0x633);
    ddr_phy_reg_wr(0x9003d, 0x159);
    ddr_phy_reg_wr(0x9003e, 0x14f);
    ddr_phy_reg_wr(0x9003f, 0x630);
    ddr_phy_reg_wr(0x90040, 0x159);
    ddr_phy_reg_wr(0x90041, 0x47);
    ddr_phy_reg_wr(0x90042, 0x633);
    ddr_phy_reg_wr(0x90043, 0x149);
    ddr_phy_reg_wr(0x90044, 0x4f);
    ddr_phy_reg_wr(0x90045, 0x633);
    ddr_phy_reg_wr(0x90046, 0x179);
    ddr_phy_reg_wr(0x90047, 0x8);
    ddr_phy_reg_wr(0x90048, 0xe0);
    ddr_phy_reg_wr(0x90049, 0x109);
    ddr_phy_reg_wr(0x9004a, 0x0);
    ddr_phy_reg_wr(0x9004b, 0x7c8);
    ddr_phy_reg_wr(0x9004c, 0x109);
    ddr_phy_reg_wr(0x9004d, 0x0);
    ddr_phy_reg_wr(0x9004e, 0x1);
    ddr_phy_reg_wr(0x9004f, 0x8);
    ddr_phy_reg_wr(0x90050, 0x0);
    ddr_phy_reg_wr(0x90051, 0x45a);
    ddr_phy_reg_wr(0x90052, 0x9);
    ddr_phy_reg_wr(0x90053, 0x0);
    ddr_phy_reg_wr(0x90054, 0x448);
    ddr_phy_reg_wr(0x90055, 0x109);
    ddr_phy_reg_wr(0x90056, 0x40);
    ddr_phy_reg_wr(0x90057, 0x633);
    ddr_phy_reg_wr(0x90058, 0x179);
    ddr_phy_reg_wr(0x90059, 0x1);
    ddr_phy_reg_wr(0x9005a, 0x618);
    ddr_phy_reg_wr(0x9005b, 0x109);
    ddr_phy_reg_wr(0x9005c, 0x40c0);
    ddr_phy_reg_wr(0x9005d, 0x633);
    ddr_phy_reg_wr(0x9005e, 0x149);
    ddr_phy_reg_wr(0x9005f, 0x8);
    ddr_phy_reg_wr(0x90060, 0x4);
    ddr_phy_reg_wr(0x90061, 0x48);
    ddr_phy_reg_wr(0x90062, 0x4040);
    ddr_phy_reg_wr(0x90063, 0x633);
    ddr_phy_reg_wr(0x90064, 0x149);
    ddr_phy_reg_wr(0x90065, 0x0);
    ddr_phy_reg_wr(0x90066, 0x4);
    ddr_phy_reg_wr(0x90067, 0x48);
    ddr_phy_reg_wr(0x90068, 0x40);
    ddr_phy_reg_wr(0x90069, 0x633);
    ddr_phy_reg_wr(0x9006a, 0x149);
    ddr_phy_reg_wr(0x9006b, 0x10);
    ddr_phy_reg_wr(0x9006c, 0x4);
    ddr_phy_reg_wr(0x9006d, 0x18);
    ddr_phy_reg_wr(0x9006e, 0x0);
    ddr_phy_reg_wr(0x9006f, 0x4);
    ddr_phy_reg_wr(0x90070, 0x78);
    ddr_phy_reg_wr(0x90071, 0x549);
    ddr_phy_reg_wr(0x90072, 0x633);
    ddr_phy_reg_wr(0x90073, 0x159);
    ddr_phy_reg_wr(0x90074, 0xd49);
    ddr_phy_reg_wr(0x90075, 0x633);
    ddr_phy_reg_wr(0x90076, 0x159);
    ddr_phy_reg_wr(0x90077, 0x94a);
    ddr_phy_reg_wr(0x90078, 0x633);
    ddr_phy_reg_wr(0x90079, 0x159);
    ddr_phy_reg_wr(0x9007a, 0x441);
    ddr_phy_reg_wr(0x9007b, 0x633);
    ddr_phy_reg_wr(0x9007c, 0x149);
    ddr_phy_reg_wr(0x9007d, 0x42);
    ddr_phy_reg_wr(0x9007e, 0x633);
    ddr_phy_reg_wr(0x9007f, 0x149);
    ddr_phy_reg_wr(0x90080, 0x1);
    ddr_phy_reg_wr(0x90081, 0x633);
    ddr_phy_reg_wr(0x90082, 0x149);
    ddr_phy_reg_wr(0x90083, 0x0);
    ddr_phy_reg_wr(0x90084, 0xe0);
    ddr_phy_reg_wr(0x90085, 0x109);
    ddr_phy_reg_wr(0x90086, 0xa);
    ddr_phy_reg_wr(0x90087, 0x10);
    ddr_phy_reg_wr(0x90088, 0x109);
    ddr_phy_reg_wr(0x90089, 0x9);
    ddr_phy_reg_wr(0x9008a, 0x3c0);
    ddr_phy_reg_wr(0x9008b, 0x149);
    ddr_phy_reg_wr(0x9008c, 0x9);
    ddr_phy_reg_wr(0x9008d, 0x3c0);
    ddr_phy_reg_wr(0x9008e, 0x159);
    ddr_phy_reg_wr(0x9008f, 0x18);
    ddr_phy_reg_wr(0x90090, 0x10);
    ddr_phy_reg_wr(0x90091, 0x109);
    ddr_phy_reg_wr(0x90092, 0x0);
    ddr_phy_reg_wr(0x90093, 0x3c0);
    ddr_phy_reg_wr(0x90094, 0x109);
    ddr_phy_reg_wr(0x90095, 0x18);
    ddr_phy_reg_wr(0x90096, 0x4);
    ddr_phy_reg_wr(0x90097, 0x48);
    ddr_phy_reg_wr(0x90098, 0x18);
    ddr_phy_reg_wr(0x90099, 0x4);
    ddr_phy_reg_wr(0x9009a, 0x58);
    ddr_phy_reg_wr(0x9009b, 0xb);
    ddr_phy_reg_wr(0x9009c, 0x10);
    ddr_phy_reg_wr(0x9009d, 0x109);
    ddr_phy_reg_wr(0x9009e, 0x1);
    ddr_phy_reg_wr(0x9009f, 0x10);
    ddr_phy_reg_wr(0x900a0, 0x109);
    ddr_phy_reg_wr(0x900a1, 0x5);
    ddr_phy_reg_wr(0x900a2, 0x7c0);
    ddr_phy_reg_wr(0x900a3, 0x109);
    ddr_phy_reg_wr(0x40000, 0x811);
    ddr_phy_reg_wr(0x40020, 0x880);
    ddr_phy_reg_wr(0x40040, 0x0);
    ddr_phy_reg_wr(0x40060, 0x0);
    ddr_phy_reg_wr(0x40001, 0x4008);
    ddr_phy_reg_wr(0x40021, 0x83);
    ddr_phy_reg_wr(0x40041, 0x4f);
    ddr_phy_reg_wr(0x40061, 0x0);
    ddr_phy_reg_wr(0x40002, 0x4040);
    ddr_phy_reg_wr(0x40022, 0x83);
    ddr_phy_reg_wr(0x40042, 0x51);
    ddr_phy_reg_wr(0x40062, 0x0);
    ddr_phy_reg_wr(0x40003, 0x811);
    ddr_phy_reg_wr(0x40023, 0x880);
    ddr_phy_reg_wr(0x40043, 0x0);
    ddr_phy_reg_wr(0x40063, 0x0);
    ddr_phy_reg_wr(0x40004, 0x720);
    ddr_phy_reg_wr(0x40024, 0xf);
    ddr_phy_reg_wr(0x40044, 0x1740);
    ddr_phy_reg_wr(0x40064, 0x0);
    ddr_phy_reg_wr(0x40005, 0x16);
    ddr_phy_reg_wr(0x40025, 0x83);
    ddr_phy_reg_wr(0x40045, 0x4b);
    ddr_phy_reg_wr(0x40065, 0x0);
    ddr_phy_reg_wr(0x40006, 0x716);
    ddr_phy_reg_wr(0x40026, 0xf);
    ddr_phy_reg_wr(0x40046, 0x2001);
    ddr_phy_reg_wr(0x40066, 0x0);
    ddr_phy_reg_wr(0x40007, 0x716);
    ddr_phy_reg_wr(0x40027, 0xf);
    ddr_phy_reg_wr(0x40047, 0x2800);
    ddr_phy_reg_wr(0x40067, 0x0);
    ddr_phy_reg_wr(0x40008, 0x716);
    ddr_phy_reg_wr(0x40028, 0xf);
    ddr_phy_reg_wr(0x40048, 0xf00);
    ddr_phy_reg_wr(0x40068, 0x0);
    ddr_phy_reg_wr(0x40009, 0x720);
    ddr_phy_reg_wr(0x40029, 0xf);
    ddr_phy_reg_wr(0x40049, 0x1400);
    ddr_phy_reg_wr(0x40069, 0x0);
    ddr_phy_reg_wr(0x4000a, 0xe08);
    ddr_phy_reg_wr(0x4002a, 0xc15);
    ddr_phy_reg_wr(0x4004a, 0x0);
    ddr_phy_reg_wr(0x4006a, 0x0);
    ddr_phy_reg_wr(0x4000b, 0x625);
    ddr_phy_reg_wr(0x4002b, 0x15);
    ddr_phy_reg_wr(0x4004b, 0x0);
    ddr_phy_reg_wr(0x4006b, 0x0);
    ddr_phy_reg_wr(0x4000c, 0x4028);
    ddr_phy_reg_wr(0x4002c, 0x80);
    ddr_phy_reg_wr(0x4004c, 0x0);
    ddr_phy_reg_wr(0x4006c, 0x0);
    ddr_phy_reg_wr(0x4000d, 0xe08);
    ddr_phy_reg_wr(0x4002d, 0xc1a);
    ddr_phy_reg_wr(0x4004d, 0x0);
    ddr_phy_reg_wr(0x4006d, 0x0);
    ddr_phy_reg_wr(0x4000e, 0x625);
    ddr_phy_reg_wr(0x4002e, 0x1a);
    ddr_phy_reg_wr(0x4004e, 0x0);
    ddr_phy_reg_wr(0x4006e, 0x0);
    ddr_phy_reg_wr(0x4000f, 0x4040);
    ddr_phy_reg_wr(0x4002f, 0x80);
    ddr_phy_reg_wr(0x4004f, 0x0);
    ddr_phy_reg_wr(0x4006f, 0x0);
    ddr_phy_reg_wr(0x40010, 0x2604);
    ddr_phy_reg_wr(0x40030, 0x15);
    ddr_phy_reg_wr(0x40050, 0x0);
    ddr_phy_reg_wr(0x40070, 0x0);
    ddr_phy_reg_wr(0x40011, 0x708);
    ddr_phy_reg_wr(0x40031, 0x5);
    ddr_phy_reg_wr(0x40051, 0x0);
    ddr_phy_reg_wr(0x40071, 0x2002);
    ddr_phy_reg_wr(0x40012, 0x8);
    ddr_phy_reg_wr(0x40032, 0x80);
    ddr_phy_reg_wr(0x40052, 0x0);
    ddr_phy_reg_wr(0x40072, 0x0);
    ddr_phy_reg_wr(0x40013, 0x2604);
    ddr_phy_reg_wr(0x40033, 0x1a);
    ddr_phy_reg_wr(0x40053, 0x0);
    ddr_phy_reg_wr(0x40073, 0x0);
    ddr_phy_reg_wr(0x40014, 0x708);
    ddr_phy_reg_wr(0x40034, 0xa);
    ddr_phy_reg_wr(0x40054, 0x0);
    ddr_phy_reg_wr(0x40074, 0x2002);
    ddr_phy_reg_wr(0x40015, 0x4040);
    ddr_phy_reg_wr(0x40035, 0x80);
    ddr_phy_reg_wr(0x40055, 0x0);
    ddr_phy_reg_wr(0x40075, 0x0);
    ddr_phy_reg_wr(0x40016, 0x60a);
    ddr_phy_reg_wr(0x40036, 0x15);
    ddr_phy_reg_wr(0x40056, 0x1200);
    ddr_phy_reg_wr(0x40076, 0x0);
    ddr_phy_reg_wr(0x40017, 0x61a);
    ddr_phy_reg_wr(0x40037, 0x15);
    ddr_phy_reg_wr(0x40057, 0x1300);
    ddr_phy_reg_wr(0x40077, 0x0);
    ddr_phy_reg_wr(0x40018, 0x60a);
    ddr_phy_reg_wr(0x40038, 0x1a);
    ddr_phy_reg_wr(0x40058, 0x1200);
    ddr_phy_reg_wr(0x40078, 0x0);
    ddr_phy_reg_wr(0x40019, 0x642);
    ddr_phy_reg_wr(0x40039, 0x1a);
    ddr_phy_reg_wr(0x40059, 0x1300);
    ddr_phy_reg_wr(0x40079, 0x0);
    ddr_phy_reg_wr(0x4001a, 0x4808);
    ddr_phy_reg_wr(0x4003a, 0x880);
    ddr_phy_reg_wr(0x4005a, 0x0);
    ddr_phy_reg_wr(0x4007a, 0x0);
    ddr_phy_reg_wr(0x900a4, 0x0);
    ddr_phy_reg_wr(0x900a5, 0x790);
    ddr_phy_reg_wr(0x900a6, 0x11a);
    ddr_phy_reg_wr(0x900a7, 0x8);
    ddr_phy_reg_wr(0x900a8, 0x7aa);
    ddr_phy_reg_wr(0x900a9, 0x2a);
    ddr_phy_reg_wr(0x900aa, 0x10);
    ddr_phy_reg_wr(0x900ab, 0x7b2);
    ddr_phy_reg_wr(0x900ac, 0x2a);
    ddr_phy_reg_wr(0x900ad, 0x0);
    ddr_phy_reg_wr(0x900ae, 0x7c8);
    ddr_phy_reg_wr(0x900af, 0x109);
    ddr_phy_reg_wr(0x900b0, 0x10);
    ddr_phy_reg_wr(0x900b1, 0x10);
    ddr_phy_reg_wr(0x900b2, 0x109);
    ddr_phy_reg_wr(0x900b3, 0x10);
    ddr_phy_reg_wr(0x900b4, 0x2a8);
    ddr_phy_reg_wr(0x900b5, 0x129);
    ddr_phy_reg_wr(0x900b6, 0x8);
    ddr_phy_reg_wr(0x900b7, 0x370);
    ddr_phy_reg_wr(0x900b8, 0x129);
    ddr_phy_reg_wr(0x900b9, 0xa);
    ddr_phy_reg_wr(0x900ba, 0x3c8);
    ddr_phy_reg_wr(0x900bb, 0x1a9);
    ddr_phy_reg_wr(0x900bc, 0xc);
    ddr_phy_reg_wr(0x900bd, 0x408);
    ddr_phy_reg_wr(0x900be, 0x199);
    ddr_phy_reg_wr(0x900bf, 0x14);
    ddr_phy_reg_wr(0x900c0, 0x790);
    ddr_phy_reg_wr(0x900c1, 0x11a);
    ddr_phy_reg_wr(0x900c2, 0x8);
    ddr_phy_reg_wr(0x900c3, 0x4);
    ddr_phy_reg_wr(0x900c4, 0x18);
    ddr_phy_reg_wr(0x900c5, 0xe);
    ddr_phy_reg_wr(0x900c6, 0x408);
    ddr_phy_reg_wr(0x900c7, 0x199);
    ddr_phy_reg_wr(0x900c8, 0x8);
    ddr_phy_reg_wr(0x900c9, 0x8568);
    ddr_phy_reg_wr(0x900ca, 0x108);
    ddr_phy_reg_wr(0x900cb, 0x18);
    ddr_phy_reg_wr(0x900cc, 0x790);
    ddr_phy_reg_wr(0x900cd, 0x16a);
    ddr_phy_reg_wr(0x900ce, 0x8);
    ddr_phy_reg_wr(0x900cf, 0x1d8);
    ddr_phy_reg_wr(0x900d0, 0x169);
    ddr_phy_reg_wr(0x900d1, 0x10);
    ddr_phy_reg_wr(0x900d2, 0x8558);
    ddr_phy_reg_wr(0x900d3, 0x168);
    ddr_phy_reg_wr(0x900d4, 0x70);
    ddr_phy_reg_wr(0x900d5, 0x788);
    ddr_phy_reg_wr(0x900d6, 0x16a);
    ddr_phy_reg_wr(0x900d7, 0x1ff8);
    ddr_phy_reg_wr(0x900d8, 0x85a8);
    ddr_phy_reg_wr(0x900d9, 0x1e8);
    ddr_phy_reg_wr(0x900da, 0x50);
    ddr_phy_reg_wr(0x900db, 0x798);
    ddr_phy_reg_wr(0x900dc, 0x16a);
    ddr_phy_reg_wr(0x900dd, 0x60);
    ddr_phy_reg_wr(0x900de, 0x7a0);
    ddr_phy_reg_wr(0x900df, 0x16a);
    ddr_phy_reg_wr(0x900e0, 0x8);
    ddr_phy_reg_wr(0x900e1, 0x8310);
    ddr_phy_reg_wr(0x900e2, 0x168);
    ddr_phy_reg_wr(0x900e3, 0x8);
    ddr_phy_reg_wr(0x900e4, 0xa310);
    ddr_phy_reg_wr(0x900e5, 0x168);
    ddr_phy_reg_wr(0x900e6, 0xa);
    ddr_phy_reg_wr(0x900e7, 0x408);
    ddr_phy_reg_wr(0x900e8, 0x169);
    ddr_phy_reg_wr(0x900e9, 0x6e);
    ddr_phy_reg_wr(0x900ea, 0x0);
    ddr_phy_reg_wr(0x900eb, 0x68);
    ddr_phy_reg_wr(0x900ec, 0x0);
    ddr_phy_reg_wr(0x900ed, 0x408);
    ddr_phy_reg_wr(0x900ee, 0x169);
    ddr_phy_reg_wr(0x900ef, 0x0);
    ddr_phy_reg_wr(0x900f0, 0x8310);
    ddr_phy_reg_wr(0x900f1, 0x168);
    ddr_phy_reg_wr(0x900f2, 0x0);
    ddr_phy_reg_wr(0x900f3, 0xa310);
    ddr_phy_reg_wr(0x900f4, 0x168);
    ddr_phy_reg_wr(0x900f5, 0x1ff8);
    ddr_phy_reg_wr(0x900f6, 0x85a8);
    ddr_phy_reg_wr(0x900f7, 0x1e8);
    ddr_phy_reg_wr(0x900f8, 0x68);
    ddr_phy_reg_wr(0x900f9, 0x798);
    ddr_phy_reg_wr(0x900fa, 0x16a);
    ddr_phy_reg_wr(0x900fb, 0x78);
    ddr_phy_reg_wr(0x900fc, 0x7a0);
    ddr_phy_reg_wr(0x900fd, 0x16a);
    ddr_phy_reg_wr(0x900fe, 0x68);
    ddr_phy_reg_wr(0x900ff, 0x790);
    ddr_phy_reg_wr(0x90100, 0x16a);
    ddr_phy_reg_wr(0x90101, 0x8);
    ddr_phy_reg_wr(0x90102, 0x8b10);
    ddr_phy_reg_wr(0x90103, 0x168);
    ddr_phy_reg_wr(0x90104, 0x8);
    ddr_phy_reg_wr(0x90105, 0xab10);
    ddr_phy_reg_wr(0x90106, 0x168);
    ddr_phy_reg_wr(0x90107, 0xa);
    ddr_phy_reg_wr(0x90108, 0x408);
    ddr_phy_reg_wr(0x90109, 0x169);
    ddr_phy_reg_wr(0x9010a, 0x58);
    ddr_phy_reg_wr(0x9010b, 0x0);
    ddr_phy_reg_wr(0x9010c, 0x68);
    ddr_phy_reg_wr(0x9010d, 0x0);
    ddr_phy_reg_wr(0x9010e, 0x408);
    ddr_phy_reg_wr(0x9010f, 0x169);
    ddr_phy_reg_wr(0x90110, 0x0);
    ddr_phy_reg_wr(0x90111, 0x8b10);
    ddr_phy_reg_wr(0x90112, 0x168);
    ddr_phy_reg_wr(0x90113, 0x1);
    ddr_phy_reg_wr(0x90114, 0xab10);
    ddr_phy_reg_wr(0x90115, 0x168);
    ddr_phy_reg_wr(0x90116, 0x0);
    ddr_phy_reg_wr(0x90117, 0x1d8);
    ddr_phy_reg_wr(0x90118, 0x169);
    ddr_phy_reg_wr(0x90119, 0x80);
    ddr_phy_reg_wr(0x9011a, 0x790);
    ddr_phy_reg_wr(0x9011b, 0x16a);
    ddr_phy_reg_wr(0x9011c, 0x18);
    ddr_phy_reg_wr(0x9011d, 0x7aa);
    ddr_phy_reg_wr(0x9011e, 0x6a);
    ddr_phy_reg_wr(0x9011f, 0xa);
    ddr_phy_reg_wr(0x90120, 0x0);
    ddr_phy_reg_wr(0x90121, 0x1e9);
    ddr_phy_reg_wr(0x90122, 0x8);
    ddr_phy_reg_wr(0x90123, 0x8080);
    ddr_phy_reg_wr(0x90124, 0x108);
    ddr_phy_reg_wr(0x90125, 0xf);
    ddr_phy_reg_wr(0x90126, 0x408);
    ddr_phy_reg_wr(0x90127, 0x169);
    ddr_phy_reg_wr(0x90128, 0xc);
    ddr_phy_reg_wr(0x90129, 0x0);
    ddr_phy_reg_wr(0x9012a, 0x68);
    ddr_phy_reg_wr(0x9012b, 0x9);
    ddr_phy_reg_wr(0x9012c, 0x0);
    ddr_phy_reg_wr(0x9012d, 0x1a9);
    ddr_phy_reg_wr(0x9012e, 0x0);
    ddr_phy_reg_wr(0x9012f, 0x408);
    ddr_phy_reg_wr(0x90130, 0x169);
    ddr_phy_reg_wr(0x90131, 0x0);
    ddr_phy_reg_wr(0x90132, 0x8080);
    ddr_phy_reg_wr(0x90133, 0x108);
    ddr_phy_reg_wr(0x90134, 0x8);
    ddr_phy_reg_wr(0x90135, 0x7aa);
    ddr_phy_reg_wr(0x90136, 0x6a);
    ddr_phy_reg_wr(0x90137, 0x0);
    ddr_phy_reg_wr(0x90138, 0x8568);
    ddr_phy_reg_wr(0x90139, 0x108);
    ddr_phy_reg_wr(0x9013a, 0xb7);
    ddr_phy_reg_wr(0x9013b, 0x790);
    ddr_phy_reg_wr(0x9013c, 0x16a);
    ddr_phy_reg_wr(0x9013d, 0x1f);
    ddr_phy_reg_wr(0x9013e, 0x0);
    ddr_phy_reg_wr(0x9013f, 0x68);
    ddr_phy_reg_wr(0x90140, 0x8);
    ddr_phy_reg_wr(0x90141, 0x8558);
    ddr_phy_reg_wr(0x90142, 0x168);
    ddr_phy_reg_wr(0x90143, 0xf);
    ddr_phy_reg_wr(0x90144, 0x408);
    ddr_phy_reg_wr(0x90145, 0x169);
    ddr_phy_reg_wr(0x90146, 0xd);
    ddr_phy_reg_wr(0x90147, 0x0);
    ddr_phy_reg_wr(0x90148, 0x68);
    ddr_phy_reg_wr(0x90149, 0x0);
    ddr_phy_reg_wr(0x9014a, 0x408);
    ddr_phy_reg_wr(0x9014b, 0x169);
    ddr_phy_reg_wr(0x9014c, 0x0);
    ddr_phy_reg_wr(0x9014d, 0x8558);
    ddr_phy_reg_wr(0x9014e, 0x168);
    ddr_phy_reg_wr(0x9014f, 0x8);
    ddr_phy_reg_wr(0x90150, 0x3c8);
    ddr_phy_reg_wr(0x90151, 0x1a9);
    ddr_phy_reg_wr(0x90152, 0x3);
    ddr_phy_reg_wr(0x90153, 0x370);
    ddr_phy_reg_wr(0x90154, 0x129);
    ddr_phy_reg_wr(0x90155, 0x20);
    ddr_phy_reg_wr(0x90156, 0x2aa);
    ddr_phy_reg_wr(0x90157, 0x9);
    ddr_phy_reg_wr(0x90158, 0x8);
    ddr_phy_reg_wr(0x90159, 0xe8);
    ddr_phy_reg_wr(0x9015a, 0x109);
    ddr_phy_reg_wr(0x9015b, 0x0);
    ddr_phy_reg_wr(0x9015c, 0x8140);
    ddr_phy_reg_wr(0x9015d, 0x10c);
    ddr_phy_reg_wr(0x9015e, 0x10);
    ddr_phy_reg_wr(0x9015f, 0x8138);
    ddr_phy_reg_wr(0x90160, 0x104);
    ddr_phy_reg_wr(0x90161, 0x8);
    ddr_phy_reg_wr(0x90162, 0x448);
    ddr_phy_reg_wr(0x90163, 0x109);
    ddr_phy_reg_wr(0x90164, 0xf);
    ddr_phy_reg_wr(0x90165, 0x7c0);
    ddr_phy_reg_wr(0x90166, 0x109);
    ddr_phy_reg_wr(0x90167, 0x0);
    ddr_phy_reg_wr(0x90168, 0xe8);
    ddr_phy_reg_wr(0x90169, 0x109);
    ddr_phy_reg_wr(0x9016a, 0x47);
    ddr_phy_reg_wr(0x9016b, 0x630);
    ddr_phy_reg_wr(0x9016c, 0x109);
    ddr_phy_reg_wr(0x9016d, 0x8);
    ddr_phy_reg_wr(0x9016e, 0x618);
    ddr_phy_reg_wr(0x9016f, 0x109);
    ddr_phy_reg_wr(0x90170, 0x8);
    ddr_phy_reg_wr(0x90171, 0xe0);
    ddr_phy_reg_wr(0x90172, 0x109);
    ddr_phy_reg_wr(0x90173, 0x0);
    ddr_phy_reg_wr(0x90174, 0x7c8);
    ddr_phy_reg_wr(0x90175, 0x109);
    ddr_phy_reg_wr(0x90176, 0x8);
    ddr_phy_reg_wr(0x90177, 0x8140);
    ddr_phy_reg_wr(0x90178, 0x10c);
    ddr_phy_reg_wr(0x90179, 0x0);
    ddr_phy_reg_wr(0x9017a, 0x478);
    ddr_phy_reg_wr(0x9017b, 0x109);
    ddr_phy_reg_wr(0x9017c, 0x0);
    ddr_phy_reg_wr(0x9017d, 0x1);
    ddr_phy_reg_wr(0x9017e, 0x8);
    ddr_phy_reg_wr(0x9017f, 0x8);
    ddr_phy_reg_wr(0x90180, 0x4);
    ddr_phy_reg_wr(0x90181, 0x0);
    ddr_phy_reg_wr(0x90006, 0x8);
    ddr_phy_reg_wr(0x90007, 0x7c8);
    ddr_phy_reg_wr(0x90008, 0x109);
    ddr_phy_reg_wr(0x90009, 0x0);
    ddr_phy_reg_wr(0x9000a, 0x400);
    ddr_phy_reg_wr(0x9000b, 0x106);
    ddr_phy_reg_wr(0xd00e7, 0x400);
    ddr_phy_reg_wr(0x90017, 0x0);
    ddr_phy_reg_wr(0x9001f, 0x29);
    ddr_phy_reg_wr(0x90026, 0x68);
    ddr_phy_reg_wr(0x400d0, 0x0);
    ddr_phy_reg_wr(0x400d1, 0x101);
    ddr_phy_reg_wr(0x400d2, 0x105);
    ddr_phy_reg_wr(0x400d3, 0x107);
    ddr_phy_reg_wr(0x400d4, 0x10f);
    ddr_phy_reg_wr(0x400d5, 0x202);
    ddr_phy_reg_wr(0x400d6, 0x20a);
    ddr_phy_reg_wr(0x400d7, 0x20b);
    ddr_phy_reg_wr(0x2003a, 0x2);
    ddr_phy_reg_wr(0x200be, 0x3);
    ddr_phy_reg_wr(0x2000b, 0x74);
    ddr_phy_reg_wr(0x2000c, 0xe9);
    ddr_phy_reg_wr(0x2000d, 0x91b);
    ddr_phy_reg_wr(0x2000e, 0x2c);
    ddr_phy_reg_wr(0x9000c, 0x0);
    ddr_phy_reg_wr(0x9000d, 0x173);
    ddr_phy_reg_wr(0x9000e, 0x60);
    ddr_phy_reg_wr(0x9000f, 0x6110);
    ddr_phy_reg_wr(0x90010, 0x2152);
    ddr_phy_reg_wr(0x90011, 0xdfbd);
    ddr_phy_reg_wr(0x90012, 0x2060);
    ddr_phy_reg_wr(0x90013, 0x6152);
    ddr_phy_reg_wr(0x20010, 0x5a);
    ddr_phy_reg_wr(0x20011, 0x3);
    ddr_phy_reg_wr(0x40080, 0xe0);
    ddr_phy_reg_wr(0x40081, 0x12);
    ddr_phy_reg_wr(0x40082, 0xe0);
    ddr_phy_reg_wr(0x40083, 0x12);
    ddr_phy_reg_wr(0x40084, 0xe0);
    ddr_phy_reg_wr(0x40085, 0x12);
    ddr_phy_reg_wr(0x400fd, 0xf);
    ddr_phy_reg_wr(0x10011, 0x1);
    ddr_phy_reg_wr(0x10012, 0x1);
    ddr_phy_reg_wr(0x10013, 0x180);
    ddr_phy_reg_wr(0x10018, 0x1);
    ddr_phy_reg_wr(0x10002, 0x6209);
    ddr_phy_reg_wr(0x100b2, 0x1);
    ddr_phy_reg_wr(0x101b4, 0x1);
    ddr_phy_reg_wr(0x102b4, 0x1);
    ddr_phy_reg_wr(0x103b4, 0x1);
    ddr_phy_reg_wr(0x104b4, 0x1);
    ddr_phy_reg_wr(0x105b4, 0x1);
    ddr_phy_reg_wr(0x106b4, 0x1);
    ddr_phy_reg_wr(0x107b4, 0x1);
    ddr_phy_reg_wr(0x108b4, 0x1);
    ddr_phy_reg_wr(0x11011, 0x1);
    ddr_phy_reg_wr(0x11012, 0x1);
    ddr_phy_reg_wr(0x11013, 0x180);
    ddr_phy_reg_wr(0x11018, 0x1);
    ddr_phy_reg_wr(0x11002, 0x6209);
    ddr_phy_reg_wr(0x110b2, 0x1);
    ddr_phy_reg_wr(0x111b4, 0x1);
    ddr_phy_reg_wr(0x112b4, 0x1);
    ddr_phy_reg_wr(0x113b4, 0x1);
    ddr_phy_reg_wr(0x114b4, 0x1);
    ddr_phy_reg_wr(0x115b4, 0x1);
    ddr_phy_reg_wr(0x116b4, 0x1);
    ddr_phy_reg_wr(0x117b4, 0x1);
    ddr_phy_reg_wr(0x118b4, 0x1);
    ddr_phy_reg_wr(0x12011, 0x1);
    ddr_phy_reg_wr(0x12012, 0x1);
    ddr_phy_reg_wr(0x12013, 0x180);
    ddr_phy_reg_wr(0x12018, 0x1);
    ddr_phy_reg_wr(0x12002, 0x6209);
    ddr_phy_reg_wr(0x120b2, 0x1);
    ddr_phy_reg_wr(0x121b4, 0x1);
    ddr_phy_reg_wr(0x122b4, 0x1);
    ddr_phy_reg_wr(0x123b4, 0x1);
    ddr_phy_reg_wr(0x124b4, 0x1);
    ddr_phy_reg_wr(0x125b4, 0x1);
    ddr_phy_reg_wr(0x126b4, 0x1);
    ddr_phy_reg_wr(0x127b4, 0x1);
    ddr_phy_reg_wr(0x128b4, 0x1);
    ddr_phy_reg_wr(0x13011, 0x1);
    ddr_phy_reg_wr(0x13012, 0x1);
    ddr_phy_reg_wr(0x13013, 0x180);
    ddr_phy_reg_wr(0x13018, 0x1);
    ddr_phy_reg_wr(0x13002, 0x6209);
    ddr_phy_reg_wr(0x130b2, 0x1);
    ddr_phy_reg_wr(0x131b4, 0x1);
    ddr_phy_reg_wr(0x132b4, 0x1);
    ddr_phy_reg_wr(0x133b4, 0x1);
    ddr_phy_reg_wr(0x134b4, 0x1);
    ddr_phy_reg_wr(0x135b4, 0x1);
    ddr_phy_reg_wr(0x136b4, 0x1);
    ddr_phy_reg_wr(0x137b4, 0x1);
    ddr_phy_reg_wr(0x138b4, 0x1);
    ddr_phy_reg_wr(0x20089, 0x1);
    ddr_phy_reg_wr(0x20088, 0x19);
    ddr_phy_reg_wr(0xc0080, 0x2);
    ddr_phy_reg_wr(0xd0000, 0x1);

    ddr_phy_broadcast_en(0);

    // required to print the register values else they return zero
    ddr_phy0_reg_wr(0xd0000, 0x0);
    ddr_phy0_reg_wr(0xc0080, 0x3);

    println!("PHY0 P Code       : {:02x}", ddr_phy_reg_rd(0x20014));
    println!("PHY0 N Code       : {:02x}", ddr_phy_reg_rd(0x20015));
    println!("PllCtrl1          : {:02x}", ddr_phy_reg_rd(0x200c7));
    println!("PllCtrl2          : {:02x}", ddr_phy_reg_rd(0x200c5));
    println!("PllCtrl4          : {:02x}", ddr_phy_reg_rd(0x200cc));
    println!("PllTestmode       : {:02x}", ddr_phy_reg_rd(0x200ca));
    println!("Trained DB0 DFIMRL: {:02x}", ddr_phy_reg_rd(0x10020));
    println!("Trained DB1 DFIMRL: {:02x}", ddr_phy_reg_rd(0x11020));
    println!("Trained DB2 DFIMRL: {:02x}", ddr_phy_reg_rd(0x12020));
    println!("Trained DB3 DFIMRL: {:02x}", ddr_phy_reg_rd(0x13020));
    println!("DQS Preamble      : {:02x}", ddr_phy_reg_rd(0x20024));
    println!("ARdPtrInitVal     : {:02x}", ddr_phy_reg_rd(0x2002e));
    println!("PHY0 DB0 VREF     : {:02x}", ddr_phy_reg_rd(0x10140));
    println!("PHY0 DB1 VREF     : {:02x}", ddr_phy_reg_rd(0x11140));
    println!("PHY0 DB2 VREF     : {:02x}", ddr_phy_reg_rd(0x12140));
    println!("PHY0 DB3 VREF     : {:02x}", ddr_phy_reg_rd(0x13140));
    println!("R0 TxDQSDly       : {:02x}", ddr_phy_reg_rd(0x100d0));
    println!("R0 TxDQSDly       : {:02x}", ddr_phy_reg_rd(0x101d0));
    println!("R1 TxDQSDly       : {:02x}", ddr_phy_reg_rd(0x100d1));
    println!("R1 TxDQSDly       : {:02x}", ddr_phy_reg_rd(0x101d1));

    // required to print the register values else they return zero
    ddr_phy1_reg_wr(0xd0000, 0x0);
    ddr_phy1_reg_wr(0xc0080, 0x3);
    println!("PHY1 P Code       : {:02x}", ddr_phy1_reg_rd(0x20014));
    println!("PHY1 N Code       : {:02x}", ddr_phy1_reg_rd(0x20015));
}
