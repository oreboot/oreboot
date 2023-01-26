use device::device_const::pcidev_path_on_root;
use payload::drivers::pci_map_bus_ops::pci_read_config32;
use soc::intel::apollolake::pci_devs::PCH_DEVFN_CNVI;
use types::pci_def::PCI_VENDOR_ID;

pub is_cnvi_held_in_reset() -> bool {
    let dev = pcidev_path_on_root(PCH_DEVFN_CNVI);
    let reg = pci_read_config32(dev, PCI_VENDOR_ID);

	// If vendor/device ID for CNVi reads as 0xffffffff, then it is safe to
	// assume that it is being held in reset.
    if reg == 0xffff_ffff {
        true
    } else {
        false
    }
}
