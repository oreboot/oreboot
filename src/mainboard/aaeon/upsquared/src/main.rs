#![feature(asm_const)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use oreboot_arch::x86_64 as arch;
use oreboot_arch::x86_64::{bzimage::BzImage, consts::*, ioport::IOPort};
use oreboot_drivers::{
    uart::{debug_port::DebugPort, i8250::I8250},
    Driver,
};
use print;
// mod romstage; // FIXME

global_asm!(
    include_str!("../../../../arch/src/x86_64/bootblock.S"),
    PSE = const x86::cr4::PSE,
    EFER = const msr::EFER,
    LME = const msr::efer::LME,
    CD = const x86::cr0::CD,
    NW = const x86::cr0::NW,
    TS = const x86::cr0::TS,
    MP = const x86::cr0::MP,
    PG = const x86::cr0::PG,
    WP = const x86::cr0::WP,
    PE = const x86::cr0::PE,
    PTE_P = const x86::pg::P,
    PTE_RW = const x86::pg::RW,
    PTE_PS = const x86::pg::PS,
    PTE2_MPAT = const x86::pg::PAT,
    MTRR_CAP_MSR = const mtrr::CAP_MSR,
    MTRR_DEF_TYPE_MSR = const mtrr::DEF_TYPE_MSR,
    MTRR_TYPE_WRBACK = const mtrr::TYPE_WRBACK,
    MTRR_PHYS_MASK_VALID = const mtrr::PHYS_MASK_VALID,
    MTRR_DEF_TYPE_EN = const mtrr::DEF_TYPE_EN,
    MTRR_TYPE_WRPROT = const mtrr::TYPE_WRPROT,
    options(att_syntax)
);

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
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
