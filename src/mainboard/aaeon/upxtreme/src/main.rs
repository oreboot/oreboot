#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
// Clippy stupidly suggests using functions from std even you we have declared
// no_std. Shame on clippy.
#![allow(clippy::zero_ptr)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use uart::debug_port::DebugPort;
use uart::i8250::I8250;

use rpp_procedural::preprocess_asm;

use fsp_common as fsp;

global_asm!(
    preprocess_asm!("../../../arch/x86/x86_64/src/bootblock_nomem.S"),
    options(att_syntax)
);

fn call_fspm(fsp_base: usize, fspm_entry: usize) -> u32 {
    let mut fspm_upd = fsp_cfl_sys::get_fspm_upd();

    unsafe {
        type FspMemoryInit = unsafe extern "win64" fn(
            FspmUpdDataPtr: *mut core::ffi::c_void,
            HobListPtr: *mut *mut core::ffi::c_void,
        ) -> u32;
        let fspm = core::mem::transmute::<usize, FspMemoryInit>(fsp_base + fspm_entry);
        fspm(core::mem::transmute(&mut fspm_upd), 0 as *mut _)
    }
}

fn call_fsps(fsp_base: usize, fsps_entry: usize) -> u32 {
    let mut fsps_upd = fsp_cfl_sys::get_fsps_upd();
    unsafe {
        type FspSiliconInit =
            unsafe extern "win64" fn(FspsUpdDataPtr: *mut core::ffi::c_void) -> u32;
        let fsps = core::mem::transmute::<usize, FspSiliconInit>(fsp_base + fsps_entry);
        fsps(core::mem::transmute(&mut fsps_upd))
    }
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

    let fsp_base = 0xFFF80000usize;
    let fsp_size = 0x40000usize;
    let fspfv = unsafe { core::slice::from_raw_parts(fsp_base as *const u8, fsp_size) };
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

        let status = call_fspm(fsp_base, fspm_entry);

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
