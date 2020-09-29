#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr;
use model::Driver;
use payloads::payload;
use print;
use uart::i8250::I8250;
mod mainboard;
use mainboard::MainBoard;
mod msr;
use msr::msrs;
use x86_64::instructions::rdmsr;
extern crate heapless; // v0.4.x
use heapless::consts::*;
use heapless::Vec;

// Until we are done hacking on this, use our private copy.
// Plan to copy it back later.
global_asm!(include_str!("bootblock.S"));

//global_asm!(include_str!("init.S"));
fn poke(v: u32, a: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let io = &mut IOPort;
    let post = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    uart0.init().unwrap();

    let mut count: u8 = 0;
    for _i in 0..1000000 {
        let mut p: [u8; 1] = [0; 1];
        for _j in 0..100000 {
            post.pread(&mut p, 0x3f8).unwrap();
        }
        count = count + 1;
        p[0] = count;
        post.pwrite(&p, 0x80).unwrap();

        uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    }

    let w = &mut print::WriteTo::new(uart0);

    msrs(w);
    // It is hard to say if we need to do this.
    if true {
        let v = rdmsr(0xc001_1004);
        write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
    if true {
        let v = rdmsr(0xc001_1005);
        write!(w, "c001_1005 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
        write!(w, "0x1b is {:x} \r\n", rdmsr(0x1b)).unwrap();
    p[0] = p[0] + 1;
    let payload = &mut BzImage {
        low_mem_size: 0x80000000,
        high_mem_start: 0x100000000,
        high_mem_size: 0,
        // TODO: get this from the FDT.
        rom_base: 0xffc00000,
        rom_size: 0x300000,
        load: 0x01000000,
        entry: 0x1000200,
    };
    p[0] = p[0] + 1;
    write!(w, "Write bios tables\r\n").unwrap();
    setup_bios_tables(w, 0xf0000, 1);
    write!(w, "Wrote bios tables, entering debug\r\n").unwrap();
    debug(w);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    poke32(0xfee000d0, 0x1000000);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    write!(w, "loading payload with fdt_address {}\r\n", fdt_address).unwrap();
    payload.load(w);
    if false {
        poke(0xfe, 0x100000);
    }
    write!(w, "Running payload\r\n").unwrap();
    payload.run(w);

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let io = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
