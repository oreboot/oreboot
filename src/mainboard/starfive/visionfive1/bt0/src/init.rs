use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};

fn write_32(reg: u32, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

fn read_32(reg: u32) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

pub const CLKGEN_BASE: u32 = 0x1180_0000;
pub const CLK_CPUNDBUS_ROOT_CTRL: u32 = CLKGEN_BASE + 0x0;
pub const CLK_DLA_ROOT_CTRL: u32 = CLKGEN_BASE + 0x4;
pub const CLK_DSP_ROOT_CTRL: u32 = CLKGEN_BASE + 0x8;
pub const CLK_GMACUSB_ROOT_CTRL: u32 = CLKGEN_BASE + 0xC;
pub const CLK_PERH0_ROOT_CTRL: u32 = CLKGEN_BASE + 0x10;

pub const CLK_DDRC0_CTRL: u32 = CLKGEN_BASE + 0x108;
pub const CLK_DDRC1_CTRL: u32 = CLKGEN_BASE + 0x10C;

pub const CLK_X2C_AXI_CTRL: u32 = CLKGEN_BASE + 0x15C;
pub const CLK_MSI_APB_CTRL: u32 = CLKGEN_BASE + 0x2D8;

pub const CLK_GMAC_AHB_CTRL: u32 = CLKGEN_BASE + 0x1E0;
pub const CLK_GMAC_PTP_REFCLK_CTRL: u32 = CLKGEN_BASE + 0x1E8;
pub const CLK_GMAC_GTXCLK_CTRL: u32 = CLKGEN_BASE + 0x1EC;

pub fn clk_cpundbus_root_pll0_out() {
    let v = read_32(CLK_CPUNDBUS_ROOT_CTRL) & !(0x3 << 24);
    write_32(CLK_CPUNDBUS_ROOT_CTRL, v | 1 << 24);
}

pub fn clk_dla_root_osc_sys() {
    let v = read_32(CLK_DLA_ROOT_CTRL) & !(0x3 << 24);
    write_32(CLK_DLA_ROOT_CTRL, v | 0 << 24);
}

pub fn clk_dla_root_pll1_out() {
    let v = read_32(CLK_DLA_ROOT_CTRL) & !(0x3 << 24);
    write_32(CLK_DLA_ROOT_CTRL, v | 1 << 24);
}

pub fn clk_dsp_root_pll2_out() {
    let v = read_32(CLK_DSP_ROOT_CTRL) & !(0x3 << 24);
    write_32(CLK_DSP_ROOT_CTRL, v | 3 << 24);
}

pub fn clk_perh0_root_pll0_out() {
    let v = read_32(CLK_PERH0_ROOT_CTRL) & !(0x1 << 24);
    write_32(CLK_PERH0_ROOT_CTRL, v | 1 << 24);
}

pub fn enable_clk_ddrc0() {
    let v = read_32(CLK_DDRC0_CTRL) & !(0x1 << 31);
    write_32(CLK_DDRC0_CTRL, v | 1 << 31);
}

pub fn enable_clk_ddrc1() {
    let v = read_32(CLK_DDRC1_CTRL) & !(0x1 << 31);
    write_32(CLK_DDRC1_CTRL, v | 1 << 31);
}

pub fn clk_ddrc0_osc_div2() {
    let v = read_32(CLK_DDRC0_CTRL) & !(0x3 << 24);
    write_32(CLK_DDRC0_CTRL, v | 0 << 24);
}

pub fn clk_ddrc0_pll_div2() {
    let v = read_32(CLK_DDRC0_CTRL) & !(0x3 << 24);
    write_32(CLK_DDRC0_CTRL, v | 0x1 << 24);
}

pub fn clk_ddrc0_pll_div4() {
    let v = read_32(CLK_DDRC0_CTRL) & !(0x3 << 24);
    write_32(CLK_DDRC0_CTRL, v | 0x2 << 24);
}

pub fn clk_ddrc1_osc_div2() {
    let v = read_32(CLK_DDRC1_CTRL) & !(0x3 << 24);
    write_32(CLK_DDRC1_CTRL, v | 0 << 24);
}

pub fn clk_ddrc1_pll_div2() {
    let v = read_32(CLK_DDRC1_CTRL) & !(0x3 << 24);
    write_32(CLK_DDRC1_CTRL, v | 0x1 << 24);
}

pub fn clk_ddrc1_pll_div4() {
    let v = read_32(CLK_DDRC1_CTRL) & !(0x3 << 24);
    write_32(CLK_DDRC1_CTRL, v | 0x2 << 24);
}

pub fn enable_clk_x2c_axi_() {
    let v = read_32(CLK_X2C_AXI_CTRL) & !(0x1 << 31);
    write_32(CLK_X2C_AXI_CTRL, v | 1 << 31);
}

pub fn enable_clk_msi_apb_() {
    let v = read_32(CLK_MSI_APB_CTRL & !(0x1 << 31));
    write_32(CLK_MSI_APB_CTRL, v | 1 << 31);
}

pub fn enable_clk_gmac_ahb_() {
    let v = read_32(CLK_GMAC_AHB_CTRL) & !(0x1 << 31);
    write_32(CLK_GMAC_AHB_CTRL, v | 1 << 31);
}

pub fn enable_clk_gmac_ptp_refclk_() {
    let v = read_32(CLK_GMAC_PTP_REFCLK_CTRL) & !(0x1 << 31);
    write_32(CLK_GMAC_PTP_REFCLK_CTRL, v | 1 << 31);
}

pub fn enable_clk_gmac_gtxclk_() {
    let v = read_32(CLK_GMAC_GTXCLK_CTRL) & !(0x1 << 31);
    write_32(CLK_GMAC_GTXCLK_CTRL, v | 1 << 31);
}

/// NOTE: Datasheet p33 / 8.1:
/// Two external oscillator OSC0 and OSC1 input
/// - OSC0 25M default for USB, GMAC and system main clock source
/// - OSC1 input 12-27MHz according to application
///
/// Three PLLs
/// - PLL0 used for system main logic, including CPU, bus
/// - PLL1 output to support DDR, DLA and DSP
/// - PLL2 output to support slow speed peripherals, video input and video
///   output
fn init_coreclk() {
    // TODO: make base a parameter.
    clk_cpundbus_root_pll0_out();
    clk_dla_root_pll1_out();
    clk_dsp_root_pll2_out();
    clk_perh0_root_pll0_out();

    // not enabled in original.
    // slow down nne bus can fix nne50 & vp6 ram scan issue,
    // as well as vin_subsys reg scan issue.
    //	clk_nne_bus_cpu_axi_;
}

pub fn clock_init() {
    // Update the peripheral clock dividers of UART, SPI and I2C to safe
    // values as we can't put them in reset before changing frequency.
    /*
    let hfclk = 1_000_000_000; // 1GHz
    let clks = [];
    for clk in clks.iter_mut() {
        if false {
            clk.set_clock_rate(hfclk);
        }
    }
    */

    init_coreclk();

    // These take like 16 cycles to actually propagate. We can't go sending
    // stuff before they come out of reset. So wait.
    // TODO: Add a register to read the current reset states, or DDR Control
    // device?
    for _ in 0..=255 {
        unsafe { asm!("nop") }
    }
    // self.init_pll_ge();
    //        self.dev_reset
    //            .set(reset_mask(false, false, false, false, false));

    unsafe { asm!("fence") };
}

pub const SYSCON_SYSMAIN_CTRL_BASE: u32 = 0x00_1185_0000;
pub const SYSCON_SYSMAIN_PLL0: u32 = SYSCON_SYSMAIN_CTRL_BASE + 0x0000;
pub const SYSCON_SYSMAIN_PLL1: u32 = SYSCON_SYSMAIN_CTRL_BASE + 0x0004;
pub const SYSCON_SYSMAIN_PLL2: u32 = SYSCON_SYSMAIN_CTRL_BASE + 0x0008;
pub const SYSCON_SYSMAIN_CTRL28: u32 = SYSCON_SYSMAIN_CTRL_BASE + 0x0070;
pub const SYSCON_SYSMAIN_CTRL68: u32 = SYSCON_SYSMAIN_CTRL_BASE + 0x0110;
pub const SYSCON_SYSMAIN_CTRL69: u32 = SYSCON_SYSMAIN_CTRL_BASE + 0x0114;

// SYSCON_SYSMAIN_PLLx [31:24] [23:16] [15:8]   [7:4] [3]    [2]   [1] [0]
//                      OD      BWADJ  CLKFDIV  CLKR  bypass infb  pd  rst

pub fn syscon_pll1_reset(r: bool) {
    if r {
        write_32(SYSCON_SYSMAIN_PLL1, 0x0029_2905);
    } else {
        write_32(SYSCON_SYSMAIN_PLL1, 0x0029_2904);
    }
}

pub fn syscon_gmac_phy_intf_sel(v: u32) {
    let nv = read_32(SYSCON_SYSMAIN_CTRL28) & !(0x7);
    write_32(SYSCON_SYSMAIN_CTRL28, nv | (v & 0x7));
}

pub fn disable_u74_memaxi_remap(v: u32) {
    let nv = read_32(SYSCON_SYSMAIN_CTRL68) & !(0x1);
    write_32(SYSCON_SYSMAIN_CTRL68, nv | (v & 0x1));
}

pub fn syscon_core1_en(v: u32) {
    let nv = read_32(SYSCON_SYSMAIN_CTRL69) & !(0x1);
    write_32(SYSCON_SYSMAIN_CTRL69, nv | (v & 0x1));
}

pub const SYSCON_IOPAD_CTRL_BASE: u32 = 0x00_1185_8000;
pub const SYSCON_IOPAD_CTRL32: u32 = SYSCON_IOPAD_CTRL_BASE + 0x80;
pub const SYSCON_IOPAD_CTRL33: u32 = SYSCON_IOPAD_CTRL_BASE + 0x84;
pub const SYSCON_IOPAD_CTRL34: u32 = SYSCON_IOPAD_CTRL_BASE + 0x88;
pub const SYSCON_IOPAD_CTRL35: u32 = SYSCON_IOPAD_CTRL_BASE + 0x8c;
pub const SYSCON_IOPAD_CTRL38: u32 = SYSCON_IOPAD_CTRL_BASE + 0x98;
pub const SYSCON_IOPAD_CTRL39: u32 = SYSCON_IOPAD_CTRL_BASE + 0x9C;
pub const SYSCON_IOPAD_CTRL50: u32 = SYSCON_IOPAD_CTRL_BASE + 0xC8;
pub const SYSCON_IOPAD_CTRL89: u32 = SYSCON_IOPAD_CTRL_BASE + 0x164;
pub const SYSCON_IOPAD_CTRL90: u32 = SYSCON_IOPAD_CTRL_BASE + 0x168;
pub const SYSCON_IOPAD_CTRL91: u32 = SYSCON_IOPAD_CTRL_BASE + 0x16C;
pub const SYSCON_IOPAD_CTRL92: u32 = SYSCON_IOPAD_CTRL_BASE + 0x170;
pub const SYSCON_IOPAD_CTRL93: u32 = SYSCON_IOPAD_CTRL_BASE + 0x174;
pub const SYSCON_IOPAD_CTRL94: u32 = SYSCON_IOPAD_CTRL_BASE + 0x178;
pub const SYSCON_IOPAD_CTRL95: u32 = SYSCON_IOPAD_CTRL_BASE + 0x17C;
pub const SYSCON_IOPAD_CTRL96: u32 = SYSCON_IOPAD_CTRL_BASE + 0x180;
pub const SYSCON_IOPAD_CTRL97: u32 = SYSCON_IOPAD_CTRL_BASE + 0x184;
pub const SYSCON_IOPAD_CTRL98: u32 = SYSCON_IOPAD_CTRL_BASE + 0x188;
pub const SYSCON_IOPAD_CTRL99: u32 = SYSCON_IOPAD_CTRL_BASE + 0x18C;
pub const SYSCON_IOPAD_CTRL100: u32 = SYSCON_IOPAD_CTRL_BASE + 0x190;
pub const SYSCON_IOPAD_CTRL101: u32 = SYSCON_IOPAD_CTRL_BASE + 0x194;
pub const SYSCON_IOPAD_CTRL102: u32 = SYSCON_IOPAD_CTRL_BASE + 0x198;
pub const SYSCON_IOPAD_CTRL103: u32 = SYSCON_IOPAD_CTRL_BASE + 0x19C;
pub const SYSCON_IOPAD_CTRL104: u32 = SYSCON_IOPAD_CTRL_BASE + 0x1A0;

pub fn syscon_func_0(v: u32) {
    // NOTE: for whatever reason, it appears that writing only works after
    // reading i.e., if you remove the `read_32`, it breaks the code
    // let's hope the compiler does not remove it in optimization
    read_32(SYSCON_IOPAD_CTRL32);
    write_32(SYSCON_IOPAD_CTRL32, v);
}

pub fn syscon_func_1(v: u32) {
    read_32(SYSCON_IOPAD_CTRL33);
    write_32(SYSCON_IOPAD_CTRL33, v);
}

pub fn syscon_func_2(v: u32) {
    read_32(SYSCON_IOPAD_CTRL34);
    write_32(SYSCON_IOPAD_CTRL34, v);
}

pub fn syscon_func_3(v: u32) {
    read_32(SYSCON_IOPAD_CTRL35);
    write_32(SYSCON_IOPAD_CTRL35, v);
}

pub fn syscon_func_6(v: u32) {
    read_32(SYSCON_IOPAD_CTRL38);
    write_32(SYSCON_IOPAD_CTRL38, v);
}

pub fn syscon_func_7(v: u32) {
    read_32(SYSCON_IOPAD_CTRL39);
    write_32(SYSCON_IOPAD_CTRL39, v);
}

pub fn syscon_func_18(v: u32) {
    read_32(SYSCON_IOPAD_CTRL50);
    write_32(SYSCON_IOPAD_CTRL50, v);
}

pub fn syscon_func_57(v: u32) {
    read_32(SYSCON_IOPAD_CTRL89);
    write_32(SYSCON_IOPAD_CTRL89, v);
}

pub fn syscon_func_58(v: u32) {
    read_32(SYSCON_IOPAD_CTRL90);
    write_32(SYSCON_IOPAD_CTRL90, v);
}

pub fn syscon_func_59(v: u32) {
    read_32(SYSCON_IOPAD_CTRL91);
    write_32(SYSCON_IOPAD_CTRL91, v);
}

pub fn syscon_func_60(v: u32) {
    read_32(SYSCON_IOPAD_CTRL92);
    write_32(SYSCON_IOPAD_CTRL92, v);
}

pub fn syscon_func_61(v: u32) {
    read_32(SYSCON_IOPAD_CTRL93);
    write_32(SYSCON_IOPAD_CTRL93, v);
}

pub fn syscon_func_62(v: u32) {
    read_32(SYSCON_IOPAD_CTRL94);
    write_32(SYSCON_IOPAD_CTRL94, v);
}

pub fn syscon_func_63(v: u32) {
    read_32(SYSCON_IOPAD_CTRL95);
    write_32(SYSCON_IOPAD_CTRL95, v);
}

pub fn syscon_func_64(v: u32) {
    read_32(SYSCON_IOPAD_CTRL96);
    write_32(SYSCON_IOPAD_CTRL96, v);
}

pub fn syscon_func_65(v: u32) {
    read_32(SYSCON_IOPAD_CTRL97);
    write_32(SYSCON_IOPAD_CTRL97, v);
}

pub fn syscon_func_66(v: u32) {
    read_32(SYSCON_IOPAD_CTRL98);
    write_32(SYSCON_IOPAD_CTRL98, v);
}
pub fn syscon_func_67(v: u32) {
    read_32(SYSCON_IOPAD_CTRL99);
    write_32(SYSCON_IOPAD_CTRL99, v);
}

pub fn syscon_func_68(v: u32) {
    read_32(SYSCON_IOPAD_CTRL100);
    write_32(SYSCON_IOPAD_CTRL100, v);
}

pub fn syscon_func_69(v: u32) {
    read_32(SYSCON_IOPAD_CTRL101);
    write_32(SYSCON_IOPAD_CTRL101, v);
}

pub fn syscon_func_70(v: u32) {
    read_32(SYSCON_IOPAD_CTRL102);
    write_32(SYSCON_IOPAD_CTRL102, v);
}

// syscon_func_72 ?
// moves the UART pins
// select the function n multiplexed signal group
pub fn syscon_io_padshare_sel(v: u32) {
    let nv = read_32(SYSCON_IOPAD_CTRL104) & !(0x7);
    write_32(SYSCON_IOPAD_CTRL104, nv | v & 0x7);
}

pub fn iopad_init() {
    syscon_func_0(0x00c0_0000);
    syscon_func_1(0x00c0_00c0);
    syscon_func_2(0x00c0_00c0);
    syscon_func_3(0x00c0_00c0);
    syscon_func_7(0x00c3_00c3);
    syscon_func_6(0x00c0_0000);
    unsafe { asm!("fence") };
}

pub const RSTGEN_BASE: u32 = 0x1184_0000;
#[allow(clippy::identity_op)]
pub const RSTGEN_SOFT_ASSERT0: u32 = RSTGEN_BASE + 0x0;
pub const RSTGEN_SOFT_ASSERT1: u32 = RSTGEN_BASE + 0x4;
pub const RSTGEN_SOFT_ASSERT2: u32 = RSTGEN_BASE + 0x8;
pub const RSTGEN_SOFT_ASSERT3: u32 = RSTGEN_BASE + 0xC;

pub const RSTGEN_SOFT_STATUS0: u32 = RSTGEN_BASE + 0x10;
pub const RSTGEN_SOFT_STATUS1: u32 = RSTGEN_BASE + 0x14;
pub const RSTGEN_SOFT_STATUS2: u32 = RSTGEN_BASE + 0x18;
pub const RSTGEN_SOFT_STATUS3: u32 = RSTGEN_BASE + 0x1C;

pub fn clear_rstgen_rstn_usbnoc_axi_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 6);
    v |= 0 << 6;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 6;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn clear_rstgen_rstn_hifi4noc_axi_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 2);
    v |= 0 << 2;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 2;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn clear_rstgen_rstn_x2c_axi_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 9);
    v |= 0 << 9;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 9;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn assert_rstgen_rstn_x2c_axi_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 9);
    v |= 1 << 9;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 9;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn clear_rstgen_rstn_msi_apb_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT3);
    v &= !(0x1 << 14);
    v |= 0 << 14;
    write_32(RSTGEN_SOFT_ASSERT3, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS3) >> 14;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn assert_rstgen_rstn_ddrphy_ahb() {
    let v = read_32(RSTGEN_SOFT_ASSERT1) & !(1 << 30);
    write_32(RSTGEN_SOFT_ASSERT1, v | 1 << 30);
    loop {
        if !((read_32(RSTGEN_SOFT_STATUS1) >> 30) & 1) != 0 {
            break;
        }
    }
}

pub fn clear_rstgen_rstn_dspx2c_axi_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 14);
    v |= 0 << 14;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 14;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn clear_rstgen_rstn_dma1p_axi_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 8);
    v |= 0 << 8;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 8;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn clear_rstgen_rstn_gmac_ahb_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 28);
    v |= 0 << 28;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 28;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn assert_rstgen_rstn_gmac_ahb_() {
    let mut v = read_32(RSTGEN_SOFT_ASSERT1);
    v &= !(0x1 << 28);
    v |= 1 << 28;
    write_32(RSTGEN_SOFT_ASSERT1, v);
    loop {
        let mut v = read_32(RSTGEN_SOFT_STATUS1) >> 28;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn rstgen_init() {
    clear_rstgen_rstn_usbnoc_axi_();
    clear_rstgen_rstn_hifi4noc_axi_();

    enable_clk_x2c_axi_();
    clear_rstgen_rstn_x2c_axi_();

    clear_rstgen_rstn_dspx2c_axi_();
    clear_rstgen_rstn_dma1p_axi_();

    enable_clk_msi_apb_();
    clear_rstgen_rstn_msi_apb_();

    assert_rstgen_rstn_x2c_axi_();
    clear_rstgen_rstn_x2c_axi_();
    unsafe { asm!("fence") };
}

pub fn gmac_init() {
    /*phy must use gpio to hardware reset*/
    enable_clk_gmac_ahb_();
    enable_clk_gmac_ptp_refclk_();
    enable_clk_gmac_gtxclk_();
    assert_rstgen_rstn_gmac_ahb_();
    // GMAC_PHY_RXCLK ...?
    syscon_func_57(0x00030080);
    syscon_func_58(0x00030080);
    syscon_func_59(0x00030003);
    syscon_func_60(0x00030003);
    syscon_func_61(0x00030003);
    syscon_func_62(0x00030003);
    syscon_func_63(0x00800003);
    syscon_func_64(0x00800080);
    syscon_func_65(0x00800080);
    syscon_func_66(0x00800080);
    syscon_func_67(0x00800080);
    syscon_func_68(0x00800080);
    syscon_func_69(0x00800080);
    // GMAC_MDC ?
    syscon_func_70(0x00800080);

    clear_rstgen_rstn_gmac_ahb_();
    syscon_gmac_phy_intf_sel(0x1); //rgmii
}
