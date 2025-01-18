#![feature(naked_functions)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

use core::{
    arch::naked_asm,
    panic::PanicInfo,
    ptr::{read_volatile, write_volatile},
};

use bl808_pac::Peripherals;
#[macro_use]
extern crate log;

mod init;
mod psram;
mod uart;
mod util;

use util::{clear_bit, read32, set_bit, sleep, udelay, write32};

// UART0 is in the E907 (aka MCU aka M0) power domain
// UART3 is in the C906 (aka MM aka D0) power domain

const MM_GLB_BASE: usize = 0x3000_7000;
const MM_SW_SYS_RESET: usize = MM_GLB_BASE + 0x0040;

const PSRAM_CONFIGURE: usize = 0x2005_2000;

const PSRAM_BASE: usize = 0x5000_0000;
const PSRAM_SIZE: usize = 0x0400_0000;

const STACK_SIZE: usize = 4 * 1024; // 4KiB

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
pub unsafe extern "C" fn start() -> ! {
    naked_asm!(
        // 1. disable and clear interrupts
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        // 2. initialize programming language runtime
        // clear bss segment
        "la     t0, __bss_start",
        "la     t1, __bss_end",
        "2:",
        "bgeu   t0, t1, 1f",
        "sw     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      2b",
        "1:",
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "call   {main}",
        stack      = sym BT0_STACK,
        stack_size = const STACK_SIZE,
        main       = sym main,
    )
}

fn init_logger(u: uart::BSerial) {
    static ONCE: spin::Once<()> = spin::Once::new();

    ONCE.call_once(|| unsafe {
        static mut SERIAL: Option<uart::BSerial> = None;
        SERIAL.replace(u);
        log::init(SERIAL.as_mut().unwrap());
    });
}

fn check32(addr: usize, val: u32) {
    let v = read32(addr);
    if v != val {
        println!("Error @ {addr:08x}: expected {val:08x}, got {v:08x}");
    }
}

const DRAM_TEST_PATTERN_0: u32 = 0x2233_ccee;
const DRAM_TEST_PATTERN_1: u32 = 0x5577_aadd;
const DRAM_TEST_PATTERN_2: u32 = 0x1144_bbff;
const DRAM_TEST_PATTERN_3: u32 = 0x6688_9900;

fn dram_test(base: usize, size: usize) {
    let limit = base + size;
    let step_size = 0x100;
    // print 64 steps, which gets slower with a higher size to test
    let print_step = size / step_size / 64;

    println!("DRAM test: write patterns...");
    for (i, a) in (base..limit).step_by(step_size).enumerate() {
        if i % print_step == 0 {
            print!(".");
        }
        write32(a + 0x0, DRAM_TEST_PATTERN_0 | i as u32);
        write32(a + 0x4, DRAM_TEST_PATTERN_1 | i as u32);
        write32(a + 0x8, DRAM_TEST_PATTERN_2 | i as u32);
        write32(a + 0xc, DRAM_TEST_PATTERN_3 | i as u32);
    }
    println!();

    println!("DRAM test: reading back...");
    for (i, a) in (base..limit).step_by(step_size).enumerate() {
        if i % print_step == 0 {
            print!(".");
        }
        check32(a + 0x0, DRAM_TEST_PATTERN_0 | i as u32);
        check32(a + 0x4, DRAM_TEST_PATTERN_1 | i as u32);
        check32(a + 0x8, DRAM_TEST_PATTERN_2 | i as u32);
        check32(a + 0xc, DRAM_TEST_PATTERN_3 | i as u32);
    }
    println!();

    println!("DRAM test: done :)");
}

fn main() {
    let p = Peripherals::take().unwrap();
    let glb = p.GLB;
    // init::gpio_uart_init(&glb);
    let serial = uart::BSerial::new(p.UART0);
    init_logger(serial);

    udelay(0x5000);
    println!("oreboot 🦀");

    init::pll();
    psram::init();

    // dram_test(PSRAM_BASE, PSRAM_SIZE);
    dram_test(PSRAM_BASE, 0x0000_0200);

    unsafe {
        loop {
            println!("LOL");
            sleep();
        }
        let reset = read_volatile(MM_SW_SYS_RESET as *mut u32);
        write_volatile(MM_SW_SYS_RESET as *mut u32, reset | (1 << 2));
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
