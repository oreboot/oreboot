use crate::util::{clear_bit, read32, set_bit, sleep, udelay, write32};

const GLB_BASE: usize = 0x2000_0000;

const LDO12_UHS: usize = GLB_BASE + 0x06d0;
const UHS_PLL_CFG0: usize = GLB_BASE + 0x07d0;
const UHS_PLL_CFG1: usize = GLB_BASE + 0x07d4;
const UHS_PLL_CFG4: usize = GLB_BASE + 0x07e0;
const UHS_PLL_CFG5: usize = GLB_BASE + 0x07e4;
const UHS_PLL_CFG6: usize = GLB_BASE + 0x07e8;

const LDO12_UHS_PU_LDO12_UHS_BIT: u32 = 0;
const LDO12_UHS_VOUT_SEL_OFFSET: u32 = 20;
const LDO12_UHS_VOUT_SEL_MASK: u32 = 0b1111 << LDO12_UHS_VOUT_SEL_OFFSET;

const UHS_PLL_CFG0_SDM_RSTB_BIT: u32 = 0;
const UHS_PLL_CFG0_FBDV_RSTB_BIT: u32 = 2;
const UHS_PLL_CFG0_PU_UHSPLL_SFREG_BIT: u32 = 9;
const UHS_PLL_CFG0_PU_UHSPLL_BIT: u32 = 10;

const UHS_PLL_CFG1_EVEN_DIV_EN_BIT: u32 = 7;
const UHS_PLL_CFG1_EVEN_DIV_RATIO_MASK: u32 = 0b0111_1111;
const UHS_PLL_CFG1_REFERENCE_DIVIDER_RATIO_OFFSET: u32 = 8;
const UHS_PLL_CFG1_REFERENCE_DIVIDER_RATIO_MASK: u32 =
    0b1111 << UHS_PLL_CFG1_REFERENCE_DIVIDER_RATIO_OFFSET;
const UHS_PLL_CFG1_REFERENCE_CLOCK_SELECT_OFFSET: u32 = 16;
const UHS_PLL_CFG1_REFERENCE_CLOCK_SELECT_MASK: u32 =
    0b11 << UHS_PLL_CFG1_REFERENCE_CLOCK_SELECT_OFFSET;

const UHS_PLL_CFG4_SEL_SAMPLE_CLK_MASK: u32 = 0b11;

const UHS_PLL_CFG5_VCO_SPEED_MASK: u32 = 0b111;

const UHS_PLL_CFG6_SD_MIN_MASK: u32 = 0x7_ffff;

pub fn glb_power_up_ldo12uhs() {
    set_bit(LDO12_UHS, LDO12_UHS_PU_LDO12_UHS_BIT);
    udelay(300);
    let ldo12_uhs = read32(LDO12_UHS);
    let m = !LDO12_UHS_VOUT_SEL_MASK;
    let v = 6 << LDO12_UHS_VOUT_SEL_OFFSET;
    write32(LDO12_UHS, (ldo12_uhs & m) | v);
    udelay(1);
}

pub fn pll() {
    // https://github.com/smaeul/u-boot/tree/bl808
    // 2c940eed61391a1adde52ce61b67bd5994f06866
    // drivers/ram/bflb/psram.c
    let cfg1 = read32(UHS_PLL_CFG1);
    let m = !UHS_PLL_CFG1_REFERENCE_CLOCK_SELECT_MASK;
    write32(UHS_PLL_CFG1, cfg1 & m);

    let cfg1 = read32(UHS_PLL_CFG1);
    let m = !UHS_PLL_CFG1_REFERENCE_DIVIDER_RATIO_MASK;
    let v = 2 << UHS_PLL_CFG1_REFERENCE_DIVIDER_RATIO_OFFSET;
    write32(UHS_PLL_CFG1, (cfg1 & m) | v);

    let cfg4 = read32(UHS_PLL_CFG4);
    let m = !UHS_PLL_CFG4_SEL_SAMPLE_CLK_MASK;
    write32(UHS_PLL_CFG4, (cfg4 & m) | 2);

    let cfg5 = read32(UHS_PLL_CFG5);
    let m = !UHS_PLL_CFG5_VCO_SPEED_MASK;
    write32(UHS_PLL_CFG5, (cfg5 & m) | 4);

    let cfg1 = read32(UHS_PLL_CFG1);
    let m = !((1 << UHS_PLL_CFG1_EVEN_DIV_EN_BIT) | UHS_PLL_CFG1_EVEN_DIV_RATIO_MASK);
    let v = (1 << UHS_PLL_CFG1_EVEN_DIV_EN_BIT) | 28;
    write32(UHS_PLL_CFG1, (cfg1 & m) | v);

    let cfg6 = read32(UHS_PLL_CFG6);
    let m = !(UHS_PLL_CFG6_SD_MIN_MASK);
    write32(UHS_PLL_CFG6, (cfg6 & m) | 143360);

    set_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_PU_UHSPLL_SFREG_BIT);
    udelay(3);

    set_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_PU_UHSPLL_BIT);
    udelay(3);

    set_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_SDM_RSTB_BIT);
    udelay(2);
    clear_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_SDM_RSTB_BIT);
    udelay(2);
    set_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_SDM_RSTB_BIT);

    set_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_FBDV_RSTB_BIT);
    udelay(2);
    clear_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_FBDV_RSTB_BIT);
    udelay(2);
    set_bit(UHS_PLL_CFG0, UHS_PLL_CFG0_FBDV_RSTB_BIT);

    udelay(45);
}
