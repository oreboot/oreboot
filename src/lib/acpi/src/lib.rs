/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

#[cfg(feature = "intel")]
use southbridge::intel::i82371eb::wakeup::acpi_get_sleep_type;

use romstage_handoff;

pub const SLP_EN: u32 = 1 << 13;
pub const SLP_TYP_S5: u32 = 7;
pub const SLP_TYP_SHIFT: u32 = 10;

/// AcpiSn assignments are defined to always equal the sleep state numbers
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum AcpiSn {
    S0 = 0,
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
    S5 = 5,
    Invalid,
}

impl From<u8> for AcpiSn {
    fn from(a: u8) -> Self {
        match a {
            0 => Self::S0,
            1 => Self::S1,
            2 => Self::S2,
            3 => Self::S3,
            4 => Self::S4,
            5 => Self::S5,
            _ => Self::Invalid,
        }
    }
}

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
