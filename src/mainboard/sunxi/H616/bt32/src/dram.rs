use crate::mem_map::*;
use core::arch::{asm, naked_asm};
use util::mmio::{read32, write32};
extern crate log;

// https://github.com/u-boot/u-boot/blob/master/arch/arm/mach-sunxi/dram_sun50i_h616.c
// Using LPDDR4 implementation as reference

// https://linux-sunxi.org/DRAM_Controller#sun50i_.28Allwinner_A133.2C_H616.29_hardware

const CCU_MBUS_CFG: usize = CCU_BASE + 0x0000_0540;
const CCU_PLL5_CFG: usize = CCU_BASE + 0x0000_0010;
const CCU_DRAM_CFG: usize = CCU_BASE + 0x0000_0800;
const CCU_DRAM_GATE_RST: usize = CCU_BASE + 0x0000_080C;

const MBUS_ENABLE: u32 = 1 << 31;
const MBUS_RESET: u32 = 1 << 30;

const DRAM_GATE: u32 = 1 << 0;
const DRAM_RST: u32 = 1 << 16;

const PLL5_CTL_EN: u32 = 1 << 31;
const PLL5_LOCK_EN: u32 = 1 << 29;
const PLL5_OUT_EN: u32 = 1 << 27;

const DRAM_MOD_RST: u32 = 1 << 30;
const DRAM_CLK_PLL5: u32 = 0 << 24;

// refer: https://github.com/u-boot/u-boot/blob/master/arch/arm/include/asm/arch-sunxi/dram_sun50i_h616.h#L28
const MCTL_COM_CR: usize = DRAM_COM_BASE + 0x000;
const MCTL_COM_UNK_0x008: usize = DRAM_COM_BASE + 0x008;
const MCTL_COM_TMR: usize = DRAM_COM_BASE + 0x00C;
const MCTL_COM_UNK_0x014: usize = DRAM_COM_BASE + 0x014;
const MCTL_COM_MAER0: usize = DRAM_COM_BASE + 0x020;
const MCTL_COM_MAER1: usize = DRAM_COM_BASE + 0x024;
const MCTL_COM_MAER2: usize = DRAM_COM_BASE + 0x028;
const MCTL_COM_BWCR: usize = DRAM_COM_BASE + 0x200;
const MCTL_COM_UNK_0x500: usize = DRAM_COM_BASE + 0x500;

// refer: https://github.com/u-boot/u-boot/blob/master/arch/arm/include/asm/arch-sunxi/dram_sun50i_h616.h#L60
const MCTL_CTL_MSTR: usize = DRAM_CTL_BASE + 0x000;
const MCTL_CTL_STATR: usize = DRAM_CTL_BASE + 0x004;
const MCTL_CTL_CLKEN: usize = DRAM_CTL_BASE + 0x00C;
const MCTL_CTL_MRCTRL0: usize = DRAM_CTL_BASE + 0x010;
const MCTL_CTL_MRCTRL1: usize = DRAM_CTL_BASE + 0x014;
const MCTL_CTL_MRSTATR: usize = DRAM_CTL_BASE + 0x018;
const MCTL_CTL_MRCTRL2: usize = DRAM_CTL_BASE + 0x01c;
const MCTL_CTL_PWRCTL: usize = DRAM_CTL_BASE + 0x030;
const MCTL_CTL_HWLPCTL: usize = DRAM_CTL_BASE + 0x038;
const MCTL_CTL_RFSHCTL13: usize = DRAM_CTL_BASE + 0x060;
const MCTL_CTL_ZQCTL: usize = DRAM_CTL_BASE + 0x180;
const MCTL_CTL_DFIUPD: usize = DRAM_CTL_BASE + 0x1a0;
const MCTL_CTL_DFIMISC: usize = DRAM_CTL_BASE + 0x1b0;
const MCTL_CTL_DFISTAT: usize = DRAM_CTL_BASE + 0x1bc;
const MCTL_CTL_SCHED: usize = DRAM_CTL_BASE + 0x250;
const MCTL_CTL_ODCFG: usize = DRAM_CTL_BASE + 0x240;
const MCTL_CTL_ODTMAP: usize = DRAM_CTL_BASE + 0x244;
const MCTL_CTL_SWCTL: usize = DRAM_CTL_BASE + 0x320;
const MCTL_CTL_SWSTAT: usize = DRAM_CTL_BASE + 0x324;

const MCTL_CTL_UNK2180: usize = DRAM_CTL_BASE + 0x2180;
const MCTL_CTL_UNK3180: usize = DRAM_CTL_BASE + 0x3180;
const MCTL_CTL_UNK4180: usize = DRAM_CTL_BASE + 0x4180;

const MCTL_CTL_UNK2240: usize = DRAM_CTL_BASE + 0x2240;
const MCTL_CTL_UNK3240: usize = DRAM_CTL_BASE + 0x3240;
const MCTL_CTL_UNK4240: usize = DRAM_CTL_BASE + 0x4240;

#[inline(always)]
fn clear_mask32(addr: usize, mask: u32) {
    let val = read32(addr);
    write32(addr, val & !mask);
}

#[inline(always)]
fn set_bits32(addr: usize, bits: u32) {
    let val = read32(addr);
    write32(addr, val | bits);
}

#[inline(always)]
fn clearset_bits32(addr: usize, clear_mask: u32, set_bits: u32) {
    let val = read32(addr);
    write32(addr, (val & !clear_mask) | set_bits);
}

// TODO: get an actual timer to replace this
fn delay(cycs: u32) {
    unsafe {
        for _ in 0..(cycs * 1000) {
            core::hint::spin_loop();
        }
    }
}

#[inline(always)]
fn pll5_ctrl_n(n: u32) -> u32 {
    (n - 1) << 8
}

// TODO: add timeout
#[inline(always)]
fn wait_for_completion(reg: usize, mask: u32, expected_value: u32) {
    while (read32(reg as usize) & mask) != expected_value {}
}

fn mctl_init(clockrate: u32) {
    // Disable and reset everything related to DRAM

    clear_mask32(CCU_MBUS_CFG, MBUS_ENABLE | MBUS_RESET);
    clear_mask32(CCU_DRAM_GATE_RST, DRAM_GATE);

    delay(5);

    clear_mask32(CCU_DRAM_GATE_RST, DRAM_RST);
    clear_mask32(CCU_PLL5_CFG, PLL5_CTL_EN);
    clear_mask32(CCU_DRAM_CFG, DRAM_MOD_RST);

    // set PLL5 rate to 2x DRAM clockrate
    let pll_conf = PLL5_CTL_EN | PLL5_LOCK_EN | PLL5_OUT_EN | pll5_ctrl_n(clockrate * 2 / 24);
    write32(CCU_PLL5_CFG, pll_conf);
    wait_for_completion(CCU_PLL5_CFG, PLL5_LOCK_EN, PLL5_LOCK_EN);

    // configure DRAM mod clock
    write32(CCU_DRAM_CFG, DRAM_CLK_PLL5);
    write32(CCU_DRAM_GATE_RST, DRAM_RST);
    delay(5);
    set_bits32(CCU_DRAM_GATE_RST, DRAM_GATE);

    // disable all channels
    write32(MCTL_COM_MAER0, 0);
    write32(MCTL_COM_MAER1, 0);
    write32(MCTL_COM_MAER2, 0);

    // configure MBUS and enable DRAM mod reset
    set_bits32(CCU_MBUS_CFG, MBUS_RESET);
    set_bits32(CCU_DRAM_CFG, MBUS_ENABLE);

    clear_mask32(MCTL_COM_UNK_0x500, 1 << 25);

    set_bits32(CCU_DRAM_CFG, DRAM_MOD_RST);
    delay(5);

    write32(MCTL_CTL_CLKEN, 0x8000);
}

#[derive(Debug, Clone, Copy)]
pub struct dram_para {
    pub clk: u32,
    //pub type_: SunxiDramType, // we only support LPDDR4 for now
    pub dx_odt: u32,
    pub dx_dri: u32,
    pub ca_dri: u32,
    pub tpr0: u32,
    pub tpr1: u32,
    pub tpr2: u32,
    pub tpr6: u32,
    pub tpr10: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct dram_config {
    pub cols: u8,
    pub rows: u8,
    pub ranks: u8,
    pub bus_full_width: bool,
    pub clk: u32,
    pub odt_en: u32,
    pub tpr11: u32,
    pub tpr12: u32,
    pub tpr14: u32,
}

// random values for now, these will
// be changed during init. Currently there just
// for initialization of these structures
impl Default for dram_para {
    fn default() -> Self {
        dram_para {
            clk: 400,
            dx_odt: 0x0000_0000,
            dx_dri: 0x0404_0404,
            ca_dri: 0x0404_0404,
            tpr0: 0x0000_0000,
            tpr1: 0x0000_0000,
            tpr2: 0x0000_0000,
            tpr6: 0x0000_0000,
            tpr10: 0x0000_0000,
        }
    }
}

impl Default for dram_config {
    fn default() -> Self {
        dram_config {
            cols: 10,
            rows: 15,
            ranks: 1,
            bus_full_width: false,
            clk: 400,
            odt_en: 0,
            tpr11: 0x0000_0000,
            tpr12: 0x0000_0000,
            tpr14: 0x0000_0000,
        }
    }
}

const DRAM_PHY_BASE: usize = 0x04800000;

#[inline(always)]
fn get_bus_width(config: &dram_config) -> u32 {
    if config.bus_full_width {
        0xf
    } else {
        0x3
    }
}

fn mctl_phy_write_leveling(para: &dram_para, config: &dram_config) -> bool {
    clearset_bits32(DRAM_PHY_BASE + 0x8, 0xC0, 0x80);

    // MR2 Value
    write32(DRAM_PHY_BASE + 0xC, 0x1b);
    write32(DRAM_PHY_BASE + 0x10, 0x0);

    set_bits32(DRAM_PHY_BASE + 0x8, 0x4);

    let val = get_bus_width(config);
    let mask = val;
    wait_for_completion(DRAM_PHY_BASE + 0x188, mask, val);

    clear_mask32(DRAM_PHY_BASE + 0x8, 0x4);

    let val = read32(DRAM_PHY_BASE + 0x258);
    if val == 0x3f || val == 0x0 {
        return false;
    }
    let val = read32(DRAM_PHY_BASE + 0x25c);
    if val == 0x3f || val == 0x0 {
        return false;
    }
    let val = read32(DRAM_PHY_BASE + 0x318);
    if val == 0x3f || val == 0x0 {
        return false;
    }
    let val = read32(DRAM_PHY_BASE + 0x31c);
    if val == 0x3f || val == 0x0 {
        return false;
    }

    clear_mask32(DRAM_PHY_BASE + 0x8, 0xC0);

    if config.ranks == 2 {
        let bus_val = get_bus_width(config);

        clearset_bits32(DRAM_PHY_BASE + 0x8, 0xC0, 0x40);
        set_bits32(DRAM_PHY_BASE + 0x8, 0x4);

        wait_for_completion(DRAM_PHY_BASE + 0x188, bus_val, bus_val);
        clear_mask32(DRAM_PHY_BASE + 0x8, 0x4);
    }

    clear_mask32(DRAM_PHY_BASE + 0x8, 0xC0);

    true
}

#[inline(always)]
fn mask_byte(reg: u32, nr: u32) -> u32 {
    (reg >> (nr * 8)) & 0x1f
}

fn mctl_phy_configure_odt(para: &dram_para) {
    // LPDDR4
    clearset_bits32(DRAM_PHY_BASE + 0x390, 1 << 5, 1 << 4);
    clearset_bits32(DRAM_PHY_BASE + 0x3d0, 1 << 5, 1 << 4);
    clearset_bits32(DRAM_PHY_BASE + 0x410, 1 << 5, 1 << 4);
    clearset_bits32(DRAM_PHY_BASE + 0x450, 1 << 5, 1 << 4);

    let val_lo = para.dx_dri;
    let val_hi = 0x04040404;
    write32(DRAM_PHY_BASE + 0x388, mask_byte(val_lo, 0));
    write32(DRAM_PHY_BASE + 0x38c, mask_byte(val_hi, 0));
    write32(DRAM_PHY_BASE + 0x3c8, mask_byte(val_lo, 1));
    write32(DRAM_PHY_BASE + 0x3cc, mask_byte(val_hi, 1));
    write32(DRAM_PHY_BASE + 0x408, mask_byte(val_lo, 2));
    write32(DRAM_PHY_BASE + 0x40c, mask_byte(val_hi, 2));
    write32(DRAM_PHY_BASE + 0x448, mask_byte(val_lo, 3));
    write32(DRAM_PHY_BASE + 0x44c, mask_byte(val_hi, 3));

    let val_lo = para.ca_dri;
    let val_hi = para.ca_dri;
    write32(DRAM_PHY_BASE + 0x340, mask_byte(val_lo, 0));
    write32(DRAM_PHY_BASE + 0x344, mask_byte(val_hi, 0));
    write32(DRAM_PHY_BASE + 0x348, mask_byte(val_lo, 1));
    write32(DRAM_PHY_BASE + 0x34c, mask_byte(val_hi, 1));

    let val_lo = para.dx_odt;
    let val_hi = 0;
    write32(DRAM_PHY_BASE + 0x380, mask_byte(val_lo, 0));
    write32(DRAM_PHY_BASE + 0x384, mask_byte(val_hi, 0));
    write32(DRAM_PHY_BASE + 0x3c0, mask_byte(val_lo, 1));
    write32(DRAM_PHY_BASE + 0x3c4, mask_byte(val_hi, 1));
    write32(DRAM_PHY_BASE + 0x400, mask_byte(val_lo, 2));
    write32(DRAM_PHY_BASE + 0x404, mask_byte(val_hi, 2));
    write32(DRAM_PHY_BASE + 0x440, mask_byte(val_lo, 3));
    write32(DRAM_PHY_BASE + 0x444, mask_byte(val_hi, 3));

    unsafe {
        asm!("dsb sy", "isb");
    }
}

// TODO
fn mctl_phy_bit_delay_compensation(para: &dram_para) {}

// Check if these are needed based on
// params configured
// TODO
fn mctl_phy_read_calibration(para: &dram_para, config: &dram_config) -> bool {
    true
}

// TODO 
fn mctl_phy_read_training(para: &dram_para, config: &dram_config) -> bool {
    true
}

// TODO
fn mctl_phy_write_training(para: &dram_para, config: &dram_config) -> bool {
    true
}

// TODO
fn mctl_set_addrmap(config: &dram_config) {}

// TODO
fn mctl_set_timing_params(para: &dram_para) {}

const phy_init: [u8; 27] = [
    0x02, 0x00, 0x17, 0x05, 0x04, 0x19, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x01, 0x18, 0x03, 0x1a,
];

// TODO
fn mctl_phy_ca_bit_delay_compensation(para: &dram_para, config: &dram_config) {}

const TPR10_CA_BIT_DELAY: u32 = 1 << 16;
const TPR10_DX_BIT_DELAY0: u32 = 1 << 17;
const TPR10_DX_BIT_DELAY1: u32 = 1 << 18;

const TPR10_WRITE_LEVELING: u32 = 1 << 20;
const TPR10_READ_CALIBRATION: u32 = 1 << 21;
const TPR10_READ_TRAINING: u32 = 1 << 22;
const TPR10_WRITE_TRAINING: u32 = 1 << 23;

fn mctl_phy_init(para: &dram_para, config: &dram_config) -> bool {
    clear_mask32(DRAM_PHY_BASE + 0x4, 0x80);

    let val = get_bus_width(config);
    clearset_bits32(DRAM_PHY_BASE + 0x3c, 0xf, val);

    let val = 20;
    let val2 = 10;

    write32(DRAM_PHY_BASE + 0x14, val);
    write32(DRAM_PHY_BASE + 0x35c, val);
    write32(DRAM_PHY_BASE + 0x368, val);
    write32(DRAM_PHY_BASE + 0x374, val);

    write32(DRAM_PHY_BASE + 0x18, 0);
    write32(DRAM_PHY_BASE + 0x360, 0);
    write32(DRAM_PHY_BASE + 0x36c, 0);
    write32(DRAM_PHY_BASE + 0x378, 0);

    write32(DRAM_PHY_BASE + 0x1c, val2);
    write32(DRAM_PHY_BASE + 0x364, val2);
    write32(DRAM_PHY_BASE + 0x370, val2);
    write32(DRAM_PHY_BASE + 0x37c, val2);

    for i in 0..phy_init.len() {
        write32(DRAM_PHY_BASE + 0xc0 + i * 4, phy_init[i] as u32);
    }

    if para.tpr10 & TPR10_CA_BIT_DELAY != 0 {
        mctl_phy_ca_bit_delay_compensation(para, config);
    }

    let val = para.tpr6 >> 24 & 0xff;

    write32(DRAM_PHY_BASE + 0x3dc, val);
    write32(DRAM_PHY_BASE + 0x45c, val);

    mctl_phy_configure_odt(para);

    let val = 0x0d;
    clearset_bits32(DRAM_PHY_BASE + 0x4, 0x7, val);
    if para.clk <= 672 {
        write32(DRAM_PHY_BASE + 0x20, 0xf);
    }

    if para.clk > 500 {
        clear_mask32(DRAM_PHY_BASE + 0x144, 1 << 7);
        clear_mask32(DRAM_PHY_BASE + 0x14c, 0xe0);
    } else {
        set_bits32(DRAM_PHY_BASE + 0x144, 1 << 7);
        clearset_bits32(DRAM_PHY_BASE + 0x14c, 0xe0, 0x20);
    }

    clear_mask32(MCTL_COM_UNK_0x500, 0x200);
    delay(10);

    clear_mask32(DRAM_PHY_BASE + 0x14c, 0x8);

    wait_for_completion(DRAM_PHY_BASE + 0x180, 4, 4);

    delay(1000);

    write32(DRAM_PHY_BASE + 0x58, 0x37);
    write32(MCTL_CTL_SWCTL, 0);
    set_bits32(MCTL_CTL_DFIMISC, 1);

    // DFI init (controller phy interface init)
    set_bits32(MCTL_CTL_DFIMISC, 0x20);
    write32(MCTL_CTL_SWCTL, 1);
    wait_for_completion(MCTL_CTL_SWSTAT, 1, 1);

    // Poll for DFI init complete
    wait_for_completion(MCTL_CTL_DFISTAT, 1, 1);
    write32(MCTL_CTL_SWCTL, 0);
    clear_mask32(MCTL_CTL_DFIMISC, 0x20);

    clear_mask32(MCTL_CTL_PWRCTL, 0x20);
    write32(MCTL_CTL_SWCTL, 1);
    wait_for_completion(MCTL_CTL_SWSTAT, 1, 1);
    wait_for_completion(MCTL_CTL_STATR, 3, 1);

    delay(500);

    write32(MCTL_CTL_SWCTL, 1);
    wait_for_completion(MCTL_CTL_SWSTAT, 1, 1);

    let mut mr0;
    let mut mr2;

    if para.tpr2 & 0x100 != 0 {
        mr0 = 0x1b50;
        mr2 = 0x10;
    } else {
        mr0 = 0x1f14;
        mr2 = 0x20;
    }

    write32(MCTL_CTL_MRCTRL1, 0x0);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0x134);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0x21b);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0x333);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0x403);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0xb04);
    delay(100);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    delay(100);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0xc72);
    delay(100);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    delay(100);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0xe09);
    delay(100);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    delay(100);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(MCTL_CTL_MRCTRL1, 0x1624);
    delay(100);
    write32(MCTL_CTL_MRCTRL0, 0x8000_0030);
    delay(100);
    wait_for_completion(MCTL_CTL_MRSTATR, 1 << 31, 0);

    write32(DRAM_PHY_BASE + 0x54, 0);
    clear_mask32(MCTL_CTL_RFSHCTL13, 0x1);
    write32(MCTL_CTL_SWCTL, 1);

    if para.tpr10 & TPR10_WRITE_LEVELING != 0 {
        for i in 0..5 {
            if mctl_phy_write_leveling(para, config) {
                return true;
            }

            if i == 4 {
                return false;
            }
        }
    }

    if para.tpr10 & TPR10_READ_CALIBRATION != 0 {
        for i in 0..5 {
            if mctl_phy_read_training(para, config) {
                break;
            }

            if i == 4 {
                return false;
            }
        }
    }

    if para.tpr10 & TPR10_READ_TRAINING != 0 {
        for i in 0..5 {
            if mctl_phy_read_training(para, config) {
                return true;
            }

            if i == 4 {
                return false;
            }
        }
    }

    if para.tpr10 & TPR10_WRITE_LEVELING != 0 {
        for i in 0..5 {
            if mctl_phy_write_training(para, config) {
                return true;
            }

            if i == 4 {
                return false;
            }
        }
    }

    mctl_phy_bit_delay_compensation(para);

    clear_mask32(DRAM_PHY_BASE + 0x60, 0x4);

    true
}

#[inline(always)]
fn mstr_active_ranks(x: u8) -> u32 {
    if x == 2 {
        3 << 24
    } else {
        1 << 24
    }
}

#[inline(always)]
fn mstr_burst_len(x: u32) -> u32 {
    (x >> 1) << 16
}

fn mctl_ctrl_init(para: &dram_para, config: &dram_config) -> bool {
    clearset_bits32(MCTL_COM_UNK_0x500, 1 << 24, 0x200);
    write32(MCTL_CTL_CLKEN, 0x8000);

    set_bits32(MCTL_COM_UNK_0x008, 0xff00);

    write32(DRAM_COM_BASE + 0x50, 1);
    clearset_bits32(MCTL_CTL_SCHED, 0xff00, 0x3000);
    write32(MCTL_CTL_HWLPCTL, 0);

    set_bits32(MCTL_COM_UNK_0x008, 0xff00);

    let mut bus_width;
    if config.bus_full_width {
        bus_width = 0 << 12;
    } else {
        bus_width = 1 << 12;
    }

    // BIT 5 is the identifier for LPDDR4
    let val = bus_width | mstr_active_ranks(config.ranks) | 1 << 5 | mstr_burst_len(16);
    write32(MCTL_CTL_MSTR, val | 1 << 31 | 1 << 30);

    if (config.ranks == 2) {
        write32(MCTL_CTL_ODTMAP, 0x0303);
    } else {
        write32(MCTL_CTL_ODTMAP, 0x0201);
    }
    let val = 0x04000400;
    write32(MCTL_CTL_ODCFG, val);
    write32(MCTL_CTL_UNK2240, val);
    write32(MCTL_CTL_UNK3240, val);
    write32(MCTL_CTL_UNK4240, val);

    write32(MCTL_COM_CR, 1 << 31);

    mctl_set_addrmap(config);
    mctl_set_timing_params(para);

    let val = 1 << 31 | 1 << 30;
    set_bits32(MCTL_CTL_PWRCTL, val);
    set_bits32(MCTL_CTL_DFIUPD, val);
    set_bits32(MCTL_CTL_UNK2180, val);
    set_bits32(MCTL_CTL_UNK3180, val);
    set_bits32(MCTL_CTL_UNK4180, val);

    set_bits32(MCTL_CTL_RFSHCTL13, 1 << 0);
    set_bits32(MCTL_CTL_DFIMISC, 1 << 0);

    write32(MCTL_COM_MAER0, 0);
    write32(MCTL_COM_MAER1, 0);
    write32(MCTL_COM_MAER2, 0);

    write32(MCTL_CTL_PWRCTL, 0x20);
    set_bits32(MCTL_CTL_CLKEN, 1 << 8);

    clearset_bits32(MCTL_COM_UNK_0x500, 1 << 24, 0x300);
    delay(10);

    if (!mctl_phy_init(para, config)) {
        return false;
    }

    write32(MCTL_CTL_SWCTL, 0);
    clear_mask32(MCTL_CTL_RFSHCTL13, 1 << 0);

    set_bits32(MCTL_COM_UNK_0x014, 1 << 31);
    write32(MCTL_COM_MAER0, 0xFFFF_FFFF);
    write32(MCTL_COM_MAER1, 0x07FF);
    write32(MCTL_COM_MAER2, 0xFFFF);

    write32(MCTL_CTL_SWCTL, 0x1);
    wait_for_completion(MCTL_CTL_SWSTAT, 1, 1);

    true
}

fn mctl_auto_detect_rank_width(para: &dram_para, config: &mut dram_config) {
    // min size supported
    config.cols = 8;
    config.rows = 13;

    // keep testing from most demanding
    // to least demanding

    // 32 bit, rank 2
    config.bus_full_width = true;
    config.ranks = 2;
    if mctl_core_init(para, config) {
        return;
    }

    // 32 bit, rank 1
    config.bus_full_width = true;
    config.ranks = 1;
    if mctl_core_init(para, config) {
        return;
    }

    // 16 bit, rank 2
    config.bus_full_width = false;
    config.ranks = 2;
    if mctl_core_init(para, config) {
        return;
    }

    // 16 bit, rank 1
    config.bus_full_width = false;
    config.ranks = 1;
    if mctl_core_init(para, config) {
        return;
    }
}

fn mctl_write_pattern() {
    let ptr = DRAM_BASE;
    for i in 0..16 {
        let addr = ptr + i * 4;
        if i & 1 != 0 {
            write32(addr, !addr as u32);
        } else {
            write32(addr, addr as u32);
        }
    }
}

fn mctl_check_pattern(offset: usize) -> bool {
    let ptr = DRAM_BASE;
    for i in 0..16 {
        let addr = ptr + i * 4;
        if i & 1 != 0 {
            if read32(addr + offset / 4) != !addr as u32 {
                return false;
            }
        } else {
            if read32(addr + offset / 4) != addr as u32 {
                return false;
            }
        }
    }
    true
}

fn mctl_auto_detect_dram_size(para: &dram_para, config: &mut dram_config) {
    let mut oldcfg: [u32; 16] = [0; 16];
    let mut fin_rows = 0;
    let mut fin_cols = 0;

    // max config for cols, not rows
    config.cols = 11;
    config.rows = 13;
    mctl_core_init(para, config);

    // store initial config to restore later
    for i in 0..16 {
        oldcfg[i] = read32(DRAM_BASE + i * 4);
    }

    mctl_write_pattern();

    let shift = config.bus_full_width as u32 + 1;

    // detect nr of cols
    for cols in 8..11 {
        if mctl_check_pattern(1 << (cols + shift)) {
            fin_cols = cols as u8;
            break;
        }
    }

    // restore config
    for i in 0..16 {
        write32(DRAM_BASE + i * 4, oldcfg[i]);
    }

    // reconfigure to access all rows
    config.cols = 8;
    config.rows = 17;
    mctl_core_init(para, config);

    // store data again
    for i in 0..16 {
        oldcfg[i] = read32(DRAM_BASE + i * 4);
    }

    mctl_write_pattern();

    // detect nr rows
    for rows in 13..17 {
        if mctl_check_pattern(1 << (rows + shift)) {
            fin_rows = rows as u8;
            break;
        }
    }

    // restore config
    for i in 0..16 {
        write32(DRAM_BASE + i * 4, oldcfg[i]);
    }

    config.cols = fin_cols;
    config.rows = fin_rows;
}

fn mctl_core_init(para: &dram_para, config: &dram_config) -> bool {
    mctl_init(para.clk);
    mctl_ctrl_init(para, config)
}

fn mctl_calc_size(config: &dram_config) -> u64 {
    let width = if config.bus_full_width { 4 } else { 2 };

    1 << (config.cols + config.rows + 3) as u64 * width as u64 * config.ranks as u64
}

// TODO
fn mctl_set_master_priority() {}

// Power reset and Clock Management
const PRCM_RES_CAL_CTRL: usize = PRCM_BASE + 0x0000_0310;
const PRCM_OHMS_240: usize = PRCM_BASE + 0x0000_0318;

fn dram_init() -> u64 {
    // Initialize DRAM settings here

    let mut config = dram_config::default();
    let mut para = dram_para::default();

    let cal_ctrl = read32(PRCM_RES_CAL_CTRL) | (1 << 8);
    write32(PRCM_RES_CAL_CTRL, cal_ctrl);

    let ohms_240 = read32(PRCM_OHMS_240) & !0x3F;
    write32(PRCM_OHMS_240, ohms_240);

    mctl_auto_detect_rank_width(&para, &mut config);
    mctl_auto_detect_dram_size(&para, &mut config);

    mctl_core_init(&para, &config);
    let size = mctl_calc_size(&config);

    mctl_set_master_priority();

    size
}
