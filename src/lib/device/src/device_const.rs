use crate::{Bus, Device, path::{DevicePath, DevicePathUnion, DevicePathType, PCIPath}};
use payload::pci::PciDevT;

pub static mut DEV_ROOT: &'static Device = &Device::new();
pub static ALL_DEVICES: Option<&'static Device> = Some(unsafe { &DEV_ROOT });

static mut PCI_ROOT: Option<&'static Bus> = None;

pub fn dev_find_path(prev_match: Option<&'static Device>, path_type: DevicePathType) -> (Option<&'static Device>, Option<&'static Device>) {
    let res_match = if let Some(p) = prev_match {
        p.next
    } else {
        ALL_DEVICES
    };

    let mut res = None;
    let mut dev = prev_match;
    while let Some(d) = &dev {
        if d.path.path_type == path_type {
            res = dev;
            break;
        }
        dev = d.next;
    }

    (res_match, res)
}

pub fn pci_root_bus() -> Option<&'static Bus> {
    let (_, pci_domain) = dev_find_path(None, DevicePathType::Domain);
    if let Some(p) = pci_domain {
        unsafe {
            PCI_ROOT = p.link_list;
            PCI_ROOT
        }
    } else {
        None
    }
}

/// See if a device structure exists for path.
///
/// @param parent The bus to find the device on.
/// @param path The relative path from the bus to the appropriate device.
/// @return Pointer to a device structure for the device on bus at path
///         or None if no device is found.
pub fn find_dev_path(parent: Option<&'static Bus>, path: &DevicePath) -> Option<&'static Device> {
    if let Some(p) = parent {
        let mut child = p.children;
        while let Some(c) = child {
            if path == &c.path {
                break;
            }
            child = c.sibling;
        }

        child
    } else {
        None
    }
}

pub fn pcidev_path_on_root(devfn: PciDevT) -> Option<&'static Device> {
    pcidev_path_behind(pci_root_bus(), devfn)
}

pub fn pcidev_path_behind(parent: Option<&'static Bus>, devfn: PciDevT) -> Option<&'static Device> {
    let path = DevicePath{
        path_type: DevicePathType::Pci,
        union: DevicePathUnion{ pci: PCIPath { devfn } },
    };
    find_dev_path(parent, &path)
}
