#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm_const)]
// Clippy stupidly suggests using functions from std even you we have declared
// no_std. Shame on clippy.
#![allow(clippy::zero_ptr)]

use arch::bzimage::BzImage;
use arch::consts::*;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use uart::debug_port::DebugPort;
use uart::i8250::I8250;

use fsp_common as fsp;

global_asm!(
    include_str!("../../../../arch/x86/x86_64/src/bootblock_nomem.S"),
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

fn call_fspm(fsp_base: u32, fspm_entry: u32) -> u32 {
    let mut fspm_upd = fsp_cfl_sys::get_fspm_upd();

    let x86_util = arch::X86Util::new_rom_util();

    let upd_adr =
        unsafe { core::mem::transmute::<&mut fsp_cfl_sys::FSPM_UPD, u64>(&mut fspm_upd) as u32 };

    x86_util.protected_mode_call(fsp_base + fspm_entry, upd_adr, 0)
}

fn call_fsps(fsp_base: u32, fsps_entry: u32) -> u32 {
    let mut fsps_upd = fsp_cfl_sys::get_fsps_upd();

    let x86_util = arch::X86Util::new_rom_util();

    let upd_adr =
        unsafe { core::mem::transmute::<&mut fsp_cfl_sys::FSPS_UPD, u64>(&mut fsps_upd) as u32 };

    x86_util.protected_mode_call(fsp_base + fsps_entry, upd_adr, 0)
}

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    // FSP has some SSE instructions.
    arch::enable_sse();

    let uart0 = &mut I8250::new(0x3f8, 0, IOPort {});
    // Note: on real hardware, use port 0x80 instead for "POST" output
    let debug = &mut DebugPort::new(0xe9, IOPort {});
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    debug.init().unwrap();
    debug
        .pwrite(b"Welcome to oreboot - debug port E9\r\n", 0)
        .unwrap();

    let w = &mut print::WriteTo::new(uart0);

    let fsp_base = 0xFFF80000u32;
    let fsp_size = 0x40000u32;
    let fspfv = unsafe { core::slice::from_raw_parts(fsp_base as *const u8, fsp_size as usize) };
    let infos = match fsp::find_fsp(fspfv) {
        Ok(x) => x,
        Err(err) => panic!("Error finding FSP: {}\r\n", err),
    };
    write!(w, "Found FSP_INFO: {:#x?}\r\n", infos).unwrap();

    if let Some(fspm_entry) = fsp::get_fspm_entry(&infos) {
        write!(
            w,
            "Calling FspMemoryInit at {:#x}+{:#x}\r\n",
            fsp_base, fspm_entry
        )
        .unwrap();

        let status = call_fspm(fsp_base, fspm_entry as u32);

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

    // TODO: Get these values from the fdt
    let payload = &mut BzImage {
        low_mem_size: 0x8000_0000,
        high_mem_start: 0x10_0000_0000,
        high_mem_size: 0,
        rom_base: 0xff00_0000,
        rom_size: 0x100_0000,
        load: 0x100_0000,
        entry: 0x100_0200,
    };

    payload.load(w).unwrap();

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
