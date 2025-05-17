#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

use embedded_hal_nb::serial::Write;

#[macro_use]
extern crate log;

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};
use log::println;

use util::mmio::write32;

mod arm;
mod mem_map;
mod uart;

const DUMP_ROM: bool = false;

pub const MASK_ROM_SIZE: usize = 32 * 1024;

const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// # Safety
///
/// Naked function.
#[unsafe(naked)]
#[export_name = "start"]
#[link_section = ".text.entry"]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn start() -> ! {
    naked_asm!(
        "init:",
        "adr     x9, init",
        // set counter timer frequency to 24MHz
        "mov     w0,#0x3600",
        "movk    w0,#0x016e, LSL #16",
        "msr     cntfrq_el0, x0",
        // 3. prepare stack
        "ldr     x1, {stack}",
        "str     x5, [x4], #0",
        "ldr     x1, {stack_size}",
        "str     x5, [x4], #0",
        "bl      {main}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        main       =   sym main
    )
}

static mut SERIAL: Option<uart::RKSerial> = None;

fn init_logger(s: uart::RKSerial) {
    unsafe {
        SERIAL.replace(s);
        if let Some(m) = SERIAL.as_mut() {
            log::init(m);
        }
    }
}

// PMU GRF (power management unit general register file) GPIO0 D IO
// multiplexing low register
const PMU_GRF_GPIO0D_IOMUX_L: usize = mem_map::PMU_GRF_BASE + 0x0018;

// SYS GRF IO function selection 3
const GRF_IOFUNC_SEL3: usize = mem_map::SYS_GRF_BASE + 0x030c;

const TIMER1: usize = mem_map::TIMER_BASE + 0x0020;

#[no_mangle]
extern "C" fn main() -> usize {
    let mut ini_pc: usize = 0;
    unsafe { asm!("mov {}, x9", out(reg) ini_pc) };

    // system timer 1 init: set to highest value
    // timer control: disable
    write32(TIMER1 + 0x10, 0x0);
    // load count register 0
    write32(TIMER1 + 0x00, 0xffffffff);
    // load count register 1
    write32(TIMER1 + 0x04, 0xffffffff);
    // timer control: enable
    write32(TIMER1 + 0x10, 0x1);

    // GPIO / UART
    // 31-16: write enable; 11: 0d1 = UART2 TX, 0d0 = UART2 RX
    write32(PMU_GRF_GPIO0D_IOMUX_L, 0x0077_0011);
    // 31-16: write enable
    write32(GRF_IOFUNC_SEL3, 0x0c00_0000);

    let mut s = uart::RKSerial::new();

    init_logger(s);
    println!();
    println!("oreboot 🦀 bt0 on Arm");
    println!("initial program counter (PC) {ini_pc:016x}");

    arm::print_cpuinfo();

    util::mem::dump_block(mem_map::SRAM_BASE, 0x40, 0x20);
    // 90010000 01000000 9073a64e 00000000  0a000000 3b8523df c2ddcbfa f73f8fff
    // fa6e0115 a4dee7b8 e0a7fb59 00a1b0da  00000000 3cd9f1cf 00000000 3c7d4ce1

    if DUMP_ROM {
        println!(">>> mask ROM dump");
        util::mem::dump_block(mem_map::MASK_ROM_BASE, MASK_ROM_SIZE, 32);
        println!("<<< mask ROM dump");
    }

    // TODO
    // dram::init();
    println!("TODO: DRAM init");

    exec_payload(0);

    panic!("DO NOT PANIC! EVERYTHING IS OKAY!");
    0
}

pub type EntryPoint = unsafe extern "C" fn();

// jump to main stage or payload
fn exec_payload(addr: usize) {
    unsafe {
        let f: EntryPoint = core::intrinsics::transmute(addr);
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
    };
    let msg = info.message();
    println!("[bt0]   {msg}");
    loop {
        core::hint::spin_loop();
    }
}
