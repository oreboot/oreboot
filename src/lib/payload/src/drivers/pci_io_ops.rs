use crate::pci::PciDevT;
use util::cpuio::{inb, inl, inw, outb, outl, outw};

pub unsafe fn pci_read_config8(dev: PciDevT, reg: u16) -> u8 {
    outl(dev | (reg & !3) as u32, 0xcf8);
    inb(0xcfc + (reg & 3) as u16)
}

pub unsafe fn pci_read_config16(dev: PciDevT, reg: u16) -> u16 {
    outl(dev | (reg & !3) as u32, 0xcf8);
    inw(0xcfc + (reg & 3))
}

pub unsafe fn pci_read_config32(dev: PciDevT, reg: u16) -> u32 {
    outl(dev | (reg & !3) as u32, 0xcf8);
    inl(0xcfc + (reg & 3))
}

pub unsafe fn pci_write_config8(dev: PciDevT, reg: u16, val: u8) {
    outl(dev | (reg & !3) as u32, 0xcf8);
    outb(val, 0xcfc + (reg & 3));
}

pub unsafe fn pci_write_config16(dev: PciDevT, reg: u16, val: u16) {
    outl(dev | (reg & !3) as u32, 0xcf8);
    outw(val, 0xcfc + (reg & 3));
}

pub unsafe fn pci_write_config32(dev: PciDevT, reg: u16, val: u32) {
    outl(dev | (reg & !3) as u32, 0xcf8);
    outl(val, 0xcfc + (reg & 3));
}
