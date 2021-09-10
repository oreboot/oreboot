#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use heapless::consts::U4;
use heapless::Vec;
use uefi::EFI_GUID as GUID;
use uefi::{fv_traverse, SectionData};

include!(concat!(env!("OUT_DIR"), "/", "fsp_bindings.rs"));

const FSP_FFS_INFORMATION_FILE_GUID: GUID = GUID(
    0x912740be,
    0x2284,
    0x4734,
    [0xb9, 0x71, 0x84, 0xb0, 0x27, 0x35, 0x3f, 0x0c],
);
const FSP_S_UPD_FFS_GUID: GUID = GUID(
    0xe3cd9b18,
    0x998c,
    0x4f76,
    [0xb6, 0x5e, 0x98, 0xb1, 0x54, 0xe5, 0x44, 0x6f],
);
const FILE_TYPES: &[u32] = &[uefi::EFI_FV_FILETYPE_RAW];

#[derive(Debug)]
pub struct FspInfoEntry {
    addr: usize,
    info: FSP_INFO_HEADER,
}
pub type FspInfos = Vec<FspInfoEntry, U4>;

pub fn get_fspm_entry(infos: &FspInfos) -> Option<usize> {
    for entry in infos.iter() {
        if entry.info.ComponentAttribute & 0xf000 == 0x2000 {
            return Some(entry.addr + entry.info.FspMemoryInitEntryOffset as usize);
        }
    }
    None
}

pub fn get_fsps_entry(infos: &FspInfos) -> Option<usize> {
    for entry in infos.iter() {
        if entry.info.ComponentAttribute & 0xf000 == 0x3000 {
            return Some(entry.addr + entry.info.FspSiliconInitEntryOffset as usize);
        }
    }
    None
}

#[no_mangle]
pub fn find_fsp(fspfv: &[u8]) -> Result<FspInfos, uefi::FvTraverseError> {
    let mut infos = FspInfos::new();

    fv_traverse(fspfv, FILE_TYPES, |sec_info, sec_data: SectionData| {
        // All three parts must match.
        match (sec_info.ffs_guid, sec_info.ffs_type, sec_info.sec_type) {
            (FSP_FFS_INFORMATION_FILE_GUID, uefi::EFI_FV_FILETYPE_RAW, uefi::EFI_SECTION_RAW) => {
                if infos.len() != infos.capacity() {
                    infos
                        .push(FspInfoEntry {
                            addr: sec_info.fv_base as usize,
                            info: unsafe { *(sec_data.as_ptr() as *const FSP_INFO_HEADER) },
                        })
                        .unwrap();
                }
            }
            (FSP_S_UPD_FFS_GUID, uefi::EFI_FV_FILETYPE_RAW, uefi::EFI_SECTION_RAW) => (),
            _ => (),
        }
    })?;
    Ok(infos)
}
