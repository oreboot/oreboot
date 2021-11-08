//#![feature(llvm_asm)]
//#![feature(lang_items, start)]
#![no_std]
#![no_main]
//#![feature(global_asm)]

use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use payloads::payload;
use soc::ccu::CCU;
use soc::gpio::GPIO;
use uart::sunxi::Sunxi;
use uart::sunxi::UART0;
use wrappers::{DoD, Memory, SectionReader};

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
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let mut uarts = [&mut uart0 as &mut dyn Driver];
    let console = &mut DoD::new(&mut uarts[..]);
    console.init().unwrap();
    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(console);
    writeln!(w, "## Loading payload\r").unwrap();

    // see ../fixed-dtfs.dts
    let mem = 0x40000000;
    let payload_offset = 0x100000;
    let payload_size = 0x120000;

    // TODO; This payload structure should be loaded from SPI rather than hardcoded.
    let kernel_segs = &[payload::Segment {
        typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
        base: mem,
        data: &mut SectionReader::new(&Memory {}, mem + payload_offset, payload_size),
    }];
    let payload: payload::Payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: mem + payload_offset, // TODO: 0 when DRAM driver is implemented
        dtb: 0,
        // TODO: These two length fields are not used.
        rom_len: 0,
        mem_len: 0,
        segs: kernel_segs,
    };
    // TODO: uncomment when driver is implemented
    // payload.load();
    writeln!(w, "Running payload entry 0x{:x}\r", payload.entry).unwrap();
    payload.run();
    writeln!(w, "Unexpected return from payload\r").unwrap();
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
