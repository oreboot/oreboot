// board/thead/light-c910/lpddr4/src/init_ddr.c

use core::ptr::write;
use crate::util::{read32, write32};

const FREQ: u16 = 3733;
const DDR_BIT_WIDTH: u8 = 64;
const RANK: u8 = 2;
const DDR_CFG0: usize = 0x0;
const _DDR_PHY_BADDR: usize = 0xfffd000000;
const _DDR_CTRL_BADDR: usize = _DDR_PHY_BADDR + 0x2000000;
const DBG1: usize = _DDR_CTRL_BADDR + 0x304;
const STAT: usize = _DDR_CTRL_BADDR + 0x4;
const MSTR: usize = _DDR_CTRL_BADDR + 0x0;
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
const DCH1_MRCTRL0: usize = _DDR_CTRL_BADDR + 0x00001b10;
const DCH1_MRCTRL1: usize = _DDR_CTRL_BADDR + 0x00001b14;
const DCH1_DERATECTL: usize = _DDR_CTRL_BADDR + 0x00001b2c;
const DCH1_PWRCTL: usize = _DDR_CTRL_BADDR + 0x00001b30;
const DCH1_HWLPCTL: usize = _DDR_CTRL_BADDR + 0x00001b38;
const DCH1_CRCPARCTL0: usize = _DDR_CTRL_BADDR + 0x00001bc0;
const DCH1_ZQCTL2: usize = _DDR_CTRL_BADDR + 0x00001c88;
const DCH1_ODTMAP: usize = _DDR_CTRL_BADDR + 0x00001d44;
const DCH1_DBG1: usize = _DDR_CTRL_BADDR + 0x00001e04;
const DCH1_DBGCMD: usize = _DDR_CTRL_BADDR + 0x00001e0c;


// clock configs
const LIGHT_APCLK_ADDRBASE: usize = 0xffff011000;
const LIGHT_AONCLK_ADDRBASE: usize = 0xfffff46000;
const LIGHT_VO_SUBSYS_ADDRBASE: usize = 0xffff401000;
const LIGHT_VO_SUBSYS_R_ADDRBASE: usize = 0xffef528000;
const LIGHT_AUDIO_SUBSYS_ADDRBASE: usize = 0xffcb000000;
const LIGHT_APSYS_RSTGEN_ADDRBASE: usize = 0xffff015000;
const LIGHT_DSP_SUBSYS_ADDRBASE: usize = 0xffff041000;


pub fn init() {
    sys_clk_config();
    lpddr4_init(RANK, FREQ, DDR_BIT_WIDTH);
}

// This function tries to implement udelay() function required in DDR init
// FIXME: try to find a better approach.
fn udelay(micros: usize) {
    unsafe {
        for _ in 0..micros {
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
    if !((tmp & 0x3fe) == 0x3fe) {
        // FIXME: fix this error handling
        println!("[bt0] pll lock check failed: {}", tmp);
    }
    /* 2. update sys_pll to frac mode, 2438.5536MHz */
    /* switch share_sram_clk to audio_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x104);
    tmp |= 0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x104, tmp);

    /* switch audio_c906_cclk to audio_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x110);
    tmp |= 0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x110, tmp);

    /* switch audio_subsys_aclk to audio_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x108);
    tmp |= 0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x108, tmp);

    /* disable audio_i2s_src_clk */
    tmp = read32(LIGHT_AUDIO_SUBSYS_ADDRBASE + 0x4);
    tmp &= !0x20200;
    write32(LIGHT_AUDIO_SUBSYS_ADDRBASE + 0x4, tmp);

    /* disable peri_i2s_src_clk */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x1f0);
    tmp &= !0x2;
    write32(LIGHT_APCLK_ADDRBASE + 0x1f0, tmp);

    udelay(1);

    /* set sys_pll_foutvco to 2438.5536MHz */
    write32(LIGHT_AONCLK_ADDRBASE + 0x14, 0x20000000);
    write32(LIGHT_AONCLK_ADDRBASE + 0x10, 0x03606501);
    write32(LIGHT_AONCLK_ADDRBASE + 0x14, 0x209b3d08);
    udelay(3);
    write32(LIGHT_AONCLK_ADDRBASE + 0x14, 0x009b3d08);
    read32(LIGHT_AONCLK_ADDRBASE + 0x90);
    read32(LIGHT_AONCLK_ADDRBASE + 0x90);
    while (read32(LIGHT_AONCLK_ADDRBASE + 0x90) & 0x2) == 0 {}
    udelay(11);

    /* switch share_sram_clk to sys_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x104);
    tmp &= !0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x104, tmp);

    /* switch audio_c906_cclk to sys_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x110);
    tmp &= !0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x110, tmp);

    /* swith audio_subsys_aclk to sys_pll_foutvco */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x108);
    tmp &= !0x2000;
    write32(LIGHT_AONCLK_ADDRBASE + 0x108, tmp);

    /* 3. update audio_pll, to frac mode, 884.736MHz */
    /* switch aonsys_clk to pad_osc_clk */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x100);
    tmp |= 0x10;
    write32(LIGHT_AONCLK_ADDRBASE + 0x100, tmp);

    /* set audio_pll_foutvco to frac mode, 884.736MHz */
    write32(LIGHT_AONCLK_ADDRBASE + 0x04, 0x20000000);
    write32(LIGHT_AONCLK_ADDRBASE + 0x00, 0x01302401);
    write32(LIGHT_AONCLK_ADDRBASE + 0x04, 0x20dd2f70);
    udelay(3);
    write32(LIGHT_AONCLK_ADDRBASE + 0x04, 0x00dd2f70);
    read32(LIGHT_AONCLK_ADDRBASE + 0x90);
    read32(LIGHT_AONCLK_ADDRBASE + 0x90);
    while (read32(LIGHT_AONCLK_ADDRBASE + 0x90) & 0x1) == 0 {}
    udelay(11);

    /* switch aonsys_clk to audio_pll_foutpostdiv */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x100);
    tmp &= !0x10;
    write32(LIGHT_AONCLK_ADDRBASE + 0x100, tmp);

    /* switch aoi2c_ic_clk to audio_pll_fout3 */
    tmp = read32(LIGHT_AONCLK_ADDRBASE + 0x11c);
    tmp &= !0x1;
    write32(LIGHT_AONCLK_ADDRBASE + 0x11c, tmp);

    /* enable audio_i2s_src_clk */
    tmp = read32(LIGHT_AUDIO_SUBSYS_ADDRBASE + 0x4);
    tmp |= 0x20200;
    write32(LIGHT_AUDIO_SUBSYS_ADDRBASE + 0x4, tmp);

    /* enable peri_i2s_src_clk */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x1f0);
    tmp |= 0x2;
    write32(LIGHT_APCLK_ADDRBASE + 0x1f0, tmp);

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
    /* perisys_ahb_hclk 250MHz  perisys_apb_pclk 62.5MHz */

    /* set dpu0_pll_div_clk to dpu0_pll_foutpostdiv/16 as 74.25MHz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x1e8);
    tmp &= !0x100;
    write32(LIGHT_APCLK_ADDRBASE + 0x1e8, tmp);
    udelay(1);
    tmp &= !0xff;
    tmp |= 0x10;
    write32(LIGHT_APCLK_ADDRBASE + 0x1e8, tmp);
    udelay(1);
    tmp |= 0x100;
    write32(LIGHT_APCLK_ADDRBASE + 0x1e8, tmp);
    udelay(1);

    /* set dpu1_pll_div_clk to dpu1_pll_foutpostdiv/16 as 74.25MHz */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x1ec);
    tmp &= !0x100;
    write32(LIGHT_APCLK_ADDRBASE + 0x1ec, tmp);
    udelay(1);
    tmp &= !0xff;
    tmp |= 0x10;
    write32(LIGHT_APCLK_ADDRBASE + 0x1ec, tmp);
    udelay(1);
    tmp |= 0x100;
    write32(LIGHT_APCLK_ADDRBASE + 0x1ec, tmp);
    udelay(1);

    /*5. enable necessary gates */
    /* enable dsp_subsys, vi_subsys, vo_subsys all clocls */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x220);
    tmp |= 0x7;
    write32(LIGHT_APCLK_ADDRBASE + 0x220, tmp);

    /* AP rst_gen: VP/VO/VI/DSP */
    write32(LIGHT_APSYS_RSTGEN_ADDRBASE + 0x220, 0xf);

    /* enable dsp0/1_cclk, dsp0/1_pclk */
    tmp = read32(LIGHT_DSP_SUBSYS_ADDRBASE + 0x24);
    tmp |= 0xf;
    write32(LIGHT_DSP_SUBSYS_ADDRBASE + 0x24, tmp);

    /* enable gpu_core_clk, gpu_cfg_aclk */
    tmp = read32(LIGHT_VO_SUBSYS_ADDRBASE + 0x50);
    tmp |= 0x18;
    write32(LIGHT_VO_SUBSYS_ADDRBASE + 0x50, tmp);

    tmp = read32(LIGHT_VO_SUBSYS_R_ADDRBASE + 0x50);
    tmp |= 0x3ff;
    write32(LIGHT_VO_SUBSYS_R_ADDRBASE + 0x50, tmp);

    /* enable dpu_pixelclk0/1, dpu_hclk, dpu_aclk, dpu_cclk */
    tmp = read32(LIGHT_VO_SUBSYS_ADDRBASE + 0x50);
    tmp |= 0x3e0;
    write32(LIGHT_VO_SUBSYS_ADDRBASE + 0x50, tmp);

    /* enable npu_axi_aclk, npu_core_clk */
    tmp = read32(LIGHT_APCLK_ADDRBASE + 0x1c8);
    tmp |= 0x30;
    write32(LIGHT_APCLK_ADDRBASE + 0x1c8, tmp);
    /* The boards other than the LightA board perform the bus down-speed operation */
}

fn lpddr4_init(rank: u8, freq: u16, bits: u8) {
    println!("[*] LPDDR4 init...");
    pll_config(freq);
    println!("[+] PLL Complete...");
    deassert_pwrok_apb(bits);
    println!("[+] deassert_pwrok_apb Complete...");
    ctrl_init(rank, freq);
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn pll_config(speed: u16) {
    const DDR_TEST: usize = DDR_CFG0 + 0xc;
    println!("[+] pll_config init...");
    write32(DDR_TEST, 0x4b000000);
    println!("[+] pll_config check point 1");
    write32(DDR_CFG0 + 0x8, 0x01204d01);
    println!("[+] pll_config before udelay(2)");
    udelay(2);
    println!("[+] pll_config after udelay(2)");
    write32(DDR_CFG0 + 0xc, 0x0b000000);
    while (read32(DDR_CFG0 + 0x18) & 1) != 0x1 { print!(".") }
    write32(DDR_CFG0 + 0x18, 0x10000);
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn deassert_pwrok_apb(bits: u8) {
    println!("[+] deassert_pwrok_apb init...");
    write32(DDR_CFG0, 0x40);  // release PwrOkIn
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);
    write32(DDR_CFG0, 0x40);

    write32(DDR_CFG0, 0xc0);  // release Phyrst
    write32(DDR_CFG0, 0xc0);  // release Phyrst
    write32(DDR_CFG0, 0xc0);  // release Phyrst
    write32(DDR_CFG0, 0xc0);  // release Phyrst

    write32(DDR_CFG0, 0xd0);  // release apb presetn
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
    write32(DDR_CFG0, 0xd0);
}

// board/thead/light-c910/lpddr4/src/ddr_common_func.c
fn ctrl_init(rank: u8, freq: u16) {
    println!("[*] CTRL Init...");
    write32(DBG1, 0x00000001);
    write32(PWRCTL, 0x00000001);
    while (read32(STAT) != 0x00000000) {}
    if (rank == 2) {
        write32(MSTR, 0x03080020);
    }
    write32(MRCTRL0, 0x00003030);
    write32(MRCTRL1, 0x0002d90f);

    if (freq == 3733) {
        println!("[+] Frequency: {freq}");
        write32(DERATEEN, 0x000013f3);
        write32(DERATEINT, 0x40000000);
        write32(DERATECTL, 0x00000001);
        write32(PWRCTL, 0x00000020);
        write32(PWRTMG, 0x0040ae04);
        write32(HWLPCTL, 0x00430000);

        write32(RFSHCTL0, 0x00210004);
        write32(RFSHCTL1, 0x000d0021);
        write32(RFSHCTL3, 0x00000001);
        write32(RFSHTMG, 0x81c00084);
        write32(RFSHTMG1, 0x00480000);

        write32(CRCPARCTL0, 0x00000000);
        write32(INIT0, 0xc0020002);
        write32(INIT1, 0x00010002);
        write32(INIT2, 0x00001a00);
        write32(INIT3, 0x0054002e); //OP[2:0] RL
        write32(INIT4, 0x0c310008); //[31:16] LP4 MR3
        write32(INIT5, 0x00040009);
        write32(INIT6, 0x00000012);
        write32(INIT7, 0x0000001a);
        write32(DIMMCTL, 0x00000000);
        write32(RANKCTL, 0x0000ab9f);
        write32(RANKCTL1, 0x00000017);
        write32(DRAMTMG0, 0x1b203622);
        write32(DRAMTMG1, 0x00060630);
        write32(DRAMTMG2, 0x07101b15);

        write32(DRAMTMG3, 0x00b0c000);
        write32(DRAMTMG4, 0x0f04080f);
        write32(DRAMTMG5, 0x02040c0c);
        write32(DRAMTMG6, 0x01010007);
        write32(DRAMTMG7, 0x00000402);
        write32(DRAMTMG8, 0x00000101);
        write32(DRAMTMG12, 0x00020000);
        write32(DRAMTMG13, 0x0c100002);
        write32(DRAMTMG14, 0x000000e6);
        write32(ZQCTL0, 0x03200018);
        write32(ZQCTL1, 0x0280ccda);
        write32(ZQCTL2, 0x00000000);
        write32(DFITMG0, 0x059b820a);
        write32(DFITMG1, 0x000c0303);
        write32(DFILPCFG0, 0x0351a101);
        write32(DFIMISC, 0x00000011);
        write32(DFITMG2, 0x00001f0c); //
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
    write32(SCHED, 0x1f829b1c);  //[2]  page-close enable [14:8] 0x1b: lpr entry num=28, hpr entry num=4
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
    //update by perf sim
    write32(PCCFG, 0x00000010);   //[4] page match limit,limits the number of consecutive same page DDRC transactions that can be granted by the Port Arbiter to four
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

    // debugging
    println!("[*] Testing the ctrl_init() function...");
    println!("[+] RankCTL   : {}", read32(RANKCTL));
    println!("[+] DRAMTMG2  : {}", read32(DRAMTMG2));
    println!("[+] DFITMG0   : {}", read32(DFITMG0));
    println!("[+] DFITMG1   : {}", read32(DFITMG1));
    println!("[+] DRAMTMG4  : {}", read32(DRAMTMG4)); //[19:16] tCCD
}