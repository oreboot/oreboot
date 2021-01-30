#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use print;
use uart::amdmmio::AMDMMIO;
use uart::debug_port::DebugPort;
use uart::i8250::I8250;
mod mainboard;
use mainboard::MainBoard;
mod msr;
use msr::msrs;
mod c00;
use c00::c00;
mod acpi;
use acpi::setup_acpi_tables;
use x86_64::registers::model_specific::Msr;
extern crate heapless; // v0.4.x
use heapless::consts::*;
use heapless::Vec;
use wrappers::DoD;

use core::ptr;
// Until we are done hacking on this, use our private copy.
// Plan to copy it back later.
global_asm!(include_str!("bootblock.S"));
fn poke32(a: u32, v: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}
fn poke8(a: u32, v: u8) -> () {
    let y = a as *mut u8;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek8(a: u32) -> u8 {
    let y = a as *mut u8;
    unsafe { ptr::read_volatile(y) }
}

/// Write 32 bits to port
unsafe fn outl(port: u16, val: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{al}"(val));
}

/// Read 32 bits from port
unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    llvm_asm!("inl %dx, %eax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    return ret;
}
fn peek32(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}
// extern "C" {
//     fn run32(w: &mut impl core::fmt::Write, start_address: usize, dtb: usize);
// }

fn peek(a: u64) -> u64 {
    let y = a as *const u64;
    unsafe { ptr::read_volatile(y) }
}

fn peekb(a: u64) -> u8 {
    let y = a as *const u8;
    unsafe { ptr::read_volatile(y) }
}

// Returns a slice of u32 for each sequence of hex chars in a.
fn hex(a: &[u8], vals: &mut Vec<u64, U8>) -> () {
    let mut started: bool = false;
    let mut val: u64 = 0u64;
    for c in a.iter() {
        let v = *c;
        if v >= b'0' && v <= b'9' {
            started = true;
            val = val << 4;
            val = val + (*c - b'0') as u64;
        } else if v >= b'a' && v <= b'f' {
            started = true;
            val = (val << 4) | (*c - b'a' + 10) as u64;
        } else if v >= b'A' && v <= b'F' {
            started = true;
            val = (val << 4) | (*c - b'A' + 10) as u64;
        } else if started {
            vals.push(val).unwrap();
            val = 0;
        }
    }
}

fn mem(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) -> () {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);

    // I wish I knew rust. This code is shit.
    for a in vals.iter() {
        let m = peek(*a);
        write!(w, "{:x?}: {:x?}\r\n", *a, m).unwrap();
    }
}

fn ind(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) -> () {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);

    // I wish I knew rust. This code is shit.
    for a in vals.iter() {
        let m = unsafe { inl(*a as u16) };
        write!(w, "{:x?}: {:x?}\r\n", *a, m).unwrap();
    }
}

fn out(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) -> () {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);

    // I wish I knew rust. This code is shit.
    for i in 0..vals.len() / 2 {
        let a = vals[i * 2] as u16;
        let v = vals[i * 2 + 1] as u32;
        unsafe {
            outl(a, v);
        };
        write!(w, "{:x?}: {:x?}\r\n", a, v).unwrap();
    }
}

fn memb(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) -> () {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);
    write!(w, "dump bytes: {:x?}\r\n", vals).expect("Failed to write.");
    for a in vals.iter() {
        for i in 0..16 {
            let m = peekb(*a + i);
            write!(w, "{:x?}: {:x?}\r\n", *a + i, m).unwrap();
        }
    }
}

#[no_mangle]
pub extern "C" fn _asdebug(w: &mut impl core::fmt::Write, a: u64) -> () {
    write!(w, "here we are in asdebug\r\n").unwrap();
    write!(w, "stack is {:x?}\r\n", a).unwrap();
    consdebug(w);
    write!(w, "back to hell\r\n").unwrap();
}

fn consdebug(w: &mut impl core::fmt::Write) -> () {
    let mut done: bool = false;
    let newline: [u8; 2] = [10, 13];
    while done == false {
        let io = &mut IOPort;
        let uart0 = &mut I8250::new(0x3f8, 0, io);
        let mut line: Vec<u8, U16> = Vec::new();
        loop {
            let mut c: [u8; 1] = [12; 1];
            uart0.pread(&mut c, 1).unwrap();
            uart0.pwrite(&c, 1).unwrap();
            line.push(c[0]).unwrap();
            if c[0] == 13 || c[0] == 10 || c[0] == 4 {
                uart0.pwrite(&newline, 2).unwrap();
                break;
            }
            if line.len() > 15 {
                break;
            }
        }
        match line[0] {
            0 | 4 => {
                done = true;
            }
            b'm' => {
                mem(w, line);
            }
            b'i' => {
                ind(w, line);
            }
            b'o' => {
                out(w, line);
            }
            b'h' => {
                memb(w, line);
            }
            _ => {}
        }
    }
}
//global_asm!(include_str!("init.S"));

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let m = &mut MainBoard::new();
    m.init().unwrap();
    let io = &mut IOPort;
    let post = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    uart0.init().unwrap();
    let debug_io = &mut IOPort;
    let debug = &mut DebugPort::new(0x80, debug_io);
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    debug.init().unwrap();
    debug.pwrite(b"Welcome to oreboot - debug port 80\r\n", 0).unwrap();
    let p0 = &mut AMDMMIO::com2();
    p0.init().unwrap();
    p0.pwrite(b"Welcome to oreboot - com2\r\n", 0).unwrap();
    let s = &mut [debug as &mut dyn Driver, uart0 as &mut dyn Driver, p0 as &mut dyn Driver];
    let console = &mut DoD::new(s);

    for _i in 1..32 {
        console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    }
    let mut p: [u8; 1] = [0xf0; 1];
    post.pwrite(&p, 0x80).unwrap();
    let w = &mut print::WriteTo::new(console);

    // It is hard to say if we need to do this.
    if true {
        let v = unsafe { Msr::new(0xc001_1004).read() };
        write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
    if true {
        let v = unsafe { Msr::new(0xc001_1005).read() };
        write!(w, "c001_1005 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
    unsafe {
        write!(w, "0x1b is {:x} \r\n", Msr::new(0x1b).read()).unwrap();
    }
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
    if true {
        msrs(w);
    }
    p[0] = p[0] + 1;
    write!(w, "Write acpi tables\r\n").unwrap();
    setup_acpi_tables(w, 0xf0000, 1);
    write!(w, "Wrote bios tables, entering debug\r\n").unwrap();
    consdebug(w);
    if false {
        msrs(w);
    }
    c00(w);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    poke32(0xfee000d0, 0x1000000);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    write!(w, "loading payload with fdt_address {}\r\n", fdt_address).unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    payload.load(w).unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    write!(w, "Back from loading payload, call debug\r\n").unwrap();
    consdebug(w);
    write!(w, "Running payload entry is {:x}\r\n", payload.entry).unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    payload.run(w);
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;

    write!(w, "Unexpected return from payload\r\n").unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
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
