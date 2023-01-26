use ec::google::chromeec::{
    ec::{log_events, Error},
    smihandler,
};
use soc::intel::common::block::gpio::{gpio_configure_pads, GpiStatus, Gpio};
use variants::baseboard::{
    self,
    ec::{
        MAINBOARD_EC_LOG_EVENTS, MAINBOARD_EC_S0IX_WAKE_EVENTS, MAINBOARD_EC_S3_WAKE_EVENTS,
        MAINBOARD_EC_S5_WAKE_EVENTS, MAINBOARD_EC_SCI_EVENTS, MAINBOARD_EC_SMI_EVENTS,
    },
    gpio::{sleep_gpio_table, EC_SMI_GPI},
};

#[cfg(feature = "bobba")]
use variants::bobba::variant;
// overkill, since bobba is the only variant atm
#[cfg(not(any(feature = "bobba")))]
use variants::baseboard::variant;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct GpioWithDelay {
    gpio: Gpio,
    delay_msecs: u32,
}

pub fn smi_gpi_handler(sts: &GpiStatus) {
    if let Ok(s) = sts.get(EC_SMI_GPI) {
        if s != 0 {
            smihandler::smi_process_events();
        }
    }
}

pub fn smi_sleep(slp_typ: u8) {
    let pads = sleep_gpio_table(slp_typ);
    gpio_configure_pads(pads);

    variant::smi_sleep(slp_typ);

    smihandler::smi_sleep(
        slp_typ,
        MAINBOARD_EC_S3_WAKE_EVENTS,
        MAINBOARD_EC_S5_WAKE_EVENTS,
    );
}

pub fn smi_apmc(apmc: u8) -> Result<(), Error> {
    smihandler::smi_apmc(apmc, MAINBOARD_EC_SCI_EVENTS, MAINBOARD_EC_SMI_EVENTS)
}

pub fn elog_gsmi_cb_mainboard_log_wake_source() {
    let _ = log_events(MAINBOARD_EC_LOG_EVENTS | MAINBOARD_EC_S0IX_WAKE_EVENTS);
}

pub fn power_off_lte_module() {
    baseboard::variant::power_off_lte_module();
}
