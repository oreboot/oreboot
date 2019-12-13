#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use payloads::payload;
use print;
use uart::opentitan::OpenTitanUART;
use wrappers::{Memory, SectionReader, SliceReader};

#[no_mangle]
pub extern "C" fn _start_boot_hart(_hart_id: usize, fdt_address: usize) -> ! {
    // Set up the pinmux. This is highly board dependent.
    // TODO: pinmux device.
    let m = &mut Memory;

    m.pwrite(&[0xc2 as u8, 0x40 as u8, 0x14 as u8, 0x06 as u8, ], 0x40070004).unwrap();
    m.pwrite(&[0x07 as u8, 0x92 as u8, 0x28 as u8, 0x0b as u8, ], 0x40070008).unwrap();
    m.pwrite(&[0x4c as u8, 0xe3 as u8, 0x3c as u8, 0x10 as u8, ], 0x4007000c).unwrap();
    m.pwrite(&[0x91 as u8, 0x34 as u8, 0x51 as u8, 0x15 as u8, ], 0x40070010).unwrap();
    m.pwrite(&[0xd6 as u8, 0x85 as u8, 0x65 as u8, 0x1a as u8, ], 0x40070014).unwrap();
    m.pwrite(&[0x1b as u8, 0xd7 as u8, 0x79 as u8, 0x1f as u8, ], 0x40070018).unwrap();
    m.pwrite(&[0x60 as u8, 0x08 as u8, 0x00 as u8, 0x00 as u8, ], 0x4007001c).unwrap();
    m.pwrite(&[0xc2 as u8, 0x40 as u8, 0x14 as u8, 0x06 as u8, ], 0x40070020).unwrap();
    m.pwrite(&[0x07 as u8, 0x92 as u8, 0x28 as u8, 0x0b as u8, ], 0x40070024).unwrap();
    m.pwrite(&[0x4c as u8, 0xe3 as u8, 0x3c as u8, 0x10 as u8, ], 0x40070028).unwrap();
    m.pwrite(&[0x91 as u8, 0x34 as u8, 0x51 as u8, 0x15 as u8, ], 0x4007002c).unwrap();
    m.pwrite(&[0xd6 as u8, 0x85 as u8, 0x65 as u8, 0x1a as u8, ], 0x40070030).unwrap();
    m.pwrite(&[0x1b as u8, 0xd7 as u8, 0x79 as u8, 0x1f as u8, ], 0x40070034).unwrap();
    m.pwrite(&[0x60 as u8, 0x08 as u8, 0x00 as u8, 0x00 as u8, ], 0x40070038).unwrap();
    let uart0 = &mut OpenTitanUART::new(0x40000000, 115200);
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
pub extern "C" fn abort() {
    panic!("abort!");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut OpenTitanUART::new(0x40000000, 115200);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    soc::halt()
}
