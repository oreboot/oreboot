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

use fsp_cfl_sys as fsp32;

// TODO: it might be nice to do some trickery where fsp_*_sys just exposes
// everything in fsp_common.
use fsp_common as fsp;

// Unless we mention the fsp_cfl_sys crate, the compiler will optimize it away. This crate
// introduces the symbols containing the FSP binary which get picked up by the linker.
extern crate fsp_cfl_sys;

global_asm!(
    preprocess_asm!("../../../arch/x86/x86_64/src/bootblock_nomem.S"),
    options(att_syntax)
);

fn call_fspm(fsp_base: usize, fspm_entry: usize) -> u32 {
    let mut fspm_cfg = fsp32::FSP_M_CONFIG::default();
    let mut fspm_test_cfg = fsp32::FSP_M_TEST_CONFIG::default();

    // These values are taken from
    // https://github.com/coreboot/coreboot/blob/master/src/soc/intel/cannonlake/romstage/fsp_params.c#L20
    // TODO: Fill out the remaining parameters
    // TODO: Use constant defines for these instead of magic numbers. For example, define TsegSize
    // as CONFIG_SMM_TSEG_SIZE. This can be defined in fsp_cfl_sys.
    fspm_cfg.InternalGfx = 0;
    fspm_cfg.IgdDvmt50PreAlloc = 0;
    fspm_cfg.TsegSize = 0x800000;
    fspm_cfg.IedSize = 0x400000;
    fspm_cfg.SaGv = 0; // Allows memory training to happen at different frequencies? Disabled for now

    fspm_test_cfg.PanelPowerEnable = 0;

    // TODO: This struct has to be aligned to 4.
    // mut because we can't make the assumption FSP won't modify it.
    let mut fspm_upd = fsp32::FSPM_UPD {
        FspUpdHeader: fsp32::FSP_UPD_HEADER {
            Signature: fsp32::FSPM_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspmArchUpd: fsp32::FSPM_ARCH_UPD {
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 3],
            NvsBufferPtr: 0 as *mut core::ffi::c_void, // non-volatile storage not available
            StackBase: 0x20000000 as *mut core::ffi::c_void, // TODO: I picked this at random
            StackSize: 0x10000,                        // TODO: I picked this at random
            BootLoaderTolumSize: 0, // Don't reserve "top of low usable memory" for bootloader.
            BootMode: fsp32::BOOT_WITH_FULL_CONFIGURATION,
            Reserved1: [0u8; 8],
        },
        FspmConfig: fspm_cfg,
        FspmTestConfig: fspm_test_cfg,
        UnusedUpdSpace6: 0u8,
        UpdTerminator: 0x55AA, // ???
    };

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
    let mut fsps_config = fsp32::FSP_S_CONFIG::default();
    let fsps_test_config = fsp32::FSP_S_TEST_CONFIG::default();

    fsps_config.LogoSize = 0;
    fsps_config.LogoPtr = 0;
    fsps_config.GraphicsConfigPtr = 0;
    fsps_config.ReservedFspsUpd = [0u8];

    // TODO: This struct has to be aligned to 4.
    // mut because we can't make the assumption FSP won't modify it.
    let mut fsps_upd = fsp32::FSPS_UPD {
        FspUpdHeader: fsp32::FSP_UPD_HEADER {
            Signature: fsp32::FSPS_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspsConfig: fsps_config,
        FspsTestConfig: fsps_test_config,
        UpdTerminator: 0x55AA, // ???
    };

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
