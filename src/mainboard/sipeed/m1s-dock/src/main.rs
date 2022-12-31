#![feature(naked_functions, asm_sym, asm_const)]
#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    ptr::{read_volatile, slice_from_raw_parts, write_volatile},
};
use embedded_hal::serial::nb::Write;
use riscv;

const GLB_BASE: usize   = 0x20000000;
const UART_CFG0: usize  = GLB_BASE + 0x0150;
const UART_CFG1: usize  = GLB_BASE + 0x0154;
const SWRST_CFG2: usize = GLB_BASE + 0x0548;
const GPIO_CFG0: usize  = GLB_BASE + 0x08c4;
const GPIO_CFG11: usize = GLB_BASE + 0x08f0;
const GPIO_CFG12: usize = GLB_BASE + 0x08f4;
const GPIO_CFG13: usize = GLB_BASE + 0x08f8;
const GPIO_CFG14: usize = GLB_BASE + 0x08fc;
const GPIO_CFG15: usize = GLB_BASE + 0x0900;

const UART0_BASE: usize = 0x2000a000;
const UART0_TX_CFG: usize = UART0_BASE + 0x0000;
const UART0_FIFO_WDATA: usize = UART0_BASE + 0x0088;

const UART_TX_STOP: u32 = 2 << 11; // stop bits
const UART_TX_LEN: u32 = 7 << 8; // word size
const UART_TX_FRM_EN: u32 = 1 << 2; // freerun mode
const UART_TX_EN: u32 = 1 << 0;
const UART_TX_CFG: u32 = UART_TX_STOP | UART_TX_LEN | UART_TX_FRM_EN | UART_TX_EN;

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
    asm!(
        // 1. disable and clear interrupts
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        // 2. initialize programming language runtime
        // clear bss segment
        "la     t0, __bss_start",
        "la     t1, __bss_end",
        "1:",
        "bgeu   t0, t1, 1f",
        "sw     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "call   {main}",
        // reset
        "li     t0, {swrst_cfg2}",
        "lw     t1, 0(t0)",
        "ori    t1, t1, 1",
        "sw     t1, 0(t0)",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        swrst_cfg2 = const SWRST_CFG2,
        main       =   sym main,
        options(noreturn)
    )
}

fn sleep() {
    unsafe {
        for i in 0..0x200000 {
            riscv::asm::nop();
        }
    }
}

fn main() {
    unsafe {
        // Set GPIO14 function to UART0, output enable
        write_volatile(GPIO_CFG14 as *mut u32, (7 << 8) | (1 << 6));
        // Set GPIO15 function to UART0, input and pull up enable
        write_volatile(GPIO_CFG15 as *mut u32, (7 << 8) | (1 << 4)| (1 << 0));
        // Enable UART clock
        let cfg0 = read_volatile(UART_CFG0 as *mut u32);
        write_volatile(UART_CFG0 as *mut u32, cfg0 | (1 << 4));
        // Mux GPIO14 to UART0 TXD, GPIO15 to UART0 RXD
        let cfg1 = read_volatile(UART_CFG1 as *mut u32);
        write_volatile(UART_CFG1 as *mut u32, cfg1 | (3 << 4) | (2 << 0));
        // TX config
        write_volatile(UART0_TX_CFG as *mut u32, UART_TX_CFG);
        // CCCCCC........
        while true {
            write_volatile(UART0_FIFO_WDATA as *mut u32, 'C' as u32);
            sleep();
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
