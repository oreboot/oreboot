use crate::intel::i82371eb::{DEFAULT_PMBASE, PMCNTRL};
use log::debug;
use util::cpuio::inw;

/// Intel 82371EB (PIIX4E) datasheet, section 7.2.3, page 142
///
/// 0: soft off/suspend to disk					                S5
/// 1: suspend to ram						                    S3
/// 2: powered on suspend, context lost				            S2
///    Note: 'context lost' means the CPU restarts at the reset
///          vector
/// 3: powered on suspend, CPU context lost			            S1
///    Note: Looks like 'CPU context lost' does _not_ mean the
///          CPU restarts at the reset vector. Most likely only
///          caches are lost, so both 0x3 and 0x4 map to acpi S1
/// 4: powered on suspend, context maintained			        S1
/// 5: working (clock control)					                S0
/// 6: reserved
/// 7: reserved
pub const ACPI_SUS_TO_SLP_TYP: [u8; 8] = [5, 3, 2, 1, 1, 0, 0, 0];

pub fn acpi_get_sleep_type() -> u8 {
    let reg = unsafe { inw(DEFAULT_PMBASE + PMCNTRL) };
    let result = ACPI_SUS_TO_SLP_TYP[((reg >> 10) & 7) as usize];

    debug!(
        "Wakeup from ACPI sleep type S{} (PMCNTRL={:04x})",
        result, reg
    );

    result
}
