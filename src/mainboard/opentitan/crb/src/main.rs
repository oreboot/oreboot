//#![feature(asm)]
//#![feature(lang_items, start)]
#![no_std]
#![no_main]
//#![feature(global_asm)]
//#![deny(warnings)]

use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use payloads::payload;
use uart::opentitan::OpenTitanUART;
use wrappers::{Memory, SectionReader, SliceReader};

const BAUDRATE: u32 = 230400;

#[no_mangle]
pub extern "C" fn _start_boot_hart(_hart_id: usize, _fdt_address: usize) -> ! {
    // Set up the pinmux. This is highly board dependent.
    // TODO: pinmux device.
    let m = &mut Memory;

    // We ignore errors instead of panicing (using unwrap), because
    // it doubles the size of the final binary.
    let _ = m.pwrite(&[0xc2u8, 0x40u8, 0x14u8, 0x06u8], 0x40070004);
    let _ = m.pwrite(&[0x07u8, 0x92u8, 0x28u8, 0x0bu8], 0x40070008);
    let _ = m.pwrite(&[0x4cu8, 0xe3u8, 0x3cu8, 0x10u8], 0x4007000c);
    let _ = m.pwrite(&[0x91u8, 0x34u8, 0x51u8, 0x15u8], 0x40070010);
    let _ = m.pwrite(&[0xd6u8, 0x85u8, 0x65u8, 0x1au8], 0x40070014);
    let _ = m.pwrite(&[0x1bu8, 0xd7u8, 0x79u8, 0x1fu8], 0x40070018);
    let _ = m.pwrite(&[0x60u8, 0x08u8, 0x00u8, 0x00u8], 0x4007001c);
    let _ = m.pwrite(&[0xc2u8, 0x40u8, 0x14u8, 0x06u8], 0x40070020);
    let _ = m.pwrite(&[0x07u8, 0x92u8, 0x28u8, 0x0bu8], 0x40070024);
    let _ = m.pwrite(&[0x4cu8, 0xe3u8, 0x3cu8, 0x10u8], 0x40070028);
    let _ = m.pwrite(&[0x91u8, 0x34u8, 0x51u8, 0x15u8], 0x4007002c);
    let _ = m.pwrite(&[0xd6u8, 0x85u8, 0x65u8, 0x1au8], 0x40070030);
    let _ = m.pwrite(&[0x1bu8, 0xd7u8, 0x79u8, 0x1fu8], 0x40070034);
    let _ = m.pwrite(&[0x60u8, 0x08u8, 0x00u8, 0x00u8], 0x40070038);
    let uart0 = &mut OpenTitanUART::new(0x40000000, BAUDRATE);
    let _ = uart0.init();
    let _ = uart0.pwrite(b"Welcome to oreboot\r\n", 0);

    let w = &mut print::WriteTo::new(uart0);

    // TODO; This payload structure should be loaded from SPI rather than hardcoded.
    let mem = 0x10000000;
    let flash = 0x20000000;
    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: mem,
            data: &mut SliceReader::new(&[0x73u8, 0x00u8, 0x50u8, 0x10u8 /*0x10500073 wfi */]),
            //data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0x10000, 0x20),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: flash + 0x10000, //fdt_address, /*mem + 10*1024*1024*/
            data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0xf000, 0x10),
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
    let _ = write!(w, "Loading payload\r\n");
    payload.load();
    let _ = write!(
        w,
        "Running payload entry 0x{:x} dtb 0x{:x}\r\n",
        payload.entry, payload.dtb
    );
    soc::halt()
    //payload.run();

    //write!(w, "Unexpected return from payload\r\n").unwrap();
    //soc::halt()
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort!");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut OpenTitanUART::new(0x40000000, BAUDRATE);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    soc::halt()
}
