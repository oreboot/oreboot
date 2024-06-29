#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

use core::{
    arch::asm,
    panic::PanicInfo,
    ptr::{read_volatile, write_volatile},
};
/*
const GLB_BASE: usize = 0x20000000;
const UART_CFG0: usize = GLB_BASE + 0x0150;
const UART_CFG1: usize = GLB_BASE + 0x0154;
const GPIO_CFG14: usize = GLB_BASE + 0x08fc;
const GPIO_CFG15: usize = GLB_BASE + 0x0900;
const GPIO_CFG16: usize = GLB_BASE + 0x0904;
const GPIO_CFG17: usize = GLB_BASE + 0x0908;
*/

// UART0 is in the E906 (aka MCU aka M0) power domain
const UART0_BASE: usize = 0x2000_a000;
const UART0_TX_CFG: usize = UART0_BASE;
const UART0_BIT_PRD: usize = UART0_BASE + 0x0008;
const UART0_FIFO_WDATA: usize = UART0_BASE + 0x0088;

const MM_GLB_BASE: usize = 0x3000_7000;
const MM_SW_SYS_RESET: usize = MM_GLB_BASE + 0x0040;

// UART3 is in the C907 (aka MM aka D0) power domain
/*
const UART3_BASE: usize = 0x3000_2000;
const UART3_TX_CFG: usize = UART3_BASE;
const UART3_BIT_PRD: usize = UART3_BASE + 0x0008;
const UART3_FIFO_WDATA: usize = UART3_BASE + 0x0088;

const UART_TX_STOP: u32 = 2 << 11; // stop bits
const UART_TX_LEN: u32 = 7 << 8; // word size
const UART_TX_FRM_EN: u32 = 1 << 2; // freerun mode
const UART_TX_EN: u32 = 1 << 0;
const UART_TX_CFG: u32 = UART_TX_STOP | UART_TX_LEN | UART_TX_FRM_EN | UART_TX_EN;
*/
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
        "li     t0, {sys_reset}",
        "lw     t1, 0(t0)",
        "ori    t1, t1, 1 << 2",
        "sw     t1, 0(t0)",
        stack      = sym BT0_STACK,
        stack_size = const STACK_SIZE,
        sys_reset  = const MM_SW_SYS_RESET,
        main       = sym main,
        options(noreturn)
    )
}

/*
const GPIO_MODE_IN: u32 = 1 << 0;
const GPIO_PULL_UP: u32 = 1 << 4;
const GPIO_MODE_OUT: u32 = 1 << 6;

const GPIO_FUN_UART: u32 = 7 << 8;
const GPIO_FUN_MM_UART: u32 = 21 << 8;
*/

fn sleep() {
    unsafe {
        for _ in 0..0x200000 {
            riscv::asm::nop();
        }
    }
}

const PSRAM_CONFIGURE: usize = 0x2005_2000;

const PSRAM_BASE: usize = 0x5000_0000;

fn main() {
    unsafe {
        /*
        /* GPIO mode config */
        let cfg_mm_uart_tx = GPIO_FUN_MM_UART | GPIO_MODE_OUT;
        let cfg_mm_uart_rx = GPIO_FUN_MM_UART | GPIO_PULL_UP | GPIO_MODE_IN;
        let cfg_uart_tx = GPIO_FUN_UART | GPIO_MODE_OUT;
        let cfg_uart_rx = GPIO_FUN_UART | GPIO_PULL_UP | GPIO_MODE_IN;
        write_volatile(GPIO_CFG14 as *mut u32, cfg_uart_tx);
        write_volatile(GPIO_CFG15 as *mut u32, cfg_uart_rx);
        write_volatile(GPIO_CFG16 as *mut u32, cfg_mm_uart_tx);
        write_volatile(GPIO_CFG17 as *mut u32, cfg_mm_uart_rx);

        /* GPIO UART function config */
        let cfg1 = read_volatile(UART_CFG1 as *mut u32);
        const GPIO14_UART0TX: u32 = 2 << 8;
        const GPIO15_UART0RX: u32 = 3 << 12;
        let uart_cfg = cfg1 & 0xffff00ff | GPIO14_UART0TX | GPIO15_UART0RX;
        write_volatile(UART_CFG1 as *mut u32, uart_cfg);

        /* Enable UART clock */
        let cfg0 = read_volatile(UART_CFG0 as *mut u32);
        write_volatile(UART_CFG0 as *mut u32, cfg0 | (1 << 4));

        // TX config
        write_volatile(UART0_TX_CFG as *mut u32, UART_TX_CFG);
        write_volatile(UART3_TX_CFG as *mut u32, UART_TX_CFG);

        /* baud rate configuration */
        // lower 16 bits are for TX; default (mask ROM) is 0x02b4 or 0x02b5
        let b0 = read_volatile(UART0_BIT_PRD as *mut u32);
        // let b1 = read_volatile(UART3_BIT_PRD as *mut u32);
        // set to the same as b0
        write_volatile(UART3_BIT_PRD as *mut u32, b0);
        */
        // CCCCCC.... AAAAAA....
        loop {
            write_volatile(UART0_FIFO_WDATA as *mut u32, 'C' as u32);
            write_volatile(UART0_FIFO_WDATA as *mut u32, '9' as u32);
            write_volatile(UART0_FIFO_WDATA as *mut u32, '0' as u32);
            write_volatile(UART0_FIFO_WDATA as *mut u32, '6' as u32);
            write_volatile(UART0_FIFO_WDATA as *mut u32, '\r' as u32);
            write_volatile(UART0_FIFO_WDATA as *mut u32, '\n' as u32);
            // write_volatile(UART3_FIFO_WDATA as *mut u32, 'A' as u32);
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
