#[cfg(feature = "ec_google_chromeec_lpc")]
use crate::google::chromeec::ec_lpc::get_switches;

#[cfg(not(feature = "ec_google_chromeec_lpc"))]
pub fn get_lid_switch() -> i32 {
    -1
}

#[cfg(feature = "ec_google_chromeec_lpc")]
pub fn get_lid_switch() -> i32 {
    if cfg!(not(feature = "vboot_lid_switch")) {
        return -1;
    }

    !!((get_switches() as i32) & EC_SWITCH_LID_OPEN as i32)
}
