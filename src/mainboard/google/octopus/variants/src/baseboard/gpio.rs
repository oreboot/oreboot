use acpi::AcpiSn;
use soc::{
    pad_cfg_gpo_iosstate_iosterm, pad_cfg_gpi_sci, pad_cfg_gpi_sci_low, pad_nc, pad_cfg_nf, pad_cfg_nf_iosstate_iosterm, pad_irq_cfg, pad_cfg_gpi, pad_cfg_gpo, pad_cfg_gpi_apic_ios, pad_iosstate, pad_pull, pad_buf, pad_func, pad_trig, pad_reset, pad_iosterm, pad_rx_pol, pad_irq_route,
    intel::{
        apollolake::gpio_glk::{
            GPIO_109, GPIO_117, GPIO_151, GPIO_154, GPIO_161, GPIO_164, GPIO_178, GPIO_189, GPIO_190,
            GPIO_41, GPIO_63, GPIO_64, GPIO_65, GPIO_67, GPIO_68, GPIO_69, GPIO_70, GPIO_71, GPIO_79, GPIO_80, GPIO_82, GPIO_83,
        },
        common::block::gpio::{gpio_defs::*, PadConfig},
    },
};

pub const GPIO_EC_IN_RW: u16 = GPIO_189;
pub const GPIO_PCH_WP: u16 = GPIO_190;
pub const EC_SMI_GPI: u16 = GPIO_41;

pub const MEM_CONFIG0: u16 = GPIO_68;
pub const MEM_CONFIG1: u16 = GPIO_69;
pub const MEM_CONFIG2: u16 = GPIO_70;
pub const MEM_CONFIG3: u16 = GPIO_71;

pub fn early_override_gpio_table() -> &'static [PadConfig] {
    &[]
}

pub static EARLY_BOOTBLOCK_GPIO_TABLE: [PadConfig; 3] = [
    // LPC_CLKRUNB -- NC for eSPI
    pad_nc!(GPIO_154 as u32, None),
    // LPSS_UART2_RXD
    pad_cfg_nf_iosstate_iosterm!(GPIO_64 as u32, Up20k, Deep, Nf1, HiZcRx1, Dispupd),
    // LPSS_UART2_TXD
    pad_cfg_nf_iosstate_iosterm!(GPIO_65 as u32, Up20k, Deep, Nf1, TxLastRxE, Dispupd),
];

pub fn early_bootblock_gpio_table() -> &'static [PadConfig] {
    EARLY_BOOTBLOCK_GPIO_TABLE.as_ref()
}

pub static EARLY_GPIO_TABLE: [PadConfig; 13] = [
    // PCH_WP_OD
    pad_cfg_gpi!(GPIO_190 as u32, None, Deep),
    // GSPI0_INT H1_PCH_INT_ODL
    pad_cfg_gpi_apic_ios!(GPIO_63 as u32, None, Deep, Level, Invert, TxDRxE, Dispupd),
    // GSPIO_CLK H1_SLAVE_SPI_CLK_R
    pad_cfg_nf!(GPIO_79 as u32, None, Deep, Nf1),
    // GSPIO_CS# H1_SLAVE_SPI_CS_L_R
    pad_cfg_nf!(GPIO_80 as u32, None, Deep, Nf1),
    // GSPIO_MISO H1_SLAVE_SPI_MISO
    pad_cfg_nf!(GPIO_82 as u32, None, Deep, Nf1),
    // GSPIO_MISO H1_SLAVE_SPI_MOSI_R
    pad_cfg_nf!(GPIO_83 as u32, None, Deep, Nf1),
    // Enable power to wifi early in bootblock and de-assert PERST#.
    // EN_PP3300_WLAN_L
    pad_cfg_gpo!(GPIO_178 as u32, 0, Deep),
    // WLAN_PE_RST
    pad_cfg_gpo!(GPIO_164 as u32, 0, Deep),
    // ESPI_IO1 acts as ALERT# (which is open-drain) and requires a weak
    // pull-up for proper operation. Since there is no external pull present
    // on this platform, configure an internal weak pull-up.
    //
    // ESPI_IO1
    pad_cfg_nf_iosstate_iosterm!(GPIO_151 as u32, Up20k, Deep, Nf2, HiZcRx1, Enpu),
    // GPIO_67 and GPIO_117 are in early_gpio_table and gpio_table. For variants
    // having LTE SKUs, these two GPIOs would be overridden to output high first
    // in the bootblock then be set to default state in gpio_table for non-LTE
    // SKUs and keep to output high for LTE SKUs in ramstage.
    //
    // UART2-CTS_B -- EN_PP3300_DX_LTE_SOC
    pad_cfg_gpo_iosstate_iosterm!(GPIO_67 as u32, 0, Deep, None, TxLastRxE, Dispupd),
    // PCIE_WAK1_B -- LTE_WAKE_L
    pad_cfg_gpi_sci_low!(GPIO_117 as u32, None, Deep, EdgeSingle),
    // GPIO_161 is in early_gpio_table and gpio_table because LTE SKU needs
    // to override this pin to output low then high respectively in two
    // stages.
    //
    // AVS_I2S1_MCLK -- LTE_OFF_ODL
    pad_cfg_gpo_iosstate_iosterm!(GPIO_161 as u32, 1, Deep, Up20k, Tx1RxDcRx0, Dispupd),
    // EC_IN_RW
    pad_cfg_gpi!(GPIO_189 as u32, None, Deep),
];

pub fn early_gpio_table() -> &'static [PadConfig] {
    EARLY_GPIO_TABLE.as_ref()
}

/// GPIO settings before entering sleep
pub static SLEEP_GPIO_TABLE: [PadConfig; 1] = [PadConfig::new()];

pub static SLEEP_S5_GPIO_TABLE: [PadConfig; 1] = [pad_cfg_gpo_iosstate_iosterm!(
    GPIO_109 as u32, 0, Deep, None, Tx0RxDcRx1, Same
)];

pub fn sleep_gpio_table(slp_typ: u8) -> &'static [PadConfig] {
    if AcpiSn::from(slp_typ) == AcpiSn::S5 {
        SLEEP_S5_GPIO_TABLE.as_ref()
    } else {
        SLEEP_GPIO_TABLE.as_ref()
    }
}
