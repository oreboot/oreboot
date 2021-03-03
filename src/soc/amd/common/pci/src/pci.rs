use vcell::*;

const MMIO_CFG_BASE: u32 = 0xE000_0000;

pub struct PciAddress {
    pub segment: u8,  // 4 bits
    pub bus: u8,      // 8 bits
    pub device: u8,   // 5 bits
    pub function: u8, // 3 bits
    pub offset: u16,  // 12 bits
}

impl PciAddress {
    #[inline(always)]
    pub const fn mmio_config_address(self) -> u32 {
        /*if self.segment >= 16 {
            panic!("PciAddress segment is out of range: {}", self.segment);
        }
        if self.device >= 32 {
            panic!("PciAddress device is out of range: {}", self.device);
        }
        if self.function >= 8 {
            panic!("PciAddress function is out of range: {}", self.function);
        }
        if self.offset >= 4096 {
            panic!("PciAddress offset is out of range: {}", self.offset);
        }*/
        MMIO_CFG_BASE | ((self.segment as u32) << 28) | ((self.bus as u32) << 20) | ((self.device as u32) << 15) | ((self.function as u32) << 12) | (self.offset as u32)
    }
}

pub fn config16(address: PciAddress) -> &'static VolatileCell<u32> {
    unsafe { &*(PciAddress::mmio_config_address(address) as *const VolatileCell<u32>) }
}
pub fn config32(address: PciAddress) -> &'static VolatileCell<u32> {
    unsafe { &*(PciAddress::mmio_config_address(address) as *const VolatileCell<u32>) }
}
