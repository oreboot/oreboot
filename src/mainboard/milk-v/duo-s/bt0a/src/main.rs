#![feature(naked_functions, asm_const)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

use embedded_hal_nb::serial::Write;

#[macro_use]
extern crate log;

use core::{arch::asm, panic::PanicInfo};
use log::{print, println};

mod uart;
mod util;

const DEBUG: bool = true;
const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

// FIXME: The compiler would add a `BRK` (breakpoint) instruction hereafter.
// No clue why, so just add `jump` as an inline word to eGON header instead.
/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
/*
#[naked]
#[link_section = ".head.text"]
#[export_name = "head_jump"]
pub unsafe extern "C" fn head_jump() {
    asm!(
        // "b.ne .+0x68", // 0x60: eGON.BT0 header; 0x08: FlashHead
        ".word 0x54000341",
        options(noreturn)
    )
}
*/

/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        "bl    .forrealsiez", // 2 bytes with compact instruction
        ".word 0", // resvered
        ".word 0", // BL2 MSID
        ".word 0", // BL2 version
        ".word 0", // rest not documented
        ".word 0",
        ".word 0",
        ".word 0",
        ".forrealsiez:",
        "ldr     x0, =0x04140000",
        "ldr     x1, =0x41",
        "str     x1, [x0], #0",
        "bl   .skip",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "ldr     x1, {stack}",
        "str     x5, [x4], #0",
        "ldr     x1, {stack_size}",
        "str     x5, [x4], #0",
        ".skip:",
        "bl      {reset}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        reset      =   sym reset,
        options(noreturn)
    )
}

use core::ptr::{self, addr_of, addr_of_mut};

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

static mut SERIAL: Option<uart::SGSerial> = None;

fn init_logger(s: uart::SGSerial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
}

pub const MASK_ROM_BASE: usize = 0x0440_0000;
pub const MASK_ROM_RV_BASE: usize = MASK_ROM_BASE + 0x0001_8000;
pub const MASK_ROM_SIZE: usize = 96 * 1024;

pub const AXI_SRAM_BASE: usize = 0x0e00_0000;

const BOOT_LOG_SIZE: usize = AXI_SRAM_BASE + 0x0008;

const TPU_SRAM_BASE: usize = 0x0c00_0000;

const BOOT_LOG_BASE: usize = TPU_SRAM_BASE + 0x0003_7000;

use aarch64_cpu::registers::*;

// The mask ROM stores its own boot log in SRAM.
fn print_boot_log() {
    let boot_log_len = util::read32(BOOT_LOG_SIZE) as usize;
    println!("boot_log_len: {boot_log_len}");
    println!();
    println!(">>> BEGIN OF BOOT LOG");

    for i in (0..boot_log_len).step_by(4) {
        let e = util::read32(BOOT_LOG_BASE + i);
        let b = e.to_le_bytes();
        if i + 4 < boot_log_len {
            for c in b {
                print!("{}", c as char);
            }
        } else {
            for cc in 0..boot_log_len % 4 {
                print!("{}", b[cc] as char);
            }
        }
    }
    println!();
    println!("<<< END OF BOOT LOG");
    println!();
}

fn print_cpuinfo() {
    let arch = match MIDR_EL1.read_as_enum(MIDR_EL1::Architecture) {
        Some(MIDR_EL1::Architecture::Value::Individual) => "individual",
        _ => "other",
        None => "N/A",
    };
    println!("Architecture: {arch}");

    let imp = match MIDR_EL1.read_as_enum(MIDR_EL1::Implementer) {
        Some(MIDR_EL1::Implementer::Value::Arm) => "Arm",
        _ => "?",
        None => "N/A",
    };
    println!("Implementer: {imp}");

    // NOTE: Needs distinguishing; this is only for Implementer == Arm
    // In theory, part numbers could overlap between vendors.
    // In practice, they might be unique. TODO: What does the spec say?
    // https://github.com/bp0/armids/blob/master/arm.ids
    let par = match MIDR_EL1.read(MIDR_EL1::PartNum) {
        0xd03 => "Cortex-A53",
        _ => "?",
    };
    println!("Part number: {par}");

    let rev = MIDR_EL1.read(MIDR_EL1::Revision);
    println!("Revision: {rev}");

    let var = MIDR_EL1.read(MIDR_EL1::Variant);
    println!("Variant: {var}");

    let rndr = match ID_AA64ISAR0_EL1.read_as_enum(ID_AA64ISAR0_EL1::RNDR) {
        Some(ID_AA64ISAR0_EL1::RNDR::Value::Supported) => "yes",
        Some(ID_AA64ISAR0_EL1::RNDR::Value::NotSupported) => "no",
        None => "N/A",
    };
    println!("ID_AA64ISAR0_EL1\n  RNDR: {rndr}");

    let tgran4 = match ID_AA64MMFR0_EL1.read_as_enum(ID_AA64MMFR0_EL1::TGran4) {
        Some(ID_AA64MMFR0_EL1::TGran4::Value::Supported) => "yes",
        Some(ID_AA64MMFR0_EL1::TGran4::Value::NotSupported) => "no",
        None => "N/A",
    };
    println!("ID_AA64MMFR0_EL1");
    println!("  TGran4: {tgran4}");

    let twed = match ID_AA64MMFR1_EL1.read_as_enum(ID_AA64MMFR1_EL1::TWED) {
        Some(ID_AA64MMFR1_EL1::TWED::Value::Supported) => "yes",
        Some(ID_AA64MMFR1_EL1::TWED::Value::Unsupported) => "no",
        _ => "?",
        None => "N/A",
    };
    println!("ID_AA64MMFR1_EL1");
    println!("  TWED: {twed}");

    let bbm = match ID_AA64MMFR2_EL1.read_as_enum(ID_AA64MMFR2_EL1::BBM) {
        Some(ID_AA64MMFR2_EL1::BBM::Value::Level0) => "Level 0",
        Some(ID_AA64MMFR2_EL1::BBM::Value::Level1) => "Level 1",
        Some(ID_AA64MMFR2_EL1::BBM::Value::Level2) => "Level 2",
        _ => "?",
        None => "N/A",
    };
    println!("ID_AA64MMFR2_EL1");
    println!("  BBM: {bbm}");
}

const SEC_SUBSYS_BASE: usize = 0x0200_0000;

const SEC_XXY_BASE: usize = SEC_SUBSYS_BASE + 0x0009_0000;
// mask ROM may set this to 0x0080_0800
const SEC_SYS_SMTH: usize = SEC_XXY_BASE + 0x005c;

const SEC_SYS_BASE: usize = SEC_SUBSYS_BASE + 0x000B_0000;

const SEC_SYS_CTRL: usize = SEC_SYS_BASE + 0x0004;

const SEC_SYS_A_ADDR_L: usize = SEC_SYS_BASE + 0x0010;
const SEC_SYS_A_ADDR_H: usize = SEC_SYS_BASE + 0x0014;

const SEC_SYS_B_ADDR_L: usize = SEC_SYS_BASE + 0x0018;
const SEC_SYS_B_ADDR_H: usize = SEC_SYS_BASE + 0x001c;

const SEC_SYS_L_ADDR_L: usize = SEC_SYS_BASE + 0x0020;
const SEC_SYS_L_ADDR_H: usize = SEC_SYS_BASE + 0x0024;

use util::read32;

fn print_sec_sys() {
    let v = read32(SEC_SYS_SMTH);
    println!("SEC_SYS_SMTH: 0x{v:08x}");

    let v = read32(SEC_SYS_CTRL);
    println!("SEC_SYS_CTRL: 0x{v:08x} (0b{v:032b})");

    // 0x0440_0000 (Arm mask ROM base address)
    let h = read32(SEC_SYS_A_ADDR_H) as usize;
    let l = read32(SEC_SYS_A_ADDR_L) as usize;
    let a = (h << 32) | l;
    println!("SEC_SYS_A_ADDR: {a:016x}");
    util::dump(a, 32);

    // 0x0442_0000 (RISC-V mask ROM base address + 0x8000)
    let h = read32(SEC_SYS_B_ADDR_H) as usize;
    let l = read32(SEC_SYS_B_ADDR_L) as usize;
    let a = (h << 32) | l;
    println!("SEC_SYS_B_ADDR: {a:016x}");
    util::dump(a, 32);

    let h = read32(SEC_SYS_L_ADDR_H) as usize;
    let l = read32(SEC_SYS_L_ADDR_L) as usize;
    let a = (h << 32) | l;
    println!("SEC_SYS_L_ADDR: {a:016x}");
    util::dump(a, 32);
}

#[no_mangle]
extern "C" fn main() -> usize {
    let s = uart::SGSerial::new();
    init_logger(s);
    println!();
    println!();
    println!();
    println!("oreboot 🦀 bt0 on Arm");

    let v = MIDR_EL1.extract();
    println!("{v:08x?}");

    if false {
        print_boot_log();
    }

    print_cpuinfo();

    print_sec_sys();

    if false {
        println!(">>> mask ROM dump");
        util::dump_block(MASK_ROM_BASE, MASK_ROM_SIZE, 32);
        println!("<<< mask ROM dump");
    }

    // const MASK_ROM_SRAM: usize = 0x0453_e540;
    const MASK_ROM_SRAM: usize = 0x0453_c000;
    // util::dump_block(MASK_ROM_SRAM, 0x20, 32);

    panic!("DO NOT PANIC! EVERYTHING IS OKAY!");
    0
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
