#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use oreboot_arch::riscv64 as arch;
use oreboot_drivers::{
    uart::ns16550::NS16550,
    wrappers::{Memory, SectionReader, SliceReader},
    Driver,
};
use payloads::payload;
use sbi_qemu::sbi_init;

global_asm!(include_str!("bootblock.S"));
global_asm!(include_str!("init.S"));

const BASE: usize = 0x8000_0000;
const FLASH_BASE: usize = 0x2000_0000;
const PAYLOAD_ADDR: usize = BASE + 0x0020_0000;
const PAYLOAD_SIZE: usize = 12 * 1024 * 1024; // 0x0060_0000
const PAYLOAD_OFFS: usize = 0x0200_0000;
const DTB_ADDR: usize = BASE + 0x0140_0000;

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
            data: &mut SectionReader::new(&Memory {}, DTB_ADDR, 0x2000),
            // data: &mut SliceReader::new(&[0u8; 0]),
        },
    ];
    let mut payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: BASE + PAYLOAD_OFFS,
        rom_len: 0,
        mem_len: 0,
        segs: kernel_segs,
        dtb: 0,
    };
    let r = unsafe { core::ptr::read_volatile((BASE + PAYLOAD_OFFS) as *mut u32) };
    writeln!(w, "Before: {:x}\r", r).unwrap();
    payload.load();
    let r = unsafe { core::ptr::read_volatile((BASE + PAYLOAD_OFFS) as *mut u32) };
    writeln!(w, "After:  {:x}\r", r).unwrap();
    if r != 0x0000aa21 {
        writeln!(w, "Payload does not look like Linux Image!\r").unwrap();
    } else {
        writeln!(w, "Payload looks like Linux Image, yay!\r").unwrap();
    }

    let r = unsafe { core::ptr::read_volatile(DTB_ADDR as *mut u32) };
    if r != 0xedfe0dd0 {
        writeln!(w, "DTB source looks wrong: {:x}\r", r).unwrap();
    } else {
        writeln!(w, "DTB source looks fine, yay!\r").unwrap();
    }
    let r = unsafe { core::ptr::read_volatile((BASE + PAYLOAD_OFFS + PAYLOAD_SIZE) as *mut u32) };
    writeln!(w, "DTB at dest:  {:x}\r", r).unwrap();
}

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let uart0 = &mut NS16550::new(0x10000000, 115200);
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    load_kernel(w, fdt_address);

    writeln!(w, "Running payload\r").unwrap();
    // payload.run();

    writeln!(
        w,
        "Handing over to SBI, will continue at 0x{:x}\r",
        BASE + PAYLOAD_OFFS
    )
    .unwrap();
    // sbi_init(BASE + PAYLOAD_OFFS, BASE + PAYLOAD_OFFS + PAYLOAD_SIZE);
    sbi_init(PAYLOAD_ADDR, DTB_ADDR);

    /*
    writeln!(w, "Unexpected return from payload\r").unwrap();
    arch::halt()
    */
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
