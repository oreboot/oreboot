#![feature(naked_functions, asm_const)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo, ptr};
use log::{print, println};
use oreboot_arch::riscv64::sbi::{self, execute::execute_supervisor};
use oreboot_compression::decompress;
use riscv::register::mhartid;
use starfive_visionfive2_lib::{dump_block, read32, resume_nonboot_harts, udelay, write32};
use uart::JH71XXSerial;

mod sbi_platform;
mod uart;

const MEM: usize = 0x8000_0000;
const SRAM0_BASE: usize = 0x0800_0000;
const SPI_FLASH_BASE: usize = 0x2100_0000;

// compressed image
// TODO: do not hardcode
const LINUXBOOT_SRC_OFFSET: usize = 0x0040_0000;
const LINUXBOOT_SRC_ADDR: usize = SPI_FLASH_BASE + LINUXBOOT_SRC_OFFSET;
const LINUXBOOT_SRC_SIZE: usize = 0x00c0_0000;

// const DTB_SRC_OFFSET: usize = 0x0010_0000;
// const DTB_SRC_ADDR: usize = SPI_FLASH_BASE + DTB_SRC_OFFSET;
// const DTB_SIZE: usize = 0x2_0000;
const DTB_SRC_OFFSET: usize = 96 * 1024;
const DTB_SRC_ADDR: usize = SRAM0_BASE + DTB_SRC_OFFSET;
const DTB_SIZE: usize = 0x8000;

const LINUXBOOT_TMP_OFFSET: usize = 0x0400_0000;
const LINUXBOOT_TMP_ADDR: usize = MEM + LINUXBOOT_TMP_OFFSET;

// target location for decompressed image
const LINUXBOOT_OFFSET: usize = 0x0020_0000;
const LINUXBOOT_ADDR: usize = MEM + LINUXBOOT_OFFSET;
const LINUXBOOT_SIZE: usize = 0x0180_0000;
// DTB_OFFSET should be >=LINUXBOOT_OFFSET+LINUXBOOT_SIZE and match bt0
// TODO: Should we just copy it to a higher address before decompressing Linux?
const DTB_OFFSET: usize = LINUXBOOT_OFFSET + LINUXBOOT_SIZE;
const DTB_ADDR: usize = MEM + DTB_OFFSET;

// TODO: copy DTB from flash to DRAM

const STACK_SIZE: usize = 32 * 1024; // 4KiB

static PLATFORM: &str = "StarFive VisionFive 2";
static VERSION: &str = env!("CARGO_PKG_VERSION");

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
        // Clear feature disable CSR
        "csrwi  0x7c1, 0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        "csrw   mtvec, zero",
        // 1. suspend non-boot hart
        // hart 0 is the S7 monitor core; 1-4 are U7 cores
        "li     a1, 1",
        "csrr   a0, mhartid",
        "bne    a0, a1, .nonboothart",
        // 2. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j      .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        "csrw   mie, 8", // 1 << 3
        "wfi",
        "call   {payload}",
        ".boothart:",
        "call   {reset}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        reset      =   sym reset,
        payload    =   sym payload,
        options(noreturn)
    )
}

const DEBUG: bool = true;

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

fn decompress_lb() {
    // check for Device Tree header, d00dfeed
    let dtb = read32(DTB_ADDR);
    if dtb != 0xedfe0dd0 {
        panic!("DTB looks wrong: {:08x}\n", dtb);
    } else {
        print!("DTB looks fine, yay!\n");
    }
    unsafe {
        decompress(LINUXBOOT_TMP_ADDR, LINUXBOOT_ADDR, LINUXBOOT_SIZE);
    }
    // check for kernel to be okay
    let a = LINUXBOOT_ADDR + 0x30;
    let r = read32(a);
    if r == u32::from_le_bytes(*b"RISC") {
        print!("Payload looks like Linux Image, yay!\n");
    } else {
        panic!("Payload does not look like Linux Image: {:x}\n", r);
    }
    // Recheck on DTB, kernel should not run into it
    let dtb = read32(DTB_ADDR);
    if dtb != 0xedfe0dd0 {
        panic!("DTB looks wrong: {:08x} - was it overridden?\n", dtb);
    } else {
        print!("DTB still fine, yay!\n");
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

fn main() {
    udelay(200);

    let mut s = JH71XXSerial::new();
    init_logger(s);
    println!("oreboot 🦀 main");

    // TODO: this should not be necessary, decompress from flash directly
    println!("lzss compressed Linux:");
    dump_block(LINUXBOOT_SRC_ADDR, 0x100, 0x20);
    let target = LINUXBOOT_TMP_ADDR;

    println!("Copy compressed Linux to DRAM... ⏳");
    for b in (0..LINUXBOOT_SRC_SIZE).step_by(4) {
        write32(target + b, read32(LINUXBOOT_SRC_ADDR + b));
        if b % 0x4_0000 == 0 {
            print!(".");
        }
    }
    println!(" done.");

    println!("Copy DTB to DRAM... ⏳");
    let target = DTB_ADDR;
    for b in (0..DTB_SIZE).step_by(4) {
        write32(target + b, read32(DTB_SRC_ADDR + b));
        if b % 0x4_0000 == 0 {
            print!(".");
        }
    }
    println!(" done.");

    decompress_lb();
    println!("Payload extracted. Preview:");
    dump_block(LINUXBOOT_ADDR, 0x100, 0x20);
    println!("Release non-boot harts =====");
    resume_nonboot_harts();
    payload();
}

fn payload() {
    let hartid = mhartid::read();
    sbi_platform::init();
    sbi::runtime::init();
    if hartid == 1 {
        sbi::info::print_info(PLATFORM, VERSION);
    }
    let (reset_type, reset_reason) = execute_supervisor(LINUXBOOT_ADDR, hartid, DTB_ADDR);
    print!("oreboot: reset reason = {reset_reason}");
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };
    if let Some(m) = info.message() {
        println!("  {m}");
    }
    loop {
        core::hint::spin_loop();
    }
}
