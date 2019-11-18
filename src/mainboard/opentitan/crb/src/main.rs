#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

use core::panic::PanicInfo;
use core::fmt::Write;
use model::Driver;
use payloads::payload;
use print;
use uart::sifive::SiFive;
use wrappers::{Memory, SectionReader, SliceReader};

#[no_mangle]
pub extern "C" fn _start_boot_hart(_hart_id: usize, fdt_address: usize) -> ! {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    uart0.init();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    // TODO; This payload structure should be loaded from SPI rather than hardcoded.
    let mem = 0x80000000;
    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: mem,
            data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0x100000, 0x600000),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: fdt_address, /*mem + 10*1024*1024*/
            data: &mut SliceReader::new(&[0u8; 0]),
        },
    ];
    let mut payload: payload::Payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: 0,
        dtb: 0,
        // TODO: These two length fields are not used.
        rom_len: 0,
        mem_len: 0,
        segs: kernel_segs,
    };
    write!(w, "Loading payload\r\n").unwrap();
    payload.load();
    write!(w, "Running payload entry 0x{:x} dtb 0x{:x}\r\n", payload.entry, payload.dtb).unwrap();
    payload.run();

    write!(w, "Unexpected return from payload\r\n").unwrap();
    soc::halt()
}

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    soc::halt()
}
