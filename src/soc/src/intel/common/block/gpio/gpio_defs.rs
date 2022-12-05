/* SPDX-License-Identifier: GPL-2.0-or-later */

use crate::intel::common::block::gpio::PadConfig;

pub const fn bit_width(x: i32) -> i32 {
    (1 << x) - 1
}

pub const fn pad_cfg0_mode_func(x: i32) -> i32 {
    x << PAD_CFG0_MODE_SHIFT
}

pub const PAD_CFG_OWN_GPIO_DRIVER: i32 = 1 << 4;

pub const PAD_CFG0_TX_STATE_BIT: i32 = 0;
pub const PAD_CFG0_TX_STATE: i32 = 1 << PAD_CFG0_TX_STATE_BIT;
pub const PAD_CFG0_RX_STATE_BIT: i32 = 0;
pub const PAD_CFG0_RX_STATE: i32 = 1 << PAD_CFG0_RX_STATE_BIT;
pub const PAD_CFG0_TX_DISABLE: i32 = 1 << 8;
pub const PAD_CFG0_RX_DISABLE: i32 = 1 << 9;
pub const PAD_CFG0_MODE_SHIFT: i32 = 10;
pub const PAD_CFG0_MODE_BIT_WIDTH: i32 = bit_width(3);
pub const PAD_CFG0_MODE_MASK: i32 = PAD_CFG0_MODE_BIT_WIDTH << PAD_CFG0_MODE_SHIFT;
pub const PAD_CFG0_BUF_RX_DISABLE: i32 = PAD_CFG0_RX_DISABLE;
pub const PAD_CFG0_MODE_GPIO: i32 = pad_cfg0_mode_func(0);

pub const PAD_CFG0_ROUTE_MASK: i32 = 0xf << 17;
pub const PAD_CFG0_ROUTE_NMI: i32 = 1 << 17;
pub const PAD_CFG0_ROUTE_SMI: i32 = 1 << 18;
pub const PAD_CFG0_ROUTE_SCI: i32 = 1 << 19;
pub const PAD_CFG0_ROUTE_IOAPIC: i32 = 1 << 20;
pub const PAD_CFG0_RXTENCFG_MASK: i32 = 3 << 21;
pub const PAD_CFG0_RXINV_MASK: i32 = 1 << 23;
pub const PAD_CFG0_RX_POL_INVERT: i32 = 1 << 23;
pub const PAD_CFG0_RX_POL_NONE: i32 = 0 << 23;
pub const PAD_CFG0_PREGFRXSEL: i32 = 1 << 24;
pub const PAD_CFG0_TRIG_MASK: i32 = 3 << 25;
pub const PAD_CFG0_TRIG_LEVEL: i32 = 0 << 25;
pub const PAD_CFG0_TRIG_EDGE_SINGLE: i32 = 1 << 25; /* controlled by RX_INVERT*/
pub const PAD_CFG0_TRIG_OFF: i32 = 2 << 25;
pub const PAD_CFG0_TRIG_EDGE_BOTH: i32 = 3 << 25;
pub const PAD_CFG0_NAFVWE_ENABLE: i32 = 1 << 27;
pub const PAD_CFG0_RXRAW1_MASK: i32 = 1 << 28;
pub const PAD_CFG0_RXPADSTSEL_MASK: i32 = 1 << 29;
pub const PAD_CFG0_RESET_MASK: i32 = 3 << 30;

pub const PAD_CFG0_LOGICAL_RESET_PWROK: u32 = 0 << 30;
pub const PAD_CFG0_LOGICAL_RESET_DEEP: u32 = 1 << 30;
pub const PAD_CFG0_LOGICAL_RESET_PLTRST: u32 = 2 << 30;
pub const PAD_CFG0_LOGICAL_RESET_RSMRST: u32 = 3 << 30;

pub const PAD_CFG1_PULL_NONE: u32 = 0x0 << 10;
pub const PAD_CFG1_IOSTERM_MASK: u32 = 0x3 << 8;
pub const PAD_CFG1_PULL_MASK: u32 = 0xf << 10;
#[cfg(feature = "soc_intel_common_block_gpio_iostandby")]
/// Mask to extract Iostandby bits
pub const PAD_CFG1_IOSSTATE_MASK: u32 = 0xf << 14;
#[cfg(not(feature = "soc_intel_common_block_gpio_iostandby"))]
pub const PAD_CFG1_IOSSTATE_MASK: u32 = 0;
pub const PAD_CFG1_IOSSTATE_TXLASTRXE: u32 = 0x0 << 14;
pub const PAD_CFG1_IRQ_MASK: u32 = 0xff << 0;

pub const PAD_CFG2_DEBOUNCE_MASK: u32 = 0x1f;

pub fn pad_cfg_gpo_deep(pad: u32, val: u32) -> PadConfig {
    PadConfig::create(
        pad,
        PAD_CFG0_MODE_GPIO as u32
            | PAD_CFG0_LOGICAL_RESET_DEEP
            | PAD_CFG0_TRIG_OFF as u32
            | PAD_CFG0_BUF_RX_DISABLE as u32
            | !!val,
        PAD_CFG1_PULL_NONE | PAD_CFG1_IOSSTATE_TXLASTRXE,
    )
}
