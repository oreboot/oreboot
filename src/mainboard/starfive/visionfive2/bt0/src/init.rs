use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};
use core::slice;

use soc::starfive::jh7110::pac;

pub fn dump(addr: usize, length: usize) {
    let s = unsafe { slice::from_raw_parts(addr as *const u8, length) };
    println!("dump {length} bytes @{addr:x}");
    for w in s.iter() {
        print!("{:02x}", w);
    }
    println!();
}

pub fn dump_block(base: usize, size: usize, step_size: usize) {
    for b in (base..base + size).step_by(step_size) {
        dump(b, step_size);
    }
}

pub fn write32(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

pub fn read32(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

pub fn clear_bit(reg: usize, bit: u32) {
    unsafe {
        let v = read32(reg);
        write32(reg, v & !(1 << bit));
    }
}

pub fn set_bit(reg: usize, bit: u32) {
    unsafe {
        let v = read32(reg);
        write32(reg, v | (1 << bit));
    }
}

pub const DDR_CTRL_BASE: usize = 0x1570_0000;
pub const DDR_SEC_CTRL_BASE: usize = DDR_CTRL_BASE + 0x1000;

// https://www.synopsys.com/designware-ip/technical-bulletin/ddr-hardening-demystified.html
pub const DDR_PHY_BASE: usize = 0x1300_0000;
pub const DDR_PHY_CTRL_BASE: usize = DDR_PHY_BASE + 0x2000;
// AC = address / command
pub const DDR_PHY_AC_BASE: usize = DDR_PHY_BASE + 0x4000;

pub const DDR_FREQ: u32 = 2133;
pub const DDR_RATE: u32 = 1066000000;

/* SYS CRG */
pub const SYS_CRG_BASE: usize = 0x1302_0000;

pub const CLK_CPU_ROOT: usize = SYS_CRG_BASE;
pub const CLK_CPU_ROOT_SW: u8 = 1; // PLL0 (?)

const CLK_PERH_ROOT_MUX_SEL: u8 = 1; // pll2

pub const CLK_BUS_ROOT_SW: u8 = 1; // PLL2 (?)

pub const CLK_AHB0: usize = SYS_CRG_BASE + 0x0024;
pub const CLK_APB0: usize = SYS_CRG_BASE + 0x0030;
pub const CLK_OSC_DIV2: usize = SYS_CRG_BASE + 0x00a0;
pub const CLK_PLL1_DIV4: usize = SYS_CRG_BASE + 0x00a4;
pub const CLK_PLL1_DIV8: usize = SYS_CRG_BASE + 0x00a8;
pub const CLK_U0_DDR_BUS: usize = SYS_CRG_BASE + 0x00ac;
pub const CLK_U0_DDR_AXI: usize = SYS_CRG_BASE + 0x00b0;

const CLK_U0_SI5_TIMER_CLK_APB: usize = SYS_CRG_BASE + 0x01f0;
const CLK_U0_SI5_TIMER_CLK_TIMER0: usize = SYS_CRG_BASE + 0x01f4;
const CLK_U0_SI5_TIMER_CLK_TIMER1: usize = SYS_CRG_BASE + 0x01f8;
const CLK_U0_SI5_TIMER_CLK_TIMER2: usize = SYS_CRG_BASE + 0x01fc;
const CLK_U0_SI5_TIMER_CLK_TIMER3: usize = SYS_CRG_BASE + 0x0200;

pub const SYS_CRG_RESET_ASSERT1: usize = SYS_CRG_BASE + 0x02fc;
pub const SYS_CRG_RESET_ASSERT3: usize = SYS_CRG_BASE + 0x0308;
pub const SYS_CRG_RESET_STATUS1: usize = SYS_CRG_BASE + 0x030c;
pub const SYS_CRG_RESET_STATUS3: usize = SYS_CRG_BASE + 0x0314;
// bits in SYS_CRG_ASSERT1
pub const RSTN_U0_DDR_AXI: u32 = 6;
pub const RSTN_U0_DDR_OSC: u32 = 7;
pub const RSTN_U0_DDR_APB: u32 = 8;

// NOTE: DDR clk is enabled by default
const DDR_AXI_ON: u32 = 1 << 31;
const DDR_AXI_OFF: u32 = 0 << 31;
// ICG = Integrated Clock Gating
const CLK_ICG_ON: u32 = 1 << 31;
// 24-29 clk_mux_sel
const DDR_BUS_MASK: u32 = !(0x3f00_0000);

const DDR_BUS_OSC_DIV2: u8 = 0;
const DDR_BUS_PLL1_DIV2: u8 = 1;
const DDR_BUS_PLL1_DIV4: u8 = 2;
const DDR_BUS_PLL1_DIV8: u8 = 3;

const CLK_QSPI_REF: usize = SYS_CRG_BASE + 0x0168;
const CLK_QSPI_REF_MUX_SEL: u8 = 1; // QSPI ref src
const CLK_NOC_BUS_STG_AXI: usize = SYS_CRG_BASE + 0x0180;
const CLK_NOC_BUS_STG_AXI_CLK_ICG_EN: u32 = 1 << 31;

pub fn clk_cpu_root() {
    // Select clk_pll0 as the CPU root clock
    pac::syscrg_reg()
        .clk_cpu_root()
        .modify(|_, w| w.clk_mux_sel().variant(CLK_CPU_ROOT_SW));
}

pub fn clk_bus_root() {
    // Select clk_pll2 as the BUS root clock
    pac::syscrg_reg()
        .clk_bus_root()
        .modify(|_, w| w.clk_mux_sel().variant(CLK_BUS_ROOT_SW));
}

pub fn clocks() {
    let syscrg = pac::syscrg_reg();

    // Set clk_pll2 as the peripheral root clock
    syscrg
        .clk_peripheral_root()
        .modify(|_, w| w.clk_mux_sel().variant(CLK_PERH_ROOT_MUX_SEL));

    // Enable the NOC STG clock
    syscrg
        .clk_noc_stg_axi()
        .modify(|_, w| w.clk_icg().set_bit());

    // Oscillator clock without divider
    pac::aoncrg_reg()
        .clk_aon_apb()
        .modify(|_, w| w.clk_mux_sel().variant(1));

    // Set clk_qspi_ref_src as the QSPI clock
    syscrg
        .clk_qspi_ref()
        .modify(|_, w| w.clk_mux_sel().variant(CLK_QSPI_REF_MUX_SEL));
}

pub fn clk_apb0() {
    pac::syscrg_reg().clk_apb0().modify(|r, w| {
        let clk = r.bits();
        println!("apb0 {clk:x}");

        // try a reset
        w.clk_icg().clear_bit();
        w.clk_icg().set_bit();

        let clk = r.bits();
        println!("apb0 {clk:x}");

        w
    });
}

pub fn clk_ddrc_axi(on: bool) {
    pac::syscrg_reg().clk_u0_ddr_axi().modify(|r, w| {
        let ddr_axi = r.bits();
        println!("ddr_axi {ddr_axi:x}");

        w.clk_icg().variant(on)
    });
}

pub fn clk_ddrc_osc_div2() {
    pac::syscrg_reg()
        .clk_ddr_bus()
        .modify(|_, w| w.clk_mux_sel().variant(DDR_BUS_OSC_DIV2));
}

pub fn clk_ddrc_pll1_div2() {
    pac::syscrg_reg()
        .clk_ddr_bus()
        .modify(|_, w| w.clk_mux_sel().variant(DDR_BUS_PLL1_DIV2));
}

pub fn clk_ddrc_pll1_div4() {
    pac::syscrg_reg()
        .clk_ddr_bus()
        .modify(|_, w| w.clk_mux_sel().variant(DDR_BUS_PLL1_DIV4));
}

pub fn clk_ddrc_pll1_div8() {
    pac::syscrg_reg()
        .clk_ddr_bus()
        .modify(|_, w| w.clk_mux_sel().variant(DDR_BUS_PLL1_DIV8));
}

//////////////
const AON_CRG_BASE: usize = 0x1700_0000;

const GMAC5_0_CLK_TX: usize = AON_CRG_BASE + 0x0014;

const GMAC5_CLK_TX_BIT: u32 = 1 << 24;
const GMAC5_CLK_TX_MASK: u32 = !GMAC5_CLK_TX_BIT;

//////////////
const AON_SYSCON_BASE: usize = 0x1701_0000;
const AON_SYSCFG_12: usize = AON_SYSCON_BASE + 0x000c;

const GMAC5_0_PHY_SELECT_MASK: u32 = !(0b111 << 18);
const GMAC5_0_PHY_SELECT_RGMII: u32 = 0x1 << 18;

//////////////
const GMAC5_1_CLK_TX: usize = SYS_CRG_BASE + 0x01a4;

const GMAC5_1_PHY_SELECT_MASK: u32 = !(0b111 << 2);
const GMAC5_1_PHY_SELECT_RGMII: u32 = 0x1 << 2;

/////////////
const AON_IOMUX_BASE: usize = 0x1702_0000;

/* Improved GMAC0 TX I/O PAD capability */
const GMAC0_XX0: usize = AON_IOMUX_BASE + 0x0078;
const GMAC0_XX1: usize = AON_IOMUX_BASE + 0x007c;
const GMAC0_XX2: usize = AON_IOMUX_BASE + 0x0080;
const GMAC0_XX3: usize = AON_IOMUX_BASE + 0x0084;
const GMAC0_XX4: usize = AON_IOMUX_BASE + 0x0088;

/* Improved GMAC1 TX I/O PAD capability */
const GMAC1_XX0: usize = SYS_IOMUX_BASE + 0x026c;
const GMAC1_XX1: usize = SYS_IOMUX_BASE + 0x0270;
const GMAC1_XX2: usize = SYS_IOMUX_BASE + 0x0274;
const GMAC1_XX3: usize = SYS_IOMUX_BASE + 0x0278;
const GMAC1_XX4: usize = SYS_IOMUX_BASE + 0x027c;

pub fn phy_cfg() {
    let v = read32(GMAC0_XX0);
    write32(GMAC0_XX0, (v & !(0b11)) | 0x1);
    let v = read32(GMAC0_XX1);
    write32(GMAC0_XX1, (v & !(0b11)) | 0x1);
    let v = read32(GMAC0_XX2);
    write32(GMAC0_XX2, (v & !(0b11)) | 0x1);
    let v = read32(GMAC0_XX3);
    write32(GMAC0_XX3, (v & !(0b11)) | 0x1);
    let v = read32(GMAC0_XX4);
    write32(GMAC0_XX4, (v & !(0b11)) | 0x1);

    let v = read32(GMAC1_XX0);
    write32(GMAC1_XX0, (v & !(0b11)) | 0x1);
    let v = read32(GMAC1_XX1);
    write32(GMAC1_XX1, (v & !(0b11)) | 0x1);
    let v = read32(GMAC1_XX2);
    write32(GMAC1_XX2, (v & !(0b11)) | 0x1);
    let v = read32(GMAC1_XX3);
    write32(GMAC1_XX3, (v & !(0b11)) | 0x1);
    let v = read32(GMAC1_XX4);
    write32(GMAC1_XX4, (v & !(0b11)) | 0x1);

    let c = read32(GMAC5_0_CLK_TX);
    write32(GMAC5_0_CLK_TX, (c & GMAC5_CLK_TX_MASK) | GMAC5_CLK_TX_BIT);
    let c = read32(GMAC5_1_CLK_TX);
    write32(GMAC5_1_CLK_TX, (c & GMAC5_CLK_TX_MASK) | GMAC5_CLK_TX_BIT);

    let c = read32(AON_SYSCFG_12);
    write32(
        AON_SYSCFG_12,
        (c & GMAC5_0_PHY_SELECT_MASK) | GMAC5_0_PHY_SELECT_RGMII,
    );
    let c = read32(SYS_SYSCON_144);
    write32(
        SYS_SYSCON_144,
        (c & GMAC5_1_PHY_SELECT_MASK) | GMAC5_1_PHY_SELECT_RGMII,
    );
}

/* SYS SYSCON */
pub const SYS_SYSCON_BASE: usize = 0x1303_0000;

pub const SYS_SYSCON_00: usize = SYS_SYSCON_BASE;
pub const SYS_SYSCON_04: usize = SYS_SYSCON_BASE + 0x0004;
pub const SYS_SYSCON_08: usize = SYS_SYSCON_BASE + 0x0008;
pub const SYS_SYSCON_12: usize = SYS_SYSCON_BASE + 0x000c;
pub const SYS_SYSCON_16: usize = SYS_SYSCON_BASE + 0x0010;
pub const SYS_SYSCON_20: usize = SYS_SYSCON_BASE + 0x0014;
pub const SYS_SYSCON_24: usize = SYS_SYSCON_BASE + 0x0018;
pub const SYS_SYSCON_28: usize = SYS_SYSCON_BASE + 0x001c;
pub const SYS_SYSCON_32: usize = SYS_SYSCON_BASE + 0x0020;
pub const SYS_SYSCON_36: usize = SYS_SYSCON_BASE + 0x0024;
pub const SYS_SYSCON_40: usize = SYS_SYSCON_BASE + 0x0028;
pub const SYS_SYSCON_44: usize = SYS_SYSCON_BASE + 0x002c;
pub const SYS_SYSCON_48: usize = SYS_SYSCON_BASE + 0x0030;
pub const SYS_SYSCON_52: usize = SYS_SYSCON_BASE + 0x0034;

pub const SYS_SYSCON_144: usize = SYS_SYSCON_BASE + 0x0090;

/* GPIO mux */

pub const SYS_IOMUX_BASE: usize = 0x1304_0000;

// NOTE: 4 GPIOs per DWORD
pub const GPIO00_03_EN: usize = SYS_IOMUX_BASE;
pub const GPIO04_07_EN: usize = SYS_IOMUX_BASE + 0x0004;
pub const GPIO08_11_EN: usize = SYS_IOMUX_BASE + 0x0008;
pub const GPIO12_15_EN: usize = SYS_IOMUX_BASE + 0x000c;
pub const GPIO16_19_EN: usize = SYS_IOMUX_BASE + 0x0010;
pub const GPIO20_23_EN: usize = SYS_IOMUX_BASE + 0x0014;
pub const GPIO24_27_EN: usize = SYS_IOMUX_BASE + 0x0018;
pub const GPIO28_31_EN: usize = SYS_IOMUX_BASE + 0x001c;
pub const GPIO32_35_EN: usize = SYS_IOMUX_BASE + 0x0020;
pub const GPIO36_39_EN: usize = SYS_IOMUX_BASE + 0x0024;
pub const GPIO40_43_EN: usize = SYS_IOMUX_BASE + 0x0028;
pub const GPIO44_47_EN: usize = SYS_IOMUX_BASE + 0x002c;
pub const GPIO48_51_EN: usize = SYS_IOMUX_BASE + 0x0030;
pub const GPIO52_55_EN: usize = SYS_IOMUX_BASE + 0x0034;
pub const GPIO56_59_EN: usize = SYS_IOMUX_BASE + 0x0038;
pub const GPIO60_63_EN: usize = SYS_IOMUX_BASE + 0x003c;

pub const GPIO00_03_DATA: usize = SYS_IOMUX_BASE + 0x0040;
pub const GPIO04_07_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0004;
pub const GPIO08_11_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0008;
pub const GPIO12_15_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x000c;
pub const GPIO16_19_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0010;
pub const GPIO20_23_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0014;
pub const GPIO24_27_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0018;
pub const GPIO28_31_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x001c;
pub const GPIO32_35_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0020;
pub const GPIO36_39_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0024;
pub const GPIO40_43_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0028;
pub const GPIO44_47_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x002c;
pub const GPIO48_51_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0030;
pub const GPIO52_55_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0034;
pub const GPIO56_59_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0038;
pub const GPIO60_63_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x003c;

// NOTE: we may not need this; copied from StarFive / U-Boot
// This is the base address for input data, AIUI from the manual.
// const SYS_IOMUX_32: usize = SYS_IOMUX_BASE + 0x0080;

pub const GPIO_DOEN_MASK: u8 = 0x3f;
pub const GPIO_DOUT_MASK: u8 = 0x7f;

/*
 * const GPIO_OUT_FUNC_OFF: u8 = 0x00;
 * const GPIO_OUT_FUNC_ON: u8 = 0x01;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x02;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x03;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x04;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x05;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x06;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x07;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x08;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x09;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x0a;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x0b;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x0c;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x0d;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x0e;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x0f;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x10;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x11;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x12;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x13;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x14;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x15;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x16;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x17;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x18;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x19;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x1a;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x1b;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x1c;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x1d;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x1e;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x1f;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x20;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x21;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x22;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x23;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x24;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x25;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x26;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x27;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x28;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x29;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x2a;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x2b;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x2c;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x2d;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x2e;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x2f;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x30;
 * const GPIO_OUT_FUNC_XXX: u8 = 0x31;
 * NOTE: GPIO OUT has 49 functions
 */
