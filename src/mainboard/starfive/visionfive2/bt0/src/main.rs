#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

use embedded_hal_nb::serial::Write;

#[macro_use]
extern crate log;
extern crate jh71xx_hal as hal;
use layoutflash::areas::{find_fdt, FdtIterator};

use core::{
    arch::asm,
    intrinsics::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
};
use riscv::register::mhartid;
use riscv::register::{marchid, mimpid, mvendorid};
use starfive_visionfive2_lib::{dump_block, read32, udelay, write32};
use uart::JH71XXSerial;

use fdt::Fdt;

use hal::uart::Serial;
use soc::starfive::jh7110::{pac, uart};

mod ddr_start;
mod ddrcsr;
mod ddrlib;
mod ddrphy;
mod dram;
mod init;
mod pll;

pub type EntryPoint = unsafe extern "C" fn();

const DEBUG: bool = true;
const BLINK_LED: bool = false;

// NOTE: JH, as in JH71x0, is short for JingHong, a city in Yunnan
// https://en.wikipedia.org/wiki/Jinghong

// The SRAM is called LIM, Loosely Integrated Memory
// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/u74_memory_map.html
const SRAM0_BASE: usize = 0x0800_0000;
const SRAM0_SIZE: usize = 0x0002_0000;

const DRAM_BASE: usize = 0x4000_0000;

// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/system_memory_map.html
const QSPI_XIP_BASE: usize = 0x2100_0000;
const FLASH_SIZE: usize = 0x0100_0000;
const LOAD_FROM_FLASH: bool = false;

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

    let bss_size = addr_of!(_ebss) as usize - addr_of!(_sbss) as usize;
    ptr::write_bytes(addr_of_mut!(_sbss), 0, bss_size);

    let data_size = addr_of!(_edata) as usize - addr_of!(_sdata) as usize;
    ptr::copy_nonoverlapping(addr_of!(_sidata), addr_of_mut!(_sdata), data_size);
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
    // See U74-MC core complex manual 21G3.
    println!("RISC-V arch {aid:08x}");
    let vendor_name = vendorid_to_name(vid);
    println!("RISC-V core vendor: {vendor_name} (0x{vid:04x})");
    let imp_name = impid_to_name(iid);
    println!("RISC-V implementation: {imp_name} (0x{iid:08x})");
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {hart_id}");
}

fn sleep(n: u32) {
    for _ in 0..n {
        unsafe { core::hint::spin_loop() };
    }
}

static mut SERIAL: Option<uart::JH71XXSerial> = None;

#[inline]
// FIXME: restore for debugging
fn blink() {
    /*
    sleep(0x0004_0000);
    write32(init::GPIO40_43_DATA, 0x8181_8181);
    sleep(0x0004_0000);
    write32(init::GPIO40_43_DATA, 0x8080_8080);
    */
}

fn init_logger(s: uart::JH71XXSerial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
}

// TODO: registers
// GPIO 13 is GMAC PHY reset (negative?)
fn reset_phy() {
    let gpio12_15_en = read32(init::GPIO12_15_EN);
    let gpio12_15_data = read32(init::GPIO12_15_DATA);
    println!("inital GPIO 12-15 en/data {gpio12_15_en:08x}/{gpio12_15_data:08x}");
    write32(
        init::GPIO12_15_DATA,
        (gpio12_15_data & 0xffff00ff) | (0x81 << 8),
    );
    unsafe { sleep(0x0004_0000) };
    write32(
        init::GPIO12_15_DATA,
        (gpio12_15_data & 0xffff00ff) | (0x80 << 8),
    );
}

fn get_main_offset_and_size(slice: &[u8]) -> (usize, usize) {
    let mut size = 0;
    if let Ok(fdt) = find_fdt(slice) {
        let mut offset = 0;
        let mut found = false;
        let areas = &mut fdt.find_all_nodes("/flash-info/areas");
        // TODO: make finding the main stage more sophisticated
        if DEBUG {
            dump_block(SRAM0_BASE + offset, 0x20, 0x20);
        }
        println!("💾 oreboot DTFS");
        for a in FdtIterator::new(areas) {
            for c in a.children() {
                let cname = c.name;
                for p in c.properties() {
                    let pname = p.name;
                    match pname {
                        "size" => {
                            let v = p.as_usize();
                            println!("  {cname} / {pname}, {v:?}");
                            let psize = v.unwrap_or(0);
                            if !found {
                                if DEBUG {
                                    println!("No main stage yet, inc offset by 0x{psize:x}");
                                }
                                offset += psize;
                            }
                            if found && size == 0 {
                                size = psize;
                            }
                            if DEBUG {
                                dump_block(SRAM0_BASE + offset, 0x20, 0x20);
                            }
                        }
                        _ => {
                            let s = p.as_str().unwrap_or("[empty]");
                            println!("  {cname} / {pname}, {s}");
                            if pname == "compatible" && s == "ore-main" {
                                found = true;
                            }
                        }
                    }
                }
            }
        }
        // FIXME: When in SRAM, the header is cut off!
        offset = if LOAD_FROM_FLASH {
            offset
        } else {
            offset - 0x400
        };
        (offset, size)
    } else {
        // FIXME: return error, let the main function print
        println!(
            "Could not find an FDT between {SRAM0_BASE:08x} and {:08x}",
            SRAM0_BASE + slice.len()
        );
        (0, size)
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

    // FIXME: restore for debugging
    if BLINK_LED {
        // enable is active low
        // write32(init::GPIO40_43_EN, 0xc0c0_c0c0);
        // write32(init::GPIO40_43_DATA, 0x8181_8181);
        blink();
    }

    // TX/RX are GPIOs 5 and 6
    pac::sys_pinctrl_reg().gpo_doen_1().modify(|_, w| {
        w.doen_5().variant(0);
        w.doen_6().variant(0b1)
    });

    pac::sys_pinctrl_reg()
        .gpo_dout_1()
        .modify(|_, w| w.dout_5().variant(20));
    pac::sys_pinctrl_reg()
        .gpi_3()
        .modify(|_, w| w.uart_sin_0().variant(6));

    let dp = pac::Peripherals::take().unwrap();

    let mut s = uart::JH71XXSerial::new_with_config(
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

    init_logger(s);
    println!("oreboot 🦀 bt0");
    print_boot_mode();
    print_ids();

    if DEBUG {
        println!("Stock firmware in flash");
        println!("Start:");
        dump_block(QSPI_XIP_BASE, 0x100, 0x20);
        println!("Presumably JH7110 recovery:");
        dump_block(QSPI_XIP_BASE + 0x0002_0000, 0x100, 0x20);
        println!("DTB:");
        dump_block(QSPI_XIP_BASE + 0x0010_0000, 0x100, 0x20);
        println!("Something:");
        dump_block(QSPI_XIP_BASE + 0x0020_0000, 0x100, 0x20);
        // we put this here
        println!("lzss compressed Linux");
        dump_block(QSPI_XIP_BASE + 0x0040_0000, 0x100, 0x20);
    }

    // TODO: Does this help?
    if true {
        reset_phy();
        init::phy_cfg();
    }

    // AXI cfg0, clk_apb_bus, clk_apb0, clk_apb12
    init::clk_apb0();
    dram::init();

    // Find and copy the main stage
    let (base, size) = if LOAD_FROM_FLASH {
        (QSPI_XIP_BASE, FLASH_SIZE)
    } else {
        (SRAM0_BASE, SRAM0_SIZE) // occupied space
    };
    let slice = unsafe { core::slice::from_raw_parts(transmute(base), size) };
    let (main_offset, main_size) = get_main_offset_and_size(slice);
    let main_addr = base + main_offset;

    let load_addr = DRAM_BASE;

    let main_size_k = main_size / 1024;
    println!("Copy {main_size_k}k main stage from {main_addr:08x} to {load_addr:08x}... ⏳");
    for b in (0..main_size).step_by(4) {
        write32(load_addr + b, read32(main_addr + b));
        if b % 0x4_0000 == 0 {
            print!(".");
        }
    }
    println!(" done.");
    if DEBUG {
        dump_block(load_addr, 0x20, 0x20);
    }

    // .....
    if false {
        println!("release non-boot harts =====\n");
        let clint = pac::clint_reg();
        clint.msip_0().write(|w| w.control().set_bit());
        clint.msip_2().write(|w| w.control().set_bit());
        clint.msip_3().write(|w| w.control().set_bit());
        clint.msip_4().write(|w| w.control().set_bit());
    }

    // GO!
    println!("Jump to main stage @{load_addr:08x}");
    exec_payload(load_addr);
    println!("Exit from main stage, resetting...");
    unsafe {
        sleep(0x0100_0000);
        reset();
        riscv::asm::wfi()
    };
}

fn exec_payload(addr: usize) {
    unsafe {
        // jump to main
        let f: EntryPoint = transmute(addr);
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
