#[cfg(feature = "12a")]
use jh71xx_pac::jh7110_vf2_12a_pac as pac;
#[cfg(feature = "13b")]
use jh71xx_pac::jh7110_vf2_13b_pac as pac;

use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};
use core::slice;

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

const CLK_PERH_ROOT: usize = SYS_CRG_BASE + 0x0010;
const CLK_PERH_ROOT_MUX_SEL: u32 = 1 << 24; // pll2

pub const CLK_BUS_ROOT: usize = SYS_CRG_BASE + 0x0014;
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

const DDR_BUS_OSC_DIV2: u32 = 0 << 24;
const DDR_BUS_PLL1_DIV2: u32 = 1 << 24;
const DDR_BUS_PLL1_DIV4: u32 = 2 << 24;
const DDR_BUS_PLL1_DIV8: u32 = 3 << 24;

const CLK_QSPI_REF: usize = SYS_CRG_BASE + 0x0168;
const CLK_QSPI_REF_MUX_SEL: u32 = 1 << 24; // QSPI ref src
const CLK_NOC_BUS_STG_AXI: usize = SYS_CRG_BASE + 0x0180;
const CLK_NOC_BUS_STG_AXI_CLK_ICG_EN: u32 = 1 << 31;

const CLK_AON_APB_FUNC: usize = SYS_AON_BASE + 0x0004;
const CLK_AON_APB_FUNC_MUX_SEL: u32 = 1 << 24; // OSC

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
fn syscrg_reg<'r>() -> &'r pac::syscrg::RegisterBlock {
    unsafe { &*pac::SYSCRG::ptr() }
}

pub fn clk_cpu_root() {
    // Select clk_pll0 as the CPU root clock
    syscrg_reg().clk_cpu_root().modify(|_, w| w.clk_mux_sel().variant(CLK_CPU_ROOT_SW));
}

pub fn clk_bus_root() {
    // Select clk_pll2 as the BUS root clock
    syscrg_reg().clk_bus_root().modify(|_, w| w.clk_mux_sel().variant(CLK_BUS_ROOT_SW));
}

pub fn clocks() {
    let c = read32(CLK_PERH_ROOT);
    write32(CLK_PERH_ROOT, c | CLK_PERH_ROOT_MUX_SEL);

    let c = read32(CLK_NOC_BUS_STG_AXI);
    write32(CLK_NOC_BUS_STG_AXI, c | CLK_NOC_BUS_STG_AXI_CLK_ICG_EN);

    // let c = read32(CLK_AON_APB_FUNC);
    // write32(CLK_AON_APB_FUNC, c | CLK_AON_APB_FUNC_MUX_SEL);
    write32(CLK_AON_APB_FUNC, 0);

    let c = read32(CLK_QSPI_REF);
    write32(CLK_QSPI_REF, c | CLK_QSPI_REF_MUX_SEL);
}

pub fn clk_apb0() {
    let clk = read32(CLK_APB0);
    println!("apb0 {clk:x}");
    write32(CLK_APB0, 0); // try a reset
    write32(CLK_APB0, clk | CLK_ICG_ON);
    let clk = read32(CLK_APB0);
    println!("apb0 {clk:x}");
}

pub fn clk_ddrc_axi(on: bool) {
    let v = if on { DDR_AXI_ON } else { DDR_AXI_OFF };
    let ddr_axi = read32(CLK_U0_DDR_AXI);
    println!("ddr_axi {ddr_axi:x}");
    write32(CLK_U0_DDR_AXI, ddr_axi & !(1 << 31) | v);
}

pub fn clk_ddrc_osc_div2() {
    let ddr_bus = read32(CLK_U0_DDR_BUS) & DDR_BUS_MASK;
    write32(CLK_U0_DDR_BUS, ddr_bus | DDR_BUS_OSC_DIV2);
}

pub fn clk_ddrc_pll1_div2() {
    let ddr_bus = read32(CLK_U0_DDR_BUS) & DDR_BUS_MASK;
    write32(CLK_U0_DDR_BUS, ddr_bus | DDR_BUS_PLL1_DIV2);
}

pub fn clk_ddrc_pll1_div4() {
    let ddr_bus = read32(CLK_U0_DDR_BUS) & DDR_BUS_MASK;
    write32(CLK_U0_DDR_BUS, ddr_bus | DDR_BUS_PLL1_DIV4);
}

pub fn clk_ddrc_pll1_div8() {
    let ddr_bus = read32(CLK_U0_DDR_BUS) & DDR_BUS_MASK;
    write32(CLK_U0_DDR_BUS, ddr_bus | DDR_BUS_PLL1_DIV8);
}

pub const SYS_AON_BASE: usize = 0x1700_0000;

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

/* GPIO mux */

pub const SYS_IOMUX_BASE: usize = 0x1304_0000;

// NOTE: 4 GPIOs per DWORD
/*
const GPIO00_03_EN: usize = SYS_IOMUX_BASE;
*/
pub const GPIO04_07_EN: usize = SYS_IOMUX_BASE + 0x0004;
/*
const GPIO08_11_EN: usize = SYS_IOMUX_BASE + 0x0008;
const GPIO12_15_EN: usize = SYS_IOMUX_BASE + 0x000c;
const GPIO16_19_EN: usize = SYS_IOMUX_BASE + 0x0010;
const GPIO20_23_EN: usize = SYS_IOMUX_BASE + 0x0014;
const GPIO24_27_EN: usize = SYS_IOMUX_BASE + 0x0018;
const GPIO28_31_EN: usize = SYS_IOMUX_BASE + 0x001c;
const GPIO32_35_EN: usize = SYS_IOMUX_BASE + 0x0020;
const GPIO36_39_EN: usize = SYS_IOMUX_BASE + 0x0024;
*/
pub const GPIO40_43_EN: usize = SYS_IOMUX_BASE + 0x0028;
/*
const GPIO44_47_EN: usize = SYS_IOMUX_BASE + 0x002c;
const GPIO48_51_EN: usize = SYS_IOMUX_BASE + 0x0030;
const GPIO52_55_EN: usize = SYS_IOMUX_BASE + 0x0034;
const GPIO56_59_EN: usize = SYS_IOMUX_BASE + 0x0038;
const GPIO60_63_EN: usize = SYS_IOMUX_BASE + 0x003c;

const GPIO00_03_DATA: usize = SYS_IOMUX_BASE + 0x0040;
const GPIO04_07_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0004;
const GPIO08_11_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0008;
const GPIO12_15_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x000c;
const GPIO16_19_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0010;
const GPIO20_23_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0014;
const GPIO24_27_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0018;
const GPIO28_31_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x001c;
const GPIO32_35_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0020;
const GPIO36_39_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0024;
*/
pub const GPIO40_43_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0028;
/*
const GPIO44_47_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x002c;
const GPIO48_51_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0030;
const GPIO52_55_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0034;
const GPIO56_59_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x0038;
const GPIO60_63_DATA: usize = SYS_IOMUX_BASE + 0x0040 + 0x003c;
*/

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
