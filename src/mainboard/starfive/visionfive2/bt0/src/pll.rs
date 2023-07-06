use crate::init::{self, read32, write32};

// see `boot/arch/riscv/cpu/jh7110/pll.c` `pll_set_rate`
// NOTE: The order may be irrelevant, which would allow for simplification.

// ---------- see JH7110 SoC manual p61
pub struct PllFreq {
    prediv: u32,
    fbdiv: u32, // feedback divider
    postdiv1: u32,
    dacpd: u32,
    dsmpd: u32,
}

pub const PLL0_1000000000: PllFreq = PllFreq {
    prediv: 3,
    fbdiv: 125,
    postdiv1: 1,
    dacpd: 1,
    dsmpd: 1,
};

pub const PLL1_DDR2133_1066000000: PllFreq = PllFreq {
    prediv: 12,
    fbdiv: 533,
    postdiv1: 1,
    dacpd: 1,
    dsmpd: 1,
};

pub const PLL1_DDR_LOW_SPEED: PllFreq = PllFreq {
    prediv: 1,
    fbdiv: 533,
    postdiv1: 1,
    dacpd: 0,
    dsmpd: 0,
};

pub const PLL2_1188000000: PllFreq = PllFreq {
    prediv: 2,
    fbdiv: 99,
    postdiv1: 1,
    dacpd: 1,
    dsmpd: 1,
};

// TODO: Tock register?

const PD_MASK: u32 = !(1 << 27);
// NOTE: active low
const PD_OFF: u32 = 1;
const PD_ON: u32 = 0;

// PLL0
const SYSCON24_PLL0_DACPD_MASK: u32 = !(1 << 24);
const SYSCON24_PLL0_DSMPD_MASK: u32 = !(1 << 25);
// NOTE: This feedback divider differs for PLL0. PLL1 and PLL2 use other bits.
const SYSCON28_PLL0_FBDIV_MASK: u32 = !(0x0000_07ff); // 0-11
const SYSCON32_PLL0_POSTDIV1_MASK: u32 = !0x3000_0000; // 28-29
const SYSCON36_PLL0_PREDIV_MASK: u32 = !0x0000_003f; // 0-5

pub fn pll0_set_freq(f: PllFreq) {
    let pd = read32(init::SYS_SYSCON_32) & PD_MASK;
    write32(init::SYS_SYSCON_32, pd | PD_OFF << 27);

    let v = read32(init::SYS_SYSCON_24) & SYSCON24_PLL0_DACPD_MASK;
    write32(init::SYS_SYSCON_24, v | f.dacpd << 24);
    let v = read32(init::SYS_SYSCON_24) & SYSCON24_PLL0_DSMPD_MASK;
    write32(init::SYS_SYSCON_24, v | f.dsmpd << 25);

    let v = read32(init::SYS_SYSCON_36) & SYSCON36_PLL0_PREDIV_MASK;
    write32(init::SYS_SYSCON_36, v | f.prediv);

    let v = read32(init::SYS_SYSCON_28) & SYSCON28_PLL0_FBDIV_MASK;
    write32(init::SYS_SYSCON_28, v | f.fbdiv);

    let v = read32(init::SYS_SYSCON_32) & SYSCON32_PLL0_POSTDIV1_MASK;
    // NOTE: Not sure why, but the original code does this shift, and defines
    // all postdiv values for all PLLs and config to be 1, effectively dropping
    // to 0 here.
    write32(init::SYS_SYSCON_32, v | ((f.postdiv1 >> 1) << 28));

    let pd = read32(init::SYS_SYSCON_32) & PD_MASK;
    write32(init::SYS_SYSCON_32, pd | PD_ON << 27);
}

const SYSCON36_PLL1_DACPD_MASK: u32 = !(1 << 15);
const SYSCON36_PLL1_DSMPD_MASK: u32 = !(1 << 16);
const SYSCON36_PLL1_FBDIV_MASK: u32 = !(0x1ffe_0000); // 17-28
const SYSCON40_PLL1_POSTDIV1_MASK: u32 = !0x3000_0000; // 28-29
const SYSCON40_PLL1_FRAC_MASK: u32 = !0x00ff_ffff; // 0-23
const SYSCON44_PLL1_PREDIV_MASK: u32 = !0x0000_003f; // 0-5

// 2133 / 1066 yields:
// PLL1: 00b02603 55e00000 00c7a601
// PLL1: 042ba603 41e00000 00c7a60c
// vs low speed:
// PLL1: 00b02603 55e00000 00c7a601
// PLL1: 042a2603 41e00000 00c7a601
pub fn pll1_set_freq(f: PllFreq) {
    let v1 = read32(init::SYS_SYSCON_36);
    let v2 = read32(init::SYS_SYSCON_40);
    let v3 = read32(init::SYS_SYSCON_44);
    println!("PLL1: {v1:08x} {v2:08x} {v3:08x}");
    let pd = read32(init::SYS_SYSCON_40) & PD_MASK;
    write32(init::SYS_SYSCON_40, pd | PD_OFF << 27);

    let v = read32(init::SYS_SYSCON_36) & SYSCON36_PLL1_DACPD_MASK;
    write32(init::SYS_SYSCON_36, v | f.dacpd << 15);
    let v = read32(init::SYS_SYSCON_36) & SYSCON36_PLL1_DSMPD_MASK;
    write32(init::SYS_SYSCON_36, v | f.dsmpd << 16);

    let frac = 0xe00000;
    let v = read32(init::SYS_SYSCON_40) & SYSCON40_PLL1_FRAC_MASK;
    write32(init::SYS_SYSCON_40, v | frac);

    let v = read32(init::SYS_SYSCON_44) & SYSCON44_PLL1_PREDIV_MASK;
    write32(init::SYS_SYSCON_44, v | f.prediv);

    let v = read32(init::SYS_SYSCON_36) & SYSCON36_PLL1_FBDIV_MASK;
    write32(init::SYS_SYSCON_36, v | f.fbdiv << 17);

    let v = read32(init::SYS_SYSCON_40) & SYSCON40_PLL1_POSTDIV1_MASK;
    // NOTE: Not sure why, but the original code does this shift, and defines
    // all postdiv values for all PLLs and config to be 1, effectively dropping
    // to 0 here.
    write32(init::SYS_SYSCON_40, v | ((f.postdiv1 >> 1) << 28));

    let pd = read32(init::SYS_SYSCON_40) & PD_MASK;
    write32(init::SYS_SYSCON_40, pd | PD_ON << 27);
    let v1 = read32(init::SYS_SYSCON_36);
    let v2 = read32(init::SYS_SYSCON_40);
    let v3 = read32(init::SYS_SYSCON_44);
    println!("PLL1: {v1:08x} {v2:08x} {v3:08x}");
}

const SYSCON44_PLL2_DACPD_MASK: u32 = !(1 << 15);
const SYSCON44_PLL2_DSMPD_MASK: u32 = !(1 << 16);
const SYSCON44_PLL2_FBDIV_MASK: u32 = !(0x1ffe_0000); // 17-28
const SYSCON48_PLL2_POSTDIV1_MASK: u32 = !0x3000_0000; // 28-29, SYSCON 48
const SYSCON52_PLL2_PREDIV_MASK: u32 = !0x0000_003f; // 0-5, SYSCON 52

pub fn pll2_set_freq(f: PllFreq) {
    let pd = read32(init::SYS_SYSCON_48) & PD_MASK;
    write32(init::SYS_SYSCON_48, pd | PD_OFF << 27);

    let v = read32(init::SYS_SYSCON_44) & SYSCON44_PLL2_DACPD_MASK;
    write32(init::SYS_SYSCON_44, v | f.dacpd << 15);
    let v = read32(init::SYS_SYSCON_44) & SYSCON44_PLL2_DSMPD_MASK;
    write32(init::SYS_SYSCON_44, v | f.dsmpd << 16);

    let v = read32(init::SYS_SYSCON_52) & SYSCON52_PLL2_PREDIV_MASK;
    write32(init::SYS_SYSCON_52, v | f.prediv);

    let v = read32(init::SYS_SYSCON_44) & SYSCON44_PLL2_FBDIV_MASK;
    write32(init::SYS_SYSCON_44, v | f.fbdiv << 17);

    let v = read32(init::SYS_SYSCON_48) & SYSCON48_PLL2_POSTDIV1_MASK;
    // NOTE: Not sure why, but the original code does this shift, and defines
    // all postdiv values for all PLLs and config to be 1, effectively dropping
    // to 0 here.
    write32(init::SYS_SYSCON_48, v | ((f.postdiv1 >> 1) << 28));

    let pd = read32(init::SYS_SYSCON_48) & PD_MASK;
    write32(init::SYS_SYSCON_48, pd | PD_ON << 27);
}
