pub fn pci_devfn(slot: u8, func: u8) -> u32 {
    (((slot & 0x1f) << 3) | (func & 0x07)) as u32
}
