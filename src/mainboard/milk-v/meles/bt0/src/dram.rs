// board/thead/light-c910/lpddr4/src/init_ddr.c

use bitfield::bitfield;

use crate::dram_helpers::{
    ddr_phy0_reg_wr, ddr_phy1_reg_rd, ddr_phy1_reg_wr, ddr_phy_broadcast_en, ddr_phy_reg_rd,
    ddr_phy_reg_wr,
};
use crate::dram_train::lp4_phy_train1d2d;
use crate::dram_training_data::{MISC_REG_LIST, RET_REG_LIST_ADDR};
use crate::util::{read32, write32};

const FREQ: u16 = 3733;
const DDR_BIT_WIDTH: Bits = Bits::B64;
const RANK: u8 = 2;

pub const DDR_SYS_BASE: usize = 0xff_ff00_5000;
pub const DDR_CFG0: usize = DDR_SYS_BASE + 0x0000;
pub const DDR_CFG1: usize = DDR_SYS_BASE + 0x0004;

pub const _DDR_PHY_BADDR: usize = 0xff_fd00_0000;
pub const _DDR_PHY1_BADDR: usize = _DDR_PHY_BADDR + 0x0100_0000;

const _DDR_CTRL_BADDR: usize = _DDR_PHY_BADDR + 0x0200_0000;
const DBG1: usize = _DDR_CTRL_BADDR + 0x0304;
const STAT: usize = _DDR_CTRL_BADDR + 0x0004;
const MSTR: usize = _DDR_CTRL_BADDR + 0x0000;
const MRCTRL0: usize = _DDR_CTRL_BADDR + 0x10;
const MRCTRL1: usize = _DDR_CTRL_BADDR + 0x14;
const DERATEEN: usize = _DDR_CTRL_BADDR + 0x20;
const DERATEINT: usize = _DDR_CTRL_BADDR + 0x24;
const DERATECTL: usize = _DDR_CTRL_BADDR + 0x2c;
const PWRCTL: usize = _DDR_CTRL_BADDR + 0x30;
const PWRTMG: usize = _DDR_CTRL_BADDR + 0x34;
const HWLPCTL: usize = _DDR_CTRL_BADDR + 0x38;
const RFSHCTL0: usize = _DDR_CTRL_BADDR + 0x50;
const RFSHCTL1: usize = _DDR_CTRL_BADDR + 0x54;
const RFSHCTL3: usize = _DDR_CTRL_BADDR + 0x60;
const RFSHTMG: usize = _DDR_CTRL_BADDR + 0x64;
const RFSHTMG1: usize = _DDR_CTRL_BADDR + 0x68;
const CRCPARCTL0: usize = _DDR_CTRL_BADDR + 0xc0;
const CRCPARSTAT: usize = _DDR_CTRL_BADDR + 0xcc;
const INIT0: usize = _DDR_CTRL_BADDR + 0xd0;
const INIT1: usize = _DDR_CTRL_BADDR + 0xd4;
const INIT2: usize = _DDR_CTRL_BADDR + 0xd8;
const INIT3: usize = _DDR_CTRL_BADDR + 0xdc;
const INIT4: usize = _DDR_CTRL_BADDR + 0xe0;
const INIT5: usize = _DDR_CTRL_BADDR + 0xe4;
const INIT6: usize = _DDR_CTRL_BADDR + 0xe8;
const INIT7: usize = _DDR_CTRL_BADDR + 0xec;
const DIMMCTL: usize = _DDR_CTRL_BADDR + 0xf0;
const RANKCTL: usize = _DDR_CTRL_BADDR + 0xf4;
const RANKCTL1: usize = _DDR_CTRL_BADDR + 0xf8;
const DRAMTMG0: usize = _DDR_CTRL_BADDR + 0x100;
const DRAMTMG1: usize = _DDR_CTRL_BADDR + 0x104;
const DRAMTMG2: usize = _DDR_CTRL_BADDR + 0x108;
const DRAMTMG3: usize = _DDR_CTRL_BADDR + 0x10c;
const DRAMTMG4: usize = _DDR_CTRL_BADDR + 0x110;
const DRAMTMG5: usize = _DDR_CTRL_BADDR + 0x114;
const DRAMTMG6: usize = _DDR_CTRL_BADDR + 0x118;
const DRAMTMG7: usize = _DDR_CTRL_BADDR + 0x11c;
const DRAMTMG8: usize = _DDR_CTRL_BADDR + 0x120;
const DRAMTMG12: usize = _DDR_CTRL_BADDR + 0x130;
const DRAMTMG13: usize = _DDR_CTRL_BADDR + 0x134;
const DRAMTMG14: usize = _DDR_CTRL_BADDR + 0x138;
const DRAMTMG17: usize = _DDR_CTRL_BADDR + 0x144;
const ZQCTL0: usize = _DDR_CTRL_BADDR + 0x180;
const ZQCTL1: usize = _DDR_CTRL_BADDR + 0x184;
const ZQCTL2: usize = _DDR_CTRL_BADDR + 0x188;
const DFITMG0: usize = _DDR_CTRL_BADDR + 0x190;
const DFITMG1: usize = _DDR_CTRL_BADDR + 0x194;
const DFILPCFG0: usize = _DDR_CTRL_BADDR + 0x198;
const DFIMISC: usize = _DDR_CTRL_BADDR + 0x1b0;
const DFITMG2: usize = _DDR_CTRL_BADDR + 0x1b4;
const DBICTL: usize = _DDR_CTRL_BADDR + 0x1c0;
const DFIPHYMSTR: usize = _DDR_CTRL_BADDR + 0x1c4;
const ADDRMAP0: usize = _DDR_CTRL_BADDR + 0x200;
const ADDRMAP1: usize = _DDR_CTRL_BADDR + 0x204;
const ADDRMAP2: usize = _DDR_CTRL_BADDR + 0x208;
const ADDRMAP3: usize = _DDR_CTRL_BADDR + 0x20c;
const ADDRMAP4: usize = _DDR_CTRL_BADDR + 0x210;
const ADDRMAP5: usize = _DDR_CTRL_BADDR + 0x214;
const ADDRMAP6: usize = _DDR_CTRL_BADDR + 0x218;
const ADDRMAP7: usize = _DDR_CTRL_BADDR + 0x21c;
const ADDRMAP8: usize = _DDR_CTRL_BADDR + 0x220;
const ADDRMAP9: usize = _DDR_CTRL_BADDR + 0x224;
const ADDRMAP10: usize = _DDR_CTRL_BADDR + 0x228;
const ADDRMAP11: usize = _DDR_CTRL_BADDR + 0x22c;
const ODTCFG: usize = _DDR_CTRL_BADDR + 0x240;
const DFIUPD0: usize = _DDR_CTRL_BADDR + 0x1a0;
const DFIUPD1: usize = _DDR_CTRL_BADDR + 0x1a4;
const DFIUPD2: usize = _DDR_CTRL_BADDR + 0x1a8;
const DFISTAT: usize = _DDR_CTRL_BADDR + 0x1bc;
const ODTMAP: usize = _DDR_CTRL_BADDR + 0x244;
const SCHED: usize = _DDR_CTRL_BADDR + 0x250;
const SCHED1: usize = _DDR_CTRL_BADDR + 0x254;
const PERFHPR1: usize = _DDR_CTRL_BADDR + 0x25c;
const PERFLPR1: usize = _DDR_CTRL_BADDR + 0x264;
const PERFWR1: usize = _DDR_CTRL_BADDR + 0x26c;
const SCHED3: usize = _DDR_CTRL_BADDR + 0x270;
const SCHED4: usize = _DDR_CTRL_BADDR + 0x274;
const DBG0: usize = _DDR_CTRL_BADDR + 0x300;
const DBGCAM: usize = _DDR_CTRL_BADDR + 0x308;
const DBGCMD: usize = _DDR_CTRL_BADDR + 0x30c;
const DBGSTAT: usize = _DDR_CTRL_BADDR + 0x310;
const SWCTL: usize = _DDR_CTRL_BADDR + 0x320;
const SWSTAT: usize = _DDR_CTRL_BADDR + 0x324;
const SWCTLSTATIC: usize = _DDR_CTRL_BADDR + 0x328;
const POISONCFG: usize = _DDR_CTRL_BADDR + 0x36c;
const POISONSTAT: usize = _DDR_CTRL_BADDR + 0x370;
const DERATESTAT: usize = _DDR_CTRL_BADDR + 0x3f0;
const PSTAT: usize = _DDR_CTRL_BADDR + 0x3fc;
const PCCFG: usize = _DDR_CTRL_BADDR + 0x400;
const PCFGR_0: usize = _DDR_CTRL_BADDR + 0x404;
const PCFGW_0: usize = _DDR_CTRL_BADDR + 0x408;
const PCTRL_0: usize = _DDR_CTRL_BADDR + 0x490;
const PCFGQOS0_0: usize = _DDR_CTRL_BADDR + 0x494;
const PCFGQOS1_0: usize = _DDR_CTRL_BADDR + 0x498;
const PCFGWQOS0_0: usize = _DDR_CTRL_BADDR + 0x49c;
const PCFGWQOS1_0: usize = _DDR_CTRL_BADDR + 0x4a0;
const PCFGR_1: usize = _DDR_CTRL_BADDR + 0x4b4;
const PCFGW_1: usize = _DDR_CTRL_BADDR + 0x4b8;
const PCTRL_1: usize = _DDR_CTRL_BADDR + 0x540;
const PCFGQOS0_1: usize = _DDR_CTRL_BADDR + 0x544;
const PCFGQOS1_1: usize = _DDR_CTRL_BADDR + 0x548;
const PCFGWQOS0_1: usize = _DDR_CTRL_BADDR + 0x54c;
const PCFGWQOS1_1: usize = _DDR_CTRL_BADDR + 0x550;
const PCFGR_2: usize = _DDR_CTRL_BADDR + 0x564;
const PCFGW_2: usize = _DDR_CTRL_BADDR + 0x568;
const PCTRL_2: usize = _DDR_CTRL_BADDR + 0x5f0;
const PCFGQOS0_2: usize = _DDR_CTRL_BADDR + 0x5f4;
const PCFGQOS1_2: usize = _DDR_CTRL_BADDR + 0x5f8;
const PCFGWQOS0_2: usize = _DDR_CTRL_BADDR + 0x5fc;
const PCFGWQOS1_2: usize = _DDR_CTRL_BADDR + 0x600;
const PCFGR_3: usize = _DDR_CTRL_BADDR + 0x614;
const PCFGW_3: usize = _DDR_CTRL_BADDR + 0x618;
const PCTRL_3: usize = _DDR_CTRL_BADDR + 0x6a0;
const PCFGR_4: usize = _DDR_CTRL_BADDR + 0x6c4;
const PCFGW_4: usize = _DDR_CTRL_BADDR + 0x6c8;
const PCTRL_4: usize = _DDR_CTRL_BADDR + 0x750;
const DCH1_MRCTRL0: usize = _DDR_CTRL_BADDR + 0x1b10;
const DCH1_MRCTRL1: usize = _DDR_CTRL_BADDR + 0x1b14;
const DCH1_DERATECTL: usize = _DDR_CTRL_BADDR + 0x1b2c;
const DCH1_PWRCTL: usize = _DDR_CTRL_BADDR + 0x1b30;
const DCH1_HWLPCTL: usize = _DDR_CTRL_BADDR + 0x1b38;
const DCH1_CRCPARCTL0: usize = _DDR_CTRL_BADDR + 0x1bc0;
const DCH1_ZQCTL2: usize = _DDR_CTRL_BADDR + 0x1c88;
const DCH1_ODTMAP: usize = _DDR_CTRL_BADDR + 0x1d44;
const DCH1_DBG1: usize = _DDR_CTRL_BADDR + 0x1e04;
const DCH1_DBGCMD: usize = _DDR_CTRL_BADDR + 0x1e0c;
const DCH1_DFISTAT: usize = _DDR_CTRL_BADDR + 0x1cbc;
const DCH1_STAT: usize = _DDR_CTRL_BADDR + 0x1b04;

// clock configs
const LIGHT_AUDIO_SUBSYS_ADDRBASE: usize = 0xff_cb00_0000;
const LIGHT_VO_SUBSYS_R_ADDRBASE: usize = 0xff_ef52_8000;
const LIGHT_APCLK_ADDRBASE: usize = 0xff_ff01_1000;
const LIGHT_APSYS_RSTGEN_ADDRBASE: usize = 0xff_ff01_5000;
const LIGHT_VO_SUBSYS_ADDRBASE: usize = 0xff_ff40_1000;
const LIGHT_DSP_SUBSYS_ADDRBASE: usize = 0xff_ff04_1000;
const LIGHT_AONCLK_ADDRBASE: usize = 0xff_fff4_6000;

pub fn init() {
    sys_clk_config();
    lpddr4_init(RANK, FREQ, DDR_BIT_WIDTH);
}

// roughly wait a microsecond, maybe better read from mcycle
// FIXME: Use the timer, i.e., mtimer from CLINT or mtime if available.
fn udelay(micros: usize) {
    unsafe {
        for _ in 0..(micros * 5) {
            core::arch::asm!("nop")
        }
    }
}

// board/thead/light-c910/sys_clk.c
fn sys_clk_config() {
    println!("[*] Starting sys_clk_config() ...");
    let mut tmp;
    udelay(60);
    /* 1. double check all pll lock */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x80);
    if tmp & 0x3fe != 0x3fe {
        // FIXME: fix this error handling
        println!("[bt0] pll lock check failed: {}", tmp);
        return;
    }
    /* 2. update sys_pll to frac mode, 2438.5536MHz */
    /* switch share_sram_clk to audio_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x104);
    tmp |= 0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x104, tmp);

    udelay(1);

    /* set sys_pll_foutvco to 2438.5536MHz */
    write32(LIGHT_AONCLK_ADDRBASE + 0x14, 0x20000000);
    write32(LIGHT_AONCLK_ADDRBASE + 0x10, 0x03606501);
    write32(LIGHT_AONCLK_ADDRBASE + 0x14, 0x209b3d08);
    udelay(3);
    write32(LIGHT_AONCLK_ADDRBASE + 0x14, 0x009b3d08);
    read32(LIGHT_AONCLK_ADDRBASE + 0x90);
    read32(LIGHT_AONCLK_ADDRBASE + 0x90);
    while read32(LIGHT_AONCLK_ADDRBASE + 0x90) & 0x2 == 0 {}
    udelay(11);

    /* switch share_sram_clk to sys_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x104);
    tmp &= !0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x104, tmp);

    /* set apb3_cpusys_pclk to ahb2_cpusys_hclk/2 */
    /* CPU AHB 125MHz  CPU pclk 125MHz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x130);
    tmp &= !0x8;
    write32(LIGHT_APCLK_ADDRBASE + 0x130, tmp);
    udelay(1);
    tmp &= !0x7;
    tmp |= 0x1;
    write32(LIGHT_APCLK_ADDRBASE + 0x130, tmp);
    udelay(1);
    tmp |= 0x8;
    write32(LIGHT_APCLK_ADDRBASE + 0x130, tmp);
    udelay(1);
    /* CPU AHB 125MHz  CPU pclk 62.5MHz */

    /* set ahb2_cpusys_hclk to 250Mhz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x120);
    tmp |= 0x20;
    write32(LIGHT_APCLK_ADDRBASE + 0x120, tmp);
    udelay(1);
    tmp &= !0x10;
    write32(LIGHT_APCLK_ADDRBASE + 0x120, tmp);
    udelay(1);
    tmp &= !0x7;
    tmp |= 0x2;
    write32(LIGHT_APCLK_ADDRBASE + 0x120, tmp);
    udelay(1);
    tmp |= 0x10;
    write32(LIGHT_APCLK_ADDRBASE + 0x120, tmp);
    udelay(1);
    tmp &= !0x20;
    write32(LIGHT_APCLK_ADDRBASE + 0x120, tmp);
    udelay(1);
    /* CPU AHB 250MHz  CPU pclk 125MHz */

    /* perisys_apb_pclk to perisys_ahb_hclk/4 */
    /* perisys_ahb_hclk 62.5MHz  perisys_apb_pclk 62.5MHz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x150);
    tmp &= !0x8;
    write32(LIGHT_APCLK_ADDRBASE + 0x150, tmp);
    udelay(1);
    tmp &= !0x7;
    tmp |= 0x3;
    write32(LIGHT_APCLK_ADDRBASE + 0x150, tmp);
    udelay(1);
    tmp |= 0x8;
    write32(LIGHT_APCLK_ADDRBASE + 0x150, tmp);
    udelay(1);
    /* perisys_ahb_hclk 62.5MHz  perisys_apb_pclk 15.625MHz */

    /* perisys_ahb_hclk to 250MHz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x140);
    tmp |= 0x20;
    write32(LIGHT_APCLK_ADDRBASE + 0x140, tmp);
    udelay(1);
    /* perisys_ahb_hclk 24MHz  perisys_apb_pclk 6MHz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x140);
    tmp &= !0x10;
    write32(LIGHT_APCLK_ADDRBASE + 0x140, tmp);
    udelay(1);
    tmp &= !0xf;
    tmp |= 0x2;
    write32(LIGHT_APCLK_ADDRBASE + 0x140, tmp);
    /* perisys_ahb_hclk 24MHz  perisys_apb_pclk 6MHz */
    udelay(1);
    tmp |= 0x10;
    write32(LIGHT_APCLK_ADDRBASE + 0x140, tmp);
    udelay(1);
    tmp &= !0x20;
    write32(LIGHT_APCLK_ADDRBASE + 0x140, tmp);
    /* The boards other than the LightA board perform the bus down-speed operation */
}

fn lpddr4_init(rank: u8, freq: u16, bits: Bits) {
    println!("[*] LPDDR4 init...");
    pll_config(freq);
    deassert_pwrok_apb(bits.clone());
    ctrl_init(rank, freq);
    println!("[+] ctrl_init Complete...");
    addrmap(rank, bits.clone());
    println!("[+] addrmap Complete...");

    // adjust_ddr_addrmap(type, rank_num, speed, bits, size);
    // msic regu restore for str
    // dwc_ddr_misc_regu_save();

    // after this step, only PwrOk is still low
    de_assert_other_reset_ddr();
    println!("[+] de_asssert_other_reset_ddr Complete...");
    dq_pinmux(bits.clone()); // pinmux config before training
    println!("[+] dq_pinmux Complete...");
    lp4_phy_train1d2d(freq, bits.clone());
    println!("[+] lp4_phy_train1d2d Complete...");
    dwc_ddrphy_phyinit_reg_interface(RegInstr::SaveRegs);
    println!("[+] dwc_ddrphy_phyinit_reg_interface Complete...");
    ctrl_en(bits.clone());
    println!("[+] ctrl_en Complete...");
    enable_axi_port(0x1f);
    println!("[+] enable_axi_port Complete...");
    enable_auto_refresh();
    println!("[+] enable_auto_refresh Complete...");
    lpddr4_auto_selref();
    println!("[+] enable_auto_selref Complete...");
}

const DDR_CFG_FREQ: usize = DDR_SYS_BASE + 0x8;
const DDR_CFG_CTRL: usize = DDR_SYS_BASE + 0xc;

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn pll_config(speed: u16) {
    let freq_val = 0x0120_4d01;
    println!("[+] pll_config freq {speed} (0x{freq_val:08x})");
    write32(DDR_CFG_CTRL, 0x4b000000);
    write32(DDR_CFG_FREQ, freq_val);
    udelay(2);
    write32(DDR_CFG_CTRL, 0x0b000000);

    let f = read32(DDR_CFG_FREQ);
    let ok = if f == freq_val { "ok" } else { "meh" };
    println!("DDR controller freq: 0x{f:08x} {ok}");
    // PLL lock
    while read32(DDR_CFG0 + 0x18) & 1 != 0x1 {}
    // core clock cg off
    write32(DDR_CFG0 + 0x18, 0x10000);
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn deassert_pwrok_apb(bits: Bits) {
    println!("[+] deassert pwrok apb");
    // release PwrOkIn
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);

    // release Phyrst
    write32(DDR_CFG0, 0xc0);
    write32(DDR_CFG0, 0xc0);
    write32(DDR_CFG0, 0xc0);
    write32(DDR_CFG0, 0xc0);

    // release apb presetn
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    if bits == Bits::B32 {
        write32(DDR_CFG0, 0xd2);
    }
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn ctrl_init(rank: u8, freq: u16) {
    println!("[*] CTRL Init ranks {rank}, frequency {freq}");
    write32(DBG1, 0x00000001);
    write32(PWRCTL, 0x00000001);
    while (read32(STAT) != 0x00000000) {}
    if (rank == 2) {
        write32(MSTR, 0x03080020);
    } else {
        write32(MSTR, 0x01080020);
    }
    write32(MRCTRL0, 0x00003030);
    write32(MRCTRL1, 0x0002d90f);

    if (freq == 3733) {
        write32(DERATEEN, 0x000013f3);
        write32(DERATEINT, 0x40000000);
        write32(DERATECTL, 0x00000001);
        write32(PWRCTL, 0x00000020);
        write32(PWRTMG, 0x0040ae04);
        write32(HWLPCTL, 0x00430000);

        write32(RFSHCTL0, 0x00210004);
        write32(RFSHCTL1, 0x000d0021);
        write32(RFSHCTL3, 0x00000001);
        if false {
            // tREFI=0x6F*32*1.083=3.846us
            // trfcpb 280ns         :0x106
            // trfcpb 380ns/1.083ns= 0x164
            write32(RFSHTMG, 0x006f8164);
        } else {
            // [31]    t_rfc_nom_x1_sel ,1
            // [27:16] -tREFI 488ns/1.083=450=0x1c2
            // [9:0]   -t_rfc_min 140ns/1.083=130=0x82
            write32(RFSHTMG, 0x81c00084);
        }
        write32(RFSHTMG1, 0x00540000);
        // We had this; where did it come from?
        // write32(RFSHTMG1, 0x00480000);

        write32(CRCPARCTL0, 0x00000000);
        write32(INIT0, 0xc0020002);
        write32(INIT1, 0x00010002);
        write32(INIT2, 0x00001f00);
        // OP[2:0] RL
        write32(INIT3, 0x00640036);
        // [31:16] LP4 MR3
        write32(INIT4, 0x00f20008);
        write32(INIT5, 0x0004000b);
        write32(INIT6, 0x00000012);
        write32(INIT7, 0x0000001a);

        write32(DIMMCTL, 0x00000000);
        write32(RANKCTL, 0x0000ab9f);
        write32(RANKCTL1, 0x00000017);

        write32(DRAMTMG0, 0x1f263f28);
        write32(DRAMTMG1, 0x00080839);
        write32(DRAMTMG2, 0x08121d17);

        write32(DRAMTMG3, 0x00d0e000);
        write32(DRAMTMG4, 0x11040a12);
        write32(DRAMTMG5, 0x02050e0e);
        write32(DRAMTMG6, 0x01010008);
        write32(DRAMTMG7, 0x00000502);
        write32(DRAMTMG8, 0x00000101);
        write32(DRAMTMG12, 0x00020000);
        write32(DRAMTMG13, 0x0d100002);
        write32(DRAMTMG14, 0x0000010c);

        write32(ZQCTL0, 0x03a50021);
        write32(ZQCTL1, 0x02f00800);
        write32(ZQCTL2, 0x00000000);

        write32(DFITMG0, 0x059f820c);
        write32(DFITMG1, 0x000c0303);
        write32(DFILPCFG0, 0x0351a101);
        write32(DFIMISC, 0x00000011);
        write32(DFITMG2, 0x00001f0c);
        write32(DBICTL, 0x00000007);
        write32(DFIPHYMSTR, 0x14000001);

        write32(ADDRMAP0, 0x0002001f);
        write32(ADDRMAP1, 0x00090909);
        write32(ADDRMAP2, 0x01010000);
        write32(ADDRMAP3, 0x01010101);
        write32(ADDRMAP4, 0x00001f1f);
        write32(ADDRMAP5, 0x080f0808);
        write32(ADDRMAP6, 0x08080808);
        write32(ADDRMAP7, 0x00000f0f);
        write32(ADDRMAP9, 0x08080808);
        write32(ADDRMAP10, 0x08080808);
        write32(ADDRMAP11, 0x00000008);
        write32(ODTCFG, 0x06090b40);
    }
    write32(DFIUPD0, 0x00400018); //[31:30]=0 use ctrlupd enable
    write32(DFIUPD1, 0x00280032); // less ctrl interval
    write32(DFIUPD2, 0x00000000); //[31]=0 disable phy ctrlupdate
    write32(ODTMAP, 0x00000000);
    write32(SCHED, 0x1f829b1c); //[2]  page-close enable [14:8] 0x1b: lpr entry num=28, hpr entry num=4
    write32(SCHED1, 0x4400b00f); //[7:0] page-close timer
    write32(PERFHPR1, 0x0f000001);
    write32(PERFLPR1, 0x0f00007f);
    write32(PERFWR1, 0x0f00007f);
    write32(SCHED3, 0x00000208);
    write32(SCHED4, 0x08400810);
    write32(DBG0, 0x00000000);
    write32(DBG1, 0x00000000);
    write32(DBGCMD, 0x00000000);
    write32(SWCTL, 0x00000001);
    write32(SWCTLSTATIC, 0x00000000);
    write32(POISONCFG, 0x00000001);
    write32(PCTRL_0, 0x00000001);
    write32(PCTRL_1, 0x00000001);
    write32(PCTRL_2, 0x00000001);
    write32(PCTRL_3, 0x00000001);
    write32(PCTRL_4, 0x00000001);
    write32(DCH1_MRCTRL0, 0x00003030);
    write32(DCH1_MRCTRL1, 0x0002d90f);
    write32(DCH1_DERATECTL, 0x00000001);
    write32(DCH1_PWRCTL, 0x00000020);
    write32(DCH1_HWLPCTL, 0x00430002);
    write32(DCH1_CRCPARCTL0, 0x00000000);
    write32(DCH1_ZQCTL2, 0x00000000);
    write32(DCH1_ODTMAP, 0x00000000);
    write32(DCH1_DBG1, 0x00000000);
    write32(DCH1_DBGCMD, 0x00000000);
    while (read32(RFSHCTL3) != 0x00000001) {}

    // update by perf sim
    // [4] page match limit, limits the number of consecutive same page DDRC
    // transactions that can be granted by the Port Arbiter to four
    write32(PCCFG, 0x00000010);
    write32(PCFGR_0, 0x0000500f); //CPU read
    write32(PCFGW_0, 0x0000500f); //CPU write
    write32(PCFGR_1, 0x00005020); //VI Read   max 32
    write32(PCFGW_1, 0x0000501f); //VI Write, sensor/isp/dw/dsp
    write32(PCFGR_2, 0x0000501f); //VO Read, DPU/GPU
    write32(PCFGW_2, 0x0000503f); //VO Write, GPU
    write32(PCFGR_3, 0x000051ff);
    write32(PCFGW_3, 0x000051ff);
    write32(PCFGR_4, 0x0000503f);
    write32(PCFGW_4, 0x0000503f);

    while (read32(PWRCTL) != 0x00000020) {}
    write32(PWRCTL, 0x00000020);
    while (read32(DCH1_PWRCTL) != 0x00000020) {}
    write32(DCH1_PWRCTL, 0x00000020);
    write32(DBG1, 0x00000000);
    while (read32(PWRCTL) != 0x00000020) {}
    write32(PWRCTL, 0x00000020);
    while (read32(PWRCTL) != 0x00000020) {}
    write32(PWRCTL, 0x00000020);
    write32(DCH1_DBG1, 0x00000000);
    while (read32(DCH1_PWRCTL) != 0x00000020) {}
    write32(DCH1_PWRCTL, 0x00000020);
    while (read32(DCH1_PWRCTL) != 0x00000020) {}
    write32(DCH1_PWRCTL, 0x00000020);

    write32(DFIPHYMSTR, 0x14000001);
    write32(SWCTL, 0x00000000);
    write32(DFIMISC, 0x00000010);
    write32(DFIMISC, 0x00000010);
    write32(DBG1, 0x00000002);
    write32(DCH1_DBG1, 0x00000002);

    // just printing the values from above to recheck
    println!("[*] ctrl_init check");
    println!("[+] RankCTL       : {:08x}", read32(RANKCTL));
    println!("[+] DRAM timing 2 : {:08x}", read32(DRAMTMG2));
    println!("[+] DFI timing 0  : {:08x}", read32(DFITMG0));
    println!("[+] DFI timing 1  : {:08x}", read32(DFITMG1));
    println!("[+] DRAM timing 4 : {:08x}", read32(DRAMTMG4)); //[19:16] tCCD
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn addrmap(rank: u8, bits: Bits) {
    // NOTE: This is hardcoded for now.
    println!("DDR 64bit mode, 256B interleaving");
    // cs_bit0: NULL
    write32(ADDRMAP0, 0x0004001f);
    // 8GB
    write32(ADDRMAP0, 0x00040018);
    // bank +2
    write32(ADDRMAP1, 0x00090909);
    // col b5+5 ~ col b2  +2
    write32(ADDRMAP2, 0x00000000);
    // col b9 ~ col b6
    write32(ADDRMAP3, 0x01010101);
    // col b11~ col b10
    write32(ADDRMAP4, 0x00001f1f);
    // row_b11 row b2_10 row b1 row b0  +6
    write32(ADDRMAP5, 0x080f0808);
    // row15
    write32(ADDRMAP6, 0x08080808);
    // row16: NULL
    write32(ADDRMAP7, 0x00000f0f);
    write32(ADDRMAP9, 0x08080808);
    write32(ADDRMAP10, 0x08080808);
    write32(ADDRMAP11, 0x00000008);
}

// struct corresponding to `DDR_SYSREG_REG_SW_DDR_CFG0_U`
bitfield! {
    pub struct DDRCfg0(u32);
    impl Debug;
    pub rg_ctl_ddr_usw_rst_reg, set_rg_ctl_ddr_usw_rst_reg: 31, 4; // [31:4] range
    pub rg_ddrc_32en, set_rg_ddrc_32en: 1;
    pub rg_broadcast_mode, set_rg_broadcast_mode: 0;
}

// union `DDR_SYSREG_REG_SW_REG_S` as a struct
#[repr(C, packed)]
pub struct DDRSysReg {
    pub ddr_sysreg_registers_struct_ddr_cfg0: DDRCfg0, // 0x0
}

static mut DDR_SYSREG: DDRSysReg = DDRSysReg {
    ddr_sysreg_registers_struct_ddr_cfg0: DDRCfg0(0),
};

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
// de_assert umctl2_reset, phy_crst, and all areset
fn de_assert_other_reset_ddr() {
    println!("de-assert areset and ctrl_crst_n, ddr_phy_crst_n by sysreg or tb");
    let cfg0 = read32(DDR_CFG0);
    println!("[<-] ddr_sysreg_cfg0: {cfg0:08x}");
    write32(DDR_CFG0, cfg0 | (0x1fa << 4));
    let cfg0 = read32(DDR_CFG0);
    println!("[->] ddr_sysreg_cfg0: {cfg0:08x}");
}

// board/thead/light-c910/lpddr4/src/pinmux.c
// pinmux config before training
fn dq_pinmux(bits: Bits) {
    // ddr_phy_broadcast_en(0);
    ddr_phy0_reg_wr(0x100a0, 0x1);
    ddr_phy0_reg_wr(0x100a1, 0x5);
    ddr_phy0_reg_wr(0x100a2, 0x3);
    ddr_phy0_reg_wr(0x100a3, 0x0);
    ddr_phy0_reg_wr(0x100a4, 0x2);
    ddr_phy0_reg_wr(0x100a5, 0x4);
    ddr_phy0_reg_wr(0x100a6, 0x6);
    ddr_phy0_reg_wr(0x100a7, 0x7);
    //PHY0 DBYTE1
    ddr_phy0_reg_wr(0x110a0, 0x7);
    ddr_phy0_reg_wr(0x110a1, 0x4);
    ddr_phy0_reg_wr(0x110a2, 0x3);
    ddr_phy0_reg_wr(0x110a3, 0x0);
    ddr_phy0_reg_wr(0x110a4, 0x2);
    ddr_phy0_reg_wr(0x110a5, 0x1);
    ddr_phy0_reg_wr(0x110a6, 0x5);
    ddr_phy0_reg_wr(0x110a7, 0x6);
    //PHY0 DBYTE2
    ddr_phy0_reg_wr(0x120a0, 0x7);
    ddr_phy0_reg_wr(0x120a1, 0x4);
    ddr_phy0_reg_wr(0x120a2, 0x3);
    ddr_phy0_reg_wr(0x120a3, 0x0);
    ddr_phy0_reg_wr(0x120a4, 0x2); // FullMask version
    ddr_phy0_reg_wr(0x120a5, 0x1); // FullMask version
    ddr_phy0_reg_wr(0x120a6, 0x5);
    ddr_phy0_reg_wr(0x120a7, 0x6);
    //PHY0 DBYTE3
    ddr_phy0_reg_wr(0x130a0, 0x7);
    ddr_phy0_reg_wr(0x130a1, 0x5);
    ddr_phy0_reg_wr(0x130a2, 0x0);
    ddr_phy0_reg_wr(0x130a3, 0x2);
    ddr_phy0_reg_wr(0x130a4, 0x1);
    ddr_phy0_reg_wr(0x130a5, 0x4);
    ddr_phy0_reg_wr(0x130a6, 0x3);
    ddr_phy0_reg_wr(0x130a7, 0x6);

    if bits == Bits::B64 {
        //PHY1 DBYTE0
        ddr_phy1_reg_wr(0x100a0, 0x7);
        ddr_phy1_reg_wr(0x100a1, 0x4);
        ddr_phy1_reg_wr(0x100a2, 0x3);
        ddr_phy1_reg_wr(0x100a3, 0x0);
        ddr_phy1_reg_wr(0x100a4, 0x1);
        ddr_phy1_reg_wr(0x100a5, 0x2);
        ddr_phy1_reg_wr(0x100a6, 0x5);
        ddr_phy1_reg_wr(0x100a7, 0x6);
        //PHY1 DBYTE1
        ddr_phy1_reg_wr(0x110a0, 0x7);
        ddr_phy1_reg_wr(0x110a1, 0x5);
        ddr_phy1_reg_wr(0x110a2, 0x0);
        ddr_phy1_reg_wr(0x110a3, 0x2);
        ddr_phy1_reg_wr(0x110a4, 0x1);
        ddr_phy1_reg_wr(0x110a5, 0x4);
        ddr_phy1_reg_wr(0x110a6, 0x3);
        ddr_phy1_reg_wr(0x110a7, 0x6);
        //PHY1 DBYTE2
        ddr_phy1_reg_wr(0x120a0, 0x1);
        ddr_phy1_reg_wr(0x120a1, 0x5);
        ddr_phy1_reg_wr(0x120a2, 0x3);
        ddr_phy1_reg_wr(0x120a3, 0x0);
        ddr_phy1_reg_wr(0x120a4, 0x2);
        ddr_phy1_reg_wr(0x120a5, 0x4);
        ddr_phy1_reg_wr(0x120a6, 0x6);
        ddr_phy1_reg_wr(0x120a7, 0x7);
        //PHY1 DBYTE3
        ddr_phy1_reg_wr(0x130a0, 0x7);
        ddr_phy1_reg_wr(0x130a1, 0x4);
        ddr_phy1_reg_wr(0x130a2, 0x3);
        ddr_phy1_reg_wr(0x130a3, 0x0);
        ddr_phy1_reg_wr(0x130a4, 0x2);
        ddr_phy1_reg_wr(0x130a5, 0x1);
        ddr_phy1_reg_wr(0x130a6, 0x5);
        ddr_phy1_reg_wr(0x130a7, 0x6);
        ddr_phy_broadcast_en(1);
    }
}

enum RegInstr {
    SaveRegs,
    RestoreRegs,
}

#[derive(Default, Copy, Clone)]
struct RegPhyAddrVal {
    address: u32,
    value0: u16,
    value1: Option<u16>,
}

impl RegPhyAddrVal {
    fn new(address: u32) -> Self {
        RegPhyAddrVal {
            address,
            value0: 0,
            value1: None,
        }
    }
}

#[derive(Default)]
struct RegMiscAddrVal {
    address: u32,
    value: u16,
}

const PHY_REG_NUM: usize = RET_REG_LIST_ADDR.len();
const MISC_REG_NUM: usize = MISC_REG_LIST.len();

// board/thead/light-c910/lpddr4/src/ddr_retention.c
fn dwc_ddrphy_phyinit_reg_interface(instr: RegInstr) {
    ddr_phy_reg_wr(0xd0000, 0x0);
    ddr_phy_reg_wr(0xc0080, 0x3);
    let mut reg_vals: [RegPhyAddrVal; PHY_REG_NUM] = {
        // Temporary init with zeroed addresses
        let mut arr = [RegPhyAddrVal::new(0); PHY_REG_NUM];
        for (i, &addr) in RET_REG_LIST_ADDR.iter().enumerate() {
            // Assign each actual address
            arr[i] = RegPhyAddrVal::new(addr);
        }
        arr
    };

    let mut misc_reg_vals: [RegMiscAddrVal; MISC_REG_NUM] = Default::default();
    for (i, &addr) in MISC_REG_LIST.iter().enumerate() {
        misc_reg_vals[i].address = addr as u32;
        misc_reg_vals[i].value = ddr_phy_reg_rd(addr as usize);
    }

    match instr {
        RegInstr::SaveRegs => {
            for (i, &addr) in RET_REG_LIST_ADDR.iter().enumerate() {
                reg_vals[i].address = addr;
                reg_vals[i].value0 = ddr_phy_reg_rd(addr as usize);
                {
                    reg_vals[i].value1 = Some(ddr_phy1_reg_rd(addr as usize));
                }
            }
        }
        RegInstr::RestoreRegs => {
            // Restore values to registers
            ddr_phy_reg_wr(0x20089, 0x1);
            for reg in &reg_vals {
                ddr_phy_reg_wr(reg.address as usize, reg.value0);
                if let Some(value1) = reg.value1 {
                    ddr_phy1_reg_wr(reg.address as usize, value1);
                }
            }
        }
    }
    ddr_phy_reg_wr(0xc0080, 0x2);
    ddr_phy_reg_wr(0xd0000, 0x1);
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn ctrl_en(bits: Bits) {
    // write32(SWCTL, 0x00000000);

    // [5]dfi_init_start
    write32(DFIMISC, 0x00000030);

    // write32(SWCTL, 0x00000001);
    // while read32(SWSTAT) != 0x00000001;

    // polling dfi_init_complete
    while read32(DFISTAT) != 0x00000001 {}
    if bits == Bits::B64 {
        while read32(DCH1_DFISTAT) != 0x00000001 {}
    }
    // write32(SWCTL, 0x00000000);
    write32(DFIMISC, 0x00000010);
    write32(DFIMISC, 0x00000011);
    //[3] dfi_dram_clk_disable [1] powerdown_en
    write32(PWRCTL, 0x0000000a);
    write32(DCH1_PWRCTL, 0x0000000a);
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}
    while read32(STAT) != 0x00000001 {}
    if bits == Bits::B64 {
        while read32(DCH1_STAT) != 0x00000001 {}
    }
    write32(DFIPHYMSTR, 0x14000001);
    write32(SWCTL, 0x00000000);
    write32(INIT0, 0x00020002);
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}
    // write32(PWRCTL, 0x0000000b);
    // write32(DCH1_PWRCTL, 0x0000000b);

    // testing the values
    println!("DFIPHYMSTR: {:08x}", read32(DFIPHYMSTR));
    println!("DFIUPD0   : {:08x}", read32(DFIUPD0));
    println!("DFIUPD1   : {:08x}", read32(DFIUPD1));
    println!("ZQCTL0    : {:08x}", read32(ZQCTL0));
    println!("ADDRMAP0  : {:08x}", read32(ADDRMAP0));
    println!("ADDRMAP1  : {:08x}", read32(ADDRMAP1));
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn enable_axi_port(port: u8) {
    // Full bypass scramble
    // write32(0xff_ff00_4008, 0xff40_0000);
    // Full bypass scramble
    // write32(0xff_ff00_4008, 0xff40_0000);
    // axi rst->release
    write32(DDR_CFG0, 0x00f0);
    write32(DDR_CFG0, 0x1ff0);
    write32(DBG1, 0);
    write32(DCH1_DBG1, 0);
    if port & 0x1 != 0 {
        write32(PCTRL_0, 1);
    }
    if port & 0x2 != 0 {
        write32(PCTRL_1, 1);
    }
    if port & 0x4 != 0 {
        write32(PCTRL_2, 1);
    }
    if port & 0x8 != 0 {
        write32(PCTRL_3, 1);
    }
    if port & 0x10 != 0 {
        write32(PCTRL_4, 1);
    }
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn enable_auto_refresh() {
    write32(RFSHCTL3, 0);
}

fn lpddr4_auto_selref() {
    // remove core clock after xx
    write32(DDR_CFG1, 0xa0000);
    write32(SWCTL, 0);
    write32(SWCTLSTATIC, 1);
    write32(PWRTMG, 0x40ae04);
    write32(HWLPCTL, 0x430003);
    write32(DCH1_HWLPCTL, 0x430003);
    write32(SWCTL, 1);
    write32(SWCTLSTATIC, 0);

    //[3] dfi_dram_clk_disable [1] powerdown_en [0]serref_en
    write32(PWRCTL, 0x0000000b);
    write32(DCH1_PWRCTL, 0x0000000b);
}

fn disable_axi_port(port: u32) {
    if port & 0x1 != 0 {
        write32(PCTRL_0, 0);
    }
    if port & 0x2 != 0 {
        write32(PCTRL_1, 0);
    }
    if port & 0x4 != 0 {
        write32(PCTRL_2, 0);
    }
    if port & 0x8 != 0 {
        write32(PCTRL_3, 0);
    }
    if port & 0x10 != 0 {
        write32(PCTRL_4, 0);
    }

    while read32(PSTAT) != 0x0 {}
    if port & 0x1F == 0x1F {
        // all ports are disabled
        write32(DBG1, 2);
        write32(DCH1_DBG1, 2);
    } else {
        // at least one port is not disabled
        write32(DBG1, 0);
        write32(DCH1_DBG1, 0);
    }
}

const GB: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq)]
pub enum Bits {
    B32,
    B64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    S2G,
    S4G,
    S8G,
    S16G,
}

// TODO: other values
#[derive(Debug, Clone, PartialEq)]
enum DdrType {
    LPDDR4X = 0,
}

fn adjust_ddr_addrmap(ddr_type: DdrType, rank_num: u32, speed: u32, bits: Bits, size: Size) {
    // if Err(e) = lpddr4_query_boundary(type, rank_num, speed, bits, size) {
    //   return Err(e);
    // }

    // 32 bits: only phy0
    // 64 bits: phy0 + phy1
    match (bits.clone(), rank_num, size.clone()) {
        (Bits::B32, 2, Size::S4G) => {
            write32(ADDRMAP0, 0x001f0017); // cs_bit0: HIF[29]
            write32(ADDRMAP7, 0x00000f0f); // row16: NULL
        }
        (Bits::B32, 2, Size::S8G) => {
            write32(ADDRMAP0, 0x001f0018); // cs_bit0: HIF[30]
            write32(ADDRMAP7, 0x00000f07); // row16: HIF[29]
        }
        (Bits::B32, 1, Size::S2G) => {
            write32(ADDRMAP0, 0x001f001f); // cs_bit0: NULL
        }
        (Bits::B64, 1, Size::S8G) => {
            write32(ADDRMAP0, 0x00040018); // cs_bit0: HIF[30]
            write32(ADDRMAP7, 0x00000f0f); // row16: NULL
        }
        (Bits::B64, 1, Size::S16G) => {
            write32(ADDRMAP0, 0x00040019); // cs_bit0: HIF[31]
            write32(ADDRMAP7, 0x00000f08); // row16: HIF[30]
        }
        (Bits::B64, 1, Size::S4G) => {
            write32(ADDRMAP0, 0x0004001f); // cs_bit0: NULL
            write32(ADDRMAP7, 0x00000f0f); // row16: NULL
        }
        _ => {
            println!("adjust_ddr_addrmap: Invalid: bits {bits:?} ranks {rank_num} size {size:?}");
        }
    }
}

fn lpddr4_selfrefresh_exit(mode: u32) {
    // put sdram into idle state
    write32(PWRCTL, 0x0);
    write32(DCH1_PWRCTL, 0x0);

    // wait sdram exit selfrefresh sate,wait umctl2 back to normal
    while read32(STAT) & 0x307 != 1 {}
    if mode == 0 {
        // wait sdram exit selfrefresh sate, wait umctl2 back to normal
        while read32(DCH1_STAT) & 0x307 != 1 {}
    }
}

fn ctrl_en_lp3_exit(bits: Bits) {
    // skip DRAM init, because this has done
    write32(SWCTL, 0x00000000);
    write32(INIT0, 0xc0020002);
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}

    // dfi frequency change proto, to PS0
    write32(SWCTL, 0x00000000);
    write32(DFIMISC, 0x00000000); // [5]dfi_freq=0x0
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}

    write32(SWCTL, 0x00000000);
    write32(DFIMISC, 0x00000020); // [5]dfi_init_start=0x1
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}

    // poll dfi_init_complete
    while read32(DFISTAT) != 0x00000001 {}
    if bits == Bits::B64 {
        while read32(DCH1_DFISTAT) != 0x00000001 {}
    }
    write32(SWCTL, 0x00000000);
    write32(DFIMISC, 0x00000000);
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}

    write32(SWCTL, 0x00000000);
    write32(DFIMISC, 0x00000001);
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}

    // for low power,
    write32(SWCTL, 0x00000000);
    write32(PWRCTL, 0x0000000a); //[3] dfi_dram_clk_disable [1] powerdown_en
    write32(DCH1_PWRCTL, 0x0000000a);
    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}
    // detect until umctrl into normal state
    while read32(STAT) != 0x00000001 {}
    if bits == Bits::B64 {
        while read32(DCH1_STAT) != 0x00000001 {}
    }

    // en phy master proto
    write32(DFIPHYMSTR, 0x14000001);
}

fn dfi_freq_change(dfi_freq: u32, skip_dram_init: u32) {
    println!("dfi_freq_change");
    // write(DBG1, 3);
    write32(SWCTL, 0x00000000);

    let v = read32(INIT0);
    write32(INIT0, (v & !(0b11 << 30)) | (skip_dram_init << 30));

    // clear dfi_init_complete_en
    let v = read32(DFIMISC);
    write32(DFIMISC, (v & !(0b11111 << 8 | 0b1)) | dfi_freq << 8);

    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x00000001 {}

    write32(SWCTL, 0x00000000);

    // set dfi_init_start
    let v = read32(DFIMISC);
    write32(DFIMISC, (v & !(0b1 << 5)) | 1 << 5);

    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) != 0x1 {}

    // wait for dfi_init_complete == 0
    read32(DFISTAT);
    while read32(DFISTAT) & 0x1 != 0x0 {
        udelay(1_000_000);
    }

    // change dfi clk freq here
    write32(SWCTL, 0x00000000);

    // clear dfi_init_start
    let v = read32(DFIMISC);
    write32(DFIMISC, v & !(0b1 << 5));

    write32(SWCTL, 0x00000001);
    while read32(SWSTAT) == 0 {}

    // return;
    // wait for dfi_init_complete == 1
    read32(DFISTAT);
    while read32(DFISTAT) & 0x1 == 0x0 {}
}

const DEBUG_DFI: bool = true;

// pwden_en: enable selfrefresh power-down, otherwise stay in selfrefresh
fn lpddr4_enter_selfrefresh(pwdn_en: bool, dis_dram_clk: bool, mode: u32) {
    println!("lpddr4_enter_selfrefresh");
    let v = read32(PWRCTL);
    let selfref_sw = 1 << 5;
    let lpddr4_sr_allowed = 1 << 8;
    let m = !(lpddr4_sr_allowed | 1 << 6 | selfref_sw);
    // self refresh powerdown after enter self refresh or stay in self refresh
    let selfref = if pwdn_en { 0 } else { 1 };
    // turn off sdram clk when in self-refresh power-down state
    let nv = (v & m) | lpddr4_sr_allowed | selfref << 6 | selfref_sw;
    let nv = if dis_dram_clk {
        (nv & !(1 << 3)) | 1 << 3
    } else {
        nv
    };
    write32(PWRCTL, nv);
    write32(DCH1_PWRCTL, nv);

    // poll self-refresh state
    let s = if pwdn_en { 2 } else { 1 };
    while ((read32(STAT) & (0b11 << 8)) >> 8) != s {}
    if mode == 0 {
        while ((read32(DCH1_STAT) & (0b11 << 8)) >> 8) != s {}
    }
}

fn lpddr4_reinit_ctrl(size: Size) {
    let rank_num = RANK;
    let speed = FREQ;
    let ddr_type = DdrType::LPDDR4X;

    // remove core clock after xx
    write32(DDR_CFG1, 0x0000011f);

    // [3] dfi_dram_clk_disable [1] powerdown_en [0]serref_en
    write32(PWRCTL, 0x00000000);
    write32(DCH1_PWRCTL, 0x00000000);

    disable_axi_port(0x1f);
    while read32(PSTAT) != 0 {}

    write32(DFIPHYMSTR, 0x14000000);
    // check status.
    while read32(STAT) & 0x3 == 0x03 {}

    // poll cam empty flag
    while read32(DBGCAM) & 0x36000000 != 0x36000000 {}

    // save phy regs
    // ??? not implemented in C code

    lpddr4_enter_selfrefresh(true, false, 0);

    // LP3 enter
    let dfi_freq = 0x1f;
    let skip_dram_init = 0x3;
    dfi_freq_change(dfi_freq, skip_dram_init);

    // PwrOkIn desassert
    let v = read32(DDR_CFG0);
    write32(DDR_CFG0, v & !(0x1 << 6));

    // Phy reset .DDR_CFG0 ALL reset
    let _ = read32(DDR_CFG0);
    write32(DDR_CFG0, 0);

    // ddr core reset ctrl sw reset
    let v = read32(DDR_CFG0);
    write32(DDR_CFG0, v & !(0x1 << 5));

    // Pwrokin dessert
    let v = read32(DDR_CFG0);
    write32(DDR_CFG0, v | (0x1 << 6));

    // NOTE: commented out in C code
    // dwc_umctl_init_skip_traing(type, rank_num, speed, bits);

    // release apb presetn
    write32(DDR_CFG0, 0x50);
    write32(DDR_CFG0, 0x50);
    write32(DDR_CFG0, 0x50);

    let bits = Bits::B64;
    if bits.clone() == Bits::B64 {
        write32(DDR_CFG0, 0x52);
    }

    if DEBUG_DFI {
        return;
    }

    ctrl_init(rank_num, speed);
    addrmap(rank_num, bits.clone());
    adjust_ddr_addrmap(ddr_type, rank_num as u32, speed as u32, bits.clone(), size);

    println!("MISSING: dwc_ddr_misc_regu_save");
    // TODO
    // misc regu restore for str
    // dwc_ddr_misc_regu_save();

    // NOTE: We get here.

    // after this step, only PwrOk is still low
    de_assert_other_reset_ddr();

    dq_pinmux(bits.clone());

    // phy restore
    dwc_ddrphy_phyinit_reg_interface(RegInstr::RestoreRegs);

    // ctrl en, hs
    ctrl_en_lp3_exit(bits);

    lpddr4_selfrefresh_exit(0);

    enable_auto_refresh();
    enable_axi_port(0x1f);

    write32(DFIPHYMSTR, 0x14000001);
    lpddr4_auto_selref();
}

// board/thead/light-c910/spl.c
pub fn setup_ddr_addrmap() {
    // TODO: wrap in loop, see C code
    let size = Size::S8G;
    lpddr4_reinit_ctrl(size);
}
