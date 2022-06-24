#![no_std]
#![no_main]
#![feature(default_alloc_error_handler, ptr_metadata, allocator_api)]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::{arch::global_asm, ptr::read_volatile, slice};
use oreboot_arch::riscv64 as arch;
use oreboot_drivers::{
    uart::ns16550::NS16550,
    wrappers::{Memory, SectionReader, SliceReader},
    Driver,
};
use payloads::payload;
use sbi_qemu::sbi_init;

// use lz4_flex::{compress_prepend_size, decompress_size_prepended};

global_asm!(include_str!("bootblock.S"));
global_asm!(include_str!("init.S"));

const EI: usize = 12;
type MyLzss = lzss::Lzss<EI, 4, 0x00, { 1 << EI }, { 2 << EI }>;

const BASE: usize = 0x8000_0000;
const FLASH_BASE: usize = 0x2000_0000;
const PAYLOAD_ADDR: usize = BASE + 0x0020_0000;
const PAYLOAD_SIZE: usize = 0x0120_0000; // 12 MB
const DTB_ADDR: usize = BASE + 0x0140_0000;
const DTB_SIZE: usize = 0x2000;

const COMPRESSED_SIZE: usize = 8207247; // 0x40_0000;
const PAYLOAD_OFFS: usize = 0x0200_0000;
const PAYLOAD_TMP_ADDR: usize = BASE + PAYLOAD_OFFS;
const DTB_TMP_ADDR: usize = BASE + PAYLOAD_OFFS + PAYLOAD_SIZE;

fn load_kernel(w: &mut print::WriteTo<NS16550>, fdt_address: usize) {
    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: BASE + PAYLOAD_OFFS,
            data: &mut SectionReader::new(&Memory {}, PAYLOAD_ADDR, PAYLOAD_SIZE),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: BASE + PAYLOAD_OFFS + PAYLOAD_SIZE,
            data: &mut SectionReader::new(&Memory {}, DTB_ADDR, DTB_SIZE),
            // data: &mut SliceReader::new(&[0u8; 0]),
        },
    ];
    let mut payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: PAYLOAD_TMP_ADDR,
        rom_len: 0,
        mem_len: 0,
        segs: kernel_segs,
        dtb: 0,
    };
    payload.load();
    let r = unsafe { read_volatile(PAYLOAD_TMP_ADDR as *mut u32) };
    writeln!(w, "Payload at tmp dest: {:08x}\r", r).unwrap();

    let r = unsafe { read_volatile(DTB_ADDR as *mut u32) };
    if r != 0xedfe0dd0 {
        writeln!(w, "DTB source looks wrong: {:08x}\r", r).unwrap();
    } else {
        writeln!(w, "DTB source looks fine, yay!\r").unwrap();
    }
    let r = unsafe { read_volatile(DTB_TMP_ADDR as *mut u32) };
    writeln!(w, "DTB at dest: {:08x}\r", r).unwrap();

    let in_ptr = PAYLOAD_TMP_ADDR as *const u8;
    let out_ptr = PAYLOAD_ADDR as *mut u8;
    writeln!(
        w,
        "Decompress {} bytes from {:?} to {:?}\r",
        COMPRESSED_SIZE, &in_ptr, &out_ptr
    )
    .unwrap();

    let input = unsafe { slice::from_raw_parts(in_ptr, COMPRESSED_SIZE) };
    let mut output = unsafe { slice::from_raw_parts_mut(out_ptr, PAYLOAD_SIZE) };

    let result = MyLzss::decompress(
        lzss::SliceReader::new(&input),
        lzss::SliceWriter::new(&mut output),
    );
    match result {
        Ok(r) => {
            writeln!(w, "Success, decompressed {r} bytes :)\r").unwrap();
            let r = unsafe { read_volatile(PAYLOAD_ADDR as *mut u32) };
            if r != 0x0000aa21 {
                writeln!(w, "Payload does not look like Linux Image!\r").unwrap();
            } else {
                writeln!(w, "Payload looks like Linux Image, yay!\r").unwrap();
            }
        }
        Err(e) => writeln!(w, "Decompression error {e}\r").unwrap(),
    }
    // writeln!(w, "result {:?}\r", output).unwrap();
}

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let uart0 = &mut NS16550::new(0x10000000, 115200);
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    load_kernel(w, fdt_address);

    // NOTE: Instead of `payload.run()`, we call `sbi_init(...)`.
    // TODO: Should we hoist SBI to `payload` somehow?
    // writeln!(w, "Running payload\r").unwrap();
    // payload.run();

    writeln!(
        w,
        "Handing over to SBI, will continue at 0x{:x}\r",
        PAYLOAD_ADDR
    )
    .unwrap();
    sbi_init(PAYLOAD_ADDR, DTB_ADDR);

    // NOTE: From SBI, we never return here. However, it is specific to RISC-V.
    // TODO: Should we rework SBI? Make it part of the framework?
    // writeln!(w, "Unexpected return from payload\r").unwrap();
    // arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut NS16550::new(0x1000_0000, 115200);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
