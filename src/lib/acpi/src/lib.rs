/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

#[cfg(feature = "intel")]
use southbridge::intel::i82371eb::wakeup::acpi_get_sleep_type;

use romstage_handoff;

#[cfg(not(feature = "intel"))]
pub fn acpi_get_sleep_type() -> u8 {
    0
}

#[repr(C)]
#[derive(PartialEq)]
pub enum SleepState {
    S0 = 0,
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
    S5 = 5,
}

pub fn acpi_s3_resume_allowed() -> bool {
    cfg!(HAVE_ACPI_RESUME)
}

pub fn acpi_is_wakeup_s3() -> bool {
    if !acpi_s3_resume_allowed() {
        return false;
    }

    if cfg!(ENV_ROMSTAGE_OR_BEFORE) {
        return acpi_get_sleep_type() == SleepState::S3 as u8;
    }

    romstage_handoff::is_resume()
}
