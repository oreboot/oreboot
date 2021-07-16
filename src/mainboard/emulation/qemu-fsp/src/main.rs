#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use print;
use uart::i8250::I8250;
use uefi::{fv_traverse, SectionData};

use heapless::consts::U4;
use heapless::Vec;

use efi::EFI_GUID as GUID;
use fsp_qemu_sys as efi;

// Unless we mention the fsp_qemu_sys crate, the compiler will optimize it away. This crate
// introduces the symbols containing the FSP binary which get picked up by the linker.
extern crate fsp_qemu_sys;

global_asm!(include_str!(
    "../../../../arch/x86/x86_64/src/bootblock_nomem.S"
));

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    // FSP has some SSE instructions.
    arch::enable_sse();

    let uart0 = &mut I8250::new(0x3f8, 0, IOPort {});
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    let fsp_base = 0xFFF80000usize;
    let fsp_size = 0x40000usize;
    let fspfv = unsafe { core::slice::from_raw_parts(fsp_base as *const u8, fsp_size) };
    let infos = match find_fsp(fspfv) {
        Ok(x) => x,
        Err(err) => panic!("Error finding FSP: {}\r\n", err),
    };
    write!(w, "Found FSP_INFO: {:#x?}\r\n", infos).unwrap();

    if let Some(fspm_entry) = fsp_memory_init_entry(&infos) {
        write!(
            w,
            "Calling FspMemoryInit at {:#x}+{:#x}\r\n",
            fsp_base, fspm_entry
        )
        .unwrap();

        // TODO: This struct has to be aligned to 4.
        // mut because we can't make the assumption FSP won't modify it.
        let mut fspm_upd = efi::FSPM_UPD {
            FspUpdHeader: efi::FSP_UPD_HEADER {
                Signature: efi::FSPM_UPD_SIGNATURE,
                Revision: 2, // FSP 2.2
                Reserved: [0u8; 23],
            },
            FspmArchUpd: efi::FSPM_ARCH_UPD {
                Revision: 2, // FSP 2.2
                Reserved: [0u8; 3],
                NvsBufferPtr: 0,        // non-volatile storage not available
                StackBase: 0x20000000,  // TODO: I picked this at random
                StackSize: 0x10000,     // TODO: I picked this at random
                BootLoaderTolumSize: 0, // Don't reserve "top of low usable memory" for bootloader.
                BootMode: efi::BOOT_WITH_FULL_CONFIGURATION,
                FspEventHandler: 0 as *mut efi::FSP_EVENT_HANDLER, // optional
                Reserved1: [0u8; 4],
            },
            FspmConfig: efi::FSP_M_CONFIG {
                SerialDebugPortAddress: 0x3f8,
                SerialDebugPortType: 1,       // I/O
                SerialDebugPortDevice: 3,     // External Device
                SerialDebugPortStrideSize: 0, // 1
                UnusedUpdSpace0: [0; 49],
                ReservedFspmUpd: [0; 4],
            },
            UnusedUpdSpace1: [0u8; 2],
            UpdTerminator: 0x55AA, // ???
        };

        let status = unsafe {
            type FspMemoryInit = unsafe extern "win64" fn(
                FspmUpdDataPtr: *mut core::ffi::c_void,
                HobListPtr: *mut *mut core::ffi::c_void,
            ) -> efi::EFI_STATUS;
            let fspm = core::mem::transmute::<usize, FspMemoryInit>(fsp_base + fspm_entry);
            fspm(core::mem::transmute(&mut fspm_upd), 0 as *mut _)
        };
        write!(w, "Returned {}\r\n", status).unwrap();
    } else {
        write!(w, "Could not find FspMemoryInit\r\n").unwrap();
    }

    if let Some(fsps_entry) = fsp_silicon_init_entry(&infos) {
        write!(
            w,
            "Calling FspSiliconInit at {:#x}+{:#x}\r\n",
            fsp_base, fsps_entry
        )
        .unwrap();

        // TODO: This struct has to be aligned to 4.
        // mut because we can't make the assumption FSP won't modify it.
        let mut fsps_upd = efi::FSPS_UPD {
            FspUpdHeader: efi::FSP_UPD_HEADER {
                Signature: efi::FSPS_UPD_SIGNATURE,
                Revision: 2, // FSP 2.2
                Reserved: [0u8; 23],
            },
            UnusedUpdSpace0: [0u8; 32],
            FspsConfig: efi::FSP_S_CONFIG {
                LogoSize: 0,
                LogoPtr: 0,
                GraphicsConfigPtr: 0,
                PciTempResourceBase: 0,
                UnusedUpdSpace1: [0; 32],
                ReservedFspsUpd: 0,
            },
            UnusedUpdSpace2: [0u8; 13],
            UpdTerminator: 0x55AA, // ???
        };

        let status = unsafe {
            type FspSiliconInit =
                unsafe extern "win64" fn(FspsUpdDataPtr: *mut core::ffi::c_void) -> efi::EFI_STATUS;
            let fsps = core::mem::transmute::<usize, FspSiliconInit>(fsp_base + fsps_entry);
            fsps(core::mem::transmute(&mut fsps_upd))
        };
        write!(w, "Returned {}\r\n", status).unwrap();
    } else {
        write!(w, "Could not find FspSiliconInit\r\n").unwrap();
    }

    // TODO: Get these values from the fdt
    let payload = &mut BzImage {
        low_mem_size: 0x80_000_000,
        high_mem_start: 0x1_000_000_000,
        high_mem_size: 0,
        rom_base: 0xff_000_000,
        rom_size: 0x1_000_000,
        load: 0x1_000_000,
        entry: 0x1_000_200,
    };

    write!(w, "Running payload\r\n").unwrap();
    payload.run(w);

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

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
const FILE_TYPES: &'static [u32] = &[efi::EFI_FV_FILETYPE_RAW];

#[derive(Debug)]
struct FspInfoEntry {
    addr: usize,
    info: efi::FSP_INFO_HEADER,
}
type FspInfos = Vec<FspInfoEntry, U4>;

fn fsp_memory_init_entry(infos: &FspInfos) -> Option<usize> {
    for entry in infos.iter() {
        if entry.info.ComponentAttribute & 0xf000 == 0x2000 {
            return Some(entry.addr + entry.info.FspMemoryInitEntryOffset as usize);
        }
    }
    None
}

fn fsp_silicon_init_entry(infos: &FspInfos) -> Option<usize> {
    for entry in infos.iter() {
        if entry.info.ComponentAttribute & 0xf000 == 0x3000 {
            return Some(entry.addr + entry.info.FspSiliconInitEntryOffset as usize);
        }
    }
    None
}

#[no_mangle]
fn find_fsp(fspfv: &[u8]) -> Result<FspInfos, uefi::FvTraverseError> {
    let mut infos = FspInfos::new();

    fv_traverse(fspfv, FILE_TYPES, |sec_info, sec_data: SectionData| {
        // All three parts must match.
        match (sec_info.ffs_guid, sec_info.ffs_type, sec_info.sec_type) {
            (FSP_FFS_INFORMATION_FILE_GUID, efi::EFI_FV_FILETYPE_RAW, efi::EFI_SECTION_RAW) => {
                if infos.len() != infos.capacity() {
                    infos
                        .push(FspInfoEntry {
                            addr: sec_info.fv_base,
                            info: unsafe { *(sec_data.as_ptr() as *const efi::FSP_INFO_HEADER) }
                                .clone(),
                        })
                        .unwrap();
                }
            }
            (FSP_S_UPD_FFS_GUID, efi::EFI_FV_FILETYPE_RAW, efi::EFI_SECTION_RAW) => (),
            _ => (),
        }
    })?;
    Ok(infos)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut I8250::new(0x3f8, 0, IOPort {});
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
