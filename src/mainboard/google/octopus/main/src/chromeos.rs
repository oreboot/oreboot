use bootmode::gfx_get_init_done;
use ec::google::chromeec::switches::get_lid_switch;
use payload::oreboot_tables::{LbGpio, LbGpios, OB_GPIO_ACTIVE_HIGH};
use soc::intel::common::block::gpio::gpio_get;
use variants::baseboard::gpio::{GPIO_EC_IN_RW, GPIO_PCH_WP};

pub const ACTIVE_HIGH: u32 = OB_GPIO_ACTIVE_HIGH as u32;

pub fn fill(gpios: &mut LbGpios) {
    let gpio_ec_in_rw = gpio_get(GPIO_EC_IN_RW as u32).unwrap_or(0);

    let chromeos_gpios = [
        LbGpio::create(u32::MAX, ACTIVE_HIGH, get_lid_switch() as u32, b"lid"),
        LbGpio::create(u32::MAX, ACTIVE_HIGH, 0, b"power"),
        LbGpio::create(u32::MAX, ACTIVE_HIGH, gfx_get_init_done() as u32, b"oprom"),
        LbGpio::create(
            GPIO_EC_IN_RW as u32,
            ACTIVE_HIGH,
            gpio_ec_in_rw,
            b"EC in RW",
        ),
    ];

    gpios.add_gpios(&chromeos_gpios);
}

pub fn get_write_protect_state() -> u32 {
    gpio_get(GPIO_PCH_WP as u32).unwrap_or(0)
}

/// EC is trusted if not in RW
pub fn get_ec_is_trusted() -> u32 {
    !gpio_get(GPIO_EC_IN_RW as u32).unwrap_or(0)
}
