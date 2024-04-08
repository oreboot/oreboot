#![feature(naked_functions, asm_const)]
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
    intrinsics::transmute,
    panic::PanicInfo,
    ptr::{self, addr_of, addr_of_mut},
    slice::from_raw_parts as slice_from,
};
use jh71xx_hal as hal;
use riscv::register::mhartid;
use riscv::register::{marchid, mimpid, mvendorid};

use layoutflash::areas::{find_fdt, FdtIterator};
use soc::starfive::jh7110::{pac, uart};
use starfive_visionfive2_lib::{dump, dump_block, read32, udelay, write32};

mod ddr_start;
mod ddrcsr;
mod ddrlib;
mod ddrphy;
mod dram;
mod init;
mod pll;

pub type EntryPoint = unsafe extern "C" fn();

const DUMP_OTP: bool = false;
const DEBUG: bool = false;
const BLINK_LED: bool = false;
const DUMP_PAYLOAD: bool = true;

// NOTE: JH, as in JH71x0, is short for JingHong, a city in Yunnan
// https://en.wikipedia.org/wiki/Jinghong

// The SRAM is called LIM, Loosely Integrated Memory
// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/u74_memory_map.html
const SRAM0_BASE: usize = 0x0800_0000;
const SRAM0_SIZE: usize = 0x0002_0000;

const ZEPHYR_LOAD_ADDR: usize = SRAM0_BASE + 0x0004_0000;

const DRAM_BASE: usize = 0x4000_0000;

// see https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/system_memory_map.html
const QSPI_XIP_BASE: usize = 0x2100_0000;
const FLASH_SIZE: usize = 0x0100_0000;
const LOAD_FROM_FLASH: bool = false;

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
    asm!(
        // Clear feature disable CSR to '0' to turn on all features
        // TODO: do in Rust
        "csrwi  0x7c1, 0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        "ld     t0, {start}",
        "csrw   mtvec, t0",
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
        stack      = sym BT0_STACK,
        stack_size = const STACK_SIZE,
        payload    = sym exec_payload,
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

static mut SERIAL: Option<uart::JH71XXSerial> = None;

#[inline]
// FIXME: restore for debugging
fn blink() {
    udelay(0x0004_0000);
    // write32(init::GPIO40_43_DATA, 0x8181_8181);
    udelay(0x0004_0000);
    // write32(init::GPIO40_43_DATA, 0x8080_8080);
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
    udelay(0x0004_0000);
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

// FIXME: just a quick hack
fn get_payload_offset_and_size(slice: &[u8]) -> (usize, usize) {
    (0x1_1000, 0x10000)
}

// Erratum: The manual says that the range is 0x0110_1000:0x0110_1fff,
// but also that the size is 8k. From the mask ROM, it is clear that
// 0x0110_0000 is the correct start address.
const DTIM_BASE: usize = 0x0110_0000;
const DTIM_SIZE: usize = 8 * 1024;

const OTPC_BASE: usize = 0x1705_0000;
const OTPC_SIZE: usize = 64 * 1024;

const SEC_SUB_SYS_BASE: usize = 0x1600_0000;

fn dump_dtim_otpc_sec() {
    if false {
        dump_block(DTIM_BASE, DTIM_SIZE, 0x20);
    }

    if false {
        dump_block(OTPC_BASE, OTPC_SIZE, 0x20);
        dump_block(OTPC_BASE, OTPC_SIZE, 0x20);
    }

    // those registers appear in the mask ROM; what are they?
    if false {
        dump(SEC_SUB_SYS_BASE + 0x0100, 4);
        dump(SEC_SUB_SYS_BASE + 0x0124, 4);
        dump(SEC_SUB_SYS_BASE + 0x0300, 4);
        dump(SEC_SUB_SYS_BASE + 0x0304, 4);
        dump(SEC_SUB_SYS_BASE + 0x0308, 4);
        dump(SEC_SUB_SYS_BASE + 0x0400, 4);
        dump(SEC_SUB_SYS_BASE + 0x0404, 4);
        dump(SEC_SUB_SYS_BASE + 0x040c, 4);
        dump(SEC_SUB_SYS_BASE + 0x0448, 4);
        dump(SEC_SUB_SYS_BASE + 0x044c, 4);
        dump(SEC_SUB_SYS_BASE + 0x04cc, 4);
        dump(SEC_SUB_SYS_BASE + 0x0508, 4);
        dump(SEC_SUB_SYS_BASE + 0x050c, 4);
        dump(SEC_SUB_SYS_BASE + 0x054c, 4);
        dump(SEC_SUB_SYS_BASE + 0x0588, 4);
        dump(SEC_SUB_SYS_BASE + 0x058c, 4);
        dump(SEC_SUB_SYS_BASE + 0x05cc, 4);
        dump(SEC_SUB_SYS_BASE + 0x060c, 4);
    }

    if false {
        dump_block(SEC_SUB_SYS_BASE, 2 * 1024, 0x20);
    }
}

fn print_otp_cfg(aon_syscon: &pac::AON_SYSCON) {
    println!("OTP config");
    let otp_cfg_1 = aon_syscon.aon_syscfg_6().read().bits();
    /*
     * | bit | name               | mode | default |
     * | --- | ------------------ | ---- | ------- |
     * | [0] | u0_otpc_chip_mode  | RO   | 0x0     |
     * | [1] | u0_otpc_crc_pass   | RO   | 0x0     |
     * | [2] | u0_otpc_dbg_enable | RO   | 0x0     |
     */
    println!("   {otp_cfg_1:032b}");
    let otp_cfg_2 = aon_syscon.aon_syscfg_7().read().bits();
    /*
     * [0:31] u0_otpc_fl_func_lock RO 0x0
     */
    println!("   {otp_cfg_2:032b}");
    let otp_cfg_3 = aon_syscon.aon_syscfg_8().read().bits();
    /*
     * [0:31] u0_otpc_fl_pll0_lock RO 0x0
     */
    println!("   {otp_cfg_3:032b}");
    let otp_cfg_4 = aon_syscon.aon_syscfg_9().read().bits();
    /*
     * [0:31] u0_otpc_fl_pll1_lock RO 0x0
     */
    println!("   {otp_cfg_4:032b}");
    let otp_cfg_5 = aon_syscon.aon_syscfg_10().read().bits();
    /*
     * | bit   | name                             | mode | default |
     * | ----- | -------------------------------- | ---- | ------- |
     * | [1]   | u0_otpc_fl_xip                   |  RO  | 0x0     |
     * | [2]   | u0_otpc_load_busy                |  RO  | 0x0     |
     * | [3]   | u0_reset_ctrl_clr_reset_status   |  WR  | 0x0     |
     * | [4]   | u0_reset_ctrl_pll_timecnt_finish |  RO  | 0x0     |
     * | [5]   | u0_reset_ctrl_rstn_sw            |  WR  | 0x1     |
     * | [9:6] | u0_reset_ctrl_sys_reset_status   |  RO  | 0x0     |
     */
    println!("   {otp_cfg_5:032b}");
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

    while dp.UART0.usr().read().busy() == true {}
    let s = uart::JH71XXSerial::new_with_config(
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

    const DISP_SUBSYS: usize = 0x2940_0000;
    const VOUT_SYSCON: usize = DISP_SUBSYS + 0x001B_0000;
    const VOUT_CRG: usize = DISP_SUBSYS + 0x001C_0000;
    const DSI_TX: usize = DISP_SUBSYS + 0x001D_0000;
    //  dump_block(VOUT_CRG + 0x0010, 0x80, 0x20);

    const VOUT_CLK_AXI: usize = VOUT_CRG + 0x0010;
    const VOUT_CLK_CORE: usize = VOUT_CRG + 0x0014;
    const VOUT_CLK_AHB: usize = VOUT_CRG + 0x0018;
    let v = read32(VOUT_CLK_AXI);
    println!("vout clk axi: {v:#010x}");
    let v = read32(VOUT_CLK_CORE);
    println!("vout clk core: {v:#010x}");
    let v = read32(VOUT_CLK_AHB);
    println!("vout clk ahb: {v:#010x}");

    const VOUT_RESET_CONTROL: usize = VOUT_CRG + 0x0038;
    const VOUT_RESET_STATUS: usize = VOUT_CRG + 0x004c;
    let vout_reset_status = read32(VOUT_RESET_STATUS);
    println!("vout_reset_status: {vout_reset_status:#010x}");

    let vout_reset_control = read32(VOUT_RESET_CONTROL);
    println!("vout_reset_control: {vout_reset_control:#010x}");
    write32(VOUT_RESET_CONTROL, vout_reset_control | 0x0fff);

    udelay(1000);

    let vout_reset_status = read32(VOUT_RESET_STATUS);
    println!("vout_reset_status: {vout_reset_status:#010x}");

    // 0x1302_0000 + 0xe8
    let vout_src = dp.SYSCRG.clk_u0_vout_src();
    let data = vout_src.read().bits();
    println!("vout_src: {data:#010x}");
    if false {
        vout_src.write(|w| unsafe { w.bits(data | 0x0fff) });
        let data = vout_src.read().bits();
        println!("vout_src: {data:#010x}");
    }

    if DUMP_OTP {
        print_otp_cfg(&dp.AON_SYSCON);
        dump_dtim_otpc_sec();
        panic!("WELP");
    }

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
    if false {
        reset_phy();
        init::phy_cfg();
    }

    // AXI cfg0, clk_apb_bus, clk_apb0, clk_apb12
    init::clk_apb0();
    dram::init();

    // Get slice to search for DTFS
    let (base, size) = if LOAD_FROM_FLASH {
        (QSPI_XIP_BASE, FLASH_SIZE)
    } else {
        (SRAM0_BASE, SRAM0_SIZE) // occupied space
    };
    let slice = unsafe { slice_from(transmute(base), size) };

    let mut load_addr = SRAM0_BASE + 0x1_1000;

    if false {
        // Find and copy the payload
        let (src_offset, src_size) = get_payload_offset_and_size(slice);
        let src_addr = base + src_offset;
        load_addr = ZEPHYR_LOAD_ADDR;

        println!(
            "[bt0] Copy {}k payload from {src_addr:08x} to {load_addr:08x}... ⏳",
            src_size / 1024
        );
        copy(src_addr, load_addr, src_size);
    }

    if true {
        // Find and copy the main stage
        let (main_offset, main_size) = get_main_offset_and_size(slice);
        let main_addr = base + main_offset;
        load_addr = DRAM_BASE;

        println!(
            "[bt0] Copy {}k main stage from {main_addr:08x} to {load_addr:08x}... ⏳",
            main_size / 1024
        );
        copy(main_addr, load_addr, main_size);
    }

    if DUMP_PAYLOAD {
        dump_block(load_addr, 0x20, 0x20);
    }

    // .....
    if false {
        println!("[bt0] Release non-boot harts =====\n");
        let clint = pac::clint_reg();
        clint.msip_0().write(|w| w.control().set_bit());
        clint.msip_2().write(|w| w.control().set_bit());
        clint.msip_3().write(|w| w.control().set_bit());
        clint.msip_4().write(|w| w.control().set_bit());
    }

    // GO!
    println!("[bt0] Jump to main stage @{load_addr:08x}");
    exec_payload(load_addr);
    println!("[bt0] Exit from main stage, resetting...");
    unsafe {
        udelay(0x0100_0000);
        reset();
        riscv::asm::wfi()
    };
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
        println!(
            "[bt0] panic in '{}' line {}",
            location.file(),
            location.line(),
        );
    } else {
        println!("[bt0] panic at unknown location");
    };
    if let Some(msg) = info.message() {
        println!("[bt0]   {msg}");
    }
    loop {
        core::hint::spin_loop();
    }
}
