use alloc::vec::Vec;

use crate::drivers::pci_map_bus_ops::{
    pci_read_config16, pci_read_config32, pci_read_config8, pci_write_config16, pci_write_config32,
    pci_write_config8,
};

pub type PciDevT = u32;

pub fn pci_bus(d: u32) -> u8 {
    ((d >> 16) & 0xff) as u8
}

pub fn pci_func(d: u32) -> u8 {
    ((d >> 8) & 0x7) as u8
}

pub fn pci_slot(d: u32) -> u8 {
    ((d >> 11) & 0x1f) as u8
}

pub fn pci_dev(bus: u8, dev: u8, func: u8) -> u32 {
    0x80000000 | (bus as u32) << 16 | (dev as u32) << 11 | (func as u32) << 8
}

pub fn pci_addr(bus: u8, dev: u8, func: u8, reg: u8) -> u32 {
    pci_dev(bus, dev, func) | (reg & !3) as u32
}

#[repr(C)]
pub struct PciDev {
    pub domain: u16,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub device_class: u16,
    pub next: *mut PciDev,
}

impl PciDev {
    pub fn libpci_to_lb(&self) -> u32 {
        pci_dev(self.bus, self.dev, self.func)
    }

    pub fn pci_read_byte(&self, pos: usize) -> u8 {
        pci_read_config8(self.libpci_to_lb(), pos as u16)
    }

    pub fn pci_read_word(&self, pos: usize) -> u16 {
        pci_read_config16(self.libpci_to_lb(), pos as u16)
    }

    pub fn pci_read_long(&self, pos: usize) -> u32 {
        pci_read_config32(self.libpci_to_lb(), pos as u16)
    }

    pub fn pci_write_byte(&self, pos: usize, data: u8) -> i32 {
        pci_write_config8(self.libpci_to_lb(), pos as u16, data);
        1
    }

    pub fn pci_write_word(&self, pos: usize, data: u16) -> i32 {
        pci_write_config16(self.libpci_to_lb(), pos as u16, data);
        1
    }

    pub fn pci_write_long(&self, pos: usize, data: u32) -> i32 {
        pci_write_config32(self.libpci_to_lb(), pos as u16, data);
        1
    }
}

#[repr(C)]
pub struct PciAccess {
    method: u32,
    devices: Vec<PciDev>,
}

impl PciAccess {
    pub fn pci_get_dev(&self, domain: u16, bus: u8, dev: u8, func: u8) -> PciDev {
        PciDev {
            bus,
            dev,
            func,
            domain,
            vendor_id: 0,
            device_id: 0,
            device_class: 0,
            next: core::ptr::null_mut(),
        }
    }
}
