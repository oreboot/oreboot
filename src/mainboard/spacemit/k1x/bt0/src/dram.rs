const DATA_RATE: u32 = 2400;
const CS_NUM: u32 = 2;
const DDRC_BASE: usize = 0xc000_0000;
const DDR_TRAINING_INFO: usize = 0xC080_0000;

use crate::util::{read32, write32};

#[repr(C)]
#[derive(Debug)]
struct DdrTrainingInfo {
    magic: u32,
    crc32: u32,
    chipid: u64,
    mac_addr: u64,
    version: u32,
    cs_num: u32,
}

const DDR_TRAINING_INFO_MAGIC: usize = 0x54524444; // DDRT
const DDR_TRAINING_INFO_VERSION: usize = 0x0001_0000;

const PLL_BASE: usize = 0xd428_2800;

const DDRC_PLL5: usize = PLL_BASE + 0x0004;
const DDRC_PLL4: usize = PLL_BASE + 0x00b0;
const DDRC_PLL3: usize = PLL_BASE + 0x00e8;
const DDRC_PLL: usize = PLL_BASE + 0x03b4;
const DDRC_PLL2: usize = PLL_BASE + 0x039c;

const BOOT_PP: u32 = 0;

fn enable_pll() {
    let v = read32(DDRC_PLL);
    write32(DDRC_PLL, v & 0xFFFF_FCFF);
    let v = read32(DDRC_PLL);
    write32(DDRC_PLL, v | (1 << 11) | (1 << 9) | (1 << 8));
    /* wait pll stable */
    let mask = 0x0003_0000;
    while (read32(DDRC_PLL) & mask) != mask {}
}

fn mck6_sw_fc_top(freq_no: u32) {
    let freq = match freq_no {
        0 => {
            /* 1200MT */
            0x0000_3B50
        }
        1 => {
            /* 1600MT */
            0x0000_3B04
        }
        2 => {
            /* 1600MT */
            0x0000_3B40
        }
        3 => {
            /* 3200MT */
            0x0000_3B00
        }
        4 => {
            println!("DDR SW frequency change to ext clk");
            0x0000_3B02
        }
        _ => {
            panic!("Unsupported DDR SW frequency change");
        }
    };
    write32(DDRC_PLL, freq);
    write32(DDRC_PLL4, 0x4060_0400);
    let v = 0x0400_0000;
    write32(DDRC_PLL5, v);
    while read32(DDRC_PLL5) & v != 0 {}
}

fn top_common_config() {
    let v = read32(DDRC_PLL2);
    write32(DDRC_PLL2, v & 0xFFFF00FF);
    let v = read32(DDRC_PLL2);
    write32(DDRC_PLL2, v | (0x3B << 8));

    enable_pll();
    mck6_sw_fc_top(BOOT_PP);

    let v = read32(DDRC_PLL3);
    write32(DDRC_PLL3, v & 0xFFFFFFFC);
    let v = read32(DDRC_PLL3);
    write32(DDRC_PLL3, v | 0x3);
}

const DFI_PHY_USER_COMMAND_0: usize = DDRC_BASE + 0x13D0;
const DPHY0_BASE_OFFSET: usize = 0x0004_0000;

// drivers/ddr/spacemit/k1x/ddr_init_asic.h
const MC_CH0_BASE_OFFSET: usize = 0x0200;
const MC_CH0_PHY_BASE_OFFSET: usize = 0x1000;

const COMMON_OFFSET: usize = 0x3000;
const SUBPHY_A_OFFSET: usize = 0x0000;
const SUBPHY_B_OFFSET: usize = 0x0200;
const FREQ_POINT_OFFSET: usize = 0x4000;

const OTHER_CONTROL_OFFSET: usize = 0x10000;

fn fp_timing_init(ddrc_base: usize) {
    let mc_ch0_base: usize = ddrc_base + MC_CH0_BASE_OFFSET;
    let mc_ch0_phy_base: usize = ddrc_base + MC_CH0_PHY_BASE_OFFSET;

    write32(mc_ch0_base + 0x0104, 0xF0800400);
    write32(mc_ch0_base + 0x0100, 0x00000E20);
    write32(mc_ch0_base + 0x010c, 0x19194314);
    write32(mc_ch0_base + 0x0110, 0x20440000);
    write32(mc_ch0_base + 0x0114, 0x20440000);
    write32(mc_ch0_base + 0x018c, 0x00000030);
    write32(mc_ch0_base + 0x0190, 0x06400030);
    write32(mc_ch0_base + 0x0194, 0x80e001c0);
    write32(mc_ch0_base + 0x01fc, 0x000C005E);
    write32(mc_ch0_base + 0x0198, 0x01CC01CC);
    write32(mc_ch0_base + 0x019c, 0x00181818);
    write32(mc_ch0_base + 0x01a0, 0x08180C0C);
    write32(mc_ch0_base + 0x01a4, 0x00000003);
    write32(mc_ch0_base + 0x01a8, 0x00000217);
    write32(mc_ch0_base + 0x01ac, 0x30651D44);
    write32(mc_ch0_base + 0x01b0, 0x1120080F);
    write32(mc_ch0_base + 0x01b4, 0x08001000);
    write32(mc_ch0_base + 0x01b8, 0x00000C00);
    write32(mc_ch0_base + 0x01bc, 0x02020404);
    write32(mc_ch0_base + 0x01c0, 0x10000004);
    write32(mc_ch0_base + 0x01c4, 0x00000006);
    write32(mc_ch0_base + 0x01d8, 0x00010190);
    write32(mc_ch0_base + 0x014c, 0x000c4090);
    write32(mc_ch0_phy_base + 0x03e4, 0x15000A02);
    write32(mc_ch0_phy_base + 0x03ec, 0x0000046c);
    write32(mc_ch0_base + 0x0104, 0xA0800400);
    write32(mc_ch0_base + 0x0100, 0x00000C18);
    write32(mc_ch0_base + 0x010c, 0x9d194314);
    write32(mc_ch0_base + 0x0110, 0x00440000);
    write32(mc_ch0_base + 0x0114, 0x00440000);
    write32(mc_ch0_base + 0x018c, 0x00430000);
    write32(mc_ch0_base + 0x0190, 0x05350028);
    write32(mc_ch0_base + 0x0194, 0x80A80151);
    write32(mc_ch0_base + 0x01fc, 0x000C005E);
    write32(mc_ch0_base + 0x0198, 0x017F017F);
    write32(mc_ch0_base + 0x019c, 0x00141414);
    write32(mc_ch0_base + 0x01a0, 0x07140A0A);
    write32(mc_ch0_base + 0x01a4, 0x00000003);
    write32(mc_ch0_base + 0x01a8, 0x00000213);
    write32(mc_ch0_base + 0x01ac, 0x36541838);
    write32(mc_ch0_base + 0x01b0, 0x1c180a18);
    write32(mc_ch0_base + 0x01b4, 0x08000E00);
    write32(mc_ch0_base + 0x01b8, 0x00000E00);
    write32(mc_ch0_base + 0x01bc, 0x02020404);
    write32(mc_ch0_base + 0x01c0, 0x10000004);
    write32(mc_ch0_base + 0x01c4, 0x00000004);
    write32(mc_ch0_base + 0x01d8, 0x0000D94E);
    write32(mc_ch0_base + 0x014c, 0x0007204a);
    write32(mc_ch0_phy_base + 0x03e4, 0x13000802);
    write32(mc_ch0_phy_base + 0x03ec, 0x00000450);
    write32(mc_ch0_base + 0x0104, 0x50800400);
    write32(mc_ch0_base + 0x0100, 0x0000080e);
    write32(mc_ch0_base + 0x010c, 0x9d194314);
    write32(mc_ch0_base + 0x0110, 0x00440000);
    write32(mc_ch0_base + 0x0114, 0x00440000);
    write32(mc_ch0_base + 0x018c, 0x00280018);
    write32(mc_ch0_base + 0x0190, 0x03200018);
    write32(mc_ch0_base + 0x0194, 0x807000e0);
    write32(mc_ch0_base + 0x01fc, 0x000C005E);
    write32(mc_ch0_base + 0x0198, 0x00e600e6);
    write32(mc_ch0_base + 0x019c, 0x000c0c0c);
    write32(mc_ch0_base + 0x01a0, 0x050c0606);
    write32(mc_ch0_base + 0x01a4, 0x00000003);
    write32(mc_ch0_base + 0x01a8, 0x0000020c);
    write32(mc_ch0_base + 0x01ac, 0x18330f22);
    write32(mc_ch0_base + 0x01b0, 0x110f080f);
    write32(mc_ch0_base + 0x01b4, 0x08000800);
    write32(mc_ch0_base + 0x01b8, 0x00000600);
    write32(mc_ch0_base + 0x01bc, 0x02020404);
    write32(mc_ch0_base + 0x01c0, 0x00000003);
    write32(mc_ch0_base + 0x01c4, 0x00000003);
    write32(mc_ch0_base + 0x01d8, 0x00008190);
    write32(mc_ch0_base + 0x014c, 0x00030848);
    write32(mc_ch0_phy_base + 0x03e4, 0x0a000402);
    write32(mc_ch0_phy_base + 0x03ec, 0x00000480);
    write32(mc_ch0_base + 0x0104, 0x00800400);
    write32(mc_ch0_base + 0x0100, 0x0000080e);
    write32(mc_ch0_base + 0x010c, 0x9d194314);
    write32(mc_ch0_base + 0x0110, 0x00440000);
    write32(mc_ch0_base + 0x0114, 0x00440000);
    write32(mc_ch0_base + 0x018c, 0x00280018);
    write32(mc_ch0_base + 0x0190, 0x03200018);
    write32(mc_ch0_base + 0x0194, 0x805400A8);
    write32(mc_ch0_base + 0x01fc, 0x000C005E);
    write32(mc_ch0_base + 0x0198, 0x00e600e6);
    write32(mc_ch0_base + 0x019c, 0x000c0c0c);
    write32(mc_ch0_base + 0x01a0, 0x050c0606);
    write32(mc_ch0_base + 0x01a4, 0x00000003);
    write32(mc_ch0_base + 0x01a8, 0x0000020c);
    write32(mc_ch0_base + 0x01ac, 0x18330f22);
    write32(mc_ch0_base + 0x01b0, 0x110f080f);
    write32(mc_ch0_base + 0x01b4, 0x08000800);
    write32(mc_ch0_base + 0x01b8, 0x00000600);
    write32(mc_ch0_base + 0x01bc, 0x02020404);
    write32(mc_ch0_base + 0x01c0, 0x00000002);
    write32(mc_ch0_base + 0x01c4, 0x00000003);
    write32(mc_ch0_base + 0x01d8, 0x00008190);
    write32(mc_ch0_base + 0x014c, 0x00030848);
    write32(mc_ch0_phy_base + 0x03e4, 0x0a000402);
    write32(mc_ch0_phy_base + 0x03ec, 0x00000480);

    let r = mc_ch0_base + 0x0108;
    write32(r, (read32(r) & 0xF00FFFFF) | (0x10 << 20));
}

fn fp_sel(ddrc_base: usize, fp: u32) {
    let mc_ch0_base: usize = ddrc_base + MC_CH0_BASE_OFFSET;

    let r = mc_ch0_base + 0x0104;
    write32(r, (read32(r) & !(0xf << 28)) | (fp << 28) | (fp << 30));

    println!("ADDR[0x{:08x}]=0x{:08x}", r, read32(r));
}

fn top_ddr_mc_init(ddrc_base: usize, fp: u32) {
    let mc_ch0_base: usize = ddrc_base + MC_CH0_BASE_OFFSET;
    let mc_ch0_phy_base: usize = ddrc_base + MC_CH0_PHY_BASE_OFFSET;
    write32(ddrc_base + 0x44, 0x00040300);
    write32(ddrc_base + 0x48, 0x00000001);
    write32(ddrc_base + 0x64, 0x100d0803);
    write32(ddrc_base + 0x50, 0x000000ff);
    write32(ddrc_base + 0x58, 0x3fd53fd5);
    write32(ddrc_base + 0x180, 0x00010200);
    write32(mc_ch0_base, 0x100001);
    write32(mc_ch0_base + 0x4, 0x0);
    write32(mc_ch0_base + 0x8, 0x100001);
    write32(mc_ch0_base + 0xc, 0x1);
    write32(ddrc_base + 0x0080, 0x00000000);
    write32(ddrc_base + 0x0a00, 0x00000000);
    write32(ddrc_base + 0x0ac0, 0x00000000);
    write32(ddrc_base + 0x0acc, 0xffffffff);
    write32(mc_ch0_base + 0x20, 0x05030732);
    write32(mc_ch0_base + 0x24, 0x05030732);
    write32(mc_ch0_base + 0xc0, 0x14008000);
    write32(mc_ch0_base + 0xc4, 0x000000b8);
    write32(mc_ch0_base + 0xc8, 0x0000FFFF);
    write32(mc_ch0_base + 0xcc, 0x200);

    fp_timing_init(ddrc_base);
    fp_sel(ddrc_base, fp);

    write32(mc_ch0_base + 0x0180, 0x30D400);
    write32(mc_ch0_base + 0x0184, 0x4E200);
    write32(mc_ch0_base + 0x0188, 0xC800000);

    let r = mc_ch0_phy_base + 0x03e0;
    write32(r, read32(r) | (fp << 2));
}

fn top_ddr_wr_ds_odt_vref(dphy0_base: usize, combination: u32) {
    let common_base = dphy0_base + COMMON_OFFSET;
    let offset_a = common_base + SUBPHY_A_OFFSET;
    let offset_b = common_base + SUBPHY_B_OFFSET;

    match combination {
        2 => {
            let d_reg2 = 0xd8;
            let data = read32(common_base + 0xc);
            let data = (data & 0xFFFF00FF) | (d_reg2 << 8);

            write32(offset_a + FREQ_POINT_OFFSET * 0 + 0xc, data);
            write32(offset_a + FREQ_POINT_OFFSET * 1 + 0xc, data);
            write32(offset_a + FREQ_POINT_OFFSET * 2 + 0xc, data);
            write32(offset_a + FREQ_POINT_OFFSET * 3 + 0xc, data);

            write32(offset_b + FREQ_POINT_OFFSET * 0 + 0xc, data);
            write32(offset_b + FREQ_POINT_OFFSET * 1 + 0xc, data);
            write32(offset_b + FREQ_POINT_OFFSET * 2 + 0xc, data);
            write32(offset_b + FREQ_POINT_OFFSET * 3 + 0xc, data);
        }
        _ => {
            println!("top_DDR_wr_ds_odt_vref: combination {combination} not supported");
        }
    }
}

fn top_ddr_rx_ds_odt_vref(dphy0_base: usize, combination: u32) {
    let common_base = dphy0_base + COMMON_OFFSET;
    let offset_a = common_base + SUBPHY_A_OFFSET;
    let offset_b = common_base + SUBPHY_B_OFFSET;

    match combination {
        2 => {
            let d_reg3 = 0xE4;
            let data = read32(common_base + 0xc);
            let data = (data & 0xFF00FFFF) | (d_reg3 << 16);

            write32(offset_a + FREQ_POINT_OFFSET * 0 + 0xc, data);
            write32(offset_a + FREQ_POINT_OFFSET * 1 + 0xc, data);
            write32(offset_a + FREQ_POINT_OFFSET * 2 + 0xc, data);
            write32(offset_a + FREQ_POINT_OFFSET * 3 + 0xc, data);

            write32(offset_b + FREQ_POINT_OFFSET * 0 + 0xc, data);
            write32(offset_b + FREQ_POINT_OFFSET * 1 + 0xc, data);
            write32(offset_b + FREQ_POINT_OFFSET * 2 + 0xc, data);
            write32(offset_b + FREQ_POINT_OFFSET * 3 + 0xc, data);

            let rx_ref_d1 = 0x55;
            let rx_ref_d2 = 0x55;
            let data = read32(common_base + 0x4);
            let data = (data & 0x0000FFFF) | (rx_ref_d1 << 16) | (rx_ref_d2 << 24);

            write32(offset_a + FREQ_POINT_OFFSET * 0 + 0x4, data);
            write32(offset_a + FREQ_POINT_OFFSET * 1 + 0x4, data);
            write32(offset_a + FREQ_POINT_OFFSET * 2 + 0x4, data);
            write32(offset_a + FREQ_POINT_OFFSET * 3 + 0x4, data);

            write32(offset_b + FREQ_POINT_OFFSET * 0 + 0x4, data);
            write32(offset_b + FREQ_POINT_OFFSET * 1 + 0x4, data);
            write32(offset_b + FREQ_POINT_OFFSET * 2 + 0x4, data);
            write32(offset_b + FREQ_POINT_OFFSET * 3 + 0x4, data);
        }
        _ => {
            println!("top_DDR_rx_ds_odt_vref: combination {combination} not supported");
        }
    }
}

fn top_ddr_amble_config(dphy0_base: usize) {
    let common_base = dphy0_base + COMMON_OFFSET;
    let offset_a = common_base + SUBPHY_A_OFFSET;
    let offset_b = common_base + SUBPHY_B_OFFSET;

    let data = read32(common_base + 0x4);
    let data = (data & 0xFFFF0FFF) | (1 << 11) | (1 << 13) | (1 << 15);

    write32(offset_a + FREQ_POINT_OFFSET * 0 + 0x4, data);
    write32(offset_a + FREQ_POINT_OFFSET * 1 + 0x4, data);
    write32(offset_a + FREQ_POINT_OFFSET * 2 + 0x4, data);
    write32(offset_a + FREQ_POINT_OFFSET * 3 + 0x4, data);

    write32(offset_b + FREQ_POINT_OFFSET * 0 + 0x4, data);
    write32(offset_b + FREQ_POINT_OFFSET * 1 + 0x4, data);
    write32(offset_b + FREQ_POINT_OFFSET * 2 + 0x4, data);
    write32(offset_b + FREQ_POINT_OFFSET * 3 + 0x4, data);
}

fn top_ddr_phy_init(ddrc_base: usize, fp: u32) {
    let dphy0_base = ddrc_base + DPHY0_BASE_OFFSET;
    let common_base = dphy0_base + COMMON_OFFSET;
    let offset_a = common_base + SUBPHY_A_OFFSET;
    let offset_b = common_base + SUBPHY_B_OFFSET;

    let r = PLL_BASE + 0x3A4;
    let v = read32(r);
    write32(r, (v & 0xFFFF00FF) | (0xF << 8));

    let r = PLL_BASE + 0x398;
    let v = read32(r);
    write32(r, v | (0x3 << 10));

    write32(offset_a, 0x0);
    write32(offset_b, 0x0);
    write32(offset_a, 0x1);
    write32(offset_b, 0x1);

    write32(dphy0_base + FREQ_POINT_OFFSET * 0 + 0x64, 0x4349);
    write32(dphy0_base + FREQ_POINT_OFFSET * 1 + 0x64, 0x4349);
    write32(dphy0_base + FREQ_POINT_OFFSET * 2 + 0x64, 0x4349);
    write32(dphy0_base + FREQ_POINT_OFFSET * 3 + 0x64, 0x4349);

    top_ddr_amble_config(dphy0_base);
    top_ddr_wr_ds_odt_vref(dphy0_base, 2);
    top_ddr_rx_ds_odt_vref(dphy0_base, 2);

    let data = read32(common_base + 0x14);
    let data = (data & 0xFF9FFFEF | (0x3 << 21));

    write32(offset_a + FREQ_POINT_OFFSET * 0 + 0x14, data);
    write32(offset_b + FREQ_POINT_OFFSET * 0 + 0x14, data);
    write32(offset_a + FREQ_POINT_OFFSET * 1 + 0x14, data);
    write32(offset_b + FREQ_POINT_OFFSET * 1 + 0x14, data);
    write32(offset_a + FREQ_POINT_OFFSET * 2 + 0x14, data);
    write32(offset_b + FREQ_POINT_OFFSET * 2 + 0x14, data);
    write32(offset_a + FREQ_POINT_OFFSET * 3 + 0x14, data);
    write32(offset_b + FREQ_POINT_OFFSET * 3 + 0x14, data);

    let data = read32(common_base + 0x10);
    let data = data | 0x10000000;

    write32(offset_a + FREQ_POINT_OFFSET * 0 + 0x10, data);
    write32(offset_b + FREQ_POINT_OFFSET * 0 + 0x10, data);
    write32(offset_a + FREQ_POINT_OFFSET * 1 + 0x10, data);
    write32(offset_b + FREQ_POINT_OFFSET * 1 + 0x10, data);
    write32(offset_a + FREQ_POINT_OFFSET * 2 + 0x10, data);
    write32(offset_b + FREQ_POINT_OFFSET * 2 + 0x10, data);
    write32(offset_a + FREQ_POINT_OFFSET * 3 + 0x10, data);
    write32(offset_b + FREQ_POINT_OFFSET * 3 + 0x10, data);

    write32(common_base + 0x30, 0x1077);
    write32(dphy0_base + OTHER_CONTROL_OFFSET + 0x24, 0x0);

    let r = dphy0_base + OTHER_CONTROL_OFFSET;
    let v = read32(r);
    write32(r, v | 0x1);
}

fn top_ddr_mc_phy_device_init(ddrc_base: usize, cs_val: u32, fp: u32) {
    let cs_num = if cs_val == 1 { 1 } else { 3 };

    top_ddr_mc_init(ddrc_base, fp);
    top_ddr_phy_init(ddrc_base, fp);

    let mc_ch0_phy_base: usize = ddrc_base + MC_CH0_PHY_BASE_OFFSET;

    // same as Marvell
    write32(DFI_PHY_USER_COMMAND_0, 0x1300_0001);
    let m = 0x8000_0000;
    while read32(mc_ch0_phy_base + 0x3fc) & m != m {}
    println!("PHY INIT done");

    write32(DFI_PHY_USER_COMMAND_0, 0x1300_0100);
    write32(DDRC_BASE + 0x20, 0x10000001 | (cs_num << 24));

    println!("wait DRAM INIT");
    while read32(ddrc_base + 0x8) & 0x11 != 0x11 {}
    println!("DRAM INIT done");

    write32(ddrc_base + 0x24, 0x10020001 | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x10020002 | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x1002000d | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x10020003 | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x10020016 | (cs_num << 24));

    write32(ddrc_base + 0x20, 0x11002000);
    write32(ddrc_base + 0x20, 0x11001000);

    if (cs_val != 1) {
        write32(ddrc_base + 0x20, 0x12002000);
        write32(ddrc_base + 0x20, 0x12001000);
    }

    write32(ddrc_base + 0x24, 0x1002000C | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x1002000E | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x1002000B | (cs_num << 24));
    write32(ddrc_base + 0x24, 0x10020017 | (cs_num << 24));

    println!("DRAM Mode register Init done.");
}

fn mem_read<'a, T>(addr: usize) -> &'a T {
    unsafe { (addr as *const T).as_ref().unwrap() }
}

const DDR_MR_DATA: usize = (DDRC_BASE + 0x370);
const DDR_MR_REG: usize = (DDRC_BASE + 0x24);

fn mode_register_read(mr: u32, ch: u32, cs: u32) -> u32 {
    write32(DDR_MR_REG, 0x10010000 + ((cs + 1) << 24) + (ch << 18) + mr);
    while ((read32(DDR_MR_DATA) & 0x80000000) == 0) {}
    read32(DDRC_BASE + 0x234) & 0xFF
}

// drivers/ddr/spacemit/k1x/ddr_freq.h
#[allow(non_camel_case_types)]
enum Density {
    DDR_1Gb = 0, // not defined
    DDR_2Gb = 256,
    DDR_3Gb = 384,
    DDR_4Gb = 512,
    DDR_6Gb = 768,
    DDR_8Gb = 1024,
    DDR_12Gb = 1536,
    DDR_16Gb = 2048,
    RESERVEDX,
}

impl Density {
    fn from_u32(value: u32) -> Density {
        match value {
            0 => Density::DDR_2Gb,
            1 => Density::DDR_3Gb,
            2 => Density::DDR_4Gb,
            3 => Density::DDR_6Gb,
            4 => Density::DDR_8Gb,
            5 => Density::DDR_12Gb,
            6 => Density::DDR_16Gb,
            12 => Density::DDR_1Gb,
            _ => panic!("Unsupported density 0x{value:08x}"),
        }
    }
}

fn format_size(density: u32, io_width: u32) -> u32 {
    let size = Density::from_u32(density) as u32;
    if io_width == 1 {
        size * 2
    } else {
        size
    }
}

fn ddr_get_density(cs_num: u32) -> u32 {
    let cs0_size = {
        let mr8_cs00 = mode_register_read(8, 0, 0);
        let mr8_cs01 = mode_register_read(8, 1, 0);

        let io_width_cs00 = if mr8_cs00 > 0 { mr8_cs00 >> 6 } else { 0 };
        let io_width_cs01 = if mr8_cs01 > 0 { mr8_cs01 >> 6 } else { 0 };

        let cs0_size = if mr8_cs00 > 0 {
            format_size((mr8_cs00 >> 2) & 0xf, io_width_cs00)
        } else {
            0
        };
        let cs0_size = cs0_size
            + if mr8_cs01 > 0 {
                format_size((mr8_cs01 >> 2) & 0xf, io_width_cs01)
            } else {
                0
            };
        cs0_size
    };
    let cs1_size = if (cs_num > 1) {
        let mr8_cs10 = mode_register_read(8, 0, 1);
        let mr8_cs11 = mode_register_read(8, 1, 1);

        let io_width_cs10 = if mr8_cs10 > 0 { mr8_cs10 >> 6 } else { 0 };
        let io_width_cs11 = if mr8_cs11 > 0 { mr8_cs11 >> 6 } else { 0 };

        let cs1_size = if mr8_cs10 > 0 {
            format_size((mr8_cs10 >> 2) & 0xf, io_width_cs10)
        } else {
            0
        };
        let cs1_size = cs1_size
            + if mr8_cs11 > 0 {
                format_size((mr8_cs11 >> 2) & 0xf, io_width_cs11)
            } else {
                0
            };
        cs1_size
    } else {
        0
    };

    cs0_size + cs1_size
}

// include/configs/k1-x.h
const DDR_CS_NUM: u32 = 1;

#[repr(u32)]
enum Bank {
    B2 = 0,
    B4,
    B8,
    Reserved,
}

#[repr(u32)]
enum Row {
    R11 = 1,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
}

#[repr(u32)]
enum Column {
    C8 = 1,
    C9,
    C10,
    C11,
    C12,
}

fn adjust_mapping(ddrc_base: usize, cs_num: u32, size_mb: u32, mr8_value: u32) {
    let area_length_mb = size_mb / cs_num;
    // area_length_mb = size_mb >> (cs_num -1);
    let area_length_cfg = match area_length_mb {
        1024 => 0xE,
        2048 => 0xF,
        4096 => 0x10,
        8192 => 0x11,
        16384 => 0x12,
        _ => {
            println!("Unsupported area length 0x{area_length_mb}MB");
            0x10
        }
    };

    let cs1_start_addr_l = (area_length_mb >> 3) & 0x1FF;
    let cs1_start_addr_h = (area_length_mb >> 12) & 0xFFFFFFFF;

    let io_width = mr8_value >> 6;
    let density = (mr8_value >> 2) & 0xf;

    const IO_X16: u32 = 0;
    const IO_X8: u32 = 1;

    let row = match (io_width, density) {
        (IO_X16, 12) => Row::R13,
        (IO_X16, 0) => Row::R14,
        (IO_X16, 1) => Row::R15,
        (IO_X16, 2) => Row::R15,
        (IO_X16, 3) => Row::R16,
        (IO_X16, 4) => Row::R16,
        (IO_X16, 5) => Row::R17,
        (IO_X16, 6) => Row::R17,
        (IO_X8, 12) => Row::R14,
        (IO_X8, 0) => Row::R15,
        (IO_X8, 1) => Row::R16,
        (IO_X8, 2) => Row::R16,
        (IO_X8, 3) => Row::R17,
        (IO_X8, 4) => Row::R17,
        (IO_X8, 5) => Row::R18,
        (IO_X8, 6) => Row::R18,
        _ => panic!("Unsupported IO width {io_width}"),
    };
    let bank = Bank::B8;
    let column = Column::C10;

    let mc_ch0_base = ddrc_base + MC_CH0_BASE_OFFSET;

    let v = read32(mc_ch0_base);
    write32(mc_ch0_base, (v & 0x0060FFFF) | (area_length_cfg << 16));

    write32(mc_ch0_base + 0x4, 0x0);

    let v = read32(mc_ch0_base + 0x8);
    write32(
        mc_ch0_base + 0x8,
        (v & 0x0060FFFF | (cs1_start_addr_l << 23) | (area_length_cfg << 16)),
    );

    write32(mc_ch0_base + 0xc, cs1_start_addr_h);

    let a = ((row as u32) << 8) | ((column as u32) << 4) | (bank as u32);
    let mask = 0xFFFFF00C;

    let v = read32(mc_ch0_base + 0x20);
    write32(mc_ch0_base + 0x20, (v & mask) | a);

    let v = read32(mc_ch0_base + 0x24);
    write32(mc_ch0_base + 0x24, (v & mask) | a);

    let reg = mc_ch0_base;
    println!("DEBUG-ADDR[0x{reg:08x}]:0x{:08x}", read32(reg));
    let reg = mc_ch0_base + 0x4;
    println!("DEBUG-ADDR[0x{reg:08x}]:0x{:08x}", read32(reg));
    let reg = mc_ch0_base + 0x8;
    println!("DEBUG-ADDR[0x{reg:08x}]:0x{:08x}", read32(reg));
    let reg = mc_ch0_base + 0xc;
    println!("DEBUG-ADDR[0x{reg:08x}]:0x{:08x}", read32(reg));
    let reg = mc_ch0_base + 0x20;
    println!("DEBUG-ADDR[0x{reg:08x}]:0x{:08x}", read32(reg));
    let reg = mc_ch0_base + 0x24;
    println!("DEBUG-ADDR[0x{reg:08x}]:0x{:08x}", read32(reg));
}

fn ddr_dfc_table_init(ddrc_base: usize) {
    write32(ddrc_base + 0x74, 0x00040303);
    write32(ddrc_base + 0x78, 0x00000044);
    write32(ddrc_base + 0x70, 0x00000000);
    write32(ddrc_base + 0x74, 0x13000008);
    write32(ddrc_base + 0x78, 0x00000020);
    write32(ddrc_base + 0x70, 0x00000001);
    write32(ddrc_base + 0x74, 0x13010000);
    write32(ddrc_base + 0x78, 0x00000028);
    write32(ddrc_base + 0x70, 0x00000002);
    write32(ddrc_base + 0x74, 0x1302000d);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000003);
    write32(ddrc_base + 0x74, 0x13020001);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000004);
    write32(ddrc_base + 0x74, 0x13020002);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000005);
    write32(ddrc_base + 0x74, 0x13020003);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000006);
    write32(ddrc_base + 0x74, 0x1302000b);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000007);
    write32(ddrc_base + 0x74, 0x1302000c);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000008);
    write32(ddrc_base + 0x74, 0x1302000e);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000009);
    write32(ddrc_base + 0x74, 0x13020016);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x0000000a);
    write32(ddrc_base + 0x74, 0x13008000);
    write32(ddrc_base + 0x78, 0x00000028);
    write32(ddrc_base + 0x70, 0x0000000b);
    write32(ddrc_base + 0x74, 0x1302000d);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x0000000c);
    write32(ddrc_base + 0x74, 0x13000010);
    write32(ddrc_base + 0x78, 0x00000020);
    write32(ddrc_base + 0x70, 0x0000000d);
    write32(ddrc_base + 0x74, 0x00000002);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x0000000e);
    write32(ddrc_base + 0x74, 0x00000002);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x0000000f);
    write32(ddrc_base + 0x74, 0x13000001);
    write32(ddrc_base + 0x78, 0x000013d0);
    write32(ddrc_base + 0x70, 0x00000010);
    write32(ddrc_base + 0x74, 0x00008000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000011);
    write32(ddrc_base + 0x74, 0x00000000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000012);
    write32(ddrc_base + 0x74, 0x00040303);
    write32(ddrc_base + 0x78, 0x00010044);
    write32(ddrc_base + 0x70, 0x00000013);
    write32(ddrc_base + 0x74, 0x10000100);
    write32(ddrc_base + 0x78, 0x000013d0);
    write32(ddrc_base + 0x70, 0x00000014);
    write32(ddrc_base + 0x74, 0x00008000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000015);
    write32(ddrc_base + 0x74, 0x00008000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000016);
    write32(ddrc_base + 0x74, 0x13000004);
    write32(ddrc_base + 0x78, 0x00000020);
    write32(ddrc_base + 0x70, 0x00000017);
    write32(ddrc_base + 0x74, 0x1302000d);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x00000018);
    write32(ddrc_base + 0x74, 0x00000002);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x00000019);
    write32(ddrc_base + 0x74, 0x00000000);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x0000001a);
    write32(ddrc_base + 0x74, 0x00040380);
    write32(ddrc_base + 0x78, 0x00020044);
    write32(ddrc_base + 0x70, 0x0000001b);
    write32(ddrc_base + 0x74, 0x00040380);
    write32(ddrc_base + 0x78, 0x00020044);
    write32(ddrc_base + 0x70, 0x0000012e);
    write32(ddrc_base + 0x74, 0x00040b43);
    write32(ddrc_base + 0x78, 0x00000044);
    write32(ddrc_base + 0x70, 0x00000180);
    write32(ddrc_base + 0x74, 0x13000010);
    write32(ddrc_base + 0x78, 0x00000020);
    write32(ddrc_base + 0x70, 0x00000181);
    write32(ddrc_base + 0x74, 0x00000002);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x00000182);
    write32(ddrc_base + 0x74, 0x00000002);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x00000183);
    write32(ddrc_base + 0x74, 0x13000001);
    write32(ddrc_base + 0x78, 0x000013d0);
    write32(ddrc_base + 0x70, 0x00000184);
    write32(ddrc_base + 0x74, 0x00008000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000185);
    write32(ddrc_base + 0x74, 0x00000000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000186);
    write32(ddrc_base + 0x74, 0x00040b43);
    write32(ddrc_base + 0x78, 0x00010044);
    write32(ddrc_base + 0x70, 0x00000187);
    write32(ddrc_base + 0x74, 0x10000100);
    write32(ddrc_base + 0x78, 0x000013d0);
    write32(ddrc_base + 0x70, 0x00000188);
    write32(ddrc_base + 0x74, 0x00008000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x00000189);
    write32(ddrc_base + 0x74, 0x00008000);
    write32(ddrc_base + 0x78, 0x000033fc);
    write32(ddrc_base + 0x70, 0x0000018a);
    write32(ddrc_base + 0x74, 0x1302000d);
    write32(ddrc_base + 0x78, 0x00000024);
    write32(ddrc_base + 0x70, 0x0000018b);
    write32(ddrc_base + 0x74, 0x00000002);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x0000018c);
    write32(ddrc_base + 0x74, 0x00000000);
    write32(ddrc_base + 0x78, 0x00002008);
    write32(ddrc_base + 0x70, 0x0000018d);
    write32(ddrc_base + 0x74, 0x00040b00);
    write32(ddrc_base + 0x78, 0x00020044);
    write32(ddrc_base + 0x70, 0x0000018e);
}

const ADDRS: [usize; 22] = [
    0x0048,
    0x0054,
    0x0058,
    0x0060,
    0x0064,
    0x0148,
    0x014c,
    MC_CH0_BASE_OFFSET + 0x0000,
    MC_CH0_BASE_OFFSET + 0x0004,
    MC_CH0_BASE_OFFSET + 0x0008,
    MC_CH0_BASE_OFFSET + 0x000c,
    MC_CH0_BASE_OFFSET + 0x0020,
    MC_CH0_BASE_OFFSET + 0x0024,
    MC_CH0_BASE_OFFSET + 0x00c4,
    MC_CH0_BASE_OFFSET + 0x00c0,
    MC_CH0_BASE_OFFSET + 0x0180,
    MC_CH0_BASE_OFFSET + 0x0184,
    MC_CH0_BASE_OFFSET + 0x0188,
    0x80,
    0xa00,
    0xac0,
    0xacc,
];

const ADDRS2: [usize; 24] = [
    MC_CH0_BASE_OFFSET + 0x0100,
    MC_CH0_BASE_OFFSET + 0x010c,
    MC_CH0_BASE_OFFSET + 0x0110,
    MC_CH0_BASE_OFFSET + 0x0114,
    MC_CH0_BASE_OFFSET + 0x018c,
    MC_CH0_BASE_OFFSET + 0x0190,
    MC_CH0_BASE_OFFSET + 0x0194,
    MC_CH0_BASE_OFFSET + 0x0198,
    MC_CH0_BASE_OFFSET + 0x019c,
    MC_CH0_BASE_OFFSET + 0x01a0,
    MC_CH0_BASE_OFFSET + 0x01a4,
    MC_CH0_BASE_OFFSET + 0x01a8,
    MC_CH0_BASE_OFFSET + 0x01ac,
    MC_CH0_BASE_OFFSET + 0x01b0,
    MC_CH0_BASE_OFFSET + 0x01b4,
    MC_CH0_BASE_OFFSET + 0x01b8,
    MC_CH0_BASE_OFFSET + 0x01bc,
    MC_CH0_BASE_OFFSET + 0x01c0,
    MC_CH0_BASE_OFFSET + 0x01c4,
    MC_CH0_BASE_OFFSET + 0x0200,
    MC_CH0_BASE_OFFSET + 0x01d8,
    MC_CH0_BASE_OFFSET + 0x014c,
    MC_CH0_PHY_BASE_OFFSET + 0x03e4,
    MC_CH0_PHY_BASE_OFFSET + 0x03ec,
];

fn init_table_mc_tim(ddrc_base: usize, idx: &mut u32) {
    for a in ADDRS2 {
        let v = read32(ddrc_base + a);
        write32(ddrc_base + 0x0074, v);
        write32(ddrc_base + 0x0078, a as u32);
        write32(ddrc_base + 0x0070, *idx);
        *idx += 1;
    }
}

fn init_table_mc_a0(ddrc_base: usize) {
    let mut idx = 0x200;
    let dphy0_base = ddrc_base + DPHY0_BASE_OFFSET;
    let mc_cfg2_offset = MC_CH0_BASE_OFFSET + 0x0104;
    let mc_cfg2_addr = ddrc_base + mc_cfg2_offset;

    let mc_ctl0_org = read32(ddrc_base + 0x44);
    write32(ddrc_base + 0x74, mc_ctl0_org | (1 << 2) | (1 << 12));
    write32(ddrc_base + 0x78, 0x00000044 | (0x1 << 16));
    write32(ddrc_base + 0x70, idx);
    idx += 1;

    for a in ADDRS {
        write32(ddrc_base + 0x74, read32(ddrc_base + a));
        write32(ddrc_base + 0x78, a as u32 & 0xffff);
        write32(ddrc_base + 0x70, idx);
        idx += 1;
    }

    let mc_cfg2_org = read32(mc_cfg2_addr);
    let temp_data = mc_cfg2_org & !(0xf << 28);

    let mc_cfg2_fp = temp_data;
    write32(mc_cfg2_addr, mc_cfg2_fp);
    write32(ddrc_base + 0x0074, mc_cfg2_fp);
    write32(ddrc_base + 0x0078, mc_cfg2_offset as u32);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;
    init_table_mc_tim(ddrc_base, &mut idx);

    let mc_cfg2_fp = temp_data | (0x5 << 28);
    write32(mc_cfg2_addr, mc_cfg2_fp);
    let mc_cfg2_fp = mc_cfg2_fp & !(0x3 << 28);
    write32(ddrc_base + 0x0074, mc_cfg2_fp);
    write32(ddrc_base + 0x0078, mc_cfg2_offset as u32);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;
    init_table_mc_tim(ddrc_base, &mut idx);

    let mc_cfg2_fp = temp_data | (0xa << 28);
    write32(mc_cfg2_addr, mc_cfg2_fp);
    let mc_cfg2_fp = mc_cfg2_fp & !(0x3 << 28);
    write32(ddrc_base + 0x0074, mc_cfg2_fp);
    write32(ddrc_base + 0x0078, mc_cfg2_offset as u32);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;
    init_table_mc_tim(ddrc_base, &mut idx);

    let mc_cfg2_fp = temp_data | (0xf << 28);
    write32(mc_cfg2_addr, mc_cfg2_fp);
    let mc_cfg2_fp = mc_cfg2_fp & !(0x3 << 28);
    write32(ddrc_base + 0x0074, mc_cfg2_fp);
    write32(ddrc_base + 0x0078, mc_cfg2_offset as u32);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;
    init_table_mc_tim(ddrc_base, &mut idx);

    write32(ddrc_base + 0x0074, 0x00020200);
    write32(ddrc_base + 0x0078, 0x000013e0);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x13000010);
    write32(ddrc_base + 0x0078, 0x000013d0);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x00010000);
    write32(ddrc_base + 0x0078, 0x000033fc);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x00010000);
    write32(ddrc_base + 0x0078, 0x000033fc);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x13000008);
    write32(ddrc_base + 0x0078, 0x00000020);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x13000004);
    write32(ddrc_base + 0x0078, 0x00000020);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x13020000);
    write32(ddrc_base + 0x0078, 0x00000028);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x13000001);
    write32(ddrc_base + 0x0078, 0x000013d0);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x00008000);
    write32(ddrc_base + 0x0078, 0x000033fc);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x00008000);
    write32(ddrc_base + 0x0078, 0x000033fc);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x10000100);
    write32(ddrc_base + 0x0078, 0x000013d0);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x00008000);
    write32(ddrc_base + 0x0078, 0x000033fc);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x00008000);
    write32(ddrc_base + 0x0078, 0x000033fc);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x1302000d);
    write32(ddrc_base + 0x0078, 0x00000024);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, 0x13020003);
    write32(ddrc_base + 0x0078, 0x00000024);
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(ddrc_base + 0x0074, mc_ctl0_org);
    write32(ddrc_base + 0x0078, 0x44 | (0x1 << 17));
    write32(ddrc_base + 0x0070, idx);
    idx += 1;

    write32(dphy0_base + 0x10104, 0x00001100);
    write32(dphy0_base + 0x10108, 0x00010000);
    write32(dphy0_base + 0x10100, 0x00000020);
    write32(dphy0_base + 0x10104, 0x000000ff);
    write32(dphy0_base + 0x10108, 0x0001001c);
    write32(dphy0_base + 0x10100, 0x00000021);
    write32(dphy0_base + 0x10104, 0x00000000);
    write32(dphy0_base + 0x10108, 0x0005001c);
    write32(dphy0_base + 0x10100, 0x00000022);

    write32(mc_cfg2_addr, mc_cfg2_org);
}

const MC_REQ_TABLE_NUM: u32 = 3;
const FREQ_PLL_CHG_MODE: u32 = 9;
const DCLK_BYPASS_DIV: u32 = 16;
const DCLK_BYPASS_RST: u32 = 21;
const DCLK_BYPASS_CLK_EN: u32 = 22;
const DDRPHY0_EN: u32 = 30;

fn ddr_dfc(ddrc_base: usize, freq_no: u32) {
    let r = PLL_BASE + 0x098;
    write32(r, read32(r) | 0x10);
    write32(ddrc_base + 0x148, 0x80ac0000);

    const BASE_CFG: u32 = (1 << DDRPHY0_EN) | (1 << DCLK_BYPASS_RST);
    fn get_cfg1(r: u32) -> u32 {
        BASE_CFG | (1 << FREQ_PLL_CHG_MODE) | (r << MC_REQ_TABLE_NUM)
    }

    const FREQ_REG1: usize = PLL_BASE + 0x3b4;
    const FREQ_REG2: usize = PLL_BASE + 0x0b0;

    match freq_no {
        0 => {
            println!("change to 1200");
            write32(FREQ_REG1, 0x00003B50);
            write32(FREQ_REG2, get_cfg1(0));
        }
        1 => {
            println!("change to 1600");
            write32(FREQ_REG1, 0x00003B04);
            write32(FREQ_REG2, get_cfg1(1));
        }
        2 => {
            println!("change to 2400");
            write32(FREQ_REG1, 0x00003B40);
            write32(FREQ_REG2, get_cfg1(2));
        }
        3 => {
            println!("change to 3200");
            write32(FREQ_REG1, 0x00003B00);
            write32(FREQ_REG2, get_cfg1(3));
        }
        4 => {
            println!("change to ext clock");
            let cfg = BASE_CFG | (1 << DCLK_BYPASS_CLK_EN) | (1 << DCLK_BYPASS_DIV);
            write32(FREQ_REG1, 0x00003B02);
            write32(FREQ_REG2, cfg);
        }
        _ => {
            println!("no this case");
        }
    }

    const DDR_FREQ_CHG_REQ: u32 = 22;
    let r = PLL_BASE + 0x0004;
    write32(r, 1 << DDR_FREQ_CHG_REQ);
    while read32(r) & 0x400000 != 0x0 {}

    println!("frequency change done!");
}

fn self_refresh(ddrc_base: usize, cs_num: u32, on_off: bool) {
    let (p1, p2) = if (cs_num == 2) { (3, 0x44) } else { (1, 4) };

    let s = if on_off { 1 } else { 2 };
    write32(ddrc_base + 0x20, (s * 0x40) | (p1 << 0x18) | 0x10000000);

    if on_off {
        while read32(ddrc_base + 0x8) & p2 != p2 {}
    } else {
        while read32(ddrc_base + 0x8) & p2 != 0 {}
    }
}

fn write_leveling(
    ddrc_base: usize,
    boot_pp: u32,
    cs_num: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
) {
    //
    let v = (p4 << 31) | (p5 << 24) | (p6 << 16) | (p7 << 8) | p8;
    write32(ddrc_base + 0x13e8, v);

    for i in 1..=cs_num {
        write32(ddrc_base + 0x13d0, 0x1040_0000 | (i << 24));
        while read32(ddrc_base + 0x13fc) & 1 == 0 {}
    }
}

fn train(
    ddrc_base: usize,
    boot_pp: u32,
    cs_num: u32,
    do_ca_training: bool,
    do_write_leveling: bool,
    do_read_gate_training: bool,
    do_read_training: bool,
    do_write_training: bool,
) {
    println!("Training start...");
    let dphy0_base = ddrc_base + DPHY0_BASE_OFFSET;

    let tmp = 0;

    // TODO: some bits missing from c0834d28..c0834d50
    write32(ddrc_base + 0x50028, 0x200);

    // fill temp

    println!("write leveling");
    write_leveling(ddrc_base, tmp, boot_pp, 0, 0xa, 0x64, 0x1e, 0x44);
}

fn top_training_fp_all(ddrc_base: usize, cs_num: u32, boot_pp: u32, info_para: &mut u32) {
    println!("self refresh start");
    self_refresh(ddrc_base, cs_num, true);
    println!("self refresh done");
    train(ddrc_base, boot_pp, cs_num, false, true, true, true, true);
    self_refresh(ddrc_base, cs_num, false);
}

pub fn init() {
    let ddr_info: &DdrTrainingInfo = mem_read(DDR_TRAINING_INFO);
    println!("{ddr_info:#08x?}");

    // NOTE: This comes from the DT in U-Boot. Default is 1 otherwise.
    let cs_num = 2;

    top_common_config();
    top_ddr_mc_phy_device_init(DDRC_BASE, cs_num, 0);

    let size_mb = ddr_get_density(cs_num);
    let mr8_value = mode_register_read(8, 0, 0) & 0xff;
    println!("DDR size (density): {size_mb}MB");
    println!("MR 8: {mr8_value}");
    adjust_mapping(DDRC_BASE, cs_num, size_mb, mr8_value);

    ddr_dfc_table_init(0xF0000000);
    init_table_mc_a0(0xF0000000);

    // TODO
    let mut info_para = 0;

    top_training_fp_all(DDRC_BASE, cs_num, 0, &mut info_para);

    panic!("TODO");

    let fp = 1;
    ddr_dfc(DDRC_BASE, fp);
    top_training_fp_all(DDRC_BASE, cs_num, fp, &mut info_para);

    let fp = 2;
    ddr_dfc(DDRC_BASE, fp);
    top_training_fp_all(DDRC_BASE, cs_num, fp, &mut info_para);

    let data_rate = 2400;
    /* change dram frequency */
    match data_rate {
        1600 => {
            ddr_dfc(DDRC_BASE, 1);
        }
        // WE HIT THIS
        2400 => {
            ddr_dfc(DDRC_BASE, 2);
        }
        1200 | _ => {
            // data_rate = 1200;
            ddr_dfc(DDRC_BASE, 0);
        }
    }
    // lpddr4_silicon_init(DDRC_BASE, DATA_RATE);
}
