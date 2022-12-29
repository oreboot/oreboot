use crate::{Bus, Device, path::DevicePathType};
use payload::pci::PciDev;

pub static DEV_ROOT: Device = Device::new();
pub static ALL_DEVICES: *const Device = &DEV_ROOT as *const Device;

static PCI_ROOT: *const Bus = core::ptr::null();

pub fn dev_find_path(prev_match: *const Device, path_type: DevicePathType) -> (*const Device, *const Device) {
    let res_match = if !prev_match.is_null() {
        unsafe { (*prev_match).next }
    } else {
        ALL_DEVICES
    };

    let mut res = core::ptr::null();
    let mut dev = prev_match;
    while !dev.is_null() {
        if unsafe { (*dev).path.path_type == path_type } {
            res = dev;
            break;
        }
        dev = (*dev).next;
    }

    (res_match, res)
}

pub fn pci_root_bus() -> *const Bus {
    let pci_domain = dev_find_path(None, DevicePathType::Domain);
    if pci_domain == core::ptr::null() {
        return pci_domain;
    }

    unsafe { PCI_ROOT = (*pci_domain).link_list; }
    PCI_ROOT
}

pub fn pcidev_path_on_root(devfn: &PciDev) -> &'static Device {
    pcidev_path_behind(pci_root_bus(), devfn)
}
