#![feature(naked_functions, asm_const)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

use embedded_hal_nb::serial::Write;

#[macro_use]
extern crate log;

use core::{
    arch::asm,
    intrinsics::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
    slice::from_raw_parts as slice_from,
};
use riscv::register::{marchid, mimpid, mvendorid};
use riscv::register::{mhartid, mip};

mod dram;
mod uart;
mod util;

use uart::K1XSerial;
use util::{read32, write32};

pub type EntryPoint = unsafe extern "C" fn();

const SRAM0_BASE: usize = 0x0020_0000;
const SRAM0_SIZE: usize = 0x0002_0000;

// TOOD
const DRAM_BASE: usize = 0x4000_0000;

// octacore, 2 clusters of 4x X60
const BOOT_HART_ID: usize = 0;

const STACK_SIZE: usize = 8 * 1024;

const STORAGE_API_P_ADDR: usize = 0xC083_8498;
const USB_BOOT_ENTRY: usize = 0xc083_81a0;
// 0xffe0_3b36
const SDCARD_API_ENTRY: usize = 0xFFE0_A548;

const GPIO_BASE: usize = 0xd401_9000;

const PINCTRL_BASE: usize = 0xd401_e000;

const GPIO68: usize = GPIO_BASE + 68 * 4;
// const GPIO68: usize = GPIO_BASE + 0x0110;

const PULL_DOWN: u32 = (5 << 13);
const PAD_1V8_DS2: u32 = (2 << 11);
const EDGE_NONE: u32 = (1 << 6);
const MUX_MODE2: u32 = 2;

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
        "csrw   mstatus, zero",

        // U-Boot does this
        //"csrr   a0, mstatus",
        //"lui    t0, 0x6",
        //"xor    a0, a0, t0",
        //"csrw   mstatus, a0",

        // cache stuff
        // dcache enable
        // "csrsi 0x7c0, 0x1",
        // icache enable
        // "csrsi 0x7c0, 0x2",
        // branch predict enable
        // csr_set(0x7c0, 0x10);
        // prefetch enable
        // csr_set(0x7c0, 0x20);

        // dcache disable
        //"csrci 0x7c0, 0x1",
        // icache disable
        //"csrci 0x7c0, 0x2",
        // branch predict disable
        //"csrci 0x7c0, 0x10",
        // prefetch disable
        //"csrci 0x7c0, 0x20",

        // TODO
        // "csrwi  0x7c1, 0", // ?
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
        0x0710 => "SpacemiT",
        _ => "unknown",
    }
}

// https://sifive.cdn.prismic.io/sifive/2dd11994-693c-4360-8aea-5453d8642c42_u74mc_core_complex_manual_21G3.pdf
fn impid_to_name<'a>(impid: usize) -> &'a str {
    match impid {
        0x0421_0427 => "21G1.02.00 / llama.02.00-general",
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

static mut SERIAL: Option<uart::K1XSerial> = None;

fn init_logger(s: uart::K1XSerial) {
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

#[no_mangle]
fn main() {
    let s = uart::K1XSerial::noinit();
    init_logger(s);
    println!("oreboot 🦀 bt0");

    print_ids();

    dram::init();

    let boot_mode = read32(STORAGE_API_P_ADDR);
    println!("Boot mode: 0x{boot_mode:08x}");
    let boot_entry = read32(boot_mode as usize);
    println!("Boot entry: 0x{boot_entry:08x}");

    unsafe {
        asm!("wfi");
    }

    // GO!
    let load_addr = 0x0; // TODO
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
