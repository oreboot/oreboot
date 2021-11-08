//#![feature(asm)]
//#![feature(lang_items, start)]
#![no_std]
//#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use model::Driver;
use uart::i8250::I8250;
mod acpi;
use acpi::setup_acpi_tables;
extern crate heapless; // v0.4.x
use heapless::consts::*;
use heapless::Vec;
mod interrupts;

use core::ptr;
global_asm!(include_str!("bootblock.S"), options(att_syntax));

fn poke32(a: u32, v: u32) {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

/// Write 32 bits to port
unsafe fn outl(port: u16, val: u32) {
    asm!("outl %eax, %dx", in("eax") val, in("dx") port, options(att_syntax));
}

/// Read 32 bits from port
unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    asm!("inl %dx, %eax", in("dx") port, out("eax") ret, options(att_syntax));
    ret
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
fn hex(a: &[u8], vals: &mut Vec<u64, U8>) {
    let mut started: bool = false;
    let mut val: u64 = 0u64;
    for c in a.iter() {
        let v = *c;
        if (b'0'..=b'9').contains(&v) {
            started = true;
            val <<= 4;
            val += (*c - b'0') as u64;
        } else if (b'a'..=b'f').contains(&v) {
            started = true;
            val = (val << 4) | (*c - b'a' + 10) as u64;
        } else if (b'A'..=b'F').contains(&v) {
            started = true;
            val = (val << 4) | (*c - b'A' + 10) as u64;
        } else if started {
            vals.push(val).unwrap();
            val = 0;
        }
    }
}

fn mem(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);

    // I wish I knew rust. This code is shit.
    for a in vals.iter() {
        let m = peek(*a);
        write!(w, "{:x?}: {:x?}\r\n", *a, m).unwrap();
    }
}

fn ind(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);

    // I wish I knew rust. This code is shit.
    for a in vals.iter() {
        let m = unsafe { inl(*a as u16) };
        write!(w, "{:x?}: {:x?}\r\n", *a, m).unwrap();
    }
}

fn out(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) {
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

fn memb(w: &mut impl core::fmt::Write, a: Vec<u8, U16>) {
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
#[allow(no_mangle_generic_items)]
pub extern "C" fn _asdebug(w: &mut impl core::fmt::Write, a: u64) {
    write!(w, "here we are in asdebug\r\n").unwrap();
    write!(w, "stack is {:x?}\r\n", a).unwrap();
    consdebug(w);
    write!(w, "back to hell\r\n").unwrap();
}

fn consdebug(w: &mut impl core::fmt::Write) {
    let mut done: bool = false;
    let newline: [u8; 2] = [10, 13];
    while !done {
        let uart0 = &mut I8250::new(0x3f8, 0, IOPort {});
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

pub fn boot(w: &mut impl core::fmt::Write, fdt_address: usize) {
    interrupts::init_pics();
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

    write!(w, "Write acpi tables\r\n").unwrap();
    setup_acpi_tables(w, 0xf_0000); // Note: Linux needs to be compiled with CONFIG_ACPI_LEGACY_TABLES_LOOKUP enabled for that address to be used.
    if false {
        write!(w, "Wrote bios tables, entering debug\r\n").unwrap();
        consdebug(w);
    }
    poke32(0xfee000d0, 0x1000000); // APICx0D0 [Logical Destination] (Core::X86::Apic::LocalDestination) 55803_B0_PUB_0_91.pdf
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    write!(w, "loading payload with fdt_address {}\r\n", fdt_address).unwrap();
    payload.load(w).unwrap();
    if false {
        write!(w, "Back from loading payload, call debug\r\n").unwrap();
        consdebug(w);
    }
    write!(w, "Running payload entry is {:x}\r\n", payload.entry).unwrap();
    payload.run(w);

    write!(w, "Unexpected return from payload\r\n").unwrap();
}
