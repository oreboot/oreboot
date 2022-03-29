#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use oreboot_arch::riscv64 as arch;
use oreboot_drivers::{
    uart::sunxi::{Sunxi, UART0},
    wrappers::{DoD, Memory, SectionReader},
    Driver,
};
use oreboot_soc::sunxi::d1::{ccu::CCU, gpio::GPIO};
use payloads::payload;
use sbi::sbi_init;

global_asm!(include_str!("../start.S"));

// hart = hardware thread (something like core)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // clock
    let mut ccu = CCU::new();
    ccu.init().unwrap();
    let mut gpio = GPIO::new();
    gpio.init().unwrap();
    let mut uart0 = Sunxi::new(UART0 as usize, 115200);
    uart0.init().unwrap();
    uart0.pwrite(b"UART0 initialized\r\n", 0).unwrap();

    let mut uarts = [&mut uart0 as &mut dyn Driver];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init().unwrap();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(console);
    writeln!(w, "## Loading payload\r").unwrap();

    // see ../fixed-dtfs.dts
    // TODO: adjust when DRAM driver is implemented / booting from SPI
    let mem = 0x4000_0000;
    let cached_mem = 0x8000_0000;
    let payload_offset = 0x2_0000;
    let payload_size = 0x1e_0000;
    let linuxboot_offset = 0x20_0000;
    let linuxboot_size = 0x120_0000;
    let dtb_offset = 0x140_0000;
    let dtb_size = 0xe000;

    // TODO; This payload structure should be loaded from boot medium rather
    // than hardcoded.
    let segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: cached_mem,
            data: &mut SectionReader::new(&Memory {}, mem + payload_offset, payload_size),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: cached_mem,
            data: &mut SectionReader::new(&Memory {}, mem + linuxboot_offset, linuxboot_size),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: cached_mem,
            data: &mut SectionReader::new(&Memory {}, mem + dtb_offset, dtb_size),
        },
    ];
    // TODO: Get this from configuration
    let use_sbi = true;
    if use_sbi {
        writeln!(
            w,
            "Handing over to SBI, will continue at 0x{:x}\r",
            mem + linuxboot_offset
        )
        .unwrap();
        sbi_init(mem + linuxboot_offset, mem + dtb_offset);
    } else {
        let entry = mem + payload_offset;
        let payload: payload::Payload = payload::Payload {
            typ: payload::ftype::CBFS_TYPE_RAW,
            compression: payload::ctype::CBFS_COMPRESS_NONE,
            offset: 0,
            entry,
            dtb: 0,
            // TODO: These two length fields are not used.
            rom_len: 0,
            mem_len: 0,
            segs,
        };
        // payload.load();
        // TODO: Write hart ID a0 and DTB phys address to a1 if using an SBI
        writeln!(w, "Running payload entry 0x{:x}\r", entry).unwrap();
        payload.run();
        writeln!(w, "Unexpected return from payload\r").unwrap();
    }
    arch::halt()
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let mut uart0 = Sunxi::new(UART0 as usize, 115200);
    let w = &mut print::WriteTo::new(&mut uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = writeln!(w, "PANIC: {}\r", info);
    arch::halt()
}
