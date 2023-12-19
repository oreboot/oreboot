use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};
use core::slice;

use crate::pac;

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

const CLINT_BASE: usize = 0x0200_0000;
const CLINT_MTIMER: usize = CLINT_BASE + 0xbff8;

pub fn udelay(t: usize) {
    let curr_time = read32(CLINT_MTIMER);
    while read32(CLINT_MTIMER) < (curr_time + 4 * t as u32) {}
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

    // Set clk_osc_div4 as the APB clock
    pac::aoncrg_reg()
        .clk_aon_apb()
        .modify(|_, w| w.clk_mux_sel().variant(0));

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
