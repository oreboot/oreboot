#![feature(naked_functions, asm_const)]
#![feature(fn_align)]
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
use riscv::register::{marchid, mhartid, mimpid, mtvec, mvendorid};

use layoutflash::areas::{find_fdt, FdtIterator};
use oreboot_arch::riscv64::sbi::{
    self,
    execute::{execute_supervisor, read32},
};

mod sbi_platform;
mod uart;

pub type EntryPoint = unsafe extern "C" fn();

const DRAM_BASE: usize = 0x8000_0000;
const LOAD_ADDR: usize = DRAM_BASE + 0x0020_0000;
const DTB_ADDR: usize = LOAD_ADDR + 0x2000;

static PLATFORM: &str = "Milk-V Duo S";
static VERSION: &str = env!("CARGO_PKG_VERSION");

const DEBUG: bool = false;

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
    if false {
        unsafe {
            asm!("csrs 0x7c0, {}", in(reg) 0x00018000);
        }
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
}

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
    // starts with a 32 bytes header
    asm!(
        // 1. clear cache and processor states
        "csrw   mie, zero",
        "csrw   mip, 0",
        "csrw   mstatus, zero",
        "ld     t0, {start}",
        "csrw   mtvec, t0",
        // MXSTATUS: enable theadisaee and maee
        // "li     t1, 0x1 << 22 | 0x1 << 21",
        // "csrs   0x7c0, t1",
        // MCOR: disable caches
        "li     t1, 0x00000022",
        "csrw   0x7c2, t1",
        // invalidate ICACHE/DCACHE/BTB/BHT
        "li     t1, 0x00030013",
        // MCOR
        "csrw   0x7c2, t1",
        // MHCR
        "csrwi  0x7c1, 0",
        // 2. prepare stack
        // FIXME: each hart needs its own stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "call   {reset}",
        stack      = sym BT0_STACK,
        stack_size = const STACK_SIZE,
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

static mut SERIAL: Option<uart::SGSerial> = None;

fn init_logger(s: uart::SGSerial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
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
    let aid = marchid::read().map(|r| r.bits()).unwrap_or(0);
    println!("RISC-V arch {aid:08x}");
    let vid = mvendorid::read().map(|r| r.bits()).unwrap_or(0);
    let vendor_name = vendorid_to_name(vid);
    println!("RISC-V core vendor: {vendor_name} (0x{vid:04x})");
    let iid = mimpid::read().map(|r| r.bits()).unwrap_or(0);
    let imp_name = impid_to_name(iid);
    println!("RISC-V implementation: {imp_name} (0x{iid:08x})");
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {hart_id}");
}

fn rdtime() -> usize {
    let mut time: usize;
    unsafe { asm!("rdtime {time}", time = out(reg) time) };
    time
}

fn delay(t: usize) {
    let later = rdtime() + t;
    while rdtime() < later {}
}

#[repr(align(4))]
fn noisr(a: usize, b: usize) {
    println!("stop it {a:08x} {b:08x}");
}

#[no_mangle]
fn main() {
    let s = uart::SGSerial::new();
    init_logger(s);
    println!();
    println!("oreboot 🦀 main");

    print_ids();

    // test mtvec
    if false {
        unsafe {
            asm!("fence.i");
            let a = noisr as *const () as usize;
            mtvec::write(a, mtvec::TrapMode::Direct);
            asm!("ecall")
            // riscv::asm::ebreak();
        };
    }

    let test = read32(LOAD_ADDR);
    println!("test {test:08x}");

    const SBI: bool = true;
    if SBI {
        sbi_payload(LOAD_ADDR);
    } else {
        exec_payload(LOAD_ADDR);
    }

    loop {
        let time = rdtime();
        println!("tick {time}");
        delay(10_000_000);
        let time = rdtime();
        println!("tock {time}");
        delay(10_000_000);
    }

    unsafe {
        // reset();
        riscv::asm::wfi()
    };
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

fn sbi_payload(payload_addr: usize) {
    sbi_platform::init();
    dump_csrs();
    init_csrs();
    dump_csrs();

    print_cpuid();

    sbi::runtime::init();
    sbi::info::print_info(PLATFORM, VERSION);
    let hartid = mhartid::read();
    println!("[main] .......");

    let (reset_type, reset_reason) = execute_supervisor(payload_addr, hartid, DTB_ADDR);
    print!("[main] oreboot: reset, type = {reset_type}, reason = {reset_reason}");
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
                "[main] panic in '{}' line {}",
                location.file(),
                location.line(),
            );
        }
    } else {
        if DEBUG {
            println!("[main] panic at unknown location");
        }
    };
    if let Some(msg) = info.message() {
        println!("[main]   {msg}");
    }
    loop {
        core::hint::spin_loop();
    }
}
