use crate::util::write32;
use oreboot_util::nop_delay;

macro_rules! genmask {
    ($h:expr, $l:expr) => {
        ((!0usize << $l) & (!0usize >> (BITS_PER_LONG - 1 - $h)))
    };
}

macro_rules! field_prep {
    ($mask:expr, $val:expr) => {
        (($val << $mask.trailing_zeros()) & $mask)
    };
}

macro_rules! bit {
    ($nr:expr) => {
        1u32 << $nr
    };
}

/* Firmware constants */
const TH1520_DDR_MAGIC: usize = 0x4452444445415448;
const TH1520_DDR_PHY0_BASE: usize = 0xfffd000000;
const TH1520_DDR_PHY1_BASE: usize = 0xfffe000000;
const TH1520_DDR_CTRL_BASE: usize = 0xffff000000;
const TH1520_DDR_SYS_BASE: usize = 0xffff005000;

const TH1520_DDR_TYPE_LPDDR4: u8 = 0;
const TH1520_DDR_TYPE_LPDDR4X: u8 = 1;

const TH1520_DDR_FREQ_2133: u8 = 0;
const TH1520_DDR_FREQ_3200: u8 = 1;
const TH1520_DDR_FREQ_3733: u32 = 2;
const TH1520_DDR_FREQ_4266: u8 = 3;

const BITS_PER_LONG: u32 = 64;

const TH1520_DDR_CFG_OP: usize = genmask!(31, 24);
const TH1520_DDR_CFG_ADDR: usize = genmask!(23, 0);

const TH1520_DDR_CFG_PHY0: u8 = 0;
const TH1520_DDR_CFG_PHY1: u8 = 1;
const TH1520_DDR_CFG_PHY: u8 = 2;
const TH1520_DDR_CFG_RANGE: u8 = 3;
const TH1520_DDR_CFG_WAITFW0: u8 = 4;
const TH1520_DDR_CFG_WAITFW1: u8 = 5;

/* Driver constants */
const TH1520_SYS_PLL_TIMEOUT_US: u8 = 30;
const TH1520_CTRL_INIT_TIMEOUT_US: u32 = 1000000;
const TH1520_PHY_MSG_TIMEOUT_US: u32 = 1000000;

/* System configuration registers */
const TH1520_SYS_DDR_CFG0: u8 = 0x00;
const TH1520_SYS_DDR_CFG0_APB_RSTN: u32 = bit!(4);
const TH1520_SYS_DDR_CFG0_CTRL_RSTN: u32 = bit!(5);
const TH1520_SYS_DDR_CFG0_PHY_PWROK_RSTN: u32 = bit!(6);
const TH1520_SYS_DDR_CFG0_PHY_CORE_RSTN: u32 = bit!(7);
// #define  TH1520_SYS_DDR_CFG0_APB_PORT_RSTN(n)	BIT(n + 4 + 4)
const TH1520_SYS_DDR_CFG1: usize = 0x04;
const TH1520_SYS_PLL_CFG0: usize = 0x08;
const TH1520_SYS_PLL_CFG0_POSTDIV2: usize = genmask!(26, 24);
const TH1520_SYS_PLL_CFG0_POSTDIV1: usize = genmask!(22, 20);
const TH1520_SYS_PLL_CFG0_FBDIV: usize = genmask!(19, 8);
const TH1520_SYS_PLL_CFG0_REFDIV: usize = genmask!(5, 0);
const TH1520_SYS_PLL_CFG1: usize = 0x0c;
const TH1520_SYS_PLL_CFG1_RST: u32 = bit!(30);
const TH1520_SYS_PLL_CFG1_FOUTPOSTDIVPD: u32 = bit!(27);
const TH1520_SYS_PLL_CFG1_FOUT4PHASEPD: u32 = bit!(25);
const TH1520_SYS_PLL_CFG1_DACPD: u32 = bit!(24);
const TH1520_SYS_PLL_CFG2: u8 = 0x10;
const TH1520_SYS_PLL_CFG3: u8 = 0x14;
const TH1520_SYS_PLL_STS: usize = 0x18;
const TH1520_SYS_PLL_STS_EN: u32 = bit!(16);
const TH1520_SYS_PLL_STS_LOCKED: u32 = bit!(0);

/* DDR Controller Registers */
const TH1520_CTRL_MSTR: u32 = 0x0000;
const TH1520_CTRL_STAT: u32 = 0x0004;
const TH1520_CTRL_MRCTRL0: u32 = 0x0010;
const TH1520_CTRL_MRCTRL1: u32 = 0x0014;
const TH1520_CTRL_MRSTAT: u32 = 0x0018;
const TH1520_CTRL_DERATEEN: u32 = 0x0020;
const TH1520_CTRL_DERATEINT: u32 = 0x0024;
const TH1520_CTRL_DERATECTL: u32 = 0x002c;
const TH1520_CTRL_PWRCTL: u32 = 0x0030;
const TH1520_CTRL_PWRTMG: u32 = 0x0034;
const TH1520_CTRL_HWLPCTL: u32 = 0x0038;
const TH1520_CTRL_RFSHCTL0: u32 = 0x0050;
const TH1520_CTRL_RFSHCTL1: u32 = 0x0054;
const TH1520_CTRL_RFSHCTL3: u32 = 0x0060;
const TH1520_CTRL_RFSHTMG: u32 = 0x0064;
const TH1520_CTRL_RFSHTMG1: u32 = 0x0068;
const TH1520_CTRL_CRCPARCTL0: u32 = 0x00c0;
const TH1520_CTRL_CRCPARSTAT: u32 = 0x00cc;
const TH1520_CTRL_INIT0: u32 = 0x00d0;
const TH1520_CTRL_INIT1: u32 = 0x00d4;
const TH1520_CTRL_INIT2: u32 = 0x00d8;
const TH1520_CTRL_INIT3: u32 = 0x00dc;
const TH1520_CTRL_INIT4: u32 = 0x00e0;
const TH1520_CTRL_INIT5: u32 = 0x00e4;
const TH1520_CTRL_INIT6: u32 = 0x00e8;
const TH1520_CTRL_INIT7: u32 = 0x00ec;
const TH1520_CTRL_DIMMCTL: u32 = 0x00f0;
const TH1520_CTRL_RANKCTL: u32 = 0x00f4;
const TH1520_CTRL_RANKCTL1: u32 = 0x00f8;
const TH1520_CTRL_DRAMTMG0: u32 = 0x0100;
const TH1520_CTRL_DRAMTMG1: u32 = 0x0104;
const TH1520_CTRL_DRAMTMG2: u32 = 0x0108;
const TH1520_CTRL_DRAMTMG3: u32 = 0x010c;
const TH1520_CTRL_DRAMTMG4: u32 = 0x0110;
const TH1520_CTRL_DRAMTMG5: u32 = 0x0114;
const TH1520_CTRL_DRAMTMG6: u32 = 0x0118;
const TH1520_CTRL_DRAMTMG7: u32 = 0x011c;
const TH1520_CTRL_DRAMTMG8: u32 = 0x0120;
const TH1520_CTRL_DRAMTMG12: u32 = 0x0130;
const TH1520_CTRL_DRAMTMG13: u32 = 0x0134;
const TH1520_CTRL_DRAMTMG14: u32 = 0x0138;
const TH1520_CTRL_DRAMTMG17: u32 = 0x0144;
const TH1520_CTRL_ZQCTL0: u32 = 0x0180;
const TH1520_CTRL_ZQCTL1: u32 = 0x0184;
const TH1520_CTRL_ZQCTL2: u32 = 0x0188;
const TH1520_CTRL_ZQSTAT: u32 = 0x018c;
const TH1520_CTRL_DFITMG0: u32 = 0x0190;
const TH1520_CTRL_DFITMG1: u32 = 0x0194;
const TH1520_CTRL_DFILPCFG0: u32 = 0x0198;
const TH1520_CTRL_DFIUPD0: u32 = 0x01a0;
const TH1520_CTRL_DFIUPD1: u32 = 0x01a4;
const TH1520_CTRL_DFIUPD2: u32 = 0x01a8;
const TH1520_CTRL_DFIMISC: u32 = 0x01b0;
const TH1520_CTRL_DFITMG2: u32 = 0x01b4;
const TH1520_CTRL_DFISTAT: u32 = 0x01bc;
const TH1520_CTRL_DBICTL: u32 = 0x01c0;
const TH1520_CTRL_DFIPHYMSTR: u32 = 0x01c4;
const TH1520_CTRL_ADDRMAP0: u32 = 0x0200;
const TH1520_CTRL_ADDRMAP1: u32 = 0x0204;
const TH1520_CTRL_ADDRMAP2: u32 = 0x0208;
const TH1520_CTRL_ADDRMAP3: u32 = 0x020c;
const TH1520_CTRL_ADDRMAP4: u32 = 0x0210;
const TH1520_CTRL_ADDRMAP5: u32 = 0x0214;
const TH1520_CTRL_ADDRMAP6: u32 = 0x0218;
const TH1520_CTRL_ADDRMAP7: u32 = 0x021c;
const TH1520_CTRL_ADDRMAP8: u32 = 0x0220;
const TH1520_CTRL_ADDRMAP9: u32 = 0x0224;
const TH1520_CTRL_ADDRMAP10: u32 = 0x0228;
const TH1520_CTRL_ADDRMAP11: u32 = 0x022c;
const TH1520_CTRL_ODTCFG: u32 = 0x0240;
const TH1520_CTRL_ODTMAP: u32 = 0x0244;
const TH1520_CTRL_SCHED: u32 = 0x0250;
const TH1520_CTRL_SCHED1: u32 = 0x0254;
const TH1520_CTRL_PERFHPR1: u32 = 0x025c;
const TH1520_CTRL_PERFLPR1: u32 = 0x0264;
const TH1520_CTRL_PERFWR1: u32 = 0x026c;
const TH1520_CTRL_SCHED3: u32 = 0x0270;
const TH1520_CTRL_SCHED4: u32 = 0x0274;
const TH1520_CTRL_DBG0: u32 = 0x0300;
const TH1520_CTRL_DBG1: u32 = 0x0304;
const TH1520_CTRL_DBGCAM: u32 = 0x0308;
const TH1520_CTRL_DBGCMD: u32 = 0x030c;
const TH1520_CTRL_DBGSTAT: u32 = 0x0310;
const TH1520_CTRL_SWCTL: u32 = 0x0320;
const TH1520_CTRL_SWSTAT: u32 = 0x0324;
const TH1520_CTRL_SWCTLSTATIC: u32 = 0x0328;
const TH1520_CTRL_POISONCFG: u32 = 0x036c;
const TH1520_CTRL_POISONSTAT: u32 = 0x0370;
const TH1520_CTRL_DERATESTAT: u32 = 0x03f0;
const TH1520_CTRL_PSTAT: u32 = 0x03fc;
const TH1520_CTRL_PCCFG: u32 = 0x0400;
const TH1520_CTRL_PCFGR_0: u32 = 0x0404;
const TH1520_CTRL_PCFGW_0: u32 = 0x0408;
const TH1520_CTRL_PCTRL_0: u32 = 0x0490;
const TH1520_CTRL_PCFGQOS0_0: u32 = 0x0494;
const TH1520_CTRL_PCFGQOS1_0: u32 = 0x0498;
const TH1520_CTRL_PCFGWQOS0_0: u32 = 0x049c;
const TH1520_CTRL_PCFGWQOS1_0: u32 = 0x04a0;
const TH1520_CTRL_PCFGR_1: u32 = 0x04b4;
const TH1520_CTRL_PCFGW_1: u32 = 0x04b8;
const TH1520_CTRL_PCTRL_1: u32 = 0x0540;
const TH1520_CTRL_PCFGQOS0_1: u32 = 0x0544;
const TH1520_CTRL_PCFGQOS1_1: u32 = 0x0548;
const TH1520_CTRL_PCFGWQOS0_1: u32 = 0x054c;
const TH1520_CTRL_PCFGWQOS1_1: u32 = 0x0550;
const TH1520_CTRL_PCFGR_2: u32 = 0x0564;
const TH1520_CTRL_PCFGW_2: u32 = 0x0568;
const TH1520_CTRL_PCTRL_2: u32 = 0x05f0;
const TH1520_CTRL_PCFGQOS0_2: u32 = 0x05f4;
const TH1520_CTRL_PCFGQOS1_2: u32 = 0x05f8;
const TH1520_CTRL_PCFGWQOS0_2: u32 = 0x05fc;
const TH1520_CTRL_PCFGWQOS1_2: u32 = 0x0600;
const TH1520_CTRL_PCFGR_3: u32 = 0x0614;
const TH1520_CTRL_PCFGW_3: u32 = 0x0618;
const TH1520_CTRL_PCTRL_3: u32 = 0x06a0;
const TH1520_CTRL_PCFGQOS0_3: u32 = 0x06a4;
const TH1520_CTRL_PCFGQOS1_3: u32 = 0x06a8;
const TH1520_CTRL_PCFGWQOS0_3: u32 = 0x06ac;
const TH1520_CTRL_PCFGWQOS1_3: u32 = 0x06b0;
const TH1520_CTRL_PCFGR_4: u32 = 0x06c4;
const TH1520_CTRL_PCFGW_4: u32 = 0x06c8;
const TH1520_CTRL_PCTRL_4: u32 = 0x0750;
const TH1520_CTRL_PCFGQOS0_4: u32 = 0x0754;
const TH1520_CTRL_PCFGQOS1_4: u32 = 0x0758;
const TH1520_CTRL_PCFGWQOS0_4: u32 = 0x075c;
const TH1520_CTRL_PCFGWQOS1_4: u32 = 0x0760;
const TH1520_CTRL_UMCTL2_VER_NUMBER: u32 = 0x0ff0;
const TH1520_CTRL_UMCTL2_VER_TYPE: u32 = 0x0ff4;
const TH1520_CTRL_DCH1_STAT: u32 = 0x1b04;
const TH1520_CTRL_DCH1_MRCTRL0: u32 = 0x1b10;
const TH1520_CTRL_DCH1_MRCTRL1: u32 = 0x1b14;
const TH1520_CTRL_DCH1_MRSTAT: u32 = 0x1b18;
const TH1520_CTRL_DCH1_DERATECTL: u32 = 0x1b2c;
const TH1520_CTRL_DCH1_PWRCTL: u32 = 0x1b30;
const TH1520_CTRL_DCH1_HWLPCTL: u32 = 0x1b38;
const TH1520_CTRL_DCH1_CRCPARCTL0: u32 = 0x1bc0;
const TH1520_CTRL_DCH1_ZQCTL2: u32 = 0x1c88;
const TH1520_CTRL_DCH1_DFISTAT: u32 = 0x1cbc;
const TH1520_CTRL_DCH1_ODTMAP: u32 = 0x1d44;
const TH1520_CTRL_DCH1_DBG1: u32 = 0x1e04;
const TH1520_CTRL_DCH1_DBGCMD: u32 = 0x1e0c;
const TH1520_CTRL_DCH1_DBGCAM: u32 = 0x1e08;

pub fn init() {
    th1520_ddr_pll_config(TH1520_DDR_FREQ_3733);
}

// drivers/ram/thead/th1520_ddr.c: static int th1520_ddr_pll_config(void __iomem *sysreg, unsigned int frequency)
fn th1520_ddr_pll_config(freq: u32) {
    let mut tmp: u32;
    tmp = TH1520_SYS_PLL_CFG1_RST
        | TH1520_SYS_PLL_CFG1_FOUTPOSTDIVPD
        | TH1520_SYS_PLL_CFG1_FOUT4PHASEPD
        | TH1520_SYS_PLL_CFG1_DACPD;
    write32(TH1520_DDR_SYS_BASE + TH1520_SYS_PLL_CFG1, tmp);

    match freq {
        TH1520_DDR_FREQ_3733 => {
            let cfg0 = (field_prep!(TH1520_SYS_PLL_CFG0_REFDIV, 1)
                | field_prep!(TH1520_SYS_PLL_CFG0_FBDIV, 77)
                | field_prep!(TH1520_SYS_PLL_CFG0_POSTDIV1, 2)
                | field_prep!(TH1520_SYS_PLL_CFG0_POSTDIV2, 1));
            write32(TH1520_DDR_SYS_BASE + TH1520_SYS_PLL_CFG0, cfg0 as u32);
        }
        _ => print!("Invalid Frequency"),
    }

    nop_delay(2);
    tmp &= !TH1520_SYS_PLL_CFG1_RST;
    write32(TH1520_DDR_SYS_BASE + TH1520_SYS_PLL_CFG1, tmp);

    // FIXME: check whether it's required, reading a bunch of stuff for some set duration
    // ret = readl_poll_timeout(sysreg + TH1520_SYS_PLL_STS, tmp,
    //                          tmp & TH1520_SYS_PLL_STS_LOCKED,
    //                          TH1520_SYS_PLL_TIMEOUT_US);

    write32(
        TH1520_DDR_SYS_BASE + TH1520_SYS_PLL_STS,
        TH1520_SYS_PLL_STS_EN,
    );
    print!("[+] th1520_ddr_pll_config Complete...");
}
