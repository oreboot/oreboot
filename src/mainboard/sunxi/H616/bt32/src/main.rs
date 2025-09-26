#![no_std]
#![no_main]
#![feature(once_cell_get_mut)]

use core::arch::naked_asm;
use core::{arch::asm, panic::PanicInfo};

use embedded_hal_nb::serial::Write;
use util::mmio::{read32, write32};

#[macro_use]
extern crate log;
mod mem_map;
mod uart;
mod dram;

use mem_map::CCU_BASE;

const STACK_SIZE: usize = 1 * 2048; // 1KiB

#[link_section = ".bss.uninit"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Clear stuff and jump to main.
/// All 64-bit capable Allwinner SoCs reset in AArch32 (and continue to
/// exectute the Boot ROM in this state), so we need to switch to AArch64
/// at some point.
/// https://github.com/u-boot/u-boot/blob/master/arch/arm/mach-sunxi/rmr_switch.S
/// See also: https://linux-sunxi.org/Arm64
///
/// # Safety
///
/// Naked function.
#[unsafe(naked)]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    naked_asm!(
        "2:",          //
        "adr  r9, 2b", //
        "bl   {reset}",
        ".word 0x140001e7", // b #0x490 in Aarch64
        reset      =   sym reset
    )
}

// see "Port controller" in the manual
const GPIO_BASE: usize = 0x0300_B000;
const GPIO_PORTC_CFG1: usize = GPIO_BASE + 0x004C; // PC8-15
const GPIO_PORTC_DATA: usize = GPIO_BASE + 0x0058;
const GPIO_PORTH_CFG0: usize = GPIO_BASE + 0x00FC;
const GPIO_PORTH_PULL: usize = GPIO_BASE + 0x0118;

const PC13_OUT: u32 = 0b001 << 20;
const PC13_HIGH: u32 = 1 << 13;

const APB2_CFG_REG: usize = CCU_BASE + 0x0524;
const UART_BGR_REG: usize = CCU_BASE + 0x090C;

// See U-Boot arch/arm/mach-sunxi/Kconfig
//   under config MACH_SUN50I_H6(16), there is select SUN50I_GEN_H6
// for H6 targets
const RVBAR: usize = 0x0901_0040;
// non-H6 targets
// const RVBAR: usize = 0x0170_00a0;

const RVBAR_ALT: usize = 0x0810_0040;
// for non-H6
// const RVBAR_ALT: usize = RVBAR;

const START_AARCH64: u32 = 0x0002_0000 + 2048;

fn sleep(t: usize) {
    for _ in 0..t {
        core::hint::spin_loop();
    }
}

// blink the LED on the MangoPi MQ-Quad
fn blink(delay: usize) {
    let cycs = delay * 0x10000;
    write32(GPIO_PORTC_DATA, PC13_HIGH);
    sleep(cycs);
    write32(GPIO_PORTC_DATA, 0);
    sleep(cycs);
}

#[inline]
fn save_regs() {
    #[allow(named_asm_labels)]
    unsafe {
        asm!(
            "b .code",
            // leave space to store information
            ".fel_stash: ",
            ".word 0x00000000", // SP
            ".word 0x00000000", // LR
            ".word 0x00000000", // CPSR
            ".word 0x00000000", // SCTLR
            ".word 0x00000000", // VBAR
            ".word 0x00000000", // SP_IRQ
            ".word 0x00000000", // ICC_PMR
            ".word 0x00000000", // ICC_IGRPEN1
            ".code:",
            "adr     r0, .fel_stash",
            "ldr     r1, .fel_stash",
            "add     r0, r0, r1",
            "str     sp, [r0]",
            "str     lr, [r0, #4]",
            "mrs     lr, CPSR",
            "str     lr, [r0, #8]",
            "mrc     p15, 0, lr, cr1, cr0, 0", // SCTLR
            "str     lr, [r0, #12]",
            "mrc     p15, 0, lr, cr12, cr0, 0", // VBAR
            "str     lr, [r0, #16]",
            "mrc     p15, 0, lr, cr12, cr12, 5", // ICC_SRE
            "tst     lr, #1",
            "beq     2f",
            "mrc     p15, 0, lr, c4, c6, 0", // ICC_PMR
            "str     lr, [r0, #24]",
            "mrc     p15, 0, lr, c12, c12, 7", // ICC_IGRPEN1
            "str     lr, [r0, #28]",
            "2:"
        );
    }
}

#[inline]
fn reset64() {
    if false {
        write32(RVBAR, START_AARCH64);
    } else {
        write32(RVBAR_ALT, START_AARCH64);
    }
    unsafe {
        asm!(
            "dsb	sy",
            "isb	sy",
            "mrc	p15, 0, r0, cr12, cr0, 2", // read RMR register
            "orr	r0, r0, #3",               // request reset in AArch64
            "mcr	p15, 0, r0, cr12, cr0, 2", // write RMR register
            "isb	sy",
        );
    }
}

// FIXME: both methods fail at the moment. Why?
fn init_logger(s: uart::SunxiSerial) {
    // This is the new method that also compiles in Rust 2024.
    use core::{cell::OnceCell, ptr::addr_of_mut};
    static mut SERIAL: OnceCell<uart::SunxiSerial> = OnceCell::new();
    unsafe {
        log::init((*addr_of_mut!(SERIAL)).get_mut_or_init(|| s));
    }
}

#[inline(always)]
// shift n by s and convert to what represents its hex digit in ASCII
fn shift_and_hex(n: u32, s: u8) -> u8 {
    // drop to a single nibble (4 bits), i.e., what a hex digit can hold
    let x = (n >> s) as u8 & 0x0f;
    // digits are in the range 0x30..0x39
    // letters start at 0x40, i.e., off by 7 from 0x3a
    if x > 9 {
        x + 0x37
    } else {
        x + 0x30
    }
}

#[inline(always)]
pub fn print_hex(s: &mut uart::SunxiSerial, i: u32) {
    s.write(b'0').ok();
    s.write(b'x').ok();
    // nibble by nibble... keep it simple
    s.write(shift_and_hex(i, 28)).ok();
    s.write(shift_and_hex(i, 24)).ok();
    s.write(shift_and_hex(i, 20)).ok();
    s.write(shift_and_hex(i, 16)).ok();
    s.write(shift_and_hex(i, 12)).ok();
    s.write(shift_and_hex(i, 8)).ok();
    s.write(shift_and_hex(i, 4)).ok();
    s.write(shift_and_hex(i, 0)).ok();
    s.write(b'\r').ok();
    s.write(b'\n').ok();
}

// #[unsafe(naked)]
#[no_mangle]
unsafe extern "C" fn reset() {
    // stack setup
    asm!("mov sp, {}", in(reg) &raw const STACK);
    asm!(
        "ldr  r1, ={stack_size}",
        "add  sp, r1",
        "bl   {main}",
        stack_size = const STACK_SIZE,
        main       = sym main
    );
}

const PRINT_PC: bool = false;
const PRINT_SP: bool = true;

// see also https://iitd-plos.github.io/col718/ref/arm-instructionset.pdf
#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut ini_pc: usize = 0;
    unsafe { asm!("mov {}, r9", out(reg) ini_pc) };
    let mut ini_sp: usize = 0;
    unsafe { asm!("mov {}, sp", out(reg) ini_sp) };

    // System init: select APB@24MHz
    let v = read32(APB2_CFG_REG) & !(0b11 << 24);
    write32(APB2_CFG_REG, v);

    // set PC13 (status LED) to output
    write32(GPIO_PORTC_CFG1, PC13_OUT);
    // first sign of life
    blink(5);

    // UART0: TX on port H pin 0, RX on port H pin 1
    let v = read32(GPIO_PORTH_CFG0) & 0xffff_ff00;
    write32(GPIO_PORTH_CFG0, v | (0b010 << 4) | (0b010 << 0));
    let v = read32(GPIO_PORTH_PULL) & 0xffff_fff0;
    write32(GPIO_PORTH_PULL, v | (0b01 << 2) | (0b01 << 0));

    const UART0_GATING: u32 = 1 << 16;
    const UART0_RESET: u32 = 1 << 0;
    // deassert reset
    let v = read32(UART_BGR_REG) & !UART0_GATING;
    write32(UART_BGR_REG, v | UART0_GATING);
    // gating pass
    let v = read32(UART_BGR_REG) & !UART0_RESET;
    write32(UART_BGR_REG, v | UART0_RESET);

    let serial = uart::SunxiSerial::new();
    init_logger(serial);
    println!("oreboot 🦀 in aarch32");
    println!("  program counter (PC): {ini_pc:016x}");
    println!("    stack pointer (SP): {ini_sp:016x}");

    loop {
        blink(42);
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
