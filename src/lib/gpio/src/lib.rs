/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

use util::timer::udelay;

#[cfg(feature = "intel")]
use soc::intel::common::block::gpio::{gpio_get, gpio_input_pulldown};

pub type Gpio = u32;

// Define default no-op if system soc is not selected
#[cfg(not(any(
    feature = "amd",
    feature = "cavium",
    feature = "intel",
    feature = "mediatek",
    feature = "nvidia",
    feature = "qualcomm",
    feature = "rockchip",
)))]
pub fn gpio_input_pulldown(_gpio: Gpio) {
    return;
}

fn _check_num(name: &str, num: usize) {
    if num > 31 || num < 1 {
        panic!("{}: {} is an invalid number of GPIOS", name, num);
    }
}

fn _gpio_base2_value(gpios: &[Gpio]) -> usize {
    // Wait until signals become stable
    udelay(10);

    let mut result = 0;
    for (i, &gpio) in gpios.iter().enumerate() {
        result |= gpio_get(gpio).unwrap_or(0) << i;
    }

    result as usize
}

pub fn gpio_base2_value(gpios: &[Gpio]) -> usize {
    _check_num("gpio_base2_value", gpios.len());

    for &gpio in gpios.iter() {
        gpio_input_pulldown(gpio);
    }

    _gpio_base2_value(gpios)
}
