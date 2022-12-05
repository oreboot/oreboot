use log::error;
use payload::drivers::pci_map_bus_ops::{pci_read_config16, pci_read_config8, pci_write_config8};
use types::{
    pci_def::PCI_VENDOR_ID,
    pci_ids::PCI_VID_INTEL,
    pci_type::{PciDevFnT, _pci_dev},
};

// FIXME: defined in asl 0:1f.0 coreboot/src/soc/intel/meteorlake/acpi/southbridge.asl
pub const ESPI: u32 = 0x1f;
pub const P2SBC: u32 = 0xe0;
pub const P2SBC_HIDE_BIT: u32 = 1 << 0;
pub const PCI_DEV_P2SB: u32 = _pci_dev(ESPI, 1);
pub const PCH_DEV_P2SB: u32 = PCI_DEV_P2SB;

pub fn p2sb_dev_is_hidden(dev: PciDevFnT) -> bool {
    let pci_vid = pci_read_config16(dev, PCI_VENDOR_ID);

    if pci_vid == 0xffff {
        return true;
    }
    if pci_vid == PCI_VID_INTEL {
        return false;
    }
    error!("P2SB PCI_VENDOR_ID is invalid, unknown if hidden");
    true
}

pub fn p2sb_dev_set_hide_bit(dev: PciDevFnT, hide: i32) {
    let reg = P2SBC as u16 + 1;
    let mask = P2SBC_HIDE_BIT;

    let mut val = pci_read_config8(dev, reg);
    val &= !mask as u8;

    if hide != 0 {
        val |= mask as u8;
    }

    pci_write_config8(dev, reg, val);
}

pub fn p2sb_unhide() {
    p2sb_dev_unhide(PCH_DEV_P2SB);
}

pub fn p2sb_dev_unhide(dev: PciDevFnT) {
    p2sb_dev_set_hide_bit(dev, 0);

    if p2sb_dev_is_hidden(dev) {
        error!("Unable to unhide the P2SB device!");
    }
}

pub fn p2sb_hide() {
    p2sb_dev_hide(PCH_DEV_P2SB);
}

pub fn p2sb_dev_hide(dev: PciDevFnT) {
    p2sb_dev_set_hide_bit(dev, 1);

    if !p2sb_dev_is_hidden(dev) {
        error!("Unable to hide the P2SB device!");
    }
}
