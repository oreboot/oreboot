#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

#[macro_use]
extern crate log;
extern crate layoutflash;
use layoutflash::areas::{find_fdt, FdtIterator};

use core::{
    arch::asm,
    intrinsics::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};
use init::{dump_block, read32, write32};
use riscv::register::mhartid;
use riscv::register::{marchid, mimpid, mvendorid};

use fdt::Fdt;

mod ddr_start;
mod ddrcsr;
mod ddrlib;
mod ddrphy;
mod dram;
mod init;
mod pac;
mod pll;
mod uart;

pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

// The SRAM is called LIM, LooselyIntegrated Memory
// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/u74_memory_map.html
const SRAM0_BASE: usize = 0x0800_0000;
const SRAM0_SIZE: usize = 0x0002_0000;
const DRAM_BASE: usize = 0x4000_0000;

// see also SiFive VICU7 manual chapter 6 (p 31)
const CLINT_BASE_ADDR: usize = 0x0200_0000;
const CLINT_HART1_MSIP: usize = CLINT_BASE_ADDR + 0x0004;
const CLINT_HART2_MSIP: usize = CLINT_BASE_ADDR + 0x0008;
const CLINT_HART3_MSIP: usize = CLINT_BASE_ADDR + 0x000c;
const CLINT_HART4_MSIP: usize = CLINT_BASE_ADDR + 0x0010;

// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/system_memory_map.html
const SPI_FLASH_BASE: usize = 0x2100_0000;

const DRAM_BLOB_BASE: usize = SPI_FLASH_BASE + 0x0001_0000;
const PAYLOAD_BASE: usize = SPI_FLASH_BASE + 0x0004_0000;

const MAIN_BLOB_BASE: usize = SRAM0_BASE + 30 * 1024;
const MAIN_BLOB_SIZE: usize = 2 * 1024;

const DTB_ADDR: usize = DRAM_BASE + 0x0020_0000 + 0x0100_0000; // TODO
const LOAD_ADDR: usize = DRAM_BASE + 0x0002_0000;
const LOAD_MAIN: bool = false;

const QSPI_XIP_BASE: usize = 0x2100_0000;
const LOAD_FROM_FLASH: bool = true;
const DEBUG: bool = false;

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
        "li     a1, 0",
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

    let count = addr_of!(_ebss) as usize - addr_of!(_sbss) as usize;
    ptr::write_bytes(addr_of_mut!(_sbss), 0, count);

    let count = addr_of!(_edata) as usize - addr_of!(_sdata) as usize;
    ptr::copy_nonoverlapping(addr_of!(_sidata), addr_of_mut!(_sdata), count);
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

static mut SERIAL: Option<uart::JH71XXSerial> = None;

fn init_logger(s: uart::JH71XXSerial) {
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
    pac::sys_syscon_reg().sys_syscfg_3().modify(|_, w| {
        w.vout0_remap_awaddr_gpio0().clear_bit();
        w.vout0_remap_awaddr_gpio1().clear_bit();
        w.vout0_remap_awaddr_gpio2().clear_bit();
        w.vout0_remap_awaddr_gpio3().clear_bit()
    });

    // TX/RX are GPIOs 5 and 6
    pac::sys_pinctrl_reg().gpo_doen_1().modify(|_, w| {
        w.doen_5().variant(0);
        w.doen_6().variant(0)
    });

    let mut s = uart::JH71XXSerial::new();
    init_logger(s);
    println!("oreboot 🦀");
    print_boot_mode();
    print_ids();

    let uart0_div = uart::uart0_divisor();
    println!("UART0 BAUD divisor: {uart0_div:#x}");

    // AXI cfg0, clk_apb_bus, clk_apb0, clk_apb12
    init::clk_apb0();
    // init::clk_apb_func();
    dram::init();

    // TODO: use this when we put Linux in flash etc
    println!("Copy payload... ⏳");
    let mut dtb: usize = 0;
    if LOAD_FROM_FLASH {
        let base = QSPI_XIP_BASE;
        // let size = 0x0100_0000; // 16M
        let size = 0x0020_0000; // occupied space
        let dram = DRAM_BASE;
        // let's find the dtb

        let slice = unsafe {
            let pointer = transmute(SRAM0_BASE);
            // The `slice` function creates a slice from the pointer.
            unsafe { core::slice::from_raw_parts(pointer, size) }
        };
        let fdt = find_fdt(slice);
        match fdt {
            Err(_) => {
                println!(
                    "Could not find an FDT between {:?} and {:?}",
                    SRAM0_BASE,
                    SRAM0_BASE + size
                );
            }
            Ok(f) => {
                dtb = SRAM0_BASE + 0x10000; // unsafe {(&f as *const _ as usize)};
                let it = &mut f.find_all_nodes("/flash-info/areas");
                let a = FdtIterator::new(it);
                for aa in a {
                    for c in aa.children() {
                        for p in c.properties() {
                            match p.name {
                                "size" => {
                                    println!("{:?} / {:?}, {:?}", c.name, p.name, p.as_usize());
                                }
                                _ => {
                                    println!("{:?} / {:?} {:?}", c.name, p.name, p.as_str());
                                }
                            }
                        }
                    }
                }
            }
        }

        for b in (0..size).step_by(4) {
            write32(dram + b, read32(base + b));
            if b % 0x4_0000 == 0 {
                print!(".");
            }
        }
        let size = 0x0010_0000; // occupied space
        let base = QSPI_XIP_BASE + 0x0030_00d4; // first 0xd4 is just 0-bytes
        let target = DRAM_BASE + 0x0020_0000;
        for b in (0..size).step_by(4) {
            write32(target + b, read32(base + b));
            if b % 0x4_0000 == 0 {
                print!(".");
            }
        }
        println!(" done.");
        if DEBUG {
            println!("Start:");
            dump_block(dram, 0x100, 0x20);
            println!("Presumably JH7110 recovery:");
            dump_block(dram + 0x0002_0000, 0x100, 0x20);
            println!("DTB:");
            dump_block(dram + 0x0010_0000, 0x100, 0x20);
            println!("Something:");
            dump_block(dram + 0x0020_0000, 0x100, 0x20);
        }
    } else {
        let base = 0x0800_0000 + 32 * 1024;
        let dram = DRAM_BASE;
        for b in (0..0x100).step_by(4) {
            write32(dram + b, read32(base + b));
            if b % 0x4_0000 == 0 {
                print!(".");
            }
        }
        println!("Payload:");
        dump_block(dram, 0x20, 0x20);
    }

    println!("lzss compressed Linux");
    dump_block(QSPI_XIP_BASE + 0x0040_0000, 0x100, 0x20);

    println!("release non-boot harts =====\n");
    {
        let clint = pac::clint_reg();
        clint.msip_1().write(|w| w.control().set_bit());
        clint.msip_2().write(|w| w.control().set_bit());
        clint.msip_3().write(|w| w.control().set_bit());
        clint.msip_4().write(|w| w.control().set_bit());
    }

    println!("Jump to payload... with dtb {dtb:#x}");
    exec_payload(dtb);
    println!("Exit from payload, resetting...");
    unsafe {
        sleep(0x0100_0000);
        reset();
        riscv::asm::wfi()
    };
}

fn exec_payload(dtb: usize) {
    let load_addr = if LOAD_FROM_FLASH {
        // U-Boot proper expects to be loaded here
        // see SYS_TEXT_BASE in U-Boot config
        DRAM_BASE + 0x0020_0000
    } else {
        DRAM_BASE
    };
    // println!("Payload @{load_addr:08x}");

    let hart_id = mhartid::read();
    // U-Boot proper
    unsafe {
        // jump to payload
        let f = transmute::<usize, EntryPoint>(load_addr);
        asm!("fence.i");
        f(hart_id, dtb);
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
