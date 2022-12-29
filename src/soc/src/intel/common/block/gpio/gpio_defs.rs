/* SPDX-License-Identifier: GPL-2.0-or-later */

use crate::intel::common::block::gpio::PadConfig;

pub const fn bit_width(x: u32) -> u32 {
    (1 << x) - 1
}

pub const fn pad_cfg0_mode_func(x: u32) -> u32 {
    x << (PadCfg0Mode::Shift as u32)
}

pub const PAD_CFG_OWN_GPIO_DRIVER: u32 = 1 << 4;
pub const PAD_CFG0_PREGFRXSEL: u32 = 1 << 24;
pub const PAD_CFG0_NAFVWE_ENABLE: u32 = 1 << 27;

#[macro_export]
macro_rules! pad_func {
    ($value:ident) => {{
        PadCfg0Mode::$value
    }};
}

#[macro_export]
macro_rules! pad_reset {
    ($value:ident) => {{
        PadCfg0LogicalReset::$value
    }};
}

#[macro_export]
macro_rules! pad_rx_pol {
    ($value:ident) => {{
        PadCfg0RxPol::$value
    }};
}

#[macro_export]
macro_rules! pad_irq_route {
    ($value:ident) => {{
        PadCfg0Route::$value
    }};
}

#[macro_export]
macro_rules! pad_trig {
    ($value:ident) => {{
        PadCfg0Trig::$value
    }};
}

#[macro_export]
macro_rules! pad_pull {
    ($value:ident) => {{
        PadCfg1Pull::$value
    }};
}

#[macro_export]
macro_rules! pad_buf {
    ($value:ident) => {{
        PadCfg0Buf::$value
    }};
}

#[macro_export]
macro_rules! pad_iosstate {
    ($value:ident) => {{
        if cfg!(feature = "iostandby") {
            PadCfg1Iosstate::$value
        } else {
            PadCfg1Iosstate::TxLastRxE
        }
    }};
}

#[macro_export]
macro_rules! pad_iosterm {
    ($value:ident) => {{
        if cfg!(feature = "iostandby") {
            PadCfg1Iosterm::$value
        } else {
            PadCfg1Iosterm::Same
        }
    }};
}

#[macro_export]
macro_rules! pad_lock {
    ($value:ident) => {{
        GpioLockAction::$value
    }};
}

#[macro_export]
macro_rules! pad_irq_cfg {
    ($route:ident, $trig:ident, $inv:ident) => {{
        (pad_irq_route!($route) as u32) | (pad_trig!($trig) as u32) | (pad_rx_pol!($inv) as u32)
    }};
}

#[macro_export]
macro_rules! pad_cfg_nf {
    ($pad:expr, $pull:ident, $rst:ident, $func:ident) => {{
        PadConfig::create(
            $pad,
            (pad_reset!($rst) as u32) | (pad_func!($func) as u32),
            (pad_pull!($pull) as u32) | (pad_iosstate!(TxLastRxE) as u32),
        )
    }};
}

#[macro_export]
macro_rules! pad_cfg_nf_lock {
    ($pad:expr, $pull:ident, $func:ident, $lock_action:ident) => {{
        PadConfig::create_lock(
            $pad,
            (pad_reset!(PwrOk) as u32) | (pad_func!($func) as u32),
            (pad_pull!($pull) as u32) | (pad_iosstate!(TxLastRxE) as u32),
            pad_lock!($lock_action),
        )
    }};
}

/// Native function configuration for standby state
#[macro_export]
macro_rules! pad_cfg_nf_iosstate {
    ($pad:expr, $pull:ident, $rst:ident, $func:ident, $iosstate:ident) => {{
        PadConfig::create(
            $pad,
            (pad_reset!($rst) as u32) | (pad_func!($func) as u32),
            (pad_pull!($pull) as u32) | (pad_iosstate!($iosstate) as u32),
        )
    }};
}

/// Native function configuration for standby state, also configuring
/// iostandby as masked
#[macro_export]
macro_rules! pad_cfg_nf_iostandby_ignore {
    ($pad:expr, $pull:ident, $rst:ident, $func:ident) => {{
        PadConfig::create($pad,
                          (pad_reset!($rst) as u32) | (pad_func!($func) as u32),
                          (pad_pull!($pull) as u32) | (pad_iosstate!(Ignore) as u32)),
    }}
}

/// Native function configuration for standby state, also configuring
/// iosstate and iosterm
#[macro_export]
macro_rules! pad_cfg_nf_iosstate_iosterm {
    ($pad:expr, $pull:ident, $rst:ident, $func:ident, $iosstate:ident, $iosterm:ident) => {{
        PadConfig::create(
            $pad,
            (pad_reset!($rst) as u32) | (pad_func!($func) as u32),
            (pad_pull!($pull) as u32)
                | (pad_iosstate!($iosstate) as u32)
                | (pad_iosterm!($iosterm) as u32),
        )
    }};
}

/// General purpose output, no pullup/down.
#[macro_export]
macro_rules! pad_cfg_gpo {
    ($pad:expr, $val:expr, $rst:ident) => {{
        PadConfig::create(
            $pad,
            (pad_func!(Gpio) as u32)
                | (pad_reset!($rst) as u32)
                | (pad_trig!(Off) as u32)
                | (pad_buf!(RxDisable) as u32)
                | !!$val,
            (pad_pull!(None) as u32) | (pad_iosstate!(TxLastRxE) as u32),
        )
    }};
}

#[macro_export]
macro_rules! pad_cfg_gpo_iosstate_iosterm {
    ($pad:expr, $val:expr, $rst:ident, $pull:ident, $iosstate:ident, $iosterm:ident) => {{
        PadConfig::create(
            $pad,
            (pad_func!(Gpio) as u32)
                | (pad_reset!($rst) as u32)
                | (pad_trig!(Off) as u32)
                | (pad_buf!(RxDisable) as u32)
                | !!$val,
            (pad_pull!($pull) as u32)
                | (pad_iosstate!($iosstate) as u32)
                | (pad_iosterm!($iosterm) as u32),
        )
    }};
}

/// General purpose input
#[macro_export]
macro_rules! pad_cfg_gpi {
    ($pad:expr, $pull:ident, $rst:ident) => {{
        PadConfig::create(
            $pad,
            (pad_func!(Gpio) as u32) | (pad_reset!($rst) as u32) | (pad_buf!(TxDisable) as u32),
            (pad_pull!($pull) as u32) | (pad_iosstate!(TxDRxE) as u32),
        )
    }};
}

#[macro_export]
macro_rules! pad_cfg_gpi_sci {
    ($pad:expr, $pull:ident, $rst:ident, $trig:ident, $inv:ident) => {{
        PadConfig::create(
            $pad,
            (pad_func!(Gpio) as u32)
                | (pad_reset!($rst) as u32)
                | (pad_buf!(TxDisable) as u32)
                | (pad_irq_cfg!(Sci, $trig, $inv) as u32),
            (pad_pull!($pull) as u32) | (pad_iosstate!(TxDRxE) as u32),
        )
    }};
}

#[macro_export]
macro_rules! pad_cfg_gpi_sci_low {
    ($pad:expr, $pull:ident, $rst:ident, $trig:ident) => {{
        pad_cfg_gpi_sci!($pad, $pull, $rst, $trig, Invert)
    }};
}

/// General purpose input
#[macro_export]
macro_rules! pad_cfg_gpi_apic_ios {
    ($pad:expr, $pull:ident, $rst:ident, $trig:ident, $inv:ident, $iosstate:ident, $iosterm:ident) => {{
        PadConfig::create(
            $pad,
            (pad_func!(Gpio) as u32)
                | (pad_reset!($rst) as u32)
                | (pad_buf!(TxDisable) as u32)
                | (pad_irq_cfg!(Ioapic, $trig, $inv) as u32),
            (pad_pull!($pull) as u32)
                | (pad_iosstate!(TxDRxE) as u32)
                | (pad_iosterm!($iosterm) as u32),
        )
    }};
}

#[macro_export]
macro_rules! pad_nc {
    ($pad:expr, $pull:ident) => {{
        PadConfig::create(
            $pad,
            (pad_func!(Gpio) as u32)
                | (pad_reset!(Deep) as u32)
                | (pad_trig!(Off) as u32)
                | (pad_buf!(TxRxDisable) as u32),
            (pad_pull!($pull) as u32) | (pad_iosstate!(TxDRxE) as u32),
        )
    }};
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Buf {
    NoDisable = 0,
    TxDisable = PadCfg0Tx::Disable as u32,
    RxDisable = PadCfg0Rx::Disable as u32,
    TxRxDisable = (PadCfg0Tx::Disable as u32) | (PadCfg0Rx::Disable as u32),
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Mode {
    Shift = 10,
    BitWidth = bit_width(3),
    Gpio = pad_cfg0_mode_func(0),
    Nf1 = pad_cfg0_mode_func(1),
    Nf2 = pad_cfg0_mode_func(2),
    Nf3 = pad_cfg0_mode_func(3),
    Nf4 = pad_cfg0_mode_func(4),
    Nf5 = pad_cfg0_mode_func(5),
    Nf6 = pad_cfg0_mode_func(6),
    Nf7 = pad_cfg0_mode_func(7),
    Nf8 = pad_cfg0_mode_func(8),
    Nf9 = pad_cfg0_mode_func(9),
    Nf10 = pad_cfg0_mode_func(10),
    Nf11 = pad_cfg0_mode_func(11),
    Nf12 = pad_cfg0_mode_func(12),
    Nf13 = pad_cfg0_mode_func(13),
    Nf14 = pad_cfg0_mode_func(14),
    Nf15 = pad_cfg0_mode_func(15),
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Route {
    Nmi = 1 << 17,
    Smi = 1 << 18,
    Sci = 1 << 19,
    Ioapic = 1 << 20,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Rx {
    Disable = 1 << 9,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0RxPol {
    None = 0 << 23,
    Invert = 1 << 23,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Tx {
    Disable = 1 << 8,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0RxState {
    StateBit = 0,
    State = 1 << (Self::StateBit as u32),
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0TxState {
    StateBit = 0,
    State = 1 << (Self::StateBit as u32),
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Trig {
    Level = 0 << 25,
    EdgeSingle = 1 << 25,
    EdgeBoth = 3 << 25,
    Off = 2 << 25,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0Mask {
    #[cfg(feature = "soc_intel_common_block_gpio_iostandby")]
    Iosstate = 0xf << 14,
    #[cfg(not(feature = "soc_intel_common_block_gpio_iostandby"))]
    Iosstate = 0,
    Iosterm = 0x3 << 8,
    Mode = (PadCfg0Mode::BitWidth as u32) << (PadCfg0Mode::Shift as u32),
    Pull = 0xf << 10,
    Reset = 3 << 30,
    Route = 0xf << 17,
    RxTenCfg = 3 << 21,
    RxInv = 1 << 23,
    RxRaw1 = 1 << 28,
    RxPadstsel = 1 << 29,
    Trig = 3 << 25,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg0LogicalReset {
    PwrOk = 0 << 30,
    Deep = 1 << 30,
    PltRst = 2 << 30,
    RsmRst = 3 << 30,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg1Iosstate {
    /// Tx enabled driving last value driven, Rx enabled
    TxLastRxE = 0x0 << 14,
    /// Tx enabled driving 0, Rx disabled and Rx driving 0 back to its controller
    /// internally
    Tx0RxDcRx0 = 0x1 << 14,
    /// Tx enabled driving 0, Rx disabled and Rx driving 1 back to its controller
    /// internally
    Tx0RxDcRx1 = 0x2 << 14,
    /// Tx enabled driving 1, Rx disabled and Rx driving 0 back to its controller
    /// internally
    Tx1RxDcRx0 = 0x3 << 14,
    /// Tx enabled driving 1, Rx disabled and Rx driving 1 back to its controller
    /// internally
    Tx1RxDcRx1 = 0x4 << 14,
    /// Tx enabled driving 0, Rx enabled
    Tx0RxE = 0x5 << 14,
    /// Tx enabled driving 1, Rx enabled
    Tx1RxE = 0x6 << 14,
    /// Hi-Z, Rx driving 0 back to its controller internally
    HiZcRx0 = 0x7 << 14,
    /// Hi-Z, Rx driving 1 back to its controller internally
    HiZcRx1 = 0x8 << 14,
    /// Tx disabled, Rx enabled
    TxDRxE = 0x9 << 14,
    /// Ignore Iostandby
    Ignore = 0xf << 14,
    Shift = 14,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg1Iosterm {
    Same = 0x0 << 8,
    Dispupd = 0x1 << 8,
    Enpd = 0x2 << 8,
    Enpu = 0x3 << 8,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg1Pull {
    None = 0x0 << 10,
    Dn5k = 0x2 << 10,
    Dn20k = 0x4 << 10,
    Up1k = 0x9 << 10,
    Up5k = 0xa << 10,
    Up2k = 0xb << 10,
    Up20k = 0xc << 10,
    Up667 = 0xd << 10,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg1Mask {
    Irq = 0xff << 0,
    Iosterm = 0x3 << 8,
    Pull = 0xf << 10,
    #[cfg(feature = "iostandby")]
    Iosstate = 0xf << 14,
    #[cfg(not(feature = "iostandby"))]
    Iosstate = 0,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg2Mask {
    Debounce = 0x1f,
}

/// Debounce Duration = (2 ^ PAD_CFG2_DEBOUNCE_x_RTC) * RTC clock duration
#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum PadCfg2Debounce {
    Rtc8 = 0x3 << 1,
    Rtc16 = 0x4 << 1,
    Rtc32 = 0x5 << 1,
    Rtc64 = 0x6 << 1,
    Rtc128 = 0x7 << 1,
    Rtc256 = 0x8 << 1,
    Rtc512 = 0x9 << 1,
    Rtc1k = 0xa << 1,
    Rtc2k = 0xb << 1,
    Rtc4k = 0xc << 1,
    Rtc8k = 0xd << 1,
    Rtc16k = 0xe << 1,
    Rtc32k = 0xf << 1,
}

pub fn pad_cfg_gpo_deep(pad: u32, val: u32) -> PadConfig {
    PadConfig::create(
        pad,
        PadCfg0Mode::Gpio as u32
            | PadCfg0LogicalReset::Deep as u32
            | PadCfg0Trig::Off as u32
            | PadCfg0Buf::RxDisable as u32
            | !!val,
        PadCfg1Pull::None as u32 | PadCfg1Iosstate::TxLastRxE as u32,
    )
}
