/* SPDX-License-Identifier: GPL-2.0-only */
#![no_std]

use pci::config32;
use pci::PciAddress;

// See coreboot:/src/soc/amd/picasso/data_fabric.c

const DF_FICAA_BIOS: u16 = 0x5C;
const DF_FICAD_LO: u16 = 0x98;
// const DF_FICAD_HI: u32 = 0x9C;

// Precondition: FCAC needs to be in broadcast mode.
fn df_access_indirect(node_id: u8, target_instance_id: Option<u8>, target_function: u8, target_offset: u16) {
    assert!(node_id < 2);
    assert!(target_function < 8);
    assert!(target_offset & 3 == 0);
    assert!(target_offset < 2048);
    let ficaa3 = config32(PciAddress { segment: 0, bus: 0, device: 0x18 + node_id, function: 0x4, offset: DF_FICAA_BIOS });
    let mut target: u32 = match target_instance_id {
        Some(target_instance_id) => 1 | ((target_instance_id as u32) << 16),
        None => 0,
    };
    target |= ((target_offset as u32) >> 2) << 2;
    target |= (target_function as u32) << 11;
    // Note: bit 14: 64 bit access.
    ficaa3.set(target);
}

pub fn df_read_indirect(node_id: u8, target_instance_id: u8, target_function: u8, target_offset: u16) -> u32 {
    df_access_indirect(node_id, Some(target_instance_id), target_function, target_offset);
    let ficad3_lo = config32(PciAddress { segment: 0, bus: 0, device: 0x18 + node_id, function: 0x4, offset: DF_FICAD_LO });
    ficad3_lo.get()
}

pub fn df_write_indirect(node_id: u8, target_instance_id: u8, target_function: u8, target_offset: u16, value: u32) {
    df_access_indirect(node_id, Some(target_instance_id), target_function, target_offset);
    let ficad3_lo = config32(PciAddress { segment: 0, bus: 0, device: 0x18 + node_id, function: 0x4, offset: DF_FICAD_LO });
    ficad3_lo.set(value)
}

pub fn df_broadcast_indirect(node_id: u8, target_function: u8, target_offset: u16, value: u32) {
    df_access_indirect(node_id, None, target_function, target_offset);
    let ficad3_lo = config32(PciAddress { segment: 0, bus: 0, device: 0x18 + node_id, function: 0x4, offset: DF_FICAD_LO });
    ficad3_lo.set(value)
}
