use crate::bobba::Sku;
use ec::google::chromeec::ec_skuid::get_board_sku;
use soc::{
    intel::{
        apollolake::gpio_glk::*,
        common::block::gpio::{gpio_defs::*, PadConfig},
    },
    pad_buf, pad_cfg_gpi_apic_ios, pad_cfg_gpo, pad_cfg_gpo_iosstate_iosterm, pad_func,
    pad_iosstate, pad_iosterm, pad_irq_cfg, pad_irq_route, pad_nc, pad_pull, pad_reset, pad_rx_pol,
    pad_trig,
};

pub static DEFAULT_OVERRIDE_TABLE: [PadConfig; 5] = [
    pad_nc!(GPIO_104 as u32, Up20k),
    // GPIO_137 -- HP_INT_ODL and would be amend by SSFC.
    pad_cfg_gpi_apic_ios!(GPIO_137 as u32, None, Deep, Level, Invert, HiZcRx1, Dispupd),
    // EN_PP3300_TOUCHSCREEN
    pad_cfg_gpo_iosstate_iosterm!(GPIO_146 as u32, 1, Deep, None, Tx0RxDcRx0, Dispupd),
    // GPIO_140 -- PEN_RESET
    pad_cfg_gpo_iosstate_iosterm!(GPIO_140 as u32, 0, Deep, None, Tx1RxDcRx0, Dispupd),
    pad_nc!(GPIO_213 as u32, Dn20k),
];

pub static LTE_OVERRIDE_TABLE: [PadConfig; 9] = [
    // Default override table.
    pad_nc!(GPIO_104 as u32, Up20k),
    // GPIO_137 -- HP_INT_ODL and would be amend by SSFC.
    pad_cfg_gpi_apic_ios!(GPIO_137 as u32, None, Deep, Level, Invert, HiZcRx1, Dispupd),
    // EN_PP3300_TOUCHSCREEN
    pad_cfg_gpo_iosstate_iosterm!(GPIO_146 as u32, 1, Deep, None, Tx0RxDcRx0, Dispupd),
    // GPIO_105 -- TOUCHSCREEN_RST
    pad_cfg_gpo_iosstate_iosterm!(GPIO_105 as u32, 0, Deep, None, Tx1RxDcRx0, Dispupd),
    // GPIO_140 -- PEN_RESET
    pad_cfg_gpo_iosstate_iosterm!(GPIO_140 as u32, 0, Deep, None, Tx1RxDcRx0, Dispupd),
    pad_nc!(GPIO_213 as u32, Dn20k),
    // Be specific to LTE SKU
    // UART2-CTS_B -- EN_PP3300_DX_LTE_SOC
    pad_cfg_gpo!(GPIO_67 as u32, 1, PwrOk),
    // PCIE_WAKE1_B -- FULL_CARD_POWER_OFF
    pad_cfg_gpo!(GPIO_117 as u32, 1, PwrOk),
    // AVS_I2S1_MCLK -- PLT_RST_LTE_L
    pad_cfg_gpo!(GPIO_161 as u32, 1, Deep),
];

pub fn override_gpio_table() -> &'static [PadConfig] {
    match Sku::from(get_board_sku()) {
        Sku::Droid37 | Sku::Droid38 | Sku::Droid39 | Sku::Droid40 => LTE_OVERRIDE_TABLE.as_ref(),
        _ => &DEFAULT_OVERRIDE_TABLE.as_ref(),
    }
}

pub static LTE_EARLY_OVERRIDE_TABLE: [PadConfig; 3] = [
    // UART2-CTS_B -- EN_PP3300_DX_LTE_SOC
    pad_cfg_gpo!(GPIO_67 as u32, 1, PwrOk),
    // PCIE_WAKE1_B -- FULL_CARD_POWER_OFF
    pad_cfg_gpo!(GPIO_117 as u32, 1, PwrOk),
    // AVS_I2S1_MCLK -- PLT_RST_LTE_L
    pad_cfg_gpo!(GPIO_161 as u32, 0, Deep),
];

pub fn early_override_gpio_table() -> &'static [PadConfig] {
    LTE_EARLY_OVERRIDE_TABLE.as_ref()
}

/// GPIOs needed to be set in romstage.
pub static ROMSTAGE_GPIO_TABLE: [PadConfig; 3] = [
    // Enable touchscreen, hold in reset
    // EN_PP3300_TOUCHSCREEN
    pad_cfg_gpo_iosstate_iosterm!(GPIO_146 as u32, 1, Deep, None, Tx0RxDcRx0, Dispupd),
    // GPIO_105 -- TOUCHSCREEN_RST
    pad_cfg_gpo_iosstate_iosterm!(GPIO_105 as u32, 1, Deep, None, Tx1RxDcRx0, Dispupd),
    // GPIO_140 -- PEN_RESET
    pad_cfg_gpo_iosstate_iosterm!(GPIO_140 as u32, 1, Deep, None, Tx1RxDcRx0, Dispupd),
];

pub fn romstage_gpio_table() -> &'static [PadConfig] {
    &ROMSTAGE_GPIO_TABLE
}
