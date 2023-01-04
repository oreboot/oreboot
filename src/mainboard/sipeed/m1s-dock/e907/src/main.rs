#![feature(naked_functions, asm_sym, asm_const)]
#![no_std]
#![no_main]

/*
    GPIO_FUN_SDH = 0,
    GPIO_FUN_SPI0 = 1,
    GPIO_FUN_FLASH = 2,
    GPIO_FUN_I2S = 3,
    GPIO_FUN_PDM = 4,
    GPIO_FUN_I2C0 = 5,
    GPIO_FUN_I2C1 = 6,
    GPIO_FUN_UART = 7,
    GPIO_FUN_ETHER_MAC = 8,
    GPIO_FUN_CAM = 9,
    GPIO_FUN_ANALOG = 10,
    GPIO_FUN_GPIO = 11,
    GPIO_FUN_PWM0 = 16,
    GPIO_FUN_PWM1 = 17,
    GPIO_FUN_SPI1 = 18,
    GPIO_FUN_I2C2 = 19,
    GPIO_FUN_I2C3 = 20,
    GPIO_FUN_MM_UART = 21,
    GPIO_FUN_DBI_B = 22,
    GPIO_FUN_DBI_C = 23,
    GPIO_FUN_DPI = 24,
    GPIO_FUN_JTAG_LP = 25,
    GPIO_FUN_JTAG_M0 = 26,
    GPIO_FUN_JTAG_D0 = 27,
    GPIO_FUN_CLOCK_OUT = 31,
 */

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    ptr::{read_volatile, slice_from_raw_parts, write_volatile},
};
use embedded_hal::serial::nb::Write;
use riscv;

const GLB_BASE: usize   = 0x2000_0000;
const UART_CFG0: usize  = GLB_BASE + 0x0150;
const UART_CFG1: usize  = GLB_BASE + 0x0154;
const SWRST_CFG2: usize = GLB_BASE + 0x0548;
const GPIO_CFG0: usize  = GLB_BASE + 0x08c4;
const GPIO_CFG11: usize = GLB_BASE + 0x08f0;
const GPIO_CFG12: usize = GLB_BASE + 0x08f4;
const GPIO_CFG13: usize = GLB_BASE + 0x08f8;
const GPIO_CFG14: usize = GLB_BASE + 0x08fc;
const GPIO_CFG15: usize = GLB_BASE + 0x0900;
const GPIO_CFG16: usize = GLB_BASE + 0x0904;
const GPIO_CFG17: usize = GLB_BASE + 0x0908;

const UART0_BASE: usize = 0x2000_a000;
const UART0_TX_CFG: usize = UART0_BASE + 0x0000;
const UART0_BIT_PRD: usize = UART0_BASE + 0x0008;
const UART0_FIFO_WDATA: usize = UART0_BASE + 0x0088;

const UART1_BASE: usize = 0x2000_a100;
const UART1_TX_CFG: usize = UART1_BASE + 0x0000;
const UART1_BIT_PRD: usize = UART1_BASE + 0x0008;
const UART1_FIFO_WDATA: usize = UART1_BASE + 0x0088;

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

/**
|    31    30    29    28                 27    26    25    24    |

|    23    22    21    20                 19    18    17    16    |

|    15    14    13    12                 11    10     9     8    |
                       ------------ GPIO function/mode -----------
|     7     6     5     4                  3     2     1     0    |
          output      pull up                              input
*/

const GPIO_FUN_UART: u32 = 7 << 8;
const GPIO_FUN_MM_UART: u32 = 21 << 8;

const GPIO14_UART0TX: u32 = 2 << 8;
const GPIO15_UART0RX: u32 = 3 << 12;
const GPIO16_UART1TX: u32 = 6 << 16;
const GPIO17_UART1RX: u32 = 7 << 20;
const UART_GPIO_CFG: u32 = GPIO14_UART0TX | GPIO15_UART0RX | GPIO16_UART1TX | GPIO17_UART1RX;
const UART_GPIO_MASK: u32 = 0xff0000ff;

const UART_CLK_EN: u32 = 1 << 4;

const GPIO_MODE_IN: u32 = 1 << 0;
const GPIO_PULL_UP: u32 = 1 << 4;
const GPIO_MODE_OUT: u32 = 1 << 6;

unsafe fn gpio_uart_init() {
    /* GPIO mode config */
    let cfg_uart_tx = GPIO_FUN_UART | GPIO_MODE_OUT;
    let cfg_uart_rx = GPIO_FUN_UART | GPIO_PULL_UP | GPIO_MODE_IN;
    write_volatile(GPIO_CFG14 as *mut u32, cfg_uart_tx);
    write_volatile(GPIO_CFG15 as *mut u32, cfg_uart_rx);
    write_volatile(GPIO_CFG16 as *mut u32, cfg_uart_tx);
    write_volatile(GPIO_CFG17 as *mut u32, cfg_uart_rx);
    
    /* GPIO UART function config */
    // GPIO14: UART0 TXD
    // GPIO15: UART0 RXD
    // GPIO16: UART1 TXD
    // GPIO17: UART1 RXD
    let cfg1 = read_volatile(UART_CFG1 as *mut u32);
    let uart_cfg = cfg1 & UART_GPIO_MASK | UART_GPIO_CFG;
    write_volatile(UART_CFG1 as *mut u32, uart_cfg);

    /* Enable UART clock */
    let cfg0 = read_volatile(UART_CFG0 as *mut u32);
    write_volatile(UART_CFG0 as *mut u32, cfg0 | UART_CLK_EN);
}

fn main() {
    unsafe {
        gpio_uart_init();

        // TX config
        write_volatile(UART0_TX_CFG as *mut u32, UART_TX_CFG);
        write_volatile(UART1_TX_CFG as *mut u32, UART_TX_CFG);

        /* baud rate configuration */
        // lower 16 bits are for TX; default (mask ROM) is 0x02b4 or 0x02b5
        let b0 = read_volatile(UART0_BIT_PRD as *mut u32);
        let b1 = read_volatile(UART1_BIT_PRD as *mut u32);
        // set to the same as b0
        write_volatile(UART1_BIT_PRD as *mut u32, b0);

        // CCCCCC.... AAAAAA....
        while true {
            write_volatile(UART0_FIFO_WDATA as *mut u32, 'C' as u32);
            write_volatile(UART1_FIFO_WDATA as *mut u32, 'A' as u32);
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
