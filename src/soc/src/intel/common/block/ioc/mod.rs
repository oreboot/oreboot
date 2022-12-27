use device::mmio::write32p;
use crate::intel::apollolake::iomap::MCH_BASE_ADDRESS;

mod gpmr;
pub use gpmr::*;

pub unsafe fn ioc_reg_write32(offset: u32, value: u32) {
    write32p((MCH_BASE_ADDRESS + offset) as usize, value);
}
