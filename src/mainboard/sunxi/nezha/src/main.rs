#![no_std]
#![no_main]
#![feature(default_alloc_error_handler, ptr_metadata, allocator_api)]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::{arch::global_asm, ptr::read_volatile, slice};
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

const MEM: usize = 0x4000_0000;
const CACHED_MEM: usize = 0x8000_0000;

// see ../fixed-dtfs.dts
const PAYLOAD_OFFSET: usize = 0x2_0000;
const PAYLOAD_SIZE: usize = 0x1e_0000;

// compressed image
const LINUXBOOT_TMP_OFFSET: usize = 0x0400_0000;
const LINUXBOOT_TMP_ADDR: usize = MEM + LINUXBOOT_TMP_OFFSET;
// FIXME: Needs to be hardcoded as of now; make this part of the build process!
const LINUXBOOT_COMPRESSED_SIZE: usize = 7258470; // 0x00c0_0000;

// target location for decompressed image
const LINUXBOOT_OFFSET: usize = 0x0020_0000;
const LINUXBOOT_ADDR: usize = MEM + LINUXBOOT_OFFSET;
const LINUXBOOT_SIZE: usize = 0x0200_0000;
const DTB_OFFSET: usize = 0x0140_0000;
const DTB_ADDR: usize = MEM + DTB_OFFSET;

const EI: usize = 12;
type MyLzss = lzss::Lzss<EI, 4, 0x00, { 1 << EI }, { 2 << EI }>;

fn decompress(w: &mut print::WriteTo<DoD>) {
    let r = unsafe { read_volatile(LINUXBOOT_TMP_ADDR as *mut u32) };
    writeln!(w, "Payload at tmp dest: {:08x}\r", r).unwrap();
    // check for Device Tree header, d00dfeed
    let r = unsafe { read_volatile(DTB_ADDR as *mut u32) };
    if r != 0xedfe0dd0 {
        writeln!(w, "DTB looks wrong: {:08x}\r", r).unwrap();
    } else {
        writeln!(w, "DTB looks fine, yay!\r").unwrap();
    }

    let in_ptr = LINUXBOOT_TMP_ADDR as *const u8;
    let out_ptr = LINUXBOOT_ADDR as *mut u8;
    writeln!(
        w,
        "Decompress {} bytes from {:?} to {:?}\r",
        LINUXBOOT_COMPRESSED_SIZE, &in_ptr, &out_ptr
    )
    .unwrap();

    let input = unsafe { slice::from_raw_parts(in_ptr, LINUXBOOT_COMPRESSED_SIZE) };
    let output = unsafe { slice::from_raw_parts_mut(out_ptr, LINUXBOOT_SIZE) };

    let result = MyLzss::decompress(
        lzss::SliceReader::new(input),
        lzss::SliceWriter::new(output),
    );
    match result {
        Ok(r) => {
            writeln!(w, "Success, decompressed {r} bytes :)\r").unwrap();
            let r = unsafe { read_volatile(LINUXBOOT_ADDR as *mut u32) };
            if r != 0x0000aa21 {
                writeln!(w, "Payload does not look like Linux Image: {:x}\r", r).unwrap();
            } else {
                writeln!(w, "Payload looks like Linux Image, yay!\r").unwrap();
            }
        }
        Err(e) => writeln!(w, "Decompression error {e}\r").unwrap(),
    }
}

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

    // TODO: Get this from configuration
    let use_sbi = true;
    if use_sbi {
        decompress(w);
        writeln!(
            w,
            "Handing over to SBI, will continue at 0x{:x}\r",
            LINUXBOOT_ADDR
        )
        .unwrap();
        sbi_init(LINUXBOOT_ADDR, DTB_ADDR);
    } else {
        // TODO; This payload structure should be loaded from boot medium rather
        // than hardcoded.
        let entry = MEM + PAYLOAD_OFFSET;
        let segs = &[payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: CACHED_MEM,
            data: &mut SectionReader::new(&Memory {}, entry, PAYLOAD_SIZE),
        }];
        let mut payload: payload::Payload = payload::Payload {
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
        payload.load();
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
