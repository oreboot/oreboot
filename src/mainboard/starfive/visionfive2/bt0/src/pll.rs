use crate::init::{self, pac, read32, udelay, write32};

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

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
fn sys_syscon_reg<'r>() -> &'r pac::sys_syscon::RegisterBlock {
    unsafe { &*pac::SYS_SYSCON::ptr() }
}

pub fn pll0_set_freq(f: PllFreq) {
    let syscon = sys_syscon_reg();

    // NOTE: all register name offset values use zero-indexed, array-based numbering
    // This is in contrast to the address-offset numbering used in the TRM
    // Basically, divide the TRM numbering by four to get the PAC numbering

    // Turn off PD by setting the bit 
    syscon.sys_syscfg_8().modify(|_, w| w.pll0_pd().set_bit());

    syscon.sys_syscfg_6().modify(|_, w| {
        w.pll0_dacpd().variant(f.dacpd != 0).pll0_dsmpd().variant(f.dsmpd != 0)
    });

    syscon.sys_syscfg_9().modify(|_, w| w.pll0_prediv().variant(f.prediv as u8));
    syscon.sys_syscfg_7().modify(|_, w| w.pll0_fbdiv().variant(f.fbdiv as u16));

    // NOTE: Not sure why, but the original code does this shift, and defines
    // all postdiv values for all PLLs and config to be 1, effectively dropping
    // to 0 here.
    syscon.sys_syscfg_8().modify(|_, w| {
        w.pll0_postdiv1().variant((f.postdiv1 >> 1) as u8);
        // Turn on PD by clearing the bit
        w.pll0_pd().clear_bit()
    });
}

// 2133 / 1066 yields:
// PLL1: 00b02603 55e00000 00c7a601
// PLL1: 042ba603 41e00000 00c7a60c
// vs low speed:
// PLL1: 00b02603 55e00000 00c7a601
// PLL1: 042a2603 41e00000 00c7a601
pub fn pll1_set_freq(f: PllFreq) {
    let syscon = sys_syscon_reg();

    let v1 = syscon.sys_syscfg_9().read().bits();
    let v2 = syscon.sys_syscfg_8().read().bits();
    let v3 = syscon.sys_syscfg_11().read().bits();
    println!("PLL1: {v1:08x} {v2:08x} {v3:08x}");

    // Turn off PD by setting the bit 
    syscon.sys_syscfg_10().modify(|_, w| w.pll1_pd().set_bit());

    syscon.sys_syscfg_9().modify(|_, w| {
        w.pll1_dacpd().variant(f.dacpd !=0).pll1_dsmpd().variant(f.dsmpd != 0)
    });

    let frac = 0xe00000;
    syscon.sys_syscfg_10().modify(|_, w| w.pll1_frac().variant(frac));

    syscon.sys_syscfg_11().modify(|_, w| w.pll1_prediv().variant(f.prediv as u8));

    syscon.sys_syscfg_9().modify(|_, w| w.pll1_fbdiv().variant(f.fbdiv as u16));

    // NOTE: Not sure why, but the original code does this shift, and defines
    // all postdiv values for all PLLs and config to be 1, effectively dropping
    // to 0 here.
    syscon.sys_syscfg_10().modify(|_, w|{ 
        w.pll1_postdiv1().variant((f.postdiv1 >> 1) as u8);
        // Turn on PD by clearing the bit 
        w.pll1_pd().clear_bit()
    });

    let v1 = syscon.sys_syscfg_9().read().bits();
    let v2 = syscon.sys_syscfg_10().read().bits();
    let v3 = syscon.sys_syscfg_11().read().bits();
    println!("PLL1: {v1:08x} {v2:08x} {v3:08x}");
}

pub fn pll2_set_freq(f: PllFreq) {
    let syscon = sys_syscon_reg();

    // Turn PD off by setting the bit
    syscon.sys_syscfg_12().modify(|_, w| w.pll2_pd().set_bit());

    syscon.sys_syscfg_11().modify(|_, w| {
        w.pll2_dacpd().variant(f.dacpd != 0).pll2_dsmpd().variant(f.dsmpd != 0)
    });

    syscon.sys_syscfg_13().modify(|_, w| w.pll2_prediv().variant(f.prediv as u8));

    syscon.sys_syscfg_11().modify(|_, w| w.pll2_fbdiv().variant(f.fbdiv as u16));

    // NOTE: Not sure why, but the original code does this shift, and defines
    // all postdiv values for all PLLs and config to be 1, effectively dropping
    // to 0 here.
    syscon.sys_syscfg_12().modify(|_, w| {
        w.pll2_postdiv1().variant((f.postdiv1 >> 1) as u8);
        // Turn PD on by clearing the bit
        w.pll2_pd().clear_bit()
    });
}
