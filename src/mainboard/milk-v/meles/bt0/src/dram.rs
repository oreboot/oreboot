use crate::dram_training_data::{
    DCCM_1D_TRAIN_FW, ICCM_1D_TRAIN_FW, MCU_START, PINMUX_CFG_PHY0, PINMUX_CFG_PHY1,
    PRE_CCM_LOADING,
};
use oreboot_util::mmio::{write16, write32};
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

macro_rules! bitn {
    ($n:expr) => {
        bit!($n + 4 + 4)
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
const TH1520_DDR_FREQ_3733: u8 = 2;
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
const TH1520_SYS_DDR_CFG0: usize = 0x00;
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
const TH1520_CTRL_MSTR: usize = 0x0000;
const TH1520_CTRL_STAT: u32 = 0x0004;
const TH1520_CTRL_MRCTRL0: usize = 0x0010;
const TH1520_CTRL_MRCTRL1: usize = 0x0014;
const TH1520_CTRL_MRSTAT: u32 = 0x0018;
const TH1520_CTRL_DERATEEN: usize = 0x0020;
const TH1520_CTRL_DERATEINT: usize = 0x0024;
const TH1520_CTRL_DERATECTL: usize = 0x002c;
const TH1520_CTRL_PWRCTL: usize = 0x0030;
const TH1520_CTRL_PWRTMG: usize = 0x0034;
const TH1520_CTRL_HWLPCTL: usize = 0x0038;
const TH1520_CTRL_RFSHCTL0: usize = 0x0050;
const TH1520_CTRL_RFSHCTL1: usize = 0x0054;
const TH1520_CTRL_RFSHCTL3: usize = 0x0060;
const TH1520_CTRL_RFSHTMG: usize = 0x0064;
const TH1520_CTRL_RFSHTMG1: usize = 0x0068;
const TH1520_CTRL_CRCPARCTL0: usize = 0x00c0;
const TH1520_CTRL_CRCPARSTAT: usize = 0x00cc;
const TH1520_CTRL_INIT0: usize = 0x00d0;
const TH1520_CTRL_INIT1: usize = 0x00d4;
const TH1520_CTRL_INIT2: usize = 0x00d8;
const TH1520_CTRL_INIT3: usize = 0x00dc;
const TH1520_CTRL_INIT4: usize = 0x00e0;
const TH1520_CTRL_INIT5: usize = 0x00e4;
const TH1520_CTRL_INIT6: usize = 0x00e8;
const TH1520_CTRL_INIT7: usize = 0x00ec;
const TH1520_CTRL_DIMMCTL: usize = 0x00f0;
const TH1520_CTRL_RANKCTL: usize = 0x00f4;
const TH1520_CTRL_RANKCTL1: usize = 0x00f8;
const TH1520_CTRL_DRAMTMG0: usize = 0x0100;
const TH1520_CTRL_DRAMTMG1: usize = 0x0104;
const TH1520_CTRL_DRAMTMG2: usize = 0x0108;
const TH1520_CTRL_DRAMTMG3: usize = 0x010c;
const TH1520_CTRL_DRAMTMG4: usize = 0x0110;
const TH1520_CTRL_DRAMTMG5: usize = 0x0114;
const TH1520_CTRL_DRAMTMG6: usize = 0x0118;
const TH1520_CTRL_DRAMTMG7: usize = 0x011c;
const TH1520_CTRL_DRAMTMG8: usize = 0x0120;
const TH1520_CTRL_DRAMTMG12: usize = 0x0130;
const TH1520_CTRL_DRAMTMG13: usize = 0x0134;
const TH1520_CTRL_DRAMTMG14: usize = 0x0138;
const TH1520_CTRL_DRAMTMG17: usize = 0x0144;
const TH1520_CTRL_ZQCTL0: usize = 0x0180;
const TH1520_CTRL_ZQCTL1: usize = 0x0184;
const TH1520_CTRL_ZQCTL2: usize = 0x0188;
const TH1520_CTRL_ZQSTAT: usize = 0x018c;
const TH1520_CTRL_DFITMG0: usize = 0x0190;
const TH1520_CTRL_DFITMG1: usize = 0x0194;
const TH1520_CTRL_DFILPCFG0: usize = 0x0198;
const TH1520_CTRL_DFIUPD0: usize = 0x01a0;
const TH1520_CTRL_DFIUPD1: usize = 0x01a4;
const TH1520_CTRL_DFIUPD2: usize = 0x01a8;
const TH1520_CTRL_DFIMISC: usize = 0x01b0;
const TH1520_CTRL_DFITMG2: usize = 0x01b4;
const TH1520_CTRL_DFISTAT: usize = 0x01bc;
const TH1520_CTRL_DBICTL: usize = 0x01c0;
const TH1520_CTRL_DFIPHYMSTR: usize = 0x01c4;
const TH1520_CTRL_ADDRMAP0: usize = 0x0200;
const TH1520_CTRL_ADDRMAP1: usize = 0x0204;
const TH1520_CTRL_ADDRMAP2: usize = 0x0208;
const TH1520_CTRL_ADDRMAP3: usize = 0x020c;
const TH1520_CTRL_ADDRMAP4: usize = 0x0210;
const TH1520_CTRL_ADDRMAP5: usize = 0x0214;
const TH1520_CTRL_ADDRMAP6: usize = 0x0218;
const TH1520_CTRL_ADDRMAP7: usize = 0x021c;
const TH1520_CTRL_ADDRMAP8: usize = 0x0220;
const TH1520_CTRL_ADDRMAP9: usize = 0x0224;
const TH1520_CTRL_ADDRMAP10: usize = 0x0228;
const TH1520_CTRL_ADDRMAP11: usize = 0x022c;
const TH1520_CTRL_ODTCFG: usize = 0x0240;
const TH1520_CTRL_ODTMAP: usize = 0x0244;
const TH1520_CTRL_SCHED: usize = 0x0250;
const TH1520_CTRL_SCHED1: usize = 0x0254;
const TH1520_CTRL_PERFHPR1: usize = 0x025c;
const TH1520_CTRL_PERFLPR1: usize = 0x0264;
const TH1520_CTRL_PERFWR1: usize = 0x026c;
const TH1520_CTRL_SCHED3: usize = 0x0270;
const TH1520_CTRL_SCHED4: usize = 0x0274;
const TH1520_CTRL_DBG0: usize = 0x0300;
const TH1520_CTRL_DBG1: usize = 0x0304;
const TH1520_CTRL_DBGCAM: usize = 0x0308;
const TH1520_CTRL_DBGCMD: usize = 0x030c;
const TH1520_CTRL_DBGSTAT: usize = 0x0310;
const TH1520_CTRL_SWCTL: usize = 0x0320;
const TH1520_CTRL_SWSTAT: usize = 0x0324;
const TH1520_CTRL_SWCTLSTATIC: usize = 0x0328;
const TH1520_CTRL_POISONCFG: usize = 0x036c;
const TH1520_CTRL_POISONSTAT: u32 = 0x0370;
const TH1520_CTRL_DERATESTAT: u32 = 0x03f0;
const TH1520_CTRL_PSTAT: u32 = 0x03fc;
const TH1520_CTRL_PCCFG: usize = 0x0400;
const TH1520_CTRL_PCFGR_0: usize = 0x0404;
const TH1520_CTRL_PCFGW_0: usize = 0x0408;
const TH1520_CTRL_PCTRL_0: usize = 0x0490;
const TH1520_CTRL_PCFGQOS0_0: usize = 0x0494;
const TH1520_CTRL_PCFGQOS1_0: usize = 0x0498;
const TH1520_CTRL_PCFGWQOS0_0: usize = 0x049c;
const TH1520_CTRL_PCFGWQOS1_0: usize = 0x04a0;
const TH1520_CTRL_PCFGR_1: usize = 0x04b4;
const TH1520_CTRL_PCFGW_1: usize = 0x04b8;
const TH1520_CTRL_PCTRL_1: usize = 0x0540;
const TH1520_CTRL_PCFGQOS0_1: usize = 0x0544;
const TH1520_CTRL_PCFGQOS1_1: usize = 0x0548;
const TH1520_CTRL_PCFGWQOS0_1: usize = 0x054c;
const TH1520_CTRL_PCFGWQOS1_1: usize = 0x0550;
const TH1520_CTRL_PCFGR_2: usize = 0x0564;
const TH1520_CTRL_PCFGW_2: usize = 0x0568;
const TH1520_CTRL_PCTRL_2: usize = 0x05f0;
const TH1520_CTRL_PCFGQOS0_2: u32 = 0x05f4;
const TH1520_CTRL_PCFGQOS1_2: u32 = 0x05f8;
const TH1520_CTRL_PCFGWQOS0_2: u32 = 0x05fc;
const TH1520_CTRL_PCFGWQOS1_2: u32 = 0x0600;
const TH1520_CTRL_PCFGR_3: usize = 0x0614;
const TH1520_CTRL_PCFGW_3: usize = 0x0618;
const TH1520_CTRL_PCTRL_3: usize = 0x06a0;
const TH1520_CTRL_PCFGQOS0_3: u32 = 0x06a4;
const TH1520_CTRL_PCFGQOS1_3: u32 = 0x06a8;
const TH1520_CTRL_PCFGWQOS0_3: u32 = 0x06ac;
const TH1520_CTRL_PCFGWQOS1_3: u32 = 0x06b0;
const TH1520_CTRL_PCFGR_4: usize = 0x06c4;
const TH1520_CTRL_PCFGW_4: usize = 0x06c8;
const TH1520_CTRL_PCTRL_4: usize = 0x0750;
const TH1520_CTRL_PCFGQOS0_4: u32 = 0x0754;
const TH1520_CTRL_PCFGQOS1_4: u32 = 0x0758;
const TH1520_CTRL_PCFGWQOS0_4: u32 = 0x075c;
const TH1520_CTRL_PCFGWQOS1_4: u32 = 0x0760;
const TH1520_CTRL_UMCTL2_VER_NUMBER: u32 = 0x0ff0;
const TH1520_CTRL_UMCTL2_VER_TYPE: u32 = 0x0ff4;
const TH1520_CTRL_DCH1_STAT: u32 = 0x1b04;
const TH1520_CTRL_DCH1_MRCTRL0: usize = 0x1b10;
const TH1520_CTRL_DCH1_MRCTRL1: usize = 0x1b14;
const TH1520_CTRL_DCH1_MRSTAT: u32 = 0x1b18;
const TH1520_CTRL_DCH1_DERATECTL: usize = 0x1b2c;
const TH1520_CTRL_DCH1_PWRCTL: usize = 0x1b30;
const TH1520_CTRL_DCH1_HWLPCTL: usize = 0x1b38;
const TH1520_CTRL_DCH1_CRCPARCTL0: usize = 0x1bc0;
const TH1520_CTRL_DCH1_ZQCTL2: usize = 0x1c88;
const TH1520_CTRL_DCH1_DFISTAT: u32 = 0x1cbc;
const TH1520_CTRL_DCH1_ODTMAP: usize = 0x1d44;
const TH1520_CTRL_DCH1_DBG1: usize = 0x1e04;
const TH1520_CTRL_DCH1_DBGCMD: usize = 0x1e0c;
const TH1520_CTRL_DCH1_DBGCAM: u32 = 0x1e08;

#[derive(Debug, Clone, PartialEq)]
pub enum Bits {
    B32,
    B64,
}
pub fn init() {
    th1520_ddr_pll_config(TH1520_DDR_FREQ_3733);

    let mut reset = TH1520_SYS_DDR_CFG0_PHY_PWROK_RSTN;
    write32(TH1520_DDR_SYS_BASE + TH1520_SYS_DDR_CFG0, reset);
    reset |= TH1520_SYS_DDR_CFG0_PHY_CORE_RSTN;
    write32(TH1520_DDR_SYS_BASE + TH1520_SYS_DDR_CFG0, reset);
    reset |= TH1520_SYS_DDR_CFG0_APB_RSTN;
    write32(TH1520_DDR_SYS_BASE + TH1520_SYS_DDR_CFG0, reset);

    th1520_ddr_ctrl_init(2, TH1520_DDR_FREQ_3733, Bits::B64);

    reset |= bitn!(0) | bitn!(1) | bitn!(2) | bitn!(3) | bitn!(4) | TH1520_SYS_DDR_CFG0_CTRL_RSTN;
    write32(TH1520_DDR_SYS_BASE + TH1520_SYS_DDR_CFG0, reset);

    lpddr4_load_firmware();
}

// drivers/ram/thead/th1520_ddr.c: static int th1520_ddr_pll_config(void __iomem *sysreg, unsigned int frequency)
fn th1520_ddr_pll_config(freq: u8) {
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
    println!("[+] th1520_ddr_pll_config Complete...");
}

// drivers/ram/thead/th1520_ddr.c: static int th1520_ddr_ctrl_init(void __iomem *ctrlreg, struct th1520_ddr_fw *fw)
fn th1520_ddr_ctrl_init(ranknum: u8, freq: u8, bitwidth: Bits) {
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBG1, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PWRCTL, 0x00000001);

    // FIXME: check whether it's required, reading a bunch of stuff for some set duration
    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_STAT, tmp,
    //                          tmp == 0x00000000,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);

    if ranknum == 2 {
        write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_MSTR, 0x03080020);
    }

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_MRCTRL0, 0x00003030);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_MRCTRL1, 0x0002d90f);

    match freq {
        TH1520_DDR_FREQ_3733 => {
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DERATEEN, 0x000013f3);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DERATEINT, 0x40000000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DERATECTL, 0x00000001);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PWRCTL, 0x00000020);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PWRTMG, 0x0040ae04);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_HWLPCTL, 0x00430000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RFSHCTL0, 0x00210004);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RFSHCTL1, 0x000d0021);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RFSHCTL3, 0x00000001);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RFSHTMG, 0x81c00084);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RFSHTMG1, 0x00540000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_CRCPARCTL0, 0x00000000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT0, 0xc0020002);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT1, 0x00010002);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT2, 0x00001f00);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT3, 0x00640036);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT4, 0x00f20008);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT5, 0x0004000b);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT6, 0x00440012);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_INIT7, 0x0004001a);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DIMMCTL, 0x00000000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RANKCTL, 0x0000ab9f);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_RANKCTL1, 0x00000017);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG0, 0x1f263f28);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG1, 0x00080839);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG2, 0x08121d17);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG3, 0x00d0e000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG4, 0x11040a12);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG5, 0x02050e0e);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG6, 0x01010008);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG7, 0x00000502);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG8, 0x00000101);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG12, 0x00020000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG13, 0x0d100002);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DRAMTMG14, 0x0000010c);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ZQCTL0, 0x03a50021);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ZQCTL1, 0x02f00800);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ZQCTL2, 0x00000000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFITMG0, 0x059f820c);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFITMG1, 0x000c0303);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFILPCFG0, 0x0351a101);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIMISC, 0x00000011);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFITMG2, 0x00001f0c);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBICTL, 0x00000007);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIPHYMSTR, 0x14000001);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ODTCFG, 0x06090b40);
        }
        _ => print!("Invalid Frequency"),
    };

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIUPD0, 0x00400018);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIUPD1, 0x00280032);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIUPD2, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ODTMAP, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SCHED, 0x1f829b1c);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SCHED1, 0x4400b00f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PERFHPR1, 0x0f000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PERFLPR1, 0x0f00007f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PERFWR1, 0x0f00007f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SCHED3, 0x00000208);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SCHED4, 0x08400810);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBG0, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBG1, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBGCMD, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SWCTL, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SWCTLSTATIC, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_POISONCFG, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCTRL_0, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCTRL_1, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCTRL_2, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCTRL_3, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCTRL_4, 0x00000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_MRCTRL0, 0x00003030);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_MRCTRL1, 0x0002d90f);
    write32(
        TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_DERATECTL,
        0x00000001,
    );
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_PWRCTL, 0x00000020);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_HWLPCTL, 0x00430002);
    write32(
        TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_CRCPARCTL0,
        0x00000000,
    );
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_ZQCTL2, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_ODTMAP, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_DBG1, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_DBGCMD, 0x00000000);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_RFSHCTL3, tmp,
    //                          tmp == 0x00000001,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //  return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCCFG, 0x00000010);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGR_0, 0x0000500f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGW_0, 0x0000500f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGR_1, 0x00005020);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGW_1, 0x0000501f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGR_2, 0x0000501f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGW_2, 0x0000503f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGR_3, 0x000051ff);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGW_3, 0x000051ff);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGR_4, 0x0000503f);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PCFGW_4, 0x0000503f);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_PWRCTL, tmp,
    //                          tmp == 0x00000020,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //     return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PWRCTL, 0x00000020);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_DCH1_PWRCTL, tmp,
    //                          tmp == 0x00000020,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //     return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_PWRCTL, 0x00000020);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBG1, 0x00000000);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_PWRCTL, tmp,
    //                          tmp == 0x00000020,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //     return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PWRCTL, 0x00000020);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_PWRCTL, tmp,
    //                          tmp == 0x00000020,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //     return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_PWRCTL, 0x00000020);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_DBG1, 0x00000000);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_DCH1_PWRCTL, tmp,
    //                          tmp == 0x00000020,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //     return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_PWRCTL, 0x00000020);

    // ret = readl_poll_timeout(ctrlreg + TH1520_CTRL_DCH1_PWRCTL, tmp,
    //                          tmp == 0x00000020,
    //                          TH1520_CTRL_INIT_TIMEOUT_US);
    // if (ret)
    //     return ret;

    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_PWRCTL, 0x00000020);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIPHYMSTR, 0x14000001);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_SWCTL, 0x00000000);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIMISC, 0x00000010);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DFIMISC, 0x00000010);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DBG1, 0x00000002);
    write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_DCH1_DBG1, 0x00000002);

    match bitwidth {
        Bits::B64 => {
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP0, 0x00040018);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP1, 0x00090909);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP2, 0x00000000);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP3, 0x01010101);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP4, 0x00001f1f);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP5, 0x080f0808);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP6, 0x08080808);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP7, 0x00000f0f);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP9, 0x08080808);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP10, 0x08080808);
            write32(TH1520_DDR_CTRL_BASE + TH1520_CTRL_ADDRMAP11, 0x00000008);
        }
        _ => print!("Invalid bitwidth"),
    }

    println!("[+] th1520_ddr_ctrl_init Complete...");
}

// drivers/ram/thead/th1520_ddr.c: static int lpddr4_load_firmware(struct th1520_ddr_priv *priv, struct th1520_ddr_fw *fw)
fn lpddr4_load_firmware() {
    // PINMUX_CFG training data
    // PHY0 data
    for data in PINMUX_CFG_PHY0 {
        write16(TH1520_DDR_PHY0_BASE + data.addr, data.value);
    }

    // PHY1 data
    for data in PINMUX_CFG_PHY1 {
        write16(TH1520_DDR_PHY1_BASE + data.addr, data.value);
    }

    // PRE_CCM_LOADING training data
    for data in PRE_CCM_LOADING {
        write16(TH1520_DDR_PHY0_BASE + data.addr, data.value);
        write16(TH1520_DDR_PHY1_BASE + data.addr, data.value);
    }

    // ICCM_1D_TRAIN_FW training data
    for (i, &data) in ICCM_1D_TRAIN_FW.iter().enumerate() {
        write16(TH1520_DDR_PHY0_BASE + 0x50000 + i * 2, data);
        write16(TH1520_DDR_PHY1_BASE + 0x50000 + i * 2, data);
    }

    // unknown operation sequence
    // { op = "phy", addr = 0xd0000, data = 0x1 },
    write16(TH1520_DDR_PHY0_BASE + 0xd0000, 0x1);
    write16(TH1520_DDR_PHY1_BASE + 0xd0000, 0x1);
    // { op = "phy", addr = 0xd0000, data = 0x0 },
    write16(TH1520_DDR_PHY0_BASE + 0xd0000, 0x0);
    write16(TH1520_DDR_PHY1_BASE + 0xd0000, 0x0);

    // DCCM_1D_TRAIN_FW training data
    for (i, &data) in DCCM_1D_TRAIN_FW.iter().enumerate() {
        write16(TH1520_DDR_PHY0_BASE + 0x54000 + i * 2, data);
        write16(TH1520_DDR_PHY1_BASE + 0x54000 + i * 2, data);
    }

    // not implemented
    // --[[ Enable debug message of the ddr firmware ]]
    // -- { op = "phy", addr = 0x54009, data = 0x4 },

    // start the MCU
    for data in MCU_START {
        write16(TH1520_DDR_PHY0_BASE + data.addr, data.value);
        write16(TH1520_DDR_PHY1_BASE + data.addr, data.value);
    }

    // --[[ Wait for firmware completion ]]
    // { op = "waitphy0" },
    // { op = "waitphy1" },

    th1520_phy_wait_pmu_completion(TH1520_DDR_PHY0_BASE);

    println!("[+] lpddr4_load_firmware complete...");
}

fn th1520_phy_wait_pmu_completion(phy_mem: usize) {}
