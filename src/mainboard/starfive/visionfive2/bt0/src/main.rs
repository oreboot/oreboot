#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

#[macro_use]
extern crate log;
extern crate layoutflash;
use layoutflash::areas::{find_fdt, FdtIterator};

use core::{arch::asm, intrinsics::transmute, panic::PanicInfo, ptr};
use riscv::register::mhartid;
use riscv::register::{marchid, mimpid, mvendorid};
use starfive_visionfive2_lib::{dump_block, read32, udelay, write32};
use uart::JH71XXSerial;

use fdt::Fdt;

mod ddr_start;
mod ddrcsr;
mod ddrlib;
mod ddrphy;
mod dram;
mod init;
mod pll;
mod uart;

pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

// The SRAM is called LIM, LooselyIntegrated Memory
// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/u74_memory_map.html
const SRAM0_BASE: usize = 0x0800_0000;
const SRAM0_SIZE: usize = 0x0002_0000;

const DRAM_BASE: usize = 0x4000_0000;

const STACK_SIZE: usize = 4 * 1024; // 4KiB

/*
const QSPI_CSR: usize = 0x1186_0000;
const QSPI_READ_CMD: usize = QSPI_CSR + 0x0004;
const SPI_FLASH_READ_CMD: u32 = 0x0003;
*/

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
        // Clear feature disable CSR to '0' to turn on all features
        // TODO: do in Rust
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
        // FIXME: each hart needs its own stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j      .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        "csrw   mie, 8", // 1 << 3
        "wfi",
        "csrw   mip, 0",
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

// 0: SPI, 1: MMC2, 2: MMC1, 3: UART
const MODE_SELECT_REG: usize = 0x1702_002c;

fn print_boot_mode() {
    // lowest two bits only; 0: SPI, 1: MMC2, 2: MMC1, 3: UART
    let mode = read32(MODE_SELECT_REG) & 0b11;
    let mode_str = match mode {
        0 => "SPI",
        1 => "MMC2",
        2 => "MMC1",
        3 => "UART",
        _ => "",
    };
    println!("boot mode: {mode_str}");
}

fn vendorid_to_name<'a>(vendorid: usize) -> &'a str {
    match vendorid {
        0x0489 => "SiFive",
        _ => "unknown",
    }
}

// https://sifive.cdn.prismic.io/sifive/2dd11994-693c-4360-8aea-5453d8642c42_u74mc_core_complex_manual_21G3.pdf
fn impid_to_name<'a>(vendorid: usize) -> &'a str {
    match vendorid {
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
    // The long number is what the `mcause` register should hold in case of
    // a machine timer interrupt. See U74-MC core complex manual 21G3.
    println!("RISC-V arch {aid:08x}");
    let vendor_name = vendorid_to_name(vid);
    println!("RISC-V core vendor: {vendor_name} (0x{vid:04x})");
    let imp_name = impid_to_name(iid);
    println!("RISC-V implementation: {imp_name} (0x{iid:08x})");
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {}", hart_id);
}

unsafe fn sleep(n: u32) {
    for _ in 0..n {
        core::hint::spin_loop();
    }
}

unsafe fn blink() {
    sleep(0x0004_0000);
    write32(init::GPIO40_43_DATA, 0x8181_8181);
    sleep(0x0004_0000);
    write32(init::GPIO40_43_DATA, 0x8080_8080);
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

#[no_mangle]
fn main() {
    // clock/PLL setup, see U-Boot board/starfive/visionfive2/spl.c
    pll::pll0_set_freq(pll::PLL0_1000000000);
    pll::pll2_set_freq(pll::PLL2_1188000000);

    /* DDR controller related clk init */
    // see U-Boot board/starfive/visionfive2/spl.c
    init::clk_cpu_root();
    init::clk_bus_root();
    init::clocks();

    // set GPIO to 3.3V
    write32(init::SYS_SYSCON_12, 0x0);

    // enable is active low
    if false {
        write32(init::GPIO40_43_EN, 0xc0c0_c0c0);
        write32(init::GPIO40_43_DATA, 0x8181_8181);
        unsafe { blink() }
    }

    // TX/RX are GPIOs 5 and 6
    write32(init::GPIO04_07_EN, 0xc0c0_c0c0);
    let mut s = JH71XXSerial::new();
    init_logger(s);
    println!("oreboot 🦀 bt0");
    print_boot_mode();
    print_ids();

    // AXI cfg0, clk_apb_bus, clk_apb0, clk_apb12
    init::clk_apb0();
    // init::clk_apb_func();
    dram::init();

    const MAIN_BLOB_BASE: usize = SRAM0_BASE + 32 * 1024;
    const MAIN_BLOB_SIZE: usize = 64 * 1024;

    // TODO: Scan SRAM for oreboot dtb
    let main_size_k = MAIN_BLOB_SIZE / 1024;
    println!("Copy {main_size_k}k main stage to DRAM... ⏳");
    for b in (0..MAIN_BLOB_SIZE).step_by(4) {
        write32(DRAM_BASE + b, read32(MAIN_BLOB_BASE + b));
        if b % 1024 == 0 {
            print!(".");
        }
    }
    println!(" done.");
    println!("Jump to main stage...");
    println!();

    exec_payload();
    println!("Exit from payload, resetting...");
    unsafe {
        sleep(0x0100_0000);
        reset();
        riscv::asm::wfi()
    };
}

fn exec_payload() {
    let hart_id = mhartid::read();
    let load_addr = DRAM_BASE;
    if hart_id == 1 {
        println!("Payload @{load_addr:08x}");
    }
    udelay(150000);
    unsafe {
        let f: fn() = transmute(load_addr);
        asm!("fence.i");
        f();
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
