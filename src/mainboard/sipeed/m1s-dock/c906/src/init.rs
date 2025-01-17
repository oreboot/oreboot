use crate::util::{clear_bit, read32, set_bit, sleep, udelay, write32};

const GLB_BASE: usize = 0x20000000;
const UHS_PLL_CFG0: usize = GLB_BASE + 0x07d0;
const UHS_PLL_CFG1: usize = GLB_BASE + 0x07d4;
const UHS_PLL_CFG4: usize = GLB_BASE + 0x07e0;
const UHS_PLL_CFG5: usize = GLB_BASE + 0x07e4;
const UHS_PLL_CFG6: usize = GLB_BASE + 0x07e8;

pub fn pll() {
    const UHS_PLL_CFG1_EVEN_DIV_EN: u32 = 1 << 7;
    const UHS_PLL_CFG1_EVEN_DIV_RATIO: u32 = 0b1111111;

    // https://github.com/smaeul/u-boot/tree/bl808
    // 2c940eed61391a1adde52ce61b67bd5994f06866
    // drivers/ram/bflb/psram.c
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");
    write32(UHS_PLL_CFG1, (cfg1 & !(0b11 << 16)));
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");
    write32(UHS_PLL_CFG1, (cfg1 & !(0b1111 << 8)) | (0b0010 << 8));
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");

    let cfg4 = read32(UHS_PLL_CFG4);
    println!("PLL CFG4: {cfg4:08x}");
    write32(UHS_PLL_CFG4, (cfg4 & !(0b11)) | 0b10);
    let cfg4 = read32(UHS_PLL_CFG4);
    println!("PLL CFG4: {cfg4:08x}");

    let cfg5 = read32(UHS_PLL_CFG5);
    println!("PLL CFG5: {cfg5:08x}");
    write32(UHS_PLL_CFG5, (cfg5 & !(0b111)) | 0b100);
    let cfg5 = read32(UHS_PLL_CFG5);
    println!("PLL CFG5: {cfg5:08x}");

    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");
    let m = !(UHS_PLL_CFG1_EVEN_DIV_EN | UHS_PLL_CFG1_EVEN_DIV_RATIO);
    write32(UHS_PLL_CFG1, (cfg1 & m) | UHS_PLL_CFG1_EVEN_DIV_EN | 28);
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");

    const UHS_PLL_CFG6_SDMIN: u32 = 0x7_ffff;
    let cfg6 = read32(UHS_PLL_CFG6);
    println!("PLL CFG6: {cfg6:08x}");
    let m = !(UHS_PLL_CFG6_SDMIN);
    write32(UHS_PLL_CFG6, (cfg6 & m) | 143360);
    let cfg6 = read32(UHS_PLL_CFG6);
    println!("PLL CFG6: {cfg6:08x}");

    const UHS_PLL_CFG0_SDM_RSTB_BIT: u32 = 0;
    const UHS_PLL_CFG0_FBDV_RSTB_BIT: u32 = 2;
    const UHS_PLL_CFG0_PU_UHSPLL_SFREG_BIT: u32 = 9;
    const UHS_PLL_CFG0_PU_UHSPLL_BIT: u32 = 10;

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
