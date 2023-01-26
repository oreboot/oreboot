use crate::intel::common::block::pm::PM1_CNT;
use acpi::{SLP_EN, SLP_TYP_S5, SLP_TYP_SHIFT};
use consts::ENV_SMM;
use util::{
    cpuio::{inl, outl},
    halt::halt,
};

#[cfg(any(feature = "apollolake", feature = "geminilake"))]
use crate::intel::apollolake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "baytrail")]
use crate::intel::baytrail::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "braswell")]
use crate::intel::braswell::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "broadwell")]
use crate::intel::broadwell::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "cannonlake")]
use crate::intel::cannonlake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "elkhartlake")]
use crate::intel::elkhartlake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "icelake")]
use crate::intel::icelake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "jasperlake")]
use crate::intel::jasperlake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "meteorlake")]
use crate::intel::meteorlake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "skylake")]
use crate::intel::skylake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "tigerlake")]
use crate::intel::tigerlake::iomap::ACPI_BASE_ADDRESS;
#[cfg(feature = "xeon_sp")]
use crate::intel::xeon_sp::iomap::ACPI_BASE_ADDRESS;

#[cfg(not(any(
    feature = "apollolake",
    feature = "geminilake",
    feature = "baytrail",
    feature = "braswell",
    feature = "broadwell",
    feature = "cannonlake",
    feature = "elkhartlake",
    feature = "icelake",
    feature = "jasperlake",
    feature = "meteorlake",
    feature = "skylake",
    feature = "tigerlake",
    feature = "xeon_sp",
)))]
pub const ACPI_BASE_ADDRESS: u32 = 0x500;

pub fn read_pm1_control() -> u32 {
    unsafe { inl((ACPI_BASE_ADDRESS + PM1_CNT as u32) as u16) }
}

pub fn write_pm1_control(pm1_cnt: u32) {
    unsafe {
        outl(pm1_cnt, (ACPI_BASE_ADDRESS + PM1_CNT as u32) as u16);
    }
}

pub fn enable_pm1_control(mask: u32) {
    let mut pm1_cnt = read_pm1_control();
    pm1_cnt |= mask;
    write_pm1_control(pm1_cnt);
}

pub fn poweroff() {
    enable_pm1_control(SLP_EN | (SLP_TYP_S5 << SLP_TYP_SHIFT));

    // Setting SLP_TYP_S5 in PM1 triggers SLP_SMI, which is handled by SMM
    // to transition to S5 state. If halt is called in SMM, then it prevents
    // the SMI handler from being triggered and system never enters S5.
    if ENV_SMM {
        halt();
    }
}
