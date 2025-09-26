use core::arch::naked_asm;
use crate::mem_map::{PRCM_BASE, CCU_BASE, DRAM_CTL_BASE, DRAM_COM_BASE};
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
const MCTL_COM_CR:        usize = DRAM_COM_BASE + 0x000;
const MCTL_COM_UNK_0x008: usize = DRAM_COM_BASE + 0x008;
const MCTL_COM_TMR:       usize = DRAM_COM_BASE + 0x00C;
const MCTL_COM_UNK_0x014: usize = DRAM_COM_BASE + 0x014;
const MCTL_COM_MAER0:     usize = DRAM_COM_BASE + 0x020;
const MCTL_COM_MAER1:     usize = DRAM_COM_BASE + 0x024;
const MCTL_COM_MAER2:     usize = DRAM_COM_BASE + 0x028;
const MCTL_COM_BWCR:      usize = DRAM_COM_BASE + 0x200;
const MCTL_COM_UNK_0x500: usize = DRAM_COM_BASE + 0x500;

// refer: https://github.com/u-boot/u-boot/blob/master/arch/arm/include/asm/arch-sunxi/dram_sun50i_h616.h#L60
const MCTL_CTL_MSTR:      usize = 0x000;
const MCTL_CTL_CLKEN:     usize = 0x00C;

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
fn wait_for_completion(reg:usize, mask:u32, expected_value:u32) {
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
    let pll_conf = PLL5_CTL_EN 
                | PLL5_LOCK_EN 
                | PLL5_OUT_EN 
                | pll5_ctrl_n(clockrate * 2 / 24);
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


// Power reset and Clock Management
const PRCM_RES_CAL_CTRL: usize = PRCM_BASE + 0x0000_0310;
const PRCM_OHMS_240: usize = PRCM_BASE + 0x0000_0318;

fn dram_init() {
    // Initialize DRAM settings here
    
    let cal_ctrl = read32(PRCM_RES_CAL_CTRL) | (1 << 8);
    write32(PRCM_RES_CAL_CTRL, cal_ctrl);

    let ohms_240 = read32(PRCM_OHMS_240) & !0x3F;
    write32(PRCM_OHMS_240, ohms_240);



    
}
