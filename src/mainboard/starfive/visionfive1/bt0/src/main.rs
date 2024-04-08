#![feature(naked_functions, asm_const)]
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
    ptr::{self, read_volatile, write_volatile},
    slice,
};
use embedded_hal_nb::serial::Write;
use init::{clock_init, iopad_init, rstgen_init};
use riscv::register::{marchid, mhartid, mimpid, mvendorid};

mod dram;
mod dram_lib;
mod dram_phy;
mod dram_pi;
mod dram_pi_start;
mod init;
mod uart;

use uart::JH71XXSerial;

// see SiFive VICU7 manual chapter 6 (p 31)
const CLINT_BASE_ADDR: usize = 0x0200_0000;
const CLINT_HART1_MSIP: usize = CLINT_BASE_ADDR + 0x0004;
// see JH7100 datasheet
const SRAM0_BASE: usize = 0x1800_0000;
const SRAM0_SIZE: usize = 0x0002_0000;
const DRAM_BASE: usize = 0x8000_0000;

const MAIN_BLOB_BASE: usize = SRAM0_BASE + 30 * 1024;
const MAIN_BLOB_SIZE: usize = 2 * 1024;
const SRAM1_BASE: usize = 0x1808_0000;
const SPI_FLASH_BASE: usize = 0x2000_0000;
const DRAM_BLOB_BASE: usize = SPI_FLASH_BASE + 0x0001_0000;
const PAYLOAD_BASE: usize = SPI_FLASH_BASE + 0x0004_0000;
const DTB_ADDR: usize = DRAM_BASE + 0x0020_0000 + 0x0100_0000; // TODO
const LOAD_ADDR: usize = DRAM_BASE;
const LOAD_MAIN: bool = true;
const DEBUG: bool = false;

const QSPI_CSR: usize = 0x1186_0000;
const QSPI_READ_CMD: usize = QSPI_CSR + 0x0004;
const SPI_FLASH_READ_CMD: u32 = 0x0003;

pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

const STACK_SIZE: usize = 4 * 1024; // 4KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

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
        // clear feature disable CSR
        "csrwi  0x7c1, 0",
        "csrw   mtvec, t0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        // suspend non-boot hart
        "li     a1, 0",
        "csrr   a0, mhartid",
        "bne    a0, a1, .nonboothart",
        // prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        "csrw   mie, 8", // 1 << 3
        "wfi",
        "call   {payload}",
        ".boothart:",
        "call   {reset}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        payload    =   sym exec_payload,
        reset      =   sym reset,
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

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);
    // Call user entry point
    main();
}

fn spi_flash_init() {
    unsafe { write_volatile(QSPI_READ_CMD as *mut u32, SPI_FLASH_READ_CMD) };
}

fn dump(addr: usize, length: usize) {
    let s = unsafe { slice::from_raw_parts(addr as *const u8, length) };
    println!("dump {length} bytes @{addr:x}");
    for w in s.iter() {
        print!("{:02x}", w);
    }
    println!();
}

fn init_logger(s: JH71XXSerial) {
    unsafe {
        static mut SERIAL: Option<JH71XXSerial> = None;
        SERIAL.replace(s);
        log::init(SERIAL.as_mut().unwrap());
    }
}

fn main() {
    clock_init();
    // for illegal instruction exception
    crate::init::syscon_func_18(0x00c000c0);
    rstgen_init();

    // enable core (?)
    crate::init::syscon_core1_en(1);

    // move UART to other header
    crate::init::syscon_io_padshare_sel(6);
    iopad_init();
    // NOTE: In mask ROM mode, the UART is already set up for 9600 baud
    // We reconfigure it to 115200, but put it on the other header so that you
    // can use both headers with the respective different baud rates.
    let serial = JH71XXSerial::new();
    init_logger(serial);
    println!("oreboot 🦀");

    spi_flash_init();

    let vid = mvendorid::read().map(|r| r.bits()).unwrap_or(0);
    let arch = marchid::read().map(|r| r.bits()).unwrap_or(0);
    let imp = mimpid::read().map(|r| r.bits()).unwrap_or(0);
    println!("RISC-V vendor {:x} arch {:x} imp {:x}", vid, arch, imp);
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {}", hart_id);

    dram::init();

    unsafe {
        asm!("fence.i");
        if LOAD_MAIN {
            println!("Read oreboot main from SRAM0 (0x{:x}):", MAIN_BLOB_BASE);
            dump(MAIN_BLOB_BASE, 32);
            // Copy the main blob from SRAM to DRAM
            for offset in (0usize..MAIN_BLOB_SIZE as usize).step_by(4) {
                let d = read_volatile((MAIN_BLOB_BASE + offset) as *mut u32);
                let t = LOAD_ADDR + offset;
                write_volatile(t as *mut u32, d);
                let b = read_volatile(t as *mut u32);
                if b != d {
                    panic!("load error @{t:x}");
                }
            }
        } else {
            let size = read_volatile((PAYLOAD_BASE) as *mut u32) as usize;
            println!("Copy payload ({} bytes)... ⏳", size);
            // Copy payload from SPI flash to DRAM, skip first 4 bytes (size)
            // NOTE: skip the first 4 bytes, which are the payload size, not code
            for offset in (0usize..size).step_by(4) {
                let d = read_volatile((PAYLOAD_BASE + 4 + offset) as *mut u32);
                let t = LOAD_ADDR + offset;
                write_volatile(t as *mut u32, d);
                let b = read_volatile(t as *mut u32);
                if b != d {
                    panic!("load error @{t:x}");
                }
                if offset % 0x2_0000 == 0 {
                    print!("➡️");
                }
            }
            println!(" .");
        }

        println!("\n\nrun payload @0x{:x}\n", LOAD_ADDR);
        // TODO: This writes to the last 4 bytes of SRAM0.
        // It is somehow necessary for ethernet to work (in U-Boot at least).
        write_volatile((SRAM0_BASE + SRAM0_SIZE - 4) as *mut u32, SRAM0_BASE as u32);
        /* restore hart1 from spinning */
        println!("release second hart =====\n");
        write_volatile(CLINT_HART1_MSIP as *mut u32, 0x1);
        exec_payload();
    }
    unsafe { riscv::asm::wfi() }
}

fn exec_payload() {
    let hart_id = mhartid::read();
    unsafe {
        // jump to payload
        let f = transmute::<usize, EntryPoint>(LOAD_ADDR);
        asm!("fence.i");
        f(hart_id, 0);
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };

    loop {
        core::hint::spin_loop();
    }
}
