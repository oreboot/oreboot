#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use payloads::payload;
use print;
use uart::ns16550::NS16550;
use wrappers::{Memory, SectionReader, SliceReader};

global_asm!(include_str!("../../../../arch/x86/x86_64/src/bootblock.S"));
//global_asm!(include_str!("init.S"));

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let uart0 = &mut NS16550::new(0x10000000, 115200);
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: 0x80000000,
            data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0x400000, 6 * 1024 * 1024),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: fdt_address,
            data: &mut SliceReader::new(&[0u8; 0]),
        },
    ];
    let payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: 0x80000000 as usize,
        rom_len: 0 as usize,
        mem_len: 0 as usize,
        segs: kernel_segs,
        dtb: 0,
    };

    write!(w, "Running payload\r\n").unwrap();
    payload.run();

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut NS16550::new(0x10000000, 115200);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
