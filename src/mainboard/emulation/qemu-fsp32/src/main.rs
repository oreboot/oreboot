#![feature(asm_const)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr;
use oreboot_arch::x86_64::{self as arch, bzimage::BzImage, consts::*, ioport::IOPort};
use oreboot_drivers::{uart::i8250::I8250, Driver};
use print;

use fsp_common as fsp;
use fsp_qemu32_sys as fsp32;

// Unless we mention the fsp_qemu_sys crate, the compiler will optimize it away. This crate
// introduces the symbols containing the FSP binary which get picked up by the linker.
extern crate fsp_qemu32_sys;

global_asm!(
    include_str!("../../../../arch/src/x86_64/bootblock_nomem.S"),
    CD = const x86::cr0::CD,
    NW = const x86::cr0::NW,
    TS = const x86::cr0::TS,
    MP = const x86::cr0::MP,
    PG = const x86::cr0::PG,
    WP = const x86::cr0::WP,
    PE = const x86::cr0::PE,
    PSE = const x86::cr4::PSE,
    PGE = const x86::cr4::PGE,
    PAE = const x86::cr4::PAE,
    EFER = const msr::EFER,
    LME = const msr::efer::LME,
    options(att_syntax)
);

fn call_fspm(
    fsp_base: u32,
    fspm_entry: u32,
    hob_list_ptr_ptr: *mut *mut fsp32::EFI_HOB_HANDOFF_INFO_TABLE,
) -> u32 {
    let mut fspm_upd = fsp32::get_fspm_upd();
    let x86_util = arch::X86Util::new_rom_util();

    let upd_adr =
        unsafe { core::mem::transmute::<&mut fsp32::FSPM_UPD, u64>(&mut fspm_upd) as u32 };
    let hob_adr = unsafe {
        core::mem::transmute::<*mut *mut fsp32::EFI_HOB_HANDOFF_INFO_TABLE, u64>(hob_list_ptr_ptr)
            as u32
    };

    x86_util.protected_mode_call(fsp_base + fspm_entry, upd_adr, hob_adr)
}

fn call_fsps(fsp_base: u32, fsps_entry: u32) -> u32 {
    let mut fsps_upd = fsp32::get_fsps_upd();

    // TODO: Making a new X86Util for each call is redundant.
    // Just make one in _start.
    let x86_util = arch::X86Util::new_rom_util();

    let upd_adr =
        unsafe { core::mem::transmute::<&mut fsp32::FSPS_UPD, u64>(&mut fsps_upd) as u32 };

    x86_util.protected_mode_call(fsp_base + fsps_entry, upd_adr, 0 as u32)
}

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    // FSP has some SSE instructions.
    arch::enable_sse();

    let uart0 = &mut I8250::new(0x3f8, 0, IOPort {});
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    let fsp_base = 0xFFF80000u32;
    let fsp_size = 0x40000u32;
    let fspfv = unsafe { core::slice::from_raw_parts(fsp_base as *const u8, fsp_size as usize) };
    let infos = match fsp::find_fsp(fspfv) {
        Ok(x) => x,
        Err(err) => panic!("Error finding FSP: {}\r\n", err),
    };
    write!(w, "Found FSP_INFO: {:#x?}\r\n", infos).unwrap();

    let mut hob_list_ptr: *mut fsp32::EFI_HOB_HANDOFF_INFO_TABLE = ptr::null_mut();
    let hob_list_ptr_ptr: *mut *mut fsp32::EFI_HOB_HANDOFF_INFO_TABLE = &mut hob_list_ptr;

    if let Some(fspm_entry) = fsp::get_fspm_entry(&infos) {
        write!(
            w,
            "Calling FspMemoryInit at {:#x}+{:#x}\r\n",
            fsp_base, fspm_entry
        )
        .unwrap();

        let status = call_fspm(fsp_base, fspm_entry as u32, hob_list_ptr_ptr);

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

        let status = call_fsps(fsp_base, fsps_entry as u32);

        write!(w, "Returned {}\r\n", status).unwrap();
    } else {
        write!(w, "Could not find FspSiliconInit\r\n").unwrap();
    }

    // "The bootloader should not expect a complete HOB list after the FSP returns
    // from this API. It is recommended for the bootloader to save this HobListPtr
    // returned from this API and parse the full HOB list after the FspSiliconInit() API."
    unsafe { fsp32::dump_fsp_hobs(hob_list_ptr, w) };

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
