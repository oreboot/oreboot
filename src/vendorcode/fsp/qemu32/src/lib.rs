#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![allow(clippy::zero_ptr)]

use uefi::EFI_GUID;

include!(concat!(env!("OUT_DIR"), "/", "bindings.rs"));

// Don't mangle as this is referenced from a linker script to place in a specific location in
// flash.
macro_rules! blob_macro {
    () => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", "QEMUFSP.fd"))
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
