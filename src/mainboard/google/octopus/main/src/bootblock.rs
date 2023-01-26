use crate::ec::mainboard_ec_init;
use soc::intel::common::block::gpio::{gpio_configure_pads, gpio_configure_pads_with_override};
use variants::baseboard::gpio::{early_bootblock_gpio_table, early_gpio_table, early_override_gpio_table};

pub fn early_init() {
    let _ = gpio_configure_pads(early_bootblock_gpio_table());
}

pub fn init() {
    // Perform EC init before configuring GPIOs. This is because variant
    // might talk to the EC to get board id and hence it will require EC
    // init to have already performed.
    mainboard_ec_init();

    gpio_configure_pads_with_override(early_gpio_table(), early_override_gpio_table());
}
