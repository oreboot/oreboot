#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use core::fmt::Write;
use device_tree::print_fdt;
use model::Driver;
use payloads::payload;
use print;
use uart::sifive::SiFive;
use wrappers::{Memory, SectionReader, SliceReader};

global_asm!(include_str!("../../../../../src/arch/riscv/rv64/src/bootblock.S"));
global_asm!(include_str!("../../../../../src/soc/sifive/fu540/src/init.S"));

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    uart0.init();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    write!(w, "## Oreboot Fixed Device Tree\r\n").unwrap();
    // Fixed DTFS is at offset 512KiB in flash. Max size 512Kib.
    let fixed_fdt = &mut SectionReader::new(&Memory {}, 0x20000000 + 512 * 1024, 512 * 1024);
    if let Err(err) = print_fdt(fixed_fdt, w) {
        write!(w, "error: {}\n", err).unwrap();
    }

    let mem = 0x80000000;

    // TODO; This payload structure should be loaded from SPI rather than hardcoded.
    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: mem,
            data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0x400000, 6 * 1024 * 1024),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: fdt_address, /*mem + 10*1024*1024*/
            data: &mut SliceReader::new(&[0u8; 0]),
        },
    ];
    let simple_segs = &[payload::Segment {
        typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
        base: mem,
        data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0xa00000, 1024),
    }];
    let payload: payload::Payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        load_addr: mem as u64,
        // TODO: These two length fields are not used.
        rom_len: 0 as u32,
        mem_len: 0 as u32,

        segs: kernel_segs,
    };
    write!(w, "Loading payload\r\n").unwrap();
    payload.load();
    write!(w, "Running payload\r\n").unwrap();
    payload.run();

    write!(w, "Unexpected return from payload\r\n").unwrap();
    architecture::halt()
}
