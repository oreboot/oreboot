#[cfg(target_arch = "x86_64")]
use arch::x86_64::mmio::{read16, read32, read8, write16, write32, write8};

use crate::{drivers::pci_qcom::pci_map_bus, pci::PciDevT};

pub fn pci_read_config8(dev: PciDevT, reg: u16) -> u8 {
    let cfg_base = pci_map_bus(dev);
    unsafe { read8(cfg_base | reg as usize) }
}

pub fn pci_read_config16(dev: PciDevT, reg: u16) -> u16 {
    let cfg_base = pci_map_bus(dev);
    unsafe { read16(cfg_base | ((reg & !1) as usize)) }
}

pub fn pci_read_config32(dev: PciDevT, reg: u16) -> u32 {
    let cfg_base = pci_map_bus(dev);
    unsafe { read32(cfg_base | (reg & !3) as usize) }
}

pub fn pci_write_config8(dev: PciDevT, reg: u16, val: u8) {
    let cfg_base = pci_map_bus(dev);
    unsafe { write8(cfg_base | reg as usize, val) };
}

pub fn pci_write_config16(dev: PciDevT, reg: u16, val: u16) {
    let cfg_base = pci_map_bus(dev);
    unsafe { write16(cfg_base | (reg & !1) as usize, val) };
}

pub fn pci_write_config32(dev: PciDevT, reg: u16, val: u32) {
    let cfg_base = pci_map_bus(dev);
    unsafe { write32(cfg_base | (reg & !3) as usize, val) };
}
