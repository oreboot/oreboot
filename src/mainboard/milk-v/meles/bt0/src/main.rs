#![feature(naked_functions, asm_const)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

#[macro_use]
extern crate log;
use core::{
    arch::asm,
    intrinsics::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut}
    ,
};

use embedded_hal_nb::serial::Write;
use riscv::register::{marchid, mhartid, mimpid, mvendorid};

use dwc3::dwc3_gadget_run;
use util::{read32, write32};


mod dram;
mod dram_helpers;
mod dram_train;
mod dram_training_data;
mod dwc3;
mod uart;
mod util;

pub type EntryPoint = unsafe extern "C" fn();

const SRAM0_BASE: usize = 0xFF_E000_0000;
const SRAM0_SIZE: usize = 0x00_0018_0000;

const USB0_BASE: usize = 0xFF_E704_0000;
const USB0_APB_BASE: usize = 0xFF_EC03_0000;
const USB0_IOPMP_BASE: usize = 0xFF_FC02_E000;

const QSPI0_BASE: usize = 0xFF_EA00_0000;
const QSPI0_SIZE: usize = 0x00_0200_0000;

const DRAM_BASE: usize = 0x0000_0000;

const BROM_BASE: usize = 0xFF_FFD0_0000;
// One sweet megabyte mask ROM
const BROM_SIZE: usize = 0x00_0010_0000;

const BOOT_HART_ID: usize = 0;

const STACK_SIZE: usize = 8 * 1024;

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Set up stack and jump to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        "auipc  s4, 0",

        "csrw   mstatus, zero",
        "csrw   mie, zero",
        "ld     t0, {start}",
        "csrw   mtvec, t0",
        // 1. suspend non-boot hart
        "li     t1, {boothart}",
        "csrr   t0, mhartid",
        "bne    t0, t1, .nonboothart",
        // 2. prepare stack
        // NOTE: non-boot harts need no stack here, they skip this
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j      .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        // "csrw   mie, (1 << 3)",
        "wfi",
        "call   {payload}",
        ".boothart:",

        "call   {reset}",
        boothart   = const BOOT_HART_ID,
        stack      = sym BT0_STACK,
        stack_size = const STACK_SIZE,
        payload    = sym exec_payload,
        reset      = sym reset,
        start      = sym start,
        options(noreturn)
    )
}

/// Initialize RAM: Clear BSS and set up data.
/// See https://docs.rust-embedded.org/embedonomicon/main.html
///
/// # Safety
/// :shrug:
#[no_mangle]
pub unsafe extern "C" fn reset() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let bss_size = addr_of!(_ebss) as usize - addr_of!(_sbss) as usize;
    ptr::write_bytes(addr_of_mut!(_sbss), 0, bss_size);

    let data_size = addr_of!(_edata) as usize - addr_of!(_sdata) as usize;
    ptr::copy_nonoverlapping(addr_of!(_sidata), addr_of_mut!(_sdata), data_size);
    // Call user entry point
    main();
}

fn vendorid_to_name<'a>(vendorid: usize) -> &'a str {
    match vendorid {
        0x0489 => "SiFive",
        0x05b7 => "T-Head",
        0x0710 => "SpacemiT",
        _ => "unknown",
    }
}

// FIXME: This really depends on the vendor first!
fn impid_to_name<'a>(impid: usize) -> &'a str {
    match impid {
        0x0000_0000_0000_0000 => "C910 or something",
        0x0000_0000_0421_0427 => "21G1.02.00 / llama.02.00-general",
        0x1000_0000_4977_2200 => "X60",
        _ => "unknown",
    }
}

/// Print RISC-V core information:
/// - vendor
/// - arch
/// - implementation
/// - hart ID
fn print_ids() {
    let vid = mvendorid::read().map(|r| r.bits()).unwrap_or(0);
    let aid = marchid::read().map(|r| r.bits()).unwrap_or(0);
    let iid = mimpid::read().map(|r| r.bits()).unwrap_or(0);
    // TODO: This prints 8000000000000007, but should be 80000007.
    // See U74-MC core complex manual 21G3.
    println!("RISC-V arch {aid:08x}");
    let vendor_name = vendorid_to_name(vid);
    println!("RISC-V core vendor: {vendor_name} (0x{vid:04x})");
    let imp_name = impid_to_name(iid);
    println!("RISC-V implementation: {imp_name} (0x{iid:08x})");
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {hart_id}");
}

static mut SERIAL: Option<uart::TH1520Serial> = None;

fn init_logger(s: uart::TH1520Serial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
}

fn copy(source: usize, target: usize, size: usize) {
    for b in (0..size).step_by(4) {
        write32(target + b, read32(source + b));
        if b % 0x4_0000 == 0 {
            print!(".");
        }
    }
    println!(" done.");
}

fn dram_test() {
    let limit = 0x8000_0000;
    let range = DRAM_BASE..limit;
    let steps = 0x1000;

    println!("DRAM test: write patterns...");

    for i in range.clone().step_by(steps) {
        write32(i + 0x0, 0x2233_ccee | i as u32);
        write32(i + 0x4, 0x5577_aadd | i as u32);
        write32(i + 0x8, 0x1144_bbff | i as u32);
        write32(i + 0xc, 0x6688_9900 | i as u32);
    }

    println!("DRAM test: reading back...");

    for i in range.clone().step_by(steps) {
        let v = read32(i + 0x0);
        let e = 0x2233_ccee | i as u32;
        if v != e {
            println!("Error: {i:08x} != {e:08x}, got {v:08x}");
        }
        let v = read32(i + 0x4);
        let e = 0x5577_aadd | i as u32;
        if v != e {
            println!("Error: {i:08x} != {e:08x}, got {v:08x}");
        }
        let v = read32(i + 0x8);
        let e = 0x1144_bbff | i as u32;
        if v != e {
            println!("Error: {i:08x} != {e:08x}, got {v:08x}");
        }
        let v = read32(i + 0xc);
        let e = 0x6688_9900 | i as u32;
        if v != e {
            println!("Error: {i:08x} != {e:08x}, got {v:08x}");
        }
    }

    println!("DRAM test: done :)");
}

#[no_mangle]
fn main() {
    let mut ini_pc: usize = 0;
    unsafe { asm!("mv {}, s4", out(reg) ini_pc) };

    let s = uart::TH1520Serial::noinit();
    init_logger(s);
    println!("oreboot 🦀 bt0");

    println!("initial PC {ini_pc:016x}");

    print_ids();

    // Can we get this to work? :-)
    dwc3_gadget_run(USB0_BASE);

    // util::dump_block(BROM_BASE, 0x1000, 32); // only gets 0000000...
    // util::dump_block(QSPI0_BASE, 0x100, 32); // hangs after 96 bytes

    dram::init();

    // dram_test();

    unsafe {
        asm!("wfi");
    }

    // GO!
    let load_addr = DRAM_BASE;
    println!("[bt0] Jump to main stage @{load_addr:08x}");
    exec_payload(load_addr);
    println!("[bt0] Exit from main stage, resetting...");
    unsafe {
        // udelay(0x0100_0000);
        reset();
        riscv::asm::wfi()
    };
}

// jump to main stage or payload
fn exec_payload(addr: usize) {
    unsafe {
        let f: EntryPoint = transmute(addr);
        asm!("fence.i");
        f();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[bt0] panic in '{}' line {}",
            location.file(),
            location.line(),
        );
    } else {
        println!("[bt0] panic at unknown location");
    };
    if let Some(msg) = info.message() {
        println!("[bt0]   {msg}");
    }
    loop {
        core::hint::spin_loop();
    }
}
