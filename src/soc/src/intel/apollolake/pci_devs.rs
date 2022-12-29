use payload::pci::pci_dev;
use device::pci_type::pci_devfn;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PchDevSlot {
    Npk = 0x00,
    Cnvi = 0x0c,
    P2sb = 0x0d,
    Hda = 0x0e,
    Cse = 0x0f,
    Ish = 0x11,
    Sata = 0x12,
    Pcie = 0x13,
    Pcie1 = 0x14,
    Xhci = 0x15,
    Sio1 = 0x16,
    Sio2 = 0x17,
    Uart = 0x18,
    Spi = 0x19,
    Pwm = 0x1a,
    Sdcard = 0x1b,
    Emmc = 0x1c,
    Ufs = 0x1d,
    Sdio = 0x1e,
    Lpc = 0x1f,
}

pub const PCH_DEV_LPC: u32 = pch_dev(PchDevSlot::Lpc, 0);
pub const PCH_DEVFN_CNVI: u32 = pch_devfn(PchDevSlot::Cnvi, 0);

pub const fn pch_dev(slot: PchDevSlot, func: u8) -> u32 {
    pci_dev(0, slot as u8, func)
}

pub const fn pch_devfn(slot: PchDevSlot, func: u8) -> u32 {
    pci_devfn(slot as u8, func)
}
