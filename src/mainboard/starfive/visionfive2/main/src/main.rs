#![feature(naked_functions, asm_const)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]

use core::{
    arch::asm,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};
use log::{print, println};
use oreboot_arch::riscv64::sbi::{self, execute::execute_supervisor};
use oreboot_compression::decompress;
use riscv::register::mhartid;
use starfive_visionfive2_lib::{dump_block, read32, resume_nonboot_harts, udelay, write32};
use uart::JH71XXSerial;

mod sbi_platform;
mod uart;

const DRAM_BASE: usize = 0x4000_0000;
const SRAM0_BASE: usize = 0x0800_0000;
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

fn main() {
    udelay(200);

    let s = JH71XXSerial::new();
    init_logger(s);
    println!("oreboot 🦀 main");

    let payload_addr = PAYLOAD_ADDR;

    println!("[main] Copy DTB to DRAM... ⏳");
    copy(DTB_SRC_ADDR, DTB_ADDR, DTB_SIZE);
    check_dtb(DTB_ADDR);

    if false {
        println!("[main] Decompress payload... ⏳");
        unsafe {
            decompress(LINUXBOOT_SRC_ADDR, LINUXBOOT_ADDR, LINUXBOOT_SIZE);
        }
        println!("[main] Payload extracted.");
        check_kernel(LINUXBOOT_ADDR);
    } else {
        println!("[main] Copy payload to DRAM... ⏳");
        dump_block(PAYLOAD_SRC_ADDR, 0x40, 0x20);
        copy(PAYLOAD_SRC_ADDR, payload_addr, PAYLOAD_SIZE);
        dump_block(payload_addr, 0x40, 0x20);
    }

    // Recheck on DTB, kernel should not run into it
    check_dtb(DTB_ADDR);

    if SMP {
        println!("[main] Release non-boot harts =====");
        resume_nonboot_harts();
    }

    for i in 0..0x1000 {
        read32(payload_addr);
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
    print!("[main] oreboot: reset reason = {reset_reason}");
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[main] panic in '{}' line {}",
            location.file(),
            location.line(),
        );
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
