#![feature(naked_functions, asm_const)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]

use core::{
    arch::asm,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};
use jh71xx_hal as hal;
use riscv::register::mhartid;

use layoutflash::areas::{find_fdt, Fdt, FdtIterator, FdtNode};
use log::{print, println};
// use soc::starfive::jh7110::{pac, uart};
use starfive_visionfive2_lib::{dump_block, read32, resume_nonboot_harts, udelay, write32};

use oreboot_arch::riscv64::sbi::{self, execute::execute_supervisor};
use oreboot_compression::decompress;
use uart::JH71XXSerial;

mod sbi_platform;
mod uart;

const DEBUG: bool = true;

const DRAM_BASE: usize = 0x4000_0000;
const SRAM0_BASE: usize = 0x0800_0000;
const SRAM0_SIZE: usize = 2 * 1024 * 1024;
const SPI_FLASH_BASE: usize = 0x2100_0000;

/// This is the compressed Linux image in boot storage (flash).
// TODO: do not hardcode; this will be handled in xtask eventually
// const LINUXBOOT_SRC_OFFSET: usize = 0x0040_0000; // VF2
const LINUXBOOT_SRC_OFFSET: usize = 0x0046_0000; // Mars CM
const LINUXBOOT_SRC_ADDR: usize = SPI_FLASH_BASE + LINUXBOOT_SRC_OFFSET;

/// This is the Linux DTB in SRAM, copied over by the mask ROM loader.
const DTB_SRC_OFFSET: usize = 0x5_1000;
const DTB_SRC_ADDR: usize = SRAM0_BASE + DTB_SRC_OFFSET;
const DTB_SIZE: usize = 0xa000; // 40K, because 32K was not enough.

// const DTB_SRC_OFFSET: usize = 96 * 1024;
// const DTB_SIZE: usize = 0x2_0000; // 128K, because 32K was not enough.

// const DTB_SRC_OFFSET: usize = 0x1_0000; // oreboot dtb
// const DTB_SIZE: usize = 0x1000; // oreboot dtb

/// This is where we copy the data to in DRAM.
/// NOTE: Kernel and DTB must not overlap. We check this after extraction.
/// I.e., DTB_OFFSET must be >=LINUXBOOT_OFFSET+LINUXBOOT_SIZE and match bt0.
const LINUXBOOT_OFFSET: usize = 0x0020_0000;
const LINUXBOOT_ADDR: usize = DRAM_BASE + LINUXBOOT_OFFSET;
const LINUXBOOT_SIZE: usize = 0x0380_0000;
const DTB_OFFSET: usize = LINUXBOOT_OFFSET + LINUXBOOT_SIZE;
const DTB_ADDR: usize = DRAM_BASE + DTB_OFFSET;

static PLATFORM: &str = "StarFive VisionFive 2";
static VERSION: &str = env!("CARGO_PKG_VERSION");

const STACK_SIZE: usize = 32 * 1024;

#[link_section = ".bss.uninit"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Set up stack and jump to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "_start"]
#[link_section = ".text.entry"]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // Clear feature disable CSR
        "csrwi  0x7c1, 0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        "csrw   mtvec, zero",
        // 1. suspend non-boot hart
        // hart 0 is the S7 monitor core; 1-4 are U7 cores
        "li     t1, 1",
        "csrr   t0, mhartid",
        "bne    t0, t1, .nonboothart",
        // 2. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j      .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        // "csrw   mie, 8", // 1 << 3
        "csrw   mie, 0", // disable wakeup
        "wfi",
        "csrw   mip, 0",
        "call   {resume}",
        ".boothart:",
        "call   {reset}",
        stack      = sym STACK,
        stack_size = const STACK_SIZE,
        reset      = sym boot_hart_reset,
        resume     = sym nonboot_hart_resume,
        options(noreturn)
    )
}

/// Initialize RAM: Clear BSS and set up data.
/// See https://docs.rust-embedded.org/embedonomicon/main.html
///
/// # Safety
/// :shrug:
#[no_mangle]
pub unsafe extern "C" fn init() {
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
}

/// # Safety
/// :shrug:
#[no_mangle]
pub unsafe extern "C" fn boot_hart_reset() {
    init();
    // Call user entry point
    main();
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

// Device Tree header, d00dfeed, in little endian
const DTB_HEADER: u32 = 0xedfe0dd0;

fn check_dtb(dtb_addr: usize) {
    let dtb = read32(dtb_addr);
    if dtb == DTB_HEADER {
        println!("[main] DTB @0x{dtb_addr:08x} looks fine, yay!");
    } else {
        panic!("[main] DTB @0x{dtb_addr:08x} looks wrong: {dtb:08x}");
    }
}

fn check_kernel(kernel_addr: usize) {
    let a = kernel_addr + 0x30;
    let r = read32(a);
    if r == u32::from_le_bytes(*b"RISC") {
        println!("[main] Payload at 0x{kernel_addr:08x} looks like Linux Image, yay!");
    } else {
        dump_block(LINUXBOOT_ADDR, 0x40, 0x20);
        panic!("[main] Payload at 0x{kernel_addr:08x} does not look like Linux Image. Expected 'RISC' at +0x30, but got: {r:x}");
    }
}

static mut SERIAL: Option<JH71XXSerial> = None;

fn init_logger(s: JH71XXSerial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
}

const SMP: bool = false;

const PAYLOAD_SIZE: usize = 192 * 1024;
const PAYLOAD_SRC_ADDR: usize = SRAM0_BASE + 0x2_1000;
const PAYLOAD_ADDR: usize = LINUXBOOT_ADDR;

const QSPI_XIP_BASE: usize = 0x2100_0000;

// FIXME: This is all hardcoded for now, just for testing. Use DTFS otherwise.
fn load_uboot(uboot_addr: usize, uboot_size: usize) {
    // U-Boot is linked to run from here
    let load_addr = LINUXBOOT_ADDR;
    println!(
        "[bt0] Copy {}k U-Boot from {uboot_addr:08x} to {load_addr:08x}... ⏳",
        uboot_size / 1024
    );
    copy(uboot_addr, load_addr, uboot_size);
    dump_block(load_addr, 0x100, 0x20);

    // whatevs
    let uboot_dtb_addr = QSPI_XIP_BASE + 0x0011_90d4;
    let uboot_dtb_size = 0xb638;
    copy(uboot_dtb_addr, DTB_ADDR, uboot_dtb_size);
}

/// Dump DT node properties, prefixed for indentation.
fn dump_props(n: &FdtNode, pre: &str) {
    for p in n.properties() {
        let pname = p.name;
        match pname {
            "addr" => {
                let addr = p.as_usize().unwrap_or(0);
                println!("{pre}  - {pname}: {addr:08x}");
            }
            "size" => {
                let size = p.as_usize().unwrap_or(0);
                println!("{pre}  - {pname}: {size} (0x{size:x})");
            }
            _ => {
                let str = p.as_str().unwrap_or("[empty]");
                println!("{pre}  - {pname}: {str}");
            }
        }
    }
}

// TODO: Should we do recursion? It's possible, but... not really necessary.
// While being less repetitive, it imposes new challenges.
fn dump_fdt_nodes(fdt: &Fdt, path: &str) {
    let nodes = &mut fdt.find_all_nodes(path);
    println!(" {path}");
    for n in FdtIterator::new(nodes) {
        for n in n.children() {
            let c = n.name;
            let pre = "  ";
            println!("{pre}↪ {c}");
            dump_props(&n, pre);
            for n in n.children() {
                let c = n.name;
                let pre = "    ";
                println!("{pre}↪ {c}");
                dump_props(&n, pre);
                for n in n.children() {
                    let c = n.name;
                    let pre = "      ";
                    println!("{pre}↪ {c}");
                    dump_props(&n, pre);
                    for n in n.children() {
                        let c = n.name;
                        let pre = "        ";
                        println!("{pre}↪ {c}");
                        dump_props(&n, pre);
                    }
                }
            }
        }
    }
}

fn get_payload_offset_and_size(fdt: &Fdt, name: &str) -> (usize, usize) {
    let mut offset = 0;
    let mut found = false;
    let mut size = 0;
    let areas = &mut fdt.find_all_nodes("/flash-info/areas");
    // TODO: make finding more sophisticated
    for a in FdtIterator::new(areas) {
        for c in a.children() {
            let cname = c.name;
            if let Some(p) = c.property("compatible") {
                let str = p.as_str().unwrap_or("[empty]");
                if str == name {
                    found = true;
                }
            }
            // Add up sizes to get the respective area's offset.
            if let Some(p) = c.property("size") {
                let s = p.as_usize().unwrap_or(0);
                offset += s;
                if found {
                    size = s;
                }
            }
            // If an offset itself is provided, just take it directly.
            if let Some(p) = c.property("offset") {
                let o = p.as_usize().unwrap_or(0);
                if o > 0 {
                    offset = o;
                }
            }
        }
    }
    (offset, size)
}

fn dump_fdt_board_info(fdt: &Fdt) {
    let nodes = &mut fdt.find_all_nodes("/board-info");
    println!("ℹ️ Board information");
    for n in FdtIterator::new(nodes) {
        for p in n.properties() {
            let pname = p.name;
            let s = p.as_str().unwrap_or("[empty]");
            println!("  {pname}: {s}");
        }
    }
}

// TODO: DTFS should tell us whether a payload expects/provides a DT.
// Should we return (payload, fdt), where both are (addr, size), or slice?
fn find_and_process_dtfs(slice: &[u8]) -> Result<(usize, usize), &str> {
    if let Ok(fdt) = find_fdt(slice) {
        dump_fdt_board_info(&fdt);
        println!("💾 DTFS");
        dump_fdt_nodes(&fdt, "/flash-info/areas");
        dump_fdt_nodes(&fdt, "/load-info");
        let (offset, size) = get_payload_offset_and_size(&fdt, "uboot-main");
        Ok((offset, size))
    } else {
        Err("DTFS blob not found")
    }
}

/// Get a slice of memory. You need to ensure that it is a valid, aligned block.
/// TODO: factor out to a library
fn get_slice<'a>(addr: &'a usize, size: &'a usize) -> &'a [u8] {
    unsafe {
        let p = core::mem::transmute(*addr);
        core::slice::from_raw_parts(p, *size)
    }
}

fn main() {
    /*
    let dp = pac::Peripherals::take().unwrap();

    for i in 0..1000 {
        if dp.UART0.usr().read().busy() == false {
            break;
        }
    }
    // let s = JH71XXSerial(hal::uart::Uart(dp.UART0));
    // write32(0x1000_0000, 0x42);
    // we get here
    let s = JH71XXSerial::new_with_config(
        dp.UART0,
        hal::uart::TIMEOUT_US,
        hal::uart::Config {
            data_len: hal::uart::DataLength::Eight,
            stop: hal::uart::Stop::One,
            parity: hal::uart::Parity::None,
            baud_rate: hal::uart::BaudRate::B115200,
            clk_hz: uart::UART_CLK_OSC,
        },
    );
    // we do not get here
    write32(0x1000_0000, 0x44);
    */

    let s = JH71XXSerial::new();
    init_logger(s);
    println!("oreboot 🦀 main");

    let slice = get_slice(&SRAM0_BASE, &SRAM0_SIZE);
    let (offset, size) = find_and_process_dtfs(slice).unwrap();
    let addr = QSPI_XIP_BASE + offset;
    println!("U-Boot @ {addr:08x} ({size} bytes)");
    // should be: 213000d4
    // was: 2103a000
    // let payload_addr = SRAM0_BASE + offset;

    let payload_addr = PAYLOAD_ADDR;
    load_uboot(addr, size);

    if false {
        println!("[main] Copy DTB to DRAM... ⏳");
        copy(DTB_SRC_ADDR, DTB_ADDR, DTB_SIZE);
        check_dtb(DTB_ADDR);
    }

    if false {
        println!("[main] Decompress payload... ⏳");
        unsafe {
            decompress(LINUXBOOT_SRC_ADDR, LINUXBOOT_ADDR, LINUXBOOT_SIZE);
        }
        println!("[main] Payload extracted.");
        check_kernel(LINUXBOOT_ADDR);
    }

    if false {
        println!("[main] Copy payload to DRAM... ⏳");
        dump_block(PAYLOAD_SRC_ADDR, 0x40, 0x20);
        copy(PAYLOAD_SRC_ADDR, payload_addr, PAYLOAD_SIZE);
        dump_block(payload_addr, 0x40, 0x20);
    }

    // Recheck on DTB, payload should not run into it
    check_dtb(DTB_ADDR);

    if SMP {
        println!("[main] Release non-boot harts =====");
        resume_nonboot_harts();
    }

    payload(payload_addr);
}

fn nonboot_hart_resume() {
    unsafe {
        init();
    }
    let payload_addr = PAYLOAD_ADDR;
    // TODO: What do we do with hart 0, the S7 monitor hart?
    let hartid = mhartid::read();
    if hartid == 0 {
        loop {
            unsafe { asm!("wfi") }
        }
    }
    payload(payload_addr);
}

fn payload(payload_addr: usize) {
    let hartid = mhartid::read();
    sbi_platform::init();
    sbi::runtime::init();
    if hartid == 1 {
        sbi::info::print_info(PLATFORM, VERSION);
    }
    let (reset_type, reset_reason) = execute_supervisor(payload_addr, hartid, DTB_ADDR);
    print!("[main] oreboot: reset, type = {reset_type}, reason = {reset_reason}");
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        let f = location.file();
        let l = location.line();
        println!("[main] panic in '{f}' line {l}");
    } else {
        println!("[main] panic at unknown location");
    };
    if let Some(m) = info.message() {
        println!("  {m}");
    }
    loop {
        core::hint::spin_loop();
    }
}
