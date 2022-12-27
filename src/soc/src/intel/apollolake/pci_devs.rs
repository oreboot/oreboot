use payload::pci::pci_dev;

// FIXME: where is this defined in coreboot?
pub const LPC: u8 = 0;
pub const PCH_DEV_LPC: u32 = pch_dev(LPC, 0);

pub const fn pch_dev(slot: u8, func: u8) -> u32 {
    pci_dev(0, slot, func)
}
