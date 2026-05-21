#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

#[macro_use]
extern crate log;
use core::{
    arch::{asm, naked_asm},
    mem::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};

use embedded_hal_nb::serial::Write;
use riscv::register::{marchid, mhartid, mimpid, mvendorid};

use util::{dump, dump_block, read32, write32};

mod mem_map;
mod sbi_platform;
mod uart;
mod util;

const DEBUG: bool = false;

pub type EntryPoint = unsafe extern "C" fn();

const BOOT_HART_ID: usize = 0;

const STACK_SIZE: usize = 8 * 1024;

static PLATFORM: &str = "Canaan Kendryte K230D";
static VERSION: &str = env!("CARGO_PKG_VERSION");

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Set up stack and jump to executable code.
///
/// # Safety
///
/// Naked function.
#[unsafe(naked)]
#[export_name = "start"]
#[link_section = ".text.entry"]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn start() -> ! {
    naked_asm!(
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
    // FIXME: why is this broken now, Rust?!
    if false {
        ptr::write_bytes(addr_of_mut!(_sbss), 0, bss_size);
    }
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
        0x1000_0000_4977_2200 => "SpacemiT X60",
        0x0000_0000_0005_0000 => "C908 (Kendryte K230)",
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

static mut SERIAL: Option<uart::K230Serial> = None;

fn init_logger(s: uart::K230Serial) {
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

fn dump_csrs() {
    let mut v: usize;
    unsafe {
        println!("==== platform CSRs ====");
        asm!("csrr {}, 0x7c0", out(reg) v);
        println!("   MXSTATUS  {v:08x}");
        asm!("csrr {}, 0x7c1", out(reg) v);
        println!("   MHCR      {v:08x}");
        asm!("csrr {}, 0x7c2", out(reg) v);
        println!("   MCOR      {v:08x}");
        asm!("csrr {}, 0x7c5", out(reg) v);
        println!("   MHINT     {v:08x}");
        println!("see C906 manual p581 ff");
        println!("=======================");
    }
}

fn init_csrs() {
    println!("Set up extension CSRs");
    if DEBUG {
        dump_csrs();
    }
    unsafe {
        // MXSTATUS: T-Head ISA extension enable, MAEE, MM, UCME, CLINTEE
        // NOTE: Linux relies on detecting errata via mvendorid, marchid and
        // mipmid. If that detection fails, and we enable MAEE, Linux won't come
        // up. When D-cache is enabled, and the detection fails, we run into
        // cache coherency issues. Welcome to the minefield! :)
        // NOTE: We already set part of this in bt0, but it seems to get lost?
        asm!("csrs 0x7c0, {}", in(reg) 0x00638000);
        // MCOR: invalidate ICACHE/DCACHE/BTB/BHT
        asm!("csrw 0x7c2, {}", in(reg) 0x00070013);
        // MHCR
        asm!("csrw 0x7c1, {}", in(reg) 0x000011ff);
        // MHINT
        asm!("csrw 0x7c5, {}", in(reg) 0x0016e30c);
    }
    if DEBUG {
        dump_csrs();
    }
}

// The machine mode processor model register (MCPUID) stores the processor
// model information. Its reset value is determined by the product itself and
// complies with the Pingtouge product definition specifications to facilitate
// software identification. By continuously reading the MCPUID register, up to
// 7 different return values can be obtained to represent C906 product
// information, as shown in Figure ??.

// T-Head CPU model register
const MCPUID: u32 = 0xfc0;
fn print_cpuid() {
    let mut id: u32;
    for i in 0..7 {
        unsafe { asm!("csrr {}, 0xfc0", out(reg) id) };
        println!("MCPUID {i}: {id:08x}");
    }
}

const MASK_ROM_LOADER: usize = mem_map::MASK_ROM_BASE;

const CPU0_RESET_CONTROL: usize = mem_map::RMU_BASE + 0x0004;

#[no_mangle]
fn main() {
    let mut ini_pc: usize = 0;
    unsafe { asm!("mv {}, s4", out(reg) ini_pc) };

    let s = uart::K230Serial::new();
    init_logger(s);
    println!("oreboot 🦀 main");
    println!("initial program counter (PC) {ini_pc:016x}");
    print_ids();
    print_cpuid();
    init_csrs();

    exec_payload();
}

fn exec_payload() {
    let payload_addr = mem_map::DRAM_BASE_ADDR + 0x20_0000;

    if DEBUG {
        println!("Payload @ {payload_addr:08x}");
        dump_block(payload_addr, 0x50, 0x10);
    }

    let use_sbi = true;
    if use_sbi {
        use oreboot_arch::riscv64::sbi as ore_sbi;
        let sbi = sbi_platform::init();

        ore_sbi::runtime::init();
        ore_sbi::info::print_info(PLATFORM, VERSION);

        let hart_id = mhartid::read();
        let dtb_addr = 0;
        let (reset_type, reset_reason) =
            ore_sbi::execute::execute_supervisor(sbi, payload_addr, hart_id, dtb_addr, None);
        println!("[oreboot] reset reason: {reset_reason}");
    } else {
        unsafe {
            let f: EntryPoint = transmute(payload_addr);
            asm!("fence.i");
            f();
        }
    }
    unsafe { riscv::asm::wfi() }
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
    let msg = info.message();
    println!("[bt0]   {msg}");
    loop {
        core::hint::spin_loop();
    }
}
