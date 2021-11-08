//#![feature(lang_items, start)]
#![no_std]
#![no_main]
//#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use print;
use uart::i8250::I8250;

use rpp_procedural::preprocess_asm;

use fsp_common as fsp;
use fsp_qemu32_sys as fsp32;

// Unless we mention the fsp_qemu_sys crate, the compiler will optimize it away. This crate
// introduces the symbols containing the FSP binary which get picked up by the linker.
extern crate fsp_qemu32_sys;

global_asm!(
    preprocess_asm!("../../../arch/x86/x86_64/src/bootblock_nomem.S"),
    options(att_syntax)
);

fn call_fspm(fsp_base: u32, fspm_entry: u32) -> u32 {
    // TODO: Is this going to be different for different boards, or
    // can it just be wrapped in the fsp-common crate?
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
            NvsBufferPtr: 0,        // non-volatile storage not available
            StackBase: 0x20000000,  // TODO: I picked this at random
            StackSize: 0x10000,     // TODO: I picked this at random
            BootLoaderTolumSize: 0, // Don't reserve "top of low usable memory" for bootloader.
            BootMode: fsp32::BOOT_WITH_FULL_CONFIGURATION,
            FspEventHandler: 0 as *mut fsp32::FSP_EVENT_HANDLER, // optional
            Reserved1: [0u8; 4],
        },
        FspmConfig: fsp32::FSP_M_CONFIG {
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

    let x86_util = arch::X86Util::new_rom_util();

    let status = unsafe {
        x86_util.protected_mode_call(
            fsp_base + fspm_entry,
            core::mem::transmute::<&mut fsp32::FSPM_UPD, u64>(&mut fspm_upd) as u32,
            0 as u32,
        )
    };

    status
}

fn call_fsps(fsp_base: u32, fsps_entry: u32) -> u32 {
    // TODO: This struct has to be aligned to 4.
    // mut because we can't make the assumption FSP won't modify it.
    let mut fsps_upd = fsp32::FSPS_UPD {
        FspUpdHeader: fsp32::FSP_UPD_HEADER {
            Signature: fsp32::FSPS_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        UnusedUpdSpace0: [0u8; 32],
        FspsConfig: fsp32::FSP_S_CONFIG {
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

    // TODO: Making a new X86Util for each call is redundant.
    // Just make one in _start.
    let x86_util = arch::X86Util::new_rom_util();

    let status = unsafe {
        x86_util.protected_mode_call(
            fsp_base + fsps_entry,
            core::mem::transmute::<&mut fsp32::FSPS_UPD, u64>(&mut fsps_upd) as u32,
            0 as u32,
        )
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
