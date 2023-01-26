/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

pub mod device_const;
pub mod device_util;
pub mod mmio;
pub mod path;
pub mod pci_type;
pub mod resource;
pub mod soundwire;

use bitfield::bitfield;
use util::fw_config::FwConfig;

use {path::DevicePath, resource::Resource};

#[derive(Debug)]
pub enum Error {
    General,
}

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq)]
    pub struct BusFields(u8);
    pub reset_needed, set_reset_needed: 1, 0;
    pub disable_relaxed_ordering, set_disable_relaxed_ordering: 1, 1;
    pub ht_link_up, set_ht_link_up: 1, 2;
    pub no_vga16, set_no_vga16: 1, 3;
}


#[derive(Clone, Copy)]
pub struct Bus {
    /// This bridge device
    pub dev: Option<&'static Device>,
    /// Devices behind this bridge
    pub children: Option<&'static Device>,
    /// The next bridge on this device
    pub next: Option<&'static Bus>,
    /// Bridge control register
    pub bridge_ctrl: u32,
    /// Bridge command register
    pub bridge_cmd: u16,
    /// The index of this link
    pub link_num: u8,
    /// Secondary bus number
    pub secondary: u16,
    /// Max subordinate bus number
    pub subordinate: u16,
    /// PCI capability offset
    pub cap: u8,
    /// For HyperTransport link
    pub hcdn_reg: u32,
    pub fields: BusFields,
}

/// There is one device structure for each slot-number/function-number
/// combination:
#[derive(Clone, Copy)]
pub struct PCIIRQInfo {
    pub ioapic_irq_pin: u32,
    pub ioapic_src_pin: u32,
    pub ioapic_dst_pin: u32,
    pub ioapic_flags: u32,
}

impl PCIIRQInfo {
    pub const fn new() -> Self {
        Self {
            ioapic_irq_pin: 0,
            ioapic_src_pin: 0,
            ioapic_dst_pin: 0,
            ioapic_flags: 0,
        }
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq)]
    pub struct DeviceFields(u8);
    pub enabled, set_enabled: 1, 0;
    pub initialize, set_initialized: 1, 1;
    pub on_mainboard, set_on_mainboard: 1, 2;
    pub disable_pcie_aspm, set_disable_pcie_aspm: 1, 3;
    pub hidden, set_hidden: 1, 4;
    pub mandatory, set_mandatory: 1, 5;
    pub hotplug_port, set_hotplug_port: 1, 6;
}

pub struct ChipInfo;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Device {
    /// Bus this device is on, for bridge devices, it is the up stream bus
    pub bus: Option<&'static Bus>,
    /// Next device on this bus
    pub sibling: Option<&'static Device>,
    /// Chain of all devices
    pub next: Option<&'static Device>,
    pub path: DevicePath,
    pub vendor: u32,
    pub device: u32,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    /// 3 bytes: (base, sub, prog-if)
    pub class: u32,
    /// PCI header type
    pub hdr_type: u32,
    pub fields: DeviceFields,
    pub command: u8,
    /// Number of hotplug buses to allocate
    pub hotplug_buses: u16,
    /// Base registers for this device. I/O, MEM and Expansion ROM
    pub resource_list: Option<&'static Resource>,
    /// Links are (downstream) buses attached to the device, usually a leaf
    /// device with no children has 0 buses attached and a bridge has 1 bus
    pub link_list: Option<&'static Bus>,
    pub pci_irq_info: [PCIIRQInfo; 4],
    /* TODO: add config for !DEVTREE_EARLY */
    pub chip_info: Option<&'static ChipInfo>,
    pub prob_list: Option<&'static FwConfig>,
}

impl Device {
    pub const fn new() -> Self {
        Self {
            bus: None,
            sibling: None,
            next: None,
            path: DevicePath::new(),
            vendor: 0,
            device: 0,
            subsystem_vendor: 0,
            subsystem_device: 0,
            class: 0,
            hdr_type: 0,
            fields: DeviceFields(0),
            command: 0,
            hotplug_buses: 0,
            resource_list: None,
            link_list: None,
            pci_irq_info: [PCIIRQInfo::new(); 4],
            chip_info: None,
            prob_list: None,
        }
    }
}

pub trait ChipOperations {
    fn enable_dev(&self, dev: &mut Device);
    fn init(&self, chip_info: &mut ChipInfo);
    fn finalize(&self, chip_info: &mut ChipInfo);
    fn initialized(&self) -> bool;
    fn finalized(&self) -> bool;
    fn name(&self) -> &str;
}
