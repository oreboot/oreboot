use core::ptr::{read_volatile, write_volatile};

// https://www.micron.com/-/media/client/global/Documents/Products/Technical%20Note/DRAM/TN4102.pdf
//
// FIXME
// - detects 512MB DRAM on MangoPi MQ-Pro pink with Elpida 8 Gbit DDR3
// (https://www.davychiu.com/wp-content/uploads/2016/11/E1958E30.pdf)
// when built with variant=nezha, 0MB for variant=lichee
// - not yet working on F133 (D1s)
// - untested on Nezha board
//
// - MangoPi MQ-Pro red, variant=lichee, Micron 1GB DD3 works fine
//   would also claim 512MB with variant=nezha

// for verbose prints
const VERBOSE: bool = true;

pub const RAM_BASE: usize = 0x40000000;

// p49 ff
const CCU: usize = 0x0200_1000;
const PLL_CPU_CTRL: usize = CCU + 0x0000;
const PLL_DDR_CTRL: usize = CCU + 0x0010;
const MBUS_CLK: usize = CCU + 0x0540;
const DRAM_CLK: usize = CCU + 0x0800;
const DRAM_BGR: usize = CCU + 0x080c;

/**
 * D1 manual p152 3.4 System Configuration
 *
 * SYS_CFG Base Address 0x03000000
 *
 * | Register Name       | Offset | Description                              |
 * | ------------------- | ------ | ---------------------------------------- |
 * | DSP_BOOT_RAMMAP_REG | 0x0008 | DSP Boot SRAM Remap Control Register     |
 * | VER_REG             | 0x0024 | Version Register                         |
 * | EMAC_EPHY_CLK_REG0  | 0x0030 | EMAC-EPHY Clock Register 0               |
 * | SYS_LDO_CTRL_REG    | 0x0150 | System LDO Control Register              |
 * | RESCAL_CTRL_REG     | 0x0160 | Resistor Calibration Control Register    |
 * | RES240_CTRL_REG     | 0x0168 | 240ohms Resistor Manual Control Register |
 * | RESCAL_STATUS_REG   | 0x016C | Resistor Calibration Status Register     |
 */

const SYS_CFG: usize = 0x0300_0000; // 0x0300_0000 - 0x0300_0FFF

// const VER_REG: usize = SYS_CFG + 0x0024;
// const EMAC_EPHY_CLK_REG0: usize = SYS_CFG + 0x0030;
const SYS_LDO_CTRL_REG: usize = SYS_CFG + 0x0150;
const RES_CAL_CTRL_REG: usize = SYS_CFG + 0x0160;
const RES240_CTRL_REG: usize = SYS_CFG + 0x0168;
const RES_CAL_STATUS_REG: usize = SYS_CFG + 0x016c;
// const ZQ_INTERNAL: usize = SYS_CFG + 0x016e;
const ZQ_VALUE: usize = SYS_CFG + 0x0172;

const BAR_BASE: usize = 0x0700_0000; // TODO: What do we call this?
const SOME_STATUS: usize = BAR_BASE + 0x05d4;

const FOO_BASE: usize = 0x0701_0000; // TODO: What do we call this?
const ANALOG_SYS_PWROFF_GATING_REG: usize = FOO_BASE + 0x0254;
const SOME_OTHER: usize = FOO_BASE + 0x0250;

const SID_BASE: usize = 0x0300_6200;
const SID_INFO: usize = SID_BASE + 0x0028;

// p32 memory mapping
// MSI + MEMC: 0x0310_2000 - 0x0330_1fff
// NOTE: MSI shares the bus clock with CE, DMAC, IOMMU and CPU_SYS; p 38
// TODO: Define *_BASE?
const MSI_MEMC_BASE: usize = 0x0310_2000; // p32 0x0310_2000 - 0x0330_1FFF

// PHY config registers; TODO: fix names
const WORK_MODE0: usize = MSI_MEMC_BASE;
const WORK_MODE1: usize = MSI_MEMC_BASE + 0x0004;

const DBGCR: usize = MSI_MEMC_BASE + 0x0008;
const MCTL_TMR: usize = MSI_MEMC_BASE + 0x000c;
const CCCR: usize = MSI_MEMC_BASE + 0x0014;

const DRAM_MASTER_CTL1: usize = MSI_MEMC_BASE + 0x0020;
const DRAM_MASTER_CTL2: usize = MSI_MEMC_BASE + 0x0024;
const DRAM_MASTER_CTL3: usize = MSI_MEMC_BASE + 0x0028;

// NOTE: From unused function `bit_delay_compensation` in the
// C code; could be for other platforms?
// const UNKNOWN6: usize = MSI_MEMC_BASE + 0x0100; // 0x3102100

// TODO:
// 0x0310_2200
// 0x0310_2210
// 0x0310_2214
// 0x0310_2230
// 0x0310_2234
// 0x0310_2240
// 0x0310_2244
// 0x0310_2260
// 0x0310_2264
// 0x0310_2290
// 0x0310_2294
// 0x0310_2470
// 0x0310_2474
// 0x0310_31c0
// 0x0310_31c8
// 0x0310_31d0

/*
// NOTE: From unused function `bit_delay_compensation` in the
// C code; could be for other platforms?
// DATX0IOCR x + 4 * size
// DATX0IOCR - DATX3IOCR: 11 registers per block, blocks 0x20 words apart
const DATX0IOCR: usize = MSI_MEMC_BASE + 0x0310; // 0x3102310
const DATX3IOCR: usize = MSI_MEMC_BASE + 0x0510; // 0x3102510
*/

const PHY_AC_MAP1: usize = MSI_MEMC_BASE + 0x0500;
const PHY_AC_MAP2: usize = MSI_MEMC_BASE + 0x0504;
const PHY_AC_MAP3: usize = MSI_MEMC_BASE + 0x0508;
const PHY_AC_MAP4: usize = MSI_MEMC_BASE + 0x050c;

// Regarding register names, see arch/arm32/mach-t113s3/reg-dram.h
// in https://github.com/szemzoa/awboot/

const MCTL_PHY_BASE: usize = MSI_MEMC_BASE + 0x1000;
const PIR: usize = MCTL_PHY_BASE;
const PHY_PWRCTL: usize = MCTL_PHY_BASE + 0x0004;
const MCTL_CLK: usize = MCTL_PHY_BASE + 0x000c;
const PGSR0: usize = MCTL_PHY_BASE + 0x0010;
const PGSR1: usize = MCTL_PHY_BASE + 0x0014;
const STATR: usize = MCTL_PHY_BASE + 0x0018;

const LP3MR11: usize = MCTL_PHY_BASE + 0x002c;
const DRAM_MR0: usize = MCTL_PHY_BASE + 0x0030;
const DRAM_MR1: usize = MCTL_PHY_BASE + 0x0034;
const DRAM_MR2: usize = MCTL_PHY_BASE + 0x0038;
const DRAM_MR3: usize = MCTL_PHY_BASE + 0x003c;

const PTR0: usize = MCTL_PHY_BASE + 0x0044;
const PTR1: usize = MCTL_PHY_BASE + 0x0048;
const PTR2: usize = MCTL_PHY_BASE + 0x004c;
const PTR3: usize = MCTL_PHY_BASE + 0x0050;
const PTR4: usize = MCTL_PHY_BASE + 0x0054;
const DRAMTMG0: usize = MCTL_PHY_BASE + 0x0058;
const DRAMTMG1: usize = MCTL_PHY_BASE + 0x005c;
const DRAMTMG2: usize = MCTL_PHY_BASE + 0x0060;
const DRAMTMG3: usize = MCTL_PHY_BASE + 0x0064;
const DRAMTMG4: usize = MCTL_PHY_BASE + 0x0068;
const DRAMTMG5: usize = MCTL_PHY_BASE + 0x006c;
const DRAMTMG6: usize = MCTL_PHY_BASE + 0x0070;
const DRAMTMG7: usize = MCTL_PHY_BASE + 0x0074;
const DRAMTMG8: usize = MCTL_PHY_BASE + 0x0078;
const ODT_CFG: usize = MCTL_PHY_BASE + 0x007c;
const PITMG0: usize = MCTL_PHY_BASE + 0x0080;
const PITMG1: usize = MCTL_PHY_BASE + 0x0084;
const LPTPR: usize = MCTL_PHY_BASE + 0x0088;
const RFSHCTL0: usize = MCTL_PHY_BASE + 0x008c;
const RFSHTMG: usize = MCTL_PHY_BASE + 0x0090;
const RFSHCTL1: usize = MCTL_PHY_BASE + 0x0094;
const PWRTMG: usize = MCTL_PHY_BASE + 0x0098;
const ASRC: usize = MCTL_PHY_BASE + 0x009c;
const ASRTC: usize = MCTL_PHY_BASE + 0x00a0;
const VTFCR: usize = MCTL_PHY_BASE + 0x00b8;
const DQSGMR: usize = MCTL_PHY_BASE + 0x00bc;
const DTCR: usize = MCTL_PHY_BASE + 0x00c0;

const PGCR0: usize = MCTL_PHY_BASE + 0x0100;
const PGCR1: usize = MCTL_PHY_BASE + 0x0104;
const PGCR2: usize = MCTL_PHY_BASE + 0x0108;
const PGCR3: usize = MCTL_PHY_BASE + 0x010c;
const IOCVR0: usize = MCTL_PHY_BASE + 0x0110;
const IOCVR1: usize = MCTL_PHY_BASE + 0x0114;
const DXCCR: usize = MCTL_PHY_BASE + 0x011c;
const ODTMAP: usize = MCTL_PHY_BASE + 0x0120;
const ZQCR: usize = MCTL_PHY_BASE + 0x0140;

const ACIOCR0: usize = MCTL_PHY_BASE + 0x0208;

const DATX0IOCR: usize = MCTL_PHY_BASE + 0x0310;

const DX0GCR0: usize = MCTL_PHY_BASE + 0x0344;
const DX0GSR0: usize = MCTL_PHY_BASE + 0x0348;

const DATX1IOCR: usize = MCTL_PHY_BASE + 0x0390;

const DX1GCR0: usize = MCTL_PHY_BASE + 0x03c4;
const DX1GSR0: usize = MCTL_PHY_BASE + 0x03c8;

// const DAT03IOCR: usize = MSI_MEMC_BASE + 0x1510; // 0x3103510

// TODO: *_BASE ?
const WORK_MODE_BASE: usize = 0x0320_0000;
const MC_WORK_MODE_RANK1_1: usize = WORK_MODE_BASE + 0x0000; // MC_WORK_MODE ?
const MC_WORK_MODE_RANK1_2: usize = WORK_MODE_BASE + 0x0004; // MC_WORK_MODE2 ?

#[repr(C)]
pub struct dram_parameters {
    pub dram_clk: u32,
    pub dram_type: u32,
    pub dram_zq: u32,
    pub dram_odt_en: u32,
    pub dram_para1: u32,
    pub dram_para2: u32,
    pub dram_mr0: u32,
    pub dram_mr1: u32,
    pub dram_mr2: u32,
    pub dram_mr3: u32,
    pub dram_tpr0: u32,
    pub dram_tpr1: u32,
    pub dram_tpr2: u32,
    pub dram_tpr3: u32,
    pub dram_tpr4: u32,
    pub dram_tpr5: u32, // IOCVR0
    pub dram_tpr6: u32, // IOCVR1
    pub dram_tpr7: u32,
    pub dram_tpr8: u32,
    pub dram_tpr9: u32,
    pub dram_tpr10: u32,
    pub dram_tpr11: u32,
    pub dram_tpr12: u32,
    pub dram_tpr13: u32,
}
// FIXME: This could be a concise struct. Let Rust piece it together.
/*
    //dram_tpr0
    tccd : [23:21]
    tfaw : [20:15]
    trrd : [14:11]
    trcd : [10:6 ]
    trc  : [ 5:0 ]

    //dram_tpr1
    txp  : [27:23]
    twtr : [22:20]
    trtp : [19:15]
    twr  : [14:11]
    trp  : [10:6 ]
    tras : [ 5:0 ]

    //dram_tpr2
    trfc : [20:12]
    trefi: [11:0 ]
*/

fn readl(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

fn writel(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

pub fn dump(addr: usize, length: usize) {
    let s = unsafe { core::slice::from_raw_parts(addr as *const u8, length) };
    println!("dump {length} bytes @{addr:x}");
    for w in s.iter() {
        print!("{w:02x}");
    }
    println!();
}

pub fn dump_block(base: usize, size: usize, step_size: usize) {
    for b in (base..base + size).step_by(step_size) {
        dump(b, step_size);
    }
}

fn sdelay(micros: usize) {
    let millis = micros * 4000;
    unsafe {
        for _ in 0..millis {
            core::arch::asm!("nop")
        }
    }
}

fn get_pmu_exists() -> bool {
    false
}

#[rustfmt::skip]
const PHY_CFG0: [u32; 22] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
];
#[rustfmt::skip]
const PHY_CFG1: [u32; 22] = [
     1,  9,  3,  7,  8, 18,  4, 13,  5,  6, 10,
     2, 14, 12,  0,  0, 21, 17, 20, 19, 11, 22,
];
#[rustfmt::skip]
const PHY_CFG2: [u32; 22] = [
     4,  9,  3,  7,  8, 18,  1, 13,  2,  6, 10,
     5, 14, 12,  0,  0, 21, 17, 20, 19, 11, 22,
];
#[rustfmt::skip]
const PHY_CFG3: [u32; 22] = [
     1,  7,  8, 12, 10, 18,  4, 13,  5,  6,  3,
     2,  9,  0,  0,  0, 21, 17, 20, 19, 11, 22,
];
#[rustfmt::skip]
const PHY_CFG4: [u32; 22] = [
     4, 12, 10,  7,  8, 18,  1, 13,  2,  6,  3,
     5,  9,  0,  0,  0, 21, 17, 20, 19, 11, 22,
];
#[rustfmt::skip]
const PHY_CFG5: [u32; 22] = [
    13,  2,  7,  9, 12, 19,  5,  1,  6,  3,  4,
     8, 10,  0,  0,  0, 21, 22, 18, 17, 11, 20,
];
#[rustfmt::skip]
const PHY_CFG6: [u32; 22] = [
     3, 10,  7, 13,  9, 11,  1,  2,  4,  6,  8,
     5, 12,  0,  0,  0, 20,  1,  0, 21, 22, 17,
];
#[rustfmt::skip]
const PHY_CFG7: [u32; 22] = [
     3,  2,  4,  7,  9,  1, 17, 12, 18, 14, 13,
     8, 15,  6, 10,  5, 19, 22, 16, 21, 20, 11,
];

// TODO: verify
// This routine seems to have several remapping tables for 22 lines.
// It is unclear which lines are being remapped.
// It seems to pick table PHY_CFG7 for the Nezha board.
unsafe fn mctl_phy_ac_remapping(para: &mut dram_parameters) {
    /*
     * It is unclear whether the LPDDRx types don't need any remapping,
     * or whether the original code just didn't provide tables.
     */
    if para.dram_type != 2 && para.dram_type != 3 {
        return;
    }
    let fuse = (readl(SID_INFO) & 0xf00) >> 8;
    println!("DDR efuse: {fuse}");

    if para.dram_type == 2 && fuse == 15 {
        return;
    }

    let cfg = if para.dram_type == 2 {
        println!("PHY cfg 6");
        PHY_CFG6
    } else if para.dram_tpr13 & 0xc0000 > 0 {
        println!("PHY cfg 7");
        PHY_CFG7
    } else {
        match fuse {
            8 => PHY_CFG2,
            9 => PHY_CFG3,
            10 => PHY_CFG5,
            11 => PHY_CFG4,
            13 | 14 => PHY_CFG0,
            12 | _ => PHY_CFG1,
        }
    };

    let val = (cfg[4] << 25) | (cfg[3] << 20) | (cfg[2] << 15) // fmt comment
        | (cfg[1] << 10) | (cfg[0] << 5);
    writel(PHY_AC_MAP1, val as u32);

    let val = (cfg[10] << 25) | (cfg[9] << 20) | (cfg[8] << 15) // x
        | (cfg[7] << 10) | (cfg[6] << 5) | cfg[5];
    writel(PHY_AC_MAP2, val as u32);

    let val = (cfg[15] << 20) | (cfg[14] << 15) | (cfg[13] << 10) // x
        | (cfg[12] << 5) | cfg[11];
    writel(PHY_AC_MAP3, val as u32);

    let val = (cfg[21] << 25) | (cfg[20] << 20) | (cfg[19] << 15) // x
        | (cfg[18] << 10) | (cfg[17] << 5) | cfg[16];
    writel(PHY_AC_MAP4, val as u32);

    let val = (cfg[4] << 25) | (cfg[3] << 20) | (cfg[2] << 15) // x
        | (cfg[1] << 10) | (cfg[0] << 5) | 1;
    writel(PHY_AC_MAP1, val as u32);
}

fn dram_vol_set(dram_para: &mut dram_parameters) {
    let vol = match dram_para.dram_type {
        2 => 47, // 1.8V
        3 => 25, // 1.5V
        _ => 0,
    };
    let mut reg = readl(SYS_LDO_CTRL_REG);
    reg &= !(0xff00);
    reg |= vol << 8;
    reg &= !(0x200000);
    writel(SYS_LDO_CTRL_REG, reg);
    sdelay(1);
}

fn set_ddr_voltage(val: usize) -> usize {
    val
}

fn handler_super_standby() {}

fn dram_enable_all_master() {
    writel(DRAM_MASTER_CTL1, 0xffffffff);
    writel(DRAM_MASTER_CTL2, 0xff);
    writel(DRAM_MASTER_CTL3, 0xffff);
    sdelay(10);
}

fn dram_disable_all_master() {
    writel(DRAM_MASTER_CTL1, 1);
    writel(DRAM_MASTER_CTL2, 0);
    writel(DRAM_MASTER_CTL3, 0);
    sdelay(10);
}

// Purpose of this routine seems to be to initialize the PLL driving
// the MBUS and sdram.
fn ccm_set_pll_ddr_clk(para: &mut dram_parameters) -> u32 {
    // FIXME: This is a bit weird, especially the scaling down and up etc
    let clk = if para.dram_tpr13 & (1 << 6) != 0 {
        para.dram_tpr9
    } else {
        para.dram_clk
    };
    let n = (clk * 2) / 24;
    println!("clk {clk} / div {n}");

    // set VCO clock divider
    let mut val = readl(PLL_DDR_CTRL);
    val &= 0xfff800fc; // clear dividers
    val |= (n - 1) << 8; // set PLL division
    val |= 0xc0000000; // enable PLL and LDO
    writel(PLL_DDR_CTRL, val);

    // Restart PLL locking
    val &= 0xdfffffff; // disbable lock
    val |= 0xc0000000; // enable PLL and LDO
    writel(PLL_DDR_CTRL, val);
    val |= 0xe0000000; // re-enable lock
    writel(PLL_DDR_CTRL, val);

    // wait for PLL to lock
    while readl(PLL_DDR_CTRL) == 0 {}
    sdelay(20);

    // enable PLL output
    let val = readl(PLL_CPU_CTRL);
    writel(PLL_CPU_CTRL, val | 0x08000000);

    // turn clock gate on
    let mut val = readl(DRAM_CLK);
    val &= 0xfcfffcfc; // select DDR clk source, n=1, m=1
    val &= 0xfcfffce0; // select DDR clk source, n=1, m=1
    val |= 1 << 31; // turn clock on
    writel(DRAM_CLK, val);

    n * 24
}

// Set up the PLL and clock gates for the DRAM controller and MBUS clocks.
// TODO: verify this
fn mctl_sys_init(para: &mut dram_parameters) {
    // assert MBUS reset
    writel(MBUS_CLK, readl(MBUS_CLK) & 0xbfffffff);
    // turn off sdram clock gate, assert sdram reset
    writel(DRAM_BGR, readl(DRAM_BGR) & 0xfffefffe);
    // toggle dram clock gating off, turn off bit 30 [??] + trigger update
    writel(DRAM_CLK, (readl(DRAM_CLK) & 0x3fffffff) | (1 << 27));
    sdelay(10);

    // set ddr pll clock
    // NOTE: This passes an additional `0` in the original, but it's unused
    let clk = ccm_set_pll_ddr_clk(para) >> 1;
    println!("new clock: {clk}");
    para.dram_clk = clk;
    sdelay(100);
    dram_disable_all_master();

    // release sdram reset
    writel(DRAM_BGR, readl(DRAM_BGR) | (1 << 16));
    // release MBUS reset
    writel(MBUS_CLK, readl(MBUS_CLK) | (1 << 30));
    // turn DRAM clock bit 30 back on [?]
    writel(DRAM_CLK, readl(DRAM_CLK) | (1 << 30));
    sdelay(5);

    // turn on sdram clock gate
    writel(DRAM_BGR, readl(DRAM_BGR) | (1 << 0));
    // turn dram clock gate on, trigger sdr clock update
    writel(DRAM_CLK, readl(DRAM_CLK) | (1 << 31) | (1 << 27));
    sdelay(5);

    // mCTL clock enable
    writel(MCTL_CLK, 1 << 15);
    sdelay(10);
}

// Set the Vref mode for the controller
fn mctl_vrefzq_init(para: &mut dram_parameters) {
    if (para.dram_tpr13 & (1 << 17)) == 0 {
        println!("set IOCVR0");
        let val = readl(IOCVR0) & 0x80808080;
        writel(IOCVR0, val | para.dram_tpr5 as u32);

        if (para.dram_tpr13 & (1 << 16)) == 0 {
            println!("set IOCVR1");
            let val = readl(IOCVR1) & 0xffffff80;
            writel(IOCVR1, val | para.dram_tpr6 as u32 & 0x7f);
        }
    }
}

// TODO: recheck this
// The main purpose of this routine seems to be to copy an address configuration
// from the dram_para1 and dram_para2 fields to the PHY configuration registers
// (0x3102000, 0x3102004).
fn mctl_com_init(para: &mut dram_parameters) {
    println!("mctl_com_init");
    // purpose ??
    writel(DBGCR, (readl(DBGCR) & 0xffffc0ff) | 1 << 13);

    println!("| set type and word width");
    // Set sdram type and word width
    let mut val = readl(WORK_MODE0);
    println!("|   WORK_MODE0 old {val:08x}");
    val = val & 0xff000fff;
    val |= (para.dram_type & 0x7) << 16; // DRAM type
    val |= (!para.dram_para2 & 0x1) << 12; // DQ width

    match para.dram_type {
        6 | 7 => {
            // type 6 and 7 (LPDDR) must use 1T
            val |= 1 << 19;
        }
        _ => {
            // 2T or 1T
            if (para.dram_tpr13 & (1 << 5)) > 0 {
                val |= 1 << 19;
            }
        }
    }
    println!("|   WORK_MODE0 new {val:08x}");
    writel(WORK_MODE0, val);

    // init rank / bank / row for single/dual or two different ranks
    let val = para.dram_para2;
    let rank = if (val & (1 << 8)) != 0 && val & 0xf000 != 0x1000 {
        2
    } else {
        1
    };

    println!("| ranks: {rank}");

    for i in 0..rank {
        let ptr = WORK_MODE0 + i * 4;
        let mut val = readl(ptr) & 0xfffff000;

        val |= (para.dram_para2 >> 12) & 0x3; // rank
        val |= ((para.dram_para1 >> (i * 16 + 12)) << 2) & 0x4; // bank - 2
        val |= (((para.dram_para1 >> (i * 16 + 4)) - 1) << 4) & 0xff; // row - 1

        // convert from page size to column addr width - 3
        val |= match (para.dram_para1 >> (i * 16)) & 0xf {
            8 => 0xa00,
            4 => 0x900,
            2 => 0x800,
            1 => 0x700,
            _ => 0x600,
        };
        writel(ptr, val);
    }

    // set ODTMAP based on number of ranks in use
    let odtmap = match readl(WORK_MODE0) & 0x1 {
        0 => 0x201,
        _ => 0x303,
    };
    writel(ODTMAP, odtmap);

    // set mctl reg 3c4 to zero when using half DQ
    if para.dram_para2 & (1 << 0) > 0 {
        writel(DX1GCR0, 0);
    }

    // purpose ??
    if para.dram_tpr4 > 0 {
        let s = (para.dram_tpr4 & 0x3) << 25;
        writel(WORK_MODE0, readl(WORK_MODE0) | s);
        let s = (para.dram_tpr4 & 0x7fc) << 10;
        writel(WORK_MODE1, readl(WORK_MODE1) | s);
    }
}

fn auto_cal_timing(time: u32, freq: u32) -> u32 {
    let t = time * freq;
    let what = if (t % 1000) != 0 { 1 } else { 0 };
    (t / 1000) + what
}

// Main purpose of the auto_set_timing routine seems to be to calculate all
// timing settings for the specific type of sdram used. Read together with
// an sdram datasheet for context on the various variables.
fn auto_set_timing_para(para: &mut dram_parameters) {
    let dfreq = para.dram_clk;
    let dtype = para.dram_type;
    let tpr13 = para.dram_tpr13;

    println!("auto set timing parameters");
    println!("  type : DDR{dtype}");
    println!("  clock: {dfreq}");
    println!("  tpr13: {tpr13:x}");

    // FIXME: Half of this is unused, wat?!
    let mut tccd: u32 = 0; // 88(sp)
    let mut trrd: u32 = 0; // s7
    let mut trcd: u32 = 0; // s3
    let mut trc: u32 = 0; // s9
    let mut tfaw: u32 = 0; // s10
    let mut tras: u32 = 0; // s11
    let mut trp: u32 = 0; // 0(sp)
    let mut twtr: u32 = 0; // s1
    let mut twr: u32 = 0; // s6
    let mut trtp: u32 = 0; // 64(sp)
    let mut txp: u32 = 0; // a6
    let mut trefi: u32 = 0; // s2
    let mut trfc: u32 = 0; // a5 / 8(sp)

    if para.dram_tpr13 & 0x2 != 0 {
        //dram_tpr0
        tccd = (para.dram_tpr0 >> 21) & 0x7; // [23:21]
        tfaw = (para.dram_tpr0 >> 15) & 0x3f; // [20:15]
        trrd = (para.dram_tpr0 >> 11) & 0xf; // [14:11]
        trcd = (para.dram_tpr0 >> 6) & 0x1f; // [10:6 ]
        trc = (para.dram_tpr0 >> 0) & 0x3f; // [ 5:0 ]

        //dram_tpr1
        txp = (para.dram_tpr1 >> 23) & 0x1f; // [27:23]
        twtr = (para.dram_tpr1 >> 20) & 0x7; // [22:20]
        trtp = (para.dram_tpr1 >> 15) & 0x1f; // [19:15]
        twr = (para.dram_tpr1 >> 11) & 0xf; // [14:11]
        trp = (para.dram_tpr1 >> 6) & 0x1f; // [10:6 ]
        tras = (para.dram_tpr1 >> 0) & 0x3f; // [ 5:0 ]

        //dram_tpr2
        trfc = (para.dram_tpr2 >> 12) & 0x1ff; // [20:12]
        trefi = (para.dram_tpr2 >> 0) & 0xfff; // [11:0 ]
    } else {
        let frq2 = dfreq >> 1; // s0
        match dtype {
            // DDR3
            3 => {
                trfc = auto_cal_timing(350, frq2);
                trefi = auto_cal_timing(7800, frq2) / 32 + 1; // XXX
                twr = auto_cal_timing(8, frq2);
                twtr = if twr < 2 { 2 } else { twr + 2 }; // + 2 ? XXX
                trcd = auto_cal_timing(15, frq2);
                twr = if trcd < 2 { 2 } else { trcd };
                if dfreq <= 800 {
                    tfaw = auto_cal_timing(50, frq2);
                    let trrdc = auto_cal_timing(10, frq2);
                    trrd = if trrd < 2 { 2 } else { trrdc };
                    trc = auto_cal_timing(53, frq2);
                    tras = auto_cal_timing(38, frq2);
                    txp = trrd; // 10
                    trp = trcd; // 15
                }
            }
            // DDR2
            2 => {
                tfaw = auto_cal_timing(50, frq2);
                trrd = auto_cal_timing(10, frq2);
                trcd = auto_cal_timing(20, frq2);
                trc = auto_cal_timing(65, frq2);
                twtr = auto_cal_timing(8, frq2);
                trp = auto_cal_timing(15, frq2);
                tras = auto_cal_timing(45, frq2);
                trefi = auto_cal_timing(7800, frq2) / 32;
                trfc = auto_cal_timing(328, frq2);
                txp = 2;
                twr = trp; // 15
            }
            /*
            // LPDDR2
            6 => {
                tfaw = auto_cal_timing(50, frq2);
                if tfaw < 4 {
                    tfaw = 4
                };
                trrd = auto_cal_timing(10, frq2);
                if trrd == 0 {
                    trrd = 1
                };
                trcd = auto_cal_timing(24, frq2);
                if trcd < 2 {
                    trcd = 2
                };
                trc = auto_cal_timing(70, frq2);
                txp = auto_cal_timing(8, frq2);
                if txp == 0 {
                    txp = 1;
                    twtr = 2;
                } else {
                    twtr = txp;
                    if txp < 2 {
                        txp = 2;
                        twtr = 2;
                    }
                }
                twr = auto_cal_timing(15, frq2);
                if twr < 2 {
                    twr = 2
                };
                trp = auto_cal_timing(17, frq2);
                tras = auto_cal_timing(42, frq2);
                trefi = auto_cal_timing(3900, frq2) / 32;
                trfc = auto_cal_timing(210, frq2);
            }
            // LPDDR3
            7 => {
                tfaw = auto_cal_timing(50, frq2);
                if tfaw < 4 {
                    tfaw = 4
                };
                trrd = auto_cal_timing(10, frq2);
                if trrd == 0 {
                    trrd = 1
                };
                trcd = auto_cal_timing(24, frq2);
                if trcd < 2 {
                    trcd = 2
                };
                trc = auto_cal_timing(70, frq2);
                twtr = auto_cal_timing(8, frq2);
                if twtr < 2 {
                    twtr = 2
                };
                twr = auto_cal_timing(15, frq2);
                if twr < 2 {
                    twr = 2
                };
                trp = auto_cal_timing(17, frq2);
                tras = auto_cal_timing(42, frq2);
                trefi = auto_cal_timing(3900, frq2) / 32;
                trfc = auto_cal_timing(210, frq2);
                txp = twtr;
            }
            _ => {
                // default
                trfc = 128;
                trp = 6;
                trefi = 98;
                txp = 10;
                twr = 8;
                twtr = 3;
                tras = 14;
                tfaw = 16;
                trc = 20;
                trcd = 6;
                trrd = 3;
            }
            */
            _ => {}
        }
        //assign the value back to the DRAM structure
        tccd = 2;
        trtp = 4; // not in .S ?
        para.dram_tpr0 = (trc << 0) | (trcd << 6) | (trrd << 11) | (tfaw << 15) | (tccd << 21);
        para.dram_tpr1 =
            (tras << 0) | (trp << 6) | (twr << 11) | (trtp << 15) | (twtr << 20) | (txp << 23);
        para.dram_tpr2 = (trefi << 0) | (trfc << 12);
    }

    let tcksrx: u32; // t1
    let tckesr: u32; // t4;
    let mut trd2wr: u32; // t6
    let trasmax: u32; // t3;
    let twtp: u32; // s6 (was twr!)
    let tcke: u32; // s8
    let tmod: u32; // t0
    let tmrd: u32; // t5
    let tmrw: u32; // a1
    let t_rdata_en: u32; // a4 (was tcwl!)
    let tcl: u32; // a0
    let wr_latency: u32; // a7
    let tcwl: u32; // first a4, then a5
    let mr3: u32; // s0
    let mr2: u32; // t2
    let mr1: u32; // s1
    let mr0: u32; // a3

    //let dmr3: u32; // 72(sp)
    //let trtp:u32;	// 64(sp)
    //let dmr1: u32; // 56(sp)
    let twr2rd: u32; // 48(sp)
    let tdinit3: u32; // 40(sp)
    let tdinit2: u32; // 32(sp)
    let tdinit1: u32; // 24(sp)
    let tdinit0: u32; // 16(sp)

    let dmr1 = para.dram_mr1;
    let dmr3 = para.dram_mr3;

    match dtype {
        // DDR2
        2 =>
        //	L59:
        {
            trasmax = dfreq / 30;
            if dfreq < 409 {
                tcl = 3;
                t_rdata_en = 1;
                mr0 = 0x06a3;
            } else {
                t_rdata_en = 2;
                tcl = 4;
                mr0 = 0x0e73;
            }
            tmrd = 2;
            twtp = twr + 5;
            tcksrx = 5;
            tckesr = 4;
            trd2wr = 4;
            tcke = 3;
            tmod = 12;
            wr_latency = 1;
            mr3 = 0;
            mr2 = 0;
            tdinit0 = 200 * dfreq + 1;
            tdinit1 = 100 * dfreq / 1000 + 1;
            tdinit2 = 200 * dfreq + 1;
            tdinit3 = 1 * dfreq + 1;
            tmrw = 0;
            twr2rd = twtr + 5;
            tcwl = 0;
            mr1 = dmr1;
        }
        // DDR3
        3 =>
        //	L57:
        {
            trasmax = dfreq / 30;
            if dfreq <= 800 {
                mr0 = 0x1c70;
                tcl = 6;
                wr_latency = 2;
                tcwl = 4;
                mr2 = 24;
            } else {
                mr0 = 0x1e14;
                tcl = 7;
                wr_latency = 3;
                tcwl = 5;
                mr2 = 32;
            }

            twtp = tcwl + 2 + twtr as u32; // WL+BL/2+tWTR
            trd2wr = tcwl + 2 + twr as u32; // WL+BL/2+tWR
            twr2rd = tcwl + twtr as u32; // WL+tWTR

            tdinit0 = 500 * dfreq + 1; // 500 us
            tdinit1 = 360 * dfreq / 1000 + 1; // 360 ns
            tdinit2 = 200 * dfreq + 1; // 200 us
            tdinit3 = 1 * dfreq + 1; //   1 us

            mr1 = dmr1;
            t_rdata_en = tcwl; // a5 <- a4
            tcksrx = 5;
            tckesr = 4;
            trd2wr = if ((tpr13 >> 2) & 0x03) == 0x01 || dfreq < 912 {
                5
            } else {
                6
            };
            tcke = 3; // not in .S ?
            tmod = 12;
            tmrd = 4;
            tmrw = 0;
            mr3 = 0;
        }
        /*
        // LPDDR2
        6 =>
        //	L61:
        {
            trasmax = dfreq / 60;
            mr3 = dmr3;
            twtp = twr as u32 + 5;
            mr2 = 6;
            //  mr1 = 5; // TODO: this is just overwritten (?!)
            tcksrx = 5;
            tckesr = 5;
            trd2wr = 10;
            tcke = 2;
            tmod = 5;
            tmrd = 5;
            tmrw = 3;
            tcl = 4;
            wr_latency = 1;
            t_rdata_en = 1;
            tdinit0 = 200 * dfreq + 1;
            tdinit1 = 100 * dfreq / 1000 + 1;
            tdinit2 = 11 * dfreq + 1;
            tdinit3 = 1 * dfreq + 1;
            twr2rd = twtr as u32 + 5;
            tcwl = 2;
            mr1 = 195;
            mr0 = 0;
        }
        // LPDDR3
        7 =>
        {
            trasmax = dfreq / 60;
            if dfreq < 800 {
                tcwl = 4;
                wr_latency = 3;
                t_rdata_en = 6;
                mr2 = 12;
            } else {
                tcwl = 3;
                // tcke = 6; // FIXME: This is always overwritten
                wr_latency = 2;
                t_rdata_en = 5;
                mr2 = 10;
            }
            twtp = tcwl + 5;
            tcl = 7;
            mr3 = dmr3;
            tcksrx = 5;
            tckesr = 5;
            trd2wr = 13;
            tcke = 3;
            tmod = 12;
            tdinit0 = 400 * dfreq + 1;
            tdinit1 = 500 * dfreq / 1000 + 1;
            tdinit2 = 11 * dfreq + 1;
            tdinit3 = 1 * dfreq + 1;
            tmrd = 5;
            tmrw = 5;
            twr2rd = tcwl + twtr as u32 + 5;
            mr1 = 195;
            mr0 = 0;
        }
        */
        _ =>
        //	L84:
        {
            twr2rd = 8; // 48(sp)
            tcksrx = 4; // t1
            tckesr = 3; // t4
            trd2wr = 4; // t6
            trasmax = 27; // t3
            twtp = 12; // s6
            tcke = 2; // s8
            tmod = 6; // t0
            tmrd = 2; // t5
            tmrw = 0; // a1
            tcwl = 3; // a5
            tcl = 3; // a0
            wr_latency = 1; // a7
            t_rdata_en = 1; // a4
            mr3 = 0; // s0
            mr2 = 0; // t2
            mr1 = 0; // s1
            mr0 = 0; // a3
            tdinit3 = 0; // 40(sp)
            tdinit2 = 0; // 32(sp)
            tdinit1 = 0; // 24(sp)
            tdinit0 = 0; // 16(sp)
        }
    }
    // L60:
    /*
    if trtp < tcl - trp + 2 {
        trtp = tcl - trp + 2;
    }
    */
    // FIXME: This always overwrites the above (?!)
    trtp = 4;

    // Update mode block when permitted
    if (para.dram_mr0 & 0xffff0000) == 0 {
        para.dram_mr0 = mr0
    };
    if (para.dram_mr1 & 0xffff0000) == 0 {
        para.dram_mr1 = mr1
    };
    if (para.dram_mr2 & 0xffff0000) == 0 {
        para.dram_mr2 = mr2
    };
    if (para.dram_mr3 & 0xffff0000) == 0 {
        para.dram_mr3 = mr3
    };

    // Set mode registers
    writel(DRAM_MR0, mr0);
    writel(DRAM_MR1, mr1);
    writel(DRAM_MR2, mr2);
    writel(DRAM_MR3, mr3);
    // TODO: dram_odt_en is either 0x0 or 0x1, so right shift looks weird
    writel(LP3MR11, (para.dram_odt_en >> 4) & 0x3);

    // Set dram timing DRAMTMG0 - DRAMTMG5
    let v = (twtp << 24) | (tfaw << 16) | (trasmax << 8) | tras;
    writel(DRAMTMG0, v);
    let v = (txp << 16) | (trtp << 8) | trc;
    writel(DRAMTMG1, v);
    let v = (tcwl << 24) | (tcl << 16) | (trd2wr << 8) | twr2rd;
    writel(DRAMTMG2, v);
    let v = (tmrw << 16) | (tmrd << 12) | tmod;
    writel(DRAMTMG3, v);
    let v = (trcd << 24) | (tccd << 16) | (trrd << 8) | trp;
    writel(DRAMTMG4, v);
    let v = (tcksrx << 24) | (tcksrx << 16) | (tckesr << 8) | tcke;
    writel(DRAMTMG5, v);

    // Set two rank timing
    let v = readl(DRAMTMG8) & 0x0fff0000;
    let m = if para.dram_clk < 800 {
        0xf0006610
    } else {
        0xf0007610
    };
    writel(DRAMTMG8, v | m);

    // Set phy interface time PITMG0, PTR3, PTR4
    let v = (0x2 << 24) | (t_rdata_en << 16) | (1 << 8) | wr_latency;
    writel(PITMG0, v);
    writel(PTR3, (tdinit0 << 0) | (tdinit1 << 20));
    writel(PTR4, (tdinit2 << 0) | (tdinit3 << 20));

    // Set refresh timing and mode
    writel(RFSHTMG, (trefi << 16) | trfc);
    writel(RFSHCTL1, (trefi << 15) & 0x0fff0000);
}

fn eye_delay_compensation(para: &mut dram_parameters) {
    let mut val: u32;

    // DATn0IOCR
    for i in 0..9 {
        let ptr = DATX0IOCR + i * 4;
        val = readl(ptr);
        val |= (para.dram_tpr11 << 9) & 0x1e00;
        val |= (para.dram_tpr12 << 1) & 0x001e;
        writel(ptr, val);
    }

    // DATn1IOCR
    for i in 0..9 {
        let ptr = DATX1IOCR + i * 4;
        val = readl(ptr);
        val |= ((para.dram_tpr11 >> 4) << 9) & 0x1e00;
        val |= ((para.dram_tpr12 >> 4) << 1) & 0x001e;
        writel(ptr, val);
    }

    // PGCR0: assert AC loopback FIFO reset
    val = readl(PGCR0);
    writel(PGCR0, val & 0xfbffffff);

    // ??
    val = readl(0x3103334);
    val |= ((para.dram_tpr11 >> 16) << 9) & 0x1e00;
    val |= ((para.dram_tpr12 >> 16) << 1) & 0x001e;
    writel(0x3103334, val);

    val = readl(0x3103338);
    val |= ((para.dram_tpr11 >> 16) << 9) & 0x1e00;
    val |= ((para.dram_tpr12 >> 16) << 1) & 0x001e;
    writel(0x3103338, val);

    val = readl(0x31033b4);
    val |= ((para.dram_tpr11 >> 20) << 9) & 0x1e00;
    val |= ((para.dram_tpr12 >> 20) << 1) & 0x001e;
    writel(0x31033b4, val);

    val = readl(0x31033b8);
    val |= ((para.dram_tpr11 >> 20) << 9) & 0x1e00;
    val |= ((para.dram_tpr12 >> 20) << 1) & 0x001e;
    writel(0x31033b8, val);

    val = readl(0x310333c);
    val |= ((para.dram_tpr11 >> 16) << 25) & 0x1e000000;
    writel(0x310333c, val);

    val = readl(0x31033bc);
    val |= ((para.dram_tpr11 >> 20) << 25) & 0x1e000000;
    writel(0x31033bc, val);

    // PGCR0: release AC loopback FIFO reset
    val = readl(PGCR0);
    writel(PGCR0, val | 0x04000000);

    sdelay(1);

    // TODO: unknown regs
    // NOTE: dram_tpr10 is set to 0x0 for D1
    for i in 0..15 {
        let ptr = 0x3103240 + i * 4;
        val = readl(ptr);
        val |= ((para.dram_tpr10 >> 4) << 8) & 0x0f00;
        writel(ptr, val);
    }

    for i in 0..6 {
        let ptr = 0x3103228 + i * 4;
        val = readl(ptr);
        val |= ((para.dram_tpr10 >> 4) << 8) & 0x0f00;
        writel(ptr, val);
    }

    let val = readl(0x3103218);
    writel(0x3103218, val | (para.dram_tpr10 << 8) & 0x0f00);
    let val = readl(0x310321c);
    writel(0x310321c, val | (para.dram_tpr10 << 8) & 0x0f00);
    let val = readl(0x3103280);
    writel(0x3103280, val | ((para.dram_tpr10 >> 12) << 8) & 0x0f00);
}

// Init the controller channel. The key part is placing commands in the main
// command register (PIR, 0x3103000) and checking command status (PGSR0, 0x3103010).
fn mctl_channel_init(para: &mut dram_parameters) -> Result<(), &'static str> {
    let dqs_gating_mode = (para.dram_tpr13 >> 2) & 0x3;
    let mut val;

    // set DDR clock to half of CPU clock
    val = (readl(MCTL_TMR) & 0xfffff000) | (para.dram_clk >> 1) - 1;
    writel(MCTL_TMR, val);

    // PGCR2 nibble 3 undocumented
    val = readl(PGCR2) & 0xfffff0ff;
    writel(PGCR2, val | 0x300);

    // DX0GCR0
    val = readl(DX0GCR0) & 0xffffffcf;
    val |= ((!para.dram_odt_en) << 5) & 0x20;
    if para.dram_clk > 672 {
        val &= 0xffff09f1;
        val |= 0x00000400;
    } else {
        val &= 0xffff0ff1;
    }
    writel(DX0GCR0, val);

    val = readl(DX1GCR0) & 0xffffffcf;
    val |= ((!para.dram_odt_en) << 5) & 0x20;
    if para.dram_clk > 672 {
        val &= 0xffff09f1;
        val |= 0x00000400;
    } else {
        val &= 0xffff0ff1;
    }
    writel(DX1GCR0, val);

    writel(ACIOCR0, readl(ACIOCR0) | (1 << 1));

    eye_delay_compensation(para);

    //set PLL SSCG ?
    val = readl(PGCR2);
    const PLL_SSCG_X: usize = 0x31030bc;
    match dqs_gating_mode {
        1 => {
            val &= !(0xc0); // FIXME
            writel(PGCR2, val);
            let val = readl(PLL_SSCG_X);
            writel(PLL_SSCG_X, val & 0xfffffef8);
        }
        2 => {
            val &= !(0xc0); // FIXME
            val |= 0x80;
            writel(PGCR2, val);

            let mut val = readl(PLL_SSCG_X);
            val &= 0xfffffef8;
            val |= ((para.dram_tpr13 >> 16) & 0x1f) - 2;
            val |= 0x100;
            writel(PLL_SSCG_X, val);

            let val = readl(DXCCR) & 0x7fffffff;
            writel(DXCCR, val | 0x08000000);
        }
        _ => {
            val &= !(0x40); // FIXME
            writel(PGCR2, val);
            sdelay(10);

            let val = readl(PGCR2);
            writel(PGCR2, val | 0xc0);
        }
    }

    /*
    if para.dram_type == 6 || para.dram_type == 7 {
        let val = readl(DXCCR);
        if dqs_gating_mode == 1 {
            val &= 0xf7ffff3f;
            val |= 0x80000000;
        } else {
            val &= 0x88ffffff;
            val |= 0x22000000;
        }
        writel(DXCCR, val);
    }
    */

    val = readl(DTCR);
    val &= 0xf0000000;
    val |= if para.dram_para2 & (1 << 12) > 0 {
        0x03000001
    } else {
        0x01000007
    }; // 0x01003087 XXX
    writel(DTCR, val);

    if readl(SOME_STATUS) & (1 << 16) > 0 {
        val = readl(SOME_OTHER);
        writel(SOME_OTHER, val & 0xfffffffd);
        sdelay(10);
    }

    // Set ZQ config
    val = readl(ZQCR) & 0xfc000000;
    val |= para.dram_zq & 0x00ffffff;
    val |= 0x02000000;
    writel(ZQCR, val);

    // Initialise DRAM controller
    val = if dqs_gating_mode == 1 {
        writel(PIR, 0x52); // prep PHY reset + PLL init + z-cal
        writel(PIR, 0x53); // Go

        while (readl(PGSR0) & 0x1) == 0 {} // wait for IDONE
        sdelay(10);

        // 0x520 = prep DQS gating + DRAM init + d-cal
        if para.dram_type == 3 {
            0x5a0
        }
        // + DRAM reset
        else {
            0x520
        }
    } else {
        if (readl(SOME_STATUS) & (1 << 16)) == 0 {
            // prep DRAM init + PHY reset + d-cal + PLL init + z-cal
            if para.dram_type == 3 {
                0x1f2
            }
            // + DRAM reset
            else {
                0x172
            }
        } else {
            // prep PHY reset + d-cal + z-cal
            0x62
        }
    };

    writel(PIR, val); // Prep
    writel(PIR, val | 1); // Go
    sdelay(10);

    while (readl(PGSR0) & 0x1) == 0 {} // wait for IDONE

    if readl(SOME_STATUS) & (1 << 16) > 0 {
        val = readl(PGCR3);
        val &= 0xf9ffffff;
        val |= 0x04000000;
        writel(PGCR3, val);
        sdelay(10);

        val = readl(PHY_PWRCTL);
        writel(PHY_PWRCTL, val | 0x1);
        while (readl(STATR) & 0x7) != 0x3 {}

        val = readl(SOME_OTHER);
        writel(SOME_OTHER, val & 0xfffffffe);
        sdelay(10);

        val = readl(PHY_PWRCTL);
        writel(PHY_PWRCTL, val & 0xfffffffe);
        while (readl(STATR) & 0x7) != 0x1 {}
        sdelay(15);

        if dqs_gating_mode == 1 {
            val = readl(PGCR2);
            val &= 0xffffff3f;
            writel(PGCR2, val);

            val = readl(PGCR3);
            val &= 0xf9ffffff;
            val |= 0x02000000;
            writel(PGCR3, val);

            sdelay(1);
            writel(PIR, 0x401);

            while (readl(PGSR0) & 0x1) == 0 {}
        }
    }

    // Check for training error
    val = readl(PGSR0);
    if ((val >> 20) & 0xff != 0) && (val & 0x100000) != 0 {
        // return Err("DRAM initialisation error : 0"); // TODO
        return Err("ZQ calibration error, check external 240 ohm resistor.");
    }

    // STATR = Zynq STAT? Wait for status 'normal'?
    while (readl(STATR) & 0x1) == 0 {}

    writel(RFSHCTL0, readl(RFSHCTL0) | (1 << 31));
    sdelay(10);
    writel(RFSHCTL0, readl(RFSHCTL0) & 0x7fffffff);
    sdelay(10);
    writel(CCCR, readl(CCCR) | (1 << 31));
    sdelay(10);
    writel(PGCR3, readl(PGCR3) & 0xf9ffffff);

    if dqs_gating_mode == 1 {
        writel(DXCCR, readl(DXCCR) & 0xffffff3f | (1 << 6));
    }
    Ok(())
}

// FIXME: Cannot you see that this could be more elegant?
// Perform an init of the controller. This is actually done 3 times. The first
// time to establish the number of ranks and DQ width. The second time to
// establish the actual ram size. The third time is final one, with the final
// settings.
fn mctl_core_init(para: &mut dram_parameters) -> Result<(), &'static str> {
    // FIXME
    mctl_sys_init(para);
    mctl_vrefzq_init(para);
    mctl_com_init(para);
    unsafe {
        mctl_phy_ac_remapping(para);
    }
    auto_set_timing_para(para);
    mctl_channel_init(para)
}

//  |     8..11    |     4..7     |      2..3     |    0..1    |
//  | page size -3 | row width -1 | bank count -2 | rank count |
fn work_mode_rank_to_size(v: u32) -> u32 {
    1 << (
        // page size - 3
        ((v >> 8) & 0xf)
        // row width - 1
        + ((v >> 4) & 0xf)
        // bank count - 2
        + ((v >> 2) & 0x3)
        // 1MB = 20 bits, minus above 6 = 14
        - 14
    )
}

// The below routine reads the dram config registers and extracts
// the number of address bits in each rank available. It then calculates
// total memory size in MB.
fn dramc_get_dram_size() -> u32 {
    let low = readl(WORK_MODE0);
    let size1 = work_mode_rank_to_size(low);
    if VERBOSE {
        println!("low {low} size {size1}");
    }
    // rank count = 0? -> done
    if low & 0x3 == 0 {
        return size1;
    }

    let high = readl(WORK_MODE1);
    // two identical ranks
    if high & 0x3 == 0 {
        return 2 * size1;
    }

    let size2 = work_mode_rank_to_size(high);
    if VERBOSE {
        println!("high {high} size {size2}");
    }
    // The total size is the sum of both ranks.
    size1 + size2
}

// NOTE:
// DQS = data strobe (clock signal for data lines)
// DQ = data signal
// "Q is some ancient notation"
// https://electronics.stackexchange.com/questions/408458/data-strobe-in-ddr-memory

// The below routine reads the command status register to extract
// DQ width and rank count. This follows the DQS training command in
// channel_init. If error bit 22 is reset, we have two ranks and full DQ.
// If there was an error, figure out whether it was half DQ, single rank,
// or both. Set bit 12 and 0 in dram_para2 with the results.
fn dqs_gate_detect(para: &mut dram_parameters) -> Result<&'static str, &'static str> {
    if readl(PGSR0) & (1 << 22) == 0 {
        let mut rval = para.dram_para2;
        rval &= 0xfffffff0;
        rval |= 0x00001000;
        para.dram_para2 = rval;
        return Ok("dual rank, full DQ");
    }
    let dx0gsr0 = readl(DX0GSR0);
    let dx1gsr0 = readl(DX1GSR0);
    let dx0 = (dx0gsr0 & (0x3 << 24)) >> 24;
    let dx1 = (dx1gsr0 & (0x3 << 24)) >> 24;
    println!("DX0 {dx0}   {dx0gsr0:x}");
    println!("DX1 {dx1}   {dx1gsr0:x}");
    if dx0 == 0 {
        let rval = (para.dram_para2 & 0xfffffff0) | 0x1001; // l 7918
        para.dram_para2 = rval;
        return Ok("dual rank, half DQ");
    }
    if dx0 == 2 {
        let v = para.dram_para2 & 0xffff0ff0;
        if dx1 == 2 {
            para.dram_para2 = v;
            // NOTE: D1 should do this here
            return Ok("single rank, full DQ");
        }
        para.dram_para2 = v | 0x1;
        return Ok("single rank, half DQ");
    }

    if para.dram_tpr13 & (1 << 29) == 0 {
        // l 7935
        // originally meant "skip dumping DX0/DX1 state"
        println!("🤷");
    }
    return Err("DQS gate detect");
}

fn dramc_simple_wr_test(mem_mb: u32, len: u32) -> Result<(), &'static str> {
    let offs: usize = (mem_mb as usize >> 1) << 18; // half of memory size
    let patt1: u32 = 0x01234567;
    let patt2: u32 = 0xfedcba98;

    for i in 0..len {
        let addr = RAM_BASE + 4 * i as usize;
        writel(addr, patt1 + i);
        writel(addr + offs, patt2 + i);
    }

    for i in 0..len {
        let addr = RAM_BASE + 4 * i as usize;
        let val = readl(addr);
        let exp = patt1 + i;
        if val != exp {
            println!("{val:x} != {exp:x} at address {addr:x}");
            return Err("base");
        }
        let addr = addr + offs;
        let val = readl(addr);
        let exp = patt2 + i;
        if val != exp {
            println!("{val:x} != {exp:x} at address {addr:x}");
            return Err("offs");
        }
    }
    Ok(())
}

const WORK_MODE_MASK: u32 = 0xfff_ff003;

const BANK_MODE_FLAGS: u32 = 0x0000_06a4;
const ROW_MODE_FLAGS: u32 = 0x0000_06f0;
const PAGE_MODE_FLAGS: u32 = 0x0000_0aa0;

fn set_work_mode_flags(reg: usize, flags: u32) -> u32 {
    let v = (readl(reg) & WORK_MODE_MASK) | flags;
    writel(reg, v);
    v
}

// Autoscan sizes a dram device by cycling through address lines and figuring
// out if it is connected to a real address line, or if the address is a mirror.
// First the column and bank bit allocations are set to low values (2 and 9 address
// lines. Then a maximum allocation (16 lines) is set for rows and this is tested.
// Next the BA2 line is checked. This seems to be placed above the column, BA0-1 and
// row addresses. Finally, the column address is allocated 13 lines and these are
// tested. The results are placed in dram_para1 and dram_para2.
fn auto_scan_dram_size(para: &mut dram_parameters) -> Result<(), &'static str> {
    mctl_core_init(para)?;

    // write test pattern
    for i in 0..64 {
        let ptr = RAM_BASE + 4 * i;
        let val = if i & 1 > 0 { ptr } else { !ptr };
        writel(ptr, val as u32);
    }

    let maxrank = if para.dram_para2 & 0xf000 == 0 { 1 } else { 2 };
    println!("maxrank {maxrank}");

    // Scan per address line, until address wraps (i.e. see shadow)
    fn scan_for_addr_wrap() -> u32 {
        for i in 11..17 {
            let mut done = true;
            for j in 0..64 {
                let ptr = RAM_BASE + j * 4;
                let chk = ptr + (1 << (i + 11));
                let exp = if j & 1 != 0 { ptr } else { !ptr };
                if readl(chk) != exp as u32 {
                    done = false;
                    break;
                }
            }
            if done {
                return i;
            }
        }
        return 16;
    }

    // Scan per address line, until address wraps (i.e. see shadow)
    fn scan_for_addr_wrap2() -> u32 {
        for i in 9..14 {
            let mut done = true;
            for j in 0..64 {
                let ptr = RAM_BASE + j * 4;
                let chk = ptr + (1 << i);
                let exp = if j & 1 != 0 { ptr } else { !ptr };
                if readl(chk) != exp as u32 {
                    done = false;
                    break;
                }
            }
            if done {
                return i;
            }
        }
        return 13;
    }

    for rank in 0..maxrank {
        let offs = rank * 16;
        let mc_work_mode = WORK_MODE0 + rank * 4;
        // Set row mode
        let v = set_work_mode_flags(mc_work_mode, ROW_MODE_FLAGS);
        while readl(mc_work_mode) != v {}

        let i = scan_for_addr_wrap();
        if VERBOSE {
            println!("rank {rank} row = {i}");
        }

        // Store rows in para 1
        let shft = 4 + offs;
        let v = (para.dram_para1 & !(0xff << shft)) | (i << shft);
        para.dram_para1 = v;

        // FIXME: differs in awboot
        if rank == 1 {
            // Set bank mode for rank0
            let _ = set_work_mode_flags(WORK_MODE0, BANK_MODE_FLAGS);
        }
        // Set bank mode for current rank
        let v = set_work_mode_flags(mc_work_mode, BANK_MODE_FLAGS);
        while readl(mc_work_mode) != v {}

        // Test if bit A23 is BA2 or mirror XXX A22?
        let mut j = 0;
        for i in 0..64 {
            // where to check
            let chk = RAM_BASE + (1 << 22) + i * 4;
            // pattern
            let ptr = RAM_BASE + i * 4;
            // expected value
            let exp = (if i & 1 != 0 { ptr } else { !ptr }) as u32;
            if readl(chk) != exp {
                j = 1;
                break;
            }
        }
        let banks = (j + 1) << 2; // 4 or 8
        if VERBOSE {
            println!("rank {rank} banks = {banks}");
        }

        // Store banks in para 1
        let shft = 12 + offs;
        let v = (para.dram_para1 & !(0xf << shft)) | (j << shft);
        para.dram_para1 = v;

        if rank == 1 {
            // Set page mode for rank0
            let _ = set_work_mode_flags(WORK_MODE0, PAGE_MODE_FLAGS);
        }

        // Set page mode for current rank
        let v = set_work_mode_flags(mc_work_mode, PAGE_MODE_FLAGS);
        while readl(mc_work_mode) != v {}

        let i = scan_for_addr_wrap2();
        let pgsize = if i == 9 { 0 } else { 1 << (i - 10) };

        if VERBOSE {
            println!("rank {rank} page size = {pgsize}KB");
        }

        // Store page size
        let v = para.dram_para1;
        para.dram_para1 = (v & !(0xf << offs)) | (pgsize << offs);

        // FIXME: This is not in awboot; do those registers exist?
        if false && rank == 0 {
            // MC_WORK_MODE
            let _ = set_work_mode_flags(MC_WORK_MODE_RANK1_1, ROW_MODE_FLAGS);
            // MC_WORK_MODE2
            let _ = set_work_mode_flags(MC_WORK_MODE_RANK1_2, ROW_MODE_FLAGS);
        }
    }
    /*
    if maxrank == 2 {
        para.dram_para2 = dram_para2 & 0xfffff0ff;
        // note: rval is equal to para.dram_para1 here
        if (rval & 0xffff) == ((rval >> 16) & 0xffff) {
            println!("rank1 config same as rank0");
        } else {
            para.dram_para2 = para.dram_para2 | 0x00000100;
            println!("rank1 config different from rank0");
        }
    }
    */
    Ok(())
}

// This routine sets up parameters with dqs_gating_mode equal to 1 and two
// ranks enabled. It then configures the core and tests for 1 or 2 ranks and
// full or half DQ width. it then resets the parameters to the original values.
// dram_para2 is updated with the rank & width findings.
fn auto_scan_dram_rank_width(para: &mut dram_parameters) -> Result<(), &'static str> {
    let tpr13 = para.dram_tpr13;
    let para1 = para.dram_para1;

    para.dram_para1 = 0x00b0_00b0;
    para.dram_para2 = (para.dram_para2 & 0xfffffff0) | (1 << 12);
    // set DQS probe mode
    para.dram_tpr13 = (tpr13 & 0xfffffff7) | (1 << 2) | 1;

    println!("auto_scan_dram_rank_width >>> mctl_core_init");
    if let Err(msg) = mctl_core_init(para) {
        println!("mctl core init error {msg}");
    };

    if readl(PGSR0) & (1 << 20) != 0 {
        return Err("auto scan rank/width");
    }
    println!("dqs_gate_detect");
    // FIXME: failing on F133 :(
    match dqs_gate_detect(para) {
        Err(e) => return Err(e),
        Ok(v) => {
            println!("DQS gate detected: {v}");
        }
    }

    para.dram_tpr13 = tpr13;
    para.dram_para1 = para1;
    Ok(())
}

/* STEP 2 */
/// This routine determines the SDRAM topology.
///
/// It first establishes the number of ranks and the DQ width. Then it scans the
/// SDRAM address lines to establish the size of each rank. It then updates
/// `dram_tpr13` to reflect that the sizes are now known: a re-init will not
/// repeat the autoscan.
fn auto_scan_dram_config(para: &mut dram_parameters) -> Result<(), &'static str> {
    // F133 dram_tpr13:  0x3400_0000,
    if para.dram_tpr13 & (1 << 14) == 0 {
        // this calls mctl core init
        auto_scan_dram_rank_width(para)?
    }
    if para.dram_tpr13 & (1 << 0) == 0 {
        // this calls mctl core init
        auto_scan_dram_size(para)?
    }
    if (para.dram_tpr13 & (1 << 15)) == 0 {
        para.dram_tpr13 |= 0x6003;
    }
    Ok(())
}

fn print_dram_type(dram_type: u32, clk: u32) {
    let dtype = match dram_type {
        2 => "DDR2",
        3 => "DDR3",
        6 => "LPDDR2",
        7 => "LPDDR3",
        _ => "?",
    };
    println!("{dtype}@{clk}MHz");
}

// NOTE: a commented call outside the if uses 64 in C code
const DRAM_TEST_BYTES: u32 = 4096;

const REF_CODE: bool = false;

/// # Safety
///
/// No warranty. Use at own risk. Be lucky to get values from vendor.
pub fn init_dram(para: &mut dram_parameters) -> usize {
    print_dram_type(para.dram_type, para.dram_clk);
    // Weird logic. Just copied from reference code.
    if VERBOSE {
        if (para.dram_odt_en & 0x1) == 0 {
            println!("On-die termination off");
        } else {
            println!("ZQ: {}", para.dram_zq);
        }
    }

    // STEP 1: ZQ, gating, calibration and voltage
    // Test ZQ status
    if para.dram_tpr13 & (1 << 16) > 0 {
        if VERBOSE {
            println!("DRAM only has internal ZQ.");
        }
        writel(RES_CAL_CTRL_REG, readl(RES_CAL_CTRL_REG) | 1 << 8);
        writel(RES240_CTRL_REG, 0);
        sdelay(10);
    } else {
        println!("pwroff gating");
        if REF_CODE {
            writel(ANALOG_SYS_PWROFF_GATING_REG, 0); // 0x7010000 + 0x254; l 9655
        }
        writel(RES_CAL_CTRL_REG, readl(RES_CAL_CTRL_REG) & 0xffff_fffc);
        if !REF_CODE {
            writel(ANALOG_SYS_PWROFF_GATING_REG, para.dram_tpr13 & (1 << 16));
        }
        sdelay(10);
        writel(
            RES_CAL_CTRL_REG,
            (readl(RES_CAL_CTRL_REG) & 0xffff_fef7) | (1 << 1),
        );
        sdelay(10);
        writel(RES_CAL_CTRL_REG, readl(RES_CAL_CTRL_REG) | 1);
        sdelay(20);
        if VERBOSE {
            let zq_val = if REF_CODE {
                // this causes a hang on F133
                readl(ZQ_VALUE)
            } else {
                readl(RES_CAL_STATUS_REG)
            };
            println!("ZQ: 0x{zq_val:08x}");
        }
    }

    // Set voltage
    if get_pmu_exists() {
        println!("PMU exists, set voltage");
        // NOTE: Does nothing at this point.
        if para.dram_type == 2 {
            set_ddr_voltage(1800);
        } else if para.dram_type == 3 {
            set_ddr_voltage(1500);
        }
    } else {
        // we get here
        dram_vol_set(para);
    }

    // STEP 2: CONFIG
    // Set SDRAM controller auto config
    if (para.dram_tpr13 & 0x1) == 0 {
        println!("auto_scan_dram_config");
        if let Err(msg) = auto_scan_dram_config(para) {
            println!("auto_scan_dram_config fail: {msg}");
            return 0;
        }
    }

    // Weird logic. Just copied from reference code.
    // report ODT
    if VERBOSE {
        if (para.dram_mr1 & 0x44) == 0 {
            println!("On-die termination off");
        } else {
            println!("On-die termination: {}", para.dram_mr1);
        }
    }

    // Init core, final run
    if let Err(msg) = mctl_core_init(para) {
        println!("init error {msg}");
        return 0;
    };

    // Get sdram size
    let rc = para.dram_para2;
    let mem_size = if rc != 0 {
        (rc & 0x7fff0000) >> 16
    } else {
        let s = dramc_get_dram_size();
        para.dram_para2 = (rc & 0xffff) | s << 16;
        s
    };

    if VERBOSE {
        println!("DRAM: {mem_size}M");
    }

    // Purpose ??
    // What is Auto SR?
    if para.dram_tpr13 & (1 << 30) != 0 {
        let rc = readl(para.dram_tpr8 as usize);
        writel(ASRTC, if rc == 0 { 0x10000200 } else { rc });
        writel(ASRC, 0x40a);
        writel(PHY_PWRCTL, readl(PHY_PWRCTL) | 1);
        // println!("Enable Auto SR");
    } else {
        writel(ASRTC, readl(ASRTC) & 0xffff0000);
        writel(PHY_PWRCTL, readl(PHY_PWRCTL) & (!0x1));
    }

    // Purpose ??
    let rc = readl(PGCR0) & !(0xf000);
    if (para.dram_tpr13 & 0x200) == 0 {
        if para.dram_type != 6 {
            writel(PGCR0, rc);
        }
    } else {
        writel(PGCR0, rc | 0x5000);
    }

    writel(ZQCR, readl(ZQCR) | (1 << 31));
    if para.dram_tpr13 & (1 << 8) != 0 {
        writel(VTFCR, readl(ZQCR) | 0x300);
    }

    let rc = readl(PGCR2);
    let rc = if para.dram_tpr13 & (1 << 16) != 0 {
        rc & 0xffffdfff
    } else {
        rc | 0x00002000
    };
    writel(PGCR2, rc);

    // Purpose ??
    if para.dram_type == 7 {
        writel(ODT_CFG, (readl(ODT_CFG) & 0xfff0ffff) | (1 << 12));
    }

    dram_enable_all_master();

    if para.dram_tpr13 & (1 << 28) != 0 {
        let rc = readl(SOME_STATUS);
        if rc & (1 << 16) != 0 {
            println!("something DRAM status bad?!");
            return 0;
        }
    }

    // If detection failed (mem_size == 0), claim 64 MB for testing
    let msize = if mem_size > 0 { mem_size } else { 64 };
    if let Err(msg) = dramc_simple_wr_test(msize, DRAM_TEST_BYTES) {
        println!("test fail {msg}");
        dump_block(0x4000_0000, 512, 32);
        return 0;
    }
    println!("test OK");

    handler_super_standby();

    mem_size as usize
}

pub fn init() -> usize {
    // taken from SPL
    // and xfel `chips/d1_f133.c`
    #[rustfmt::skip]
    let mut dram_para: dram_parameters = dram_parameters {
        #[cfg(feature="f133")]
        dram_clk:    528,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_clk:    792,

        #[cfg(feature="f133")]
        dram_type:   0x0000_0002,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_type:   0x0000_0003,

        #[cfg(feature="f133")]
        dram_zq:     0x007b_7bf9,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_zq:     0x007b_7bfb,

        #[cfg(feature="f133")]
        dram_odt_en: 0x0000_0000,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_odt_en: 0x0000_0001,

        #[cfg(feature="f133")]
        dram_para1:  0x0000_00d2,
        #[cfg(feature="nezha")]
        dram_para1:  0x0000_10f2,
        #[cfg(feature="lichee")]
        dram_para1:  0x0000_10d2,
        dram_para2:  0x0000_0000,

        #[cfg(feature="f133")]
        dram_mr0:    0x0000_0e73,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_mr0:    0x0000_1c70,
        #[cfg(feature="f133")]
        dram_mr1:    0x0000_0002,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_mr1:    0x0000_0042,
        #[cfg(any(feature="f133",feature="nezha"))]
        dram_mr2:    0x0000_0000,
        #[cfg(feature="lichee")]
        dram_mr2:    0x0000_0018,
        dram_mr3:    0x0000_0000,

        // timing paramters
        #[cfg(feature="f133")]
        dram_tpr0:   0x0047_1992,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_tpr0:   0x004a_2195,

        #[cfg(feature="f133")]
        dram_tpr1:   0x0131_a10c,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_tpr1:   0x0242_3190,

        #[cfg(feature="f133")]
        dram_tpr2:   0x0005_7041,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_tpr2:   0x0008_b061,

        dram_tpr3:   0xb478_7896,
        dram_tpr4:   0x0000_0000,
        dram_tpr5:   0x4848_4848, // IOCVR0
        dram_tpr6:   0x0000_0048, // IOCVR1
                                  //
        #[cfg(feature="f133")]
        dram_tpr7:   0x1621_121e,
        #[cfg(any(feature="lichee",feature="nezha"))]
        dram_tpr7:   0x1620_121e,

        dram_tpr8:   0x0000_0000,
        dram_tpr9:   0x0000_0000,
        dram_tpr10:  0x0000_0000,

        #[cfg(feature="f133")]
        dram_tpr11:  0x0003_0010,
        #[cfg(feature="nezha")]
        dram_tpr11:  0x0076_0000,
        #[cfg(feature="lichee")]
        dram_tpr11:  0x0087_0000,

        #[cfg(any(feature="nezha", feature="f133"))]
        dram_tpr12:  0x0000_0035,
        #[cfg(feature="lichee")]
        dram_tpr12:  0x0000_0024,

        #[cfg(feature="f133")]
        dram_tpr13:  0x3400_0000,
        #[cfg(feature="nezha")]
        dram_tpr13:  0x3405_0101,
        #[cfg(feature="lichee")]
        dram_tpr13:  0x3405_0100,
    };

    println!("DRAM INIT");
    return init_dram(&mut dram_para);
}
