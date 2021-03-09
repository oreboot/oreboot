/* SPDX-License-Identifier: GPL-2.0-only */
#![no_std]
#![feature(const_in_array_repeat_expressions)]

use heapless::consts::U256;
use heapless::Vec;
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

pub fn df_read_broadcast_indirect(node_id: u8, target_function: u8, target_offset: u16) -> u32 {
    df_access_indirect(node_id, None, target_function, target_offset);
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

#[derive(Debug)]
pub enum FabricInstanceType {
    Unknown = -1,
    CCM = 0,
    GCM = 1,
    NCM = 2,
    IOMS = 3,
    CS = 4,
    NCS = 5,
    TCDX = 6,
    PIE = 7,
    SPF = 8,
    LLC = 9,
    CAKE = 0xA,
}

#[derive(Debug)]
pub struct FabricComponent {
    pub instance_id: u8,
    pub instance_type: FabricInstanceType,
    pub enabled: bool,
    pub fabric_id: Option<u8>,
    // netmask: D18F1x208 [System Fabric ID Mask 0] (DF::SystemFabricIdMask0) etc
}

pub struct FabricTopology {
    pub components: Vec<FabricComponent, U256>,
}

impl FabricTopology {
    pub fn new() -> Self {
        let mut result = Self { components: Vec::new() };
        let total_count: usize = (df_read_broadcast_indirect(0, 0, 0x40) & 0xFF) as usize;
        for x_instance_id in 0..=255 {
            if result.components.len() >= total_count {
                break;
            }

            let info0 = df_read_indirect(0, x_instance_id, 0, 0x44);
            let instance_type = match info0 & 0xF {
                0 => FabricInstanceType::CCM,
                1 => FabricInstanceType::GCM,
                2 => FabricInstanceType::NCM,
                3 => FabricInstanceType::IOMS,
                4 => FabricInstanceType::CS,
                5 => FabricInstanceType::NCS,
                6 => FabricInstanceType::TCDX,
                7 => FabricInstanceType::PIE,
                8 => FabricInstanceType::SPF,
                9 => FabricInstanceType::LLC,
                0xA => FabricInstanceType::CAKE,
                _ => FabricInstanceType::Unknown,
            };
            let enabled = match info0 & (1 << 6) {
                0 => false,
                _ => true,
            };

            let ids = df_read_indirect(0, x_instance_id, 0, 0x50); // _inst[PIE0,IOM[3:0],CCM[7:0],CCIX[3:0],CS[7:0],BCST]_aliasHOST;
            let instance_id: u8 = (ids & 0xFF) as u8; // Note: 0 usually means off, except for the first entry
            let fabric_id: u8 = ((ids >> 8) & 0x3F) as u8; // Note: 0 usually means None, except for the first entry
            if instance_id == 0 && result.components.len() > 0 {
                // component is off
                // instance_id = 0 should be skipped except for the first entry
            } else {
                assert!(instance_id == x_instance_id);
                result.components.push(FabricComponent { instance_id, instance_type, enabled, fabric_id: if fabric_id != 0 || result.components.len() == 0 { Some(fabric_id) } else { None } });
            }
        }
        result
    }
}
