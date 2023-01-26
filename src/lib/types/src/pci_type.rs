pub type PciDevFnT = u32;

pub const fn pci_dev(segbus: u32, dev: u32, func: u32) -> u32 {
    ((segbus & 0xfff) << 20) | ((dev & 0x1f) << 15) | ((func & 0x07) << 12)
}

pub const fn _pci_dev(slot: u32, func: u32) -> u32 {
    pci_dev(0, slot, func)
}
