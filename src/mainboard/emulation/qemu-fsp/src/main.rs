#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr;
use model::Driver;
use uart::i8250::I8250;

use rpp_procedural::preprocess_asm;

use fsp_common as fsp;
use fsp_qemu_sys as fsp64;

global_asm!(
    preprocess_asm!("../../../arch/x86/x86_64/src/bootblock_nomem.S"),
    options(att_syntax)
);

fn call_fspm(
    fsp_base: usize,
    fspm_entry: usize,
    hob_list_ptr_ptr: *mut *mut fsp64::EFI_HOB_HANDOFF_INFO_TABLE,
) -> fsp64::EFI_STATUS {
    let mut fspm_upd = fsp64::get_fspm_upd();
    let status = unsafe {
        type FspMemoryInit = unsafe extern "win64" fn(
            FspmUpdDataPtr: *mut core::ffi::c_void,
            HobListPtr: *mut *mut core::ffi::c_void,
        ) -> fsp64::EFI_STATUS;
        let fspm = core::mem::transmute::<usize, FspMemoryInit>(fsp_base + fspm_entry);
        fspm(
            core::mem::transmute(&mut fspm_upd),
            core::mem::transmute(hob_list_ptr_ptr),
        )
    };

    status
}

fn call_fsps(fsp_base: usize, fsps_entry: usize) -> fsp64::EFI_STATUS {
    let mut fsps_upd = fsp64::get_fsps_upd();
    let status = unsafe {
        type FspSiliconInit =
            unsafe extern "win64" fn(FspsUpdDataPtr: *mut core::ffi::c_void) -> fsp64::EFI_STATUS;
        let fsps = core::mem::transmute::<usize, FspSiliconInit>(fsp_base + fsps_entry);
        fsps(core::mem::transmute(&mut fsps_upd))
    };

    status
}

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
    let infos = match fsp::find_fsp(fspfv) {
        Ok(x) => x,
        Err(err) => panic!("Error finding FSP: {}\r\n", err),
    };
    write!(w, "Found FSP_INFO: {:#x?}\r\n", infos).unwrap();

    let mut hob_list_ptr: *mut fsp64::EFI_HOB_HANDOFF_INFO_TABLE = ptr::null_mut();
    let mut hob_list_ptr_ptr: *mut *mut fsp64::EFI_HOB_HANDOFF_INFO_TABLE = &mut hob_list_ptr;

    if let Some(fspm_entry) = fsp::get_fspm_entry(&infos) {
        write!(
            w,
            "Calling FspMemoryInit at {:#x}+{:#x}\r\n",
            fsp_base, fspm_entry
        )
        .unwrap();

        let status = call_fspm(fsp_base, fspm_entry, hob_list_ptr_ptr);
        write!(w, "Returned {}\r\n", status).unwrap();
    } else {
        write!(w, "Could not find FspMemoryInit\r\n").unwrap();
    }

    if let Some(fsps_entry) = fsp::get_fsps_entry(&infos) {
        write!(
            w,
            "Calling FspSiliconInit at {:#x}+{:#x}\r\n",
            fsp_base, fsps_entry
        )
        .unwrap();

        let status = call_fsps(fsp_base, fsps_entry);

        write!(w, "Returned {}\r\n", status).unwrap();
    } else {
        write!(w, "Could not find FspSiliconInit\r\n").unwrap();
    }

    // "The bootloader should not expect a complete HOB list after the FSP returns
    // from this API. It is recommended for the bootloader to save this HobListPtr
    // returned from this API and parse the full HOB list after the FspSiliconInit() API."
    unsafe {
        write!(w, "EFI_HOB_HANDOFF_INFO_TABLE\r\n");
        write!(w, "========================================\r\n");
        write!(w, "Version = {}\r\n", (*hob_list_ptr).Version);
        write!(w, "BootMode = {}\r\n", (*hob_list_ptr).BootMode);
        write!(w, "EfiMemoryTop = {:#x?}\r\n", (*hob_list_ptr).EfiMemoryTop);
        write!(
            w,
            "EfiMemoryBottom = {:#x?}\r\n",
            (*hob_list_ptr).EfiMemoryBottom
        );
        write!(
            w,
            "EfiFreeMemoryTop = {:#x?}\r\n",
            (*hob_list_ptr).EfiFreeMemoryTop
        );
        write!(
            w,
            "EfiFreeMemoryBottom = {:#x?}\r\n",
            (*hob_list_ptr).EfiFreeMemoryBottom
        );
        write!(
            w,
            "EfiEndOfHobList = {:#x?}\r\n",
            (*hob_list_ptr).EfiEndOfHobList
        );

        let end_address: u64 = (*hob_list_ptr).EfiEndOfHobList;

        let mut hob_list_bytes_offset: isize = 0;
        let mut hob_list_bytes_ptr: *const u8 = core::mem::transmute(hob_list_ptr);

        while (hob_list_ptr as u64) < end_address {
            write!(w, "Hob @ {:#x?}\r\n", hob_list_ptr);
            write!(w, "Header.HobType = {}\r\n", (*hob_list_ptr).Header.HobType);
            write!(
                w,
                "Header.HobLength = {}\r\n",
                (*hob_list_ptr).Header.HobLength
            );
            hob_list_bytes_offset += (*hob_list_ptr).Header.HobLength as isize;
            hob_list_ptr = core::mem::transmute(hob_list_bytes_ptr.offset(hob_list_bytes_offset));
        }
    }

    // TODO: Get these values from the fdt
    let payload = &mut BzImage {
        low_mem_size: 0x8000_0000,
        high_mem_start: 0x100000_0000,
        high_mem_size: 0,
        rom_base: 0xff00_0000,
        rom_size: 0x100_0000,
        load: 0x100_0000,
        entry: 0x100_0200,
    };

    write!(w, "Running payload\r\n").unwrap();
    payload.run(w);

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
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
