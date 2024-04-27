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
    mem::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
    slice::from_raw_parts as slice_from,
};
use riscv::register::mhartid;
use riscv::register::{marchid, mimpid, mvendorid};

use layoutflash::areas::{find_fdt, FdtIterator};

mod dram;
mod uart;
mod util;

use util::{read32, write32};

pub type EntryPoint = unsafe extern "C" fn();

type GetBootSrc = unsafe extern "C" fn() -> BootSrc;

const DEBUG: bool = false;

const DRAM_BASE: usize = 0x8000_0000;

const STACK_SIZE: usize = 512;

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
    // starts with a 32 bytes header
    asm!(
        "j      .forrealsiez", // 2 bytes with compact instruction
        ".byte 0",
        ".byte 0",
        ".word 0", // resvered
        ".word 0", // BL2 MSID
        ".word 0", // BL2 version
        ".word 0", // rest not documented
        ".word 0",
        ".word 0",
        ".word 0",
        ".forrealsiez:",
        "li     t0, 0x04140000",
        "li     t1, 0x42",
        "sw     t1, 0(t0)",
        // Clear feature disable CSR to '0' to turn on all features
        // TODO: do in Rust
        // "csrwi  0x7c1, 0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        "ld     t0, {start}",
        "csrw   mtvec, t0",
        // 1. suspend non-boot hart
        // hart 0 is the S7 monitor core; 1-4 are U7 cores
        "li     a1, 1",
        "csrr   a0, mhartid",
        "bne    a0, a1, .nonboothart",
        // 2. prepare stack
        // FIXME: each hart needs its own stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j      .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        "j      .boothart",
        "csrw   mie, 8", // 1 << 3
        "wfi",
        "csrw   mip, 0",
        "call   {payload}",
        ".boothart:",
        "call   {reset}",
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
        _ => "unknown",
    }
}

fn impid_to_name<'a>(impid: usize) -> &'a str {
    match impid {
        0x0421_0427 => "21G1.02.00 / llama.02.00-general",
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

static mut SERIAL: Option<uart::SGSerial> = None;

fn init_logger(s: uart::SGSerial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
}

const TOP_BASE: usize = 0x0300_0000;
const TOP_MISC: usize = TOP_BASE;
const CONF: usize = TOP_MISC + 0x0004;

const EFUSE: usize = TOP_BASE + 0x0005_0000;
const EFUSE_STATUS: usize = EFUSE + 0x0010;
const EFUSE_LEAKAGE: usize = EFUSE + 0x0108;
const FTSN3: usize = EFUSE + 0x010C;
const FTSN4: usize = EFUSE + 0x0110;
const EFUSE_W_LOCK0: usize = EFUSE + 0x0198;

// 64k mask ROM
const MASK_ROM_BASE: usize = 0x0440_0000;
// plat/cv181x/include/riscv/rom_api_refer.h
// const P_ROM_API_GET_BOOT_SRC: usize = MASK_ROM_BASE + 0x0001_8020;
// plat/cv180x/include/riscv/rom_api_refer.h
const P_ROM_API_GET_BOOT_SRC: usize = MASK_ROM_BASE + 0x0000_0020;
const P_ROM_API_GET_NUMBER_OF_RETRIES: usize = MASK_ROM_BASE + 0x0000_00C0;

const BOOT_SRC_TAG: u32 = 0xCE00;

const AXI_SRAM_BASE: usize = 0x0E00_0000;
const CP_STATE: usize = AXI_SRAM_BASE + 0x0018;

#[derive(Debug)]
#[repr(u32)]
enum BootSrc {
    SpiNand = BOOT_SRC_TAG | 0x00,
    SpiNor = BOOT_SRC_TAG | 0x02,
    Emmc = BOOT_SRC_TAG | 0x03,
    Sd = BOOT_SRC_TAG | 0xa0,
    Usb = BOOT_SRC_TAG | 0xa3,
    Uart = BOOT_SRC_TAG | 0xa5,
}

impl core::fmt::Display for BootSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            BootSrc::SpiNand => write!(f, "SPI NAND"),
            BootSrc::SpiNor => write!(f, "SPI NOR"),
            BootSrc::Emmc => write!(f, "EMMC"),
            BootSrc::Sd => write!(f, "SD card"),
            BootSrc::Usb => write!(f, "USB"),
            BootSrc::Uart => write!(f, "UART"),
        }
    }
}

#[no_mangle]
fn main() {
    let s = uart::SGSerial::new();
    init_logger(s);
    println!();
    println!();
    println!();
    println!("oreboot 🦀 bt0");
    print_ids();

    unsafe {
        let get_boot_src: GetBootSrc = transmute(P_ROM_API_GET_BOOT_SRC);
        let boot_src = get_boot_src();
        println!("boot src: {boot_src}");

        let get_entry_count: GetBootSrc = transmute(P_ROM_API_GET_NUMBER_OF_RETRIES);
        let entry_count = get_entry_count();
        println!("entries: {entry_count}");
    }

    let w_lock0 = read32(EFUSE_W_LOCK0);
    println!("W_LOCK0:       {w_lock0:08x}");
    let efuse_status = read32(EFUSE_STATUS);
    println!("EFUSE_STATUS:  {efuse_status:08x}");

    let conf = read32(CONF);
    println!("CONF:          {conf:08x}");
    let efuse_leakage = read32(EFUSE_LEAKAGE);
    println!("EFUSE_LEAKAGE: {efuse_leakage:08x}");
    let v = read32(FTSN3);
    println!("FTSN3:         {v:08x}");
    let v = read32(FTSN4);
    println!("FTSN4:         {v:08x}");

    let chip_type = (conf >> 28) & 0b111;
    let chip_type = match chip_type {
        3 => "CV1800B / 64MB DDR2 RAM 1333",
        _ => "unknown",
    };
    println!("TYPE:          {chip_type}");

    // 1: 512Mbit
    let dram_capacity = (efuse_leakage >> 26) & 0b111;
    // 4: ESMT 512Mbit DDR2
    let dram_vendor = (efuse_leakage >> 21) & 0b11111;

    let cp_state = read32(CP_STATE);
    println!("CP_STATE:      {cp_state:08x}");

    /*
     * W_LOCK0:       00000000
     * EFUSE_STATUS:  00000070
     * CONF:          3500032a
     * EFUSE_LEAKAGE: 64800024
     * FTSN3:         e1a5e4ca
     * FTSN4:         15274190
     * TYPE:          CV1800B / 64MB DDR2 RAM 1333
     * CP_STATE:      00000000
     */

    let load_addr = DRAM_BASE + 0x0020_0000;
    println!("[bt0] Jump to main stage @{load_addr:08x}");
    exec_payload(load_addr);

    println!("[bt0] Exit from main stage, resetting...");

    unsafe {
        reset();
        riscv::asm::wfi()
    };
}

fn exec_payload(addr: usize) {
    unsafe {
        // jump to main
        let f: EntryPoint = transmute(addr);
        // asm!("fence.i");
        f();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        if DEBUG {
            println!(
                "[bt0] panic in '{}' line {}",
                location.file(),
                location.line(),
            );
        }
    } else {
        if DEBUG {
            println!("[bt0] panic at unknown location");
        }
    };
    if let Some(msg) = info.message() {
        println!("[bt0]   {msg}");
    }
    loop {
        core::hint::spin_loop();
    }
}
