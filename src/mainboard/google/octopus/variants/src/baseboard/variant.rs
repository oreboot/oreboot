use soc::intel::{
    apollolake::gpio_glk::{GPIO_117, GPIO_161, GPIO_67},
    common::block::gpio::{gpio_output, Gpio},
};
use util::timer::mdelay;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GpioWithDelay {
    gpio: Gpio,
    delay_msecs: u32,
}

impl GpioWithDelay {
    pub const fn create(gpio: Gpio, delay_msecs: u32) -> Self {
        Self { gpio, delay_msecs }
    }

    pub fn gpio(&self) -> Gpio {
        self.gpio
    }

    pub fn delay_msecs(&self) -> u32 {
        self.delay_msecs
    }
}

pub fn power_off_lte_module() {
    const LTE_POWER_OFF_GPIOS: [GpioWithDelay; 3] = [
        // AVS_I2S1_MCLK -- PLT_RST_LTE_L
        GpioWithDelay::create(GPIO_161 as u32, 30),
        // PCIE_WAKE1_B -- FULL_CARD_POWER_OFF
        GpioWithDelay::create(GPIO_117 as u32, 100),
        // UART2-CTS_B -- EN_PP3300_DX_LTE_SOC
        GpioWithDelay::create(GPIO_67 as u32, 0),
    ];

    for gpio in LTE_POWER_OFF_GPIOS.iter() {
        let _ = gpio_output(gpio.gpio(), 0);
        mdelay(gpio.delay_msecs());
    }
}

pub fn smi_sleep(_slp_typ: u8) {}
