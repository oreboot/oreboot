#[cfg(feature = "soc_intel_common_block_ioc")]
pub use crate::intel::common::block::ioc::*;
#[cfg(not(feature = "soc_intel_common_block_ioc"))]
pub use crate::intel::common::block::pcr::*;

#[cfg(any(feature = "apollolake", feature = "geminilake"))]
use crate::intel::apollolake::pcr_ids::PID_DMI;

#[cfg(not(feature = "soc_intel_common_block_ioc"))]
pub fn ioc_reg_write32(_offset: u32, _val: u32) {}

pub fn gpmr_write32(offset: u16, val: u32) {
    if cfg!(feature = "soc_intel_common_block_ioc") {
        ioc_reg_write32(offset as u32, val);
    } else {
        pcr_write32(PID_DMI as u8, offset, val);
    }
}
