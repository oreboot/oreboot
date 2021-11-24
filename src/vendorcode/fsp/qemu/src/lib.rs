#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![allow(clippy::zero_ptr)]

// Rust types are used instead of generated ones.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EFI_GUID(pub UINT32, pub UINT16, pub UINT16, pub [UINT8; 8]);
pub type GUID = EFI_GUID;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Don't mangle as this is referenced from a linker script to place in a specific location in
// flash.
macro_rules! blob_macro {
    () => {
        include_bytes!(concat!(env!("OUT_DIR"), "/QEMUFSP.fd"))
    };
}
#[no_mangle]
#[used]
#[link_section = ".fspblob"]
static FSP_BLOB: [u8; blob_macro!().len()] = *blob_macro!();

pub fn get_fspm_upd() -> FSPM_UPD {
    FSPM_UPD {
        FspUpdHeader: FSP_UPD_HEADER {
            Signature: FSPM_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspmArchUpd: FSPM_ARCH_UPD {
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 3],
            NvsBufferPtr: 0,        // non-volatile storage not available
            StackBase: 0x20000000,  // TODO: I picked this at random
            StackSize: 0x10000,     // TODO: I picked this at random
            BootLoaderTolumSize: 0, // Don't reserve "top of low usable memory" for bootloader.
            BootMode: BOOT_WITH_FULL_CONFIGURATION,
            FspEventHandler: 0 as *mut FSP_EVENT_HANDLER, // optional
            Reserved1: [0u8; 4],
        },
        FspmConfig: FSP_M_CONFIG {
            SerialDebugPortAddress: 0x3f8,
            SerialDebugPortType: 1,       // I/O
            SerialDebugPortDevice: 3,     // External Device
            SerialDebugPortStrideSize: 0, // 1
            UnusedUpdSpace0: [0; 49],
            ReservedFspmUpd: [0; 4],
        },
        UnusedUpdSpace1: [0u8; 2],
        UpdTerminator: 0x55AA, // ???
    }
}

pub fn get_fsps_upd() -> FSPS_UPD {
    FSPS_UPD {
        FspUpdHeader: FSP_UPD_HEADER {
            Signature: FSPS_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        UnusedUpdSpace0: [0u8; 32],
        FspsConfig: FSP_S_CONFIG {
            LogoSize: 0,
            LogoPtr: 0,
            GraphicsConfigPtr: 0,
            PciTempResourceBase: 0,
            UnusedUpdSpace1: [0; 32],
            ReservedFspsUpd: 0,
        },
        UnusedUpdSpace2: [0u8; 13],
        UpdTerminator: 0x55AA, // ???
    }
}

pub fn dump_fsp_hobs(
    mut hob_list_ptr: *const EFI_HOB_HANDOFF_INFO_TABLE,
    w: &mut impl core::fmt::Write,
) {
    let hob_list: &EFI_HOB_HANDOFF_INFO_TABLE = unsafe { &*hob_list_ptr };
    write!(w, "EFI_HOB_HANDOFF_INFO_TABLE\r\n").unwrap();
    write!(w, "========================================\r\n").unwrap();
    write!(w, "Version = {}\r\n", hob_list.Version).unwrap();
    write!(w, "BootMode = {}\r\n", hob_list.BootMode).unwrap();
    write!(w, "EfiMemoryTop = {:#x?}\r\n", hob_list.EfiMemoryTop).unwrap();
    write!(w, "EfiMemoryBottom = {:#x?}\r\n", hob_list.EfiMemoryBottom).unwrap();
    write!(
        w,
        "EfiFreeMemoryTop = {:#x?}\r\n",
        hob_list.EfiFreeMemoryTop
    )
    .unwrap();
    write!(
        w,
        "EfiFreeMemoryBottom = {:#x?}\r\n",
        hob_list.EfiFreeMemoryBottom
    )
    .unwrap();
    write!(w, "EfiEndOfHobList = {:#x?}\r\n", hob_list.EfiEndOfHobList).unwrap();

    let end_address: u64 = hob_list.EfiEndOfHobList;
    let mut hob_list_bytes_offset: isize = 0;
    let hob_list_bytes_ptr: *const u8 = unsafe { core::mem::transmute(hob_list_ptr) };

    while (hob_list_ptr as u64) < end_address {
        let hob_list: &EFI_HOB_HANDOFF_INFO_TABLE = unsafe { &*hob_list_ptr };
        write!(w, "Hob @ {:#x?}\r\n", hob_list_ptr).unwrap();
        write!(w, "Header.HobType = {}\r\n", hob_list.Header.HobType).unwrap();
        write!(w, "Header.HobLength = {}\r\n", hob_list.Header.HobLength).unwrap();
        hob_list_bytes_offset += hob_list.Header.HobLength as isize;
        hob_list_ptr =
            unsafe { core::mem::transmute(hob_list_bytes_ptr.offset(hob_list_bytes_offset)) };
    }
}
