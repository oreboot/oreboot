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

mod uart;
mod util;

use util::{read32, sleep, udelay, write32};

const GLB_BASE: usize = 0x20000000;
const UHS_PLL_CFG0: usize = GLB_BASE + 0x07d0;
const UHS_PLL_CFG1: usize = GLB_BASE + 0x07d4;
const UHS_PLL_CFG4: usize = GLB_BASE + 0x07e0;
const UHS_PLL_CFG5: usize = GLB_BASE + 0x07e4;
const UHS_PLL_CFG6: usize = GLB_BASE + 0x07e8;

/*
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

const PSRAM_CONTROLLER: usize = 0x3000_F000;
const PSRAM_CONFIGURE: usize = 0x2005_2000;

const PSRAM_BASE: usize = 0x5000_0000;

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

fn main() {
    let p = Peripherals::take().unwrap();
    let glb = p.GLB;
    // init::gpio_uart_init(&glb);
    let serial = uart::BSerial::new(p.UART0);
    init_logger(serial);

    udelay(0x5000);
    println!("oreboot 🦀");

    const UHS_PLL_CFG1_EVEN_DIV_EN: u32 = 1 << 7;
    const UHS_PLL_CFG1_EVEN_DIV_RATIO: u32 = 0b1111111;

    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");
    write32(UHS_PLL_CFG1, (cfg1 & !(0b11 << 16)));
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");
    write32(UHS_PLL_CFG1, (cfg1 & !(0b1111 << 8)) | (0b0010 << 8));
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");

    let cfg4 = read32(UHS_PLL_CFG4);
    println!("PLL CFG4: {cfg4:08x}");
    write32(UHS_PLL_CFG4, (cfg4 & !(0b11)) | 0b10);
    let cfg4 = read32(UHS_PLL_CFG4);
    println!("PLL CFG4: {cfg4:08x}");

    let cfg5 = read32(UHS_PLL_CFG5);
    println!("PLL CFG5: {cfg5:08x}");
    write32(UHS_PLL_CFG5, (cfg5 & !(0b111)) | 0b100);
    let cfg5 = read32(UHS_PLL_CFG5);
    println!("PLL CFG5: {cfg5:08x}");

    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");
    let m = !(UHS_PLL_CFG1_EVEN_DIV_EN | UHS_PLL_CFG1_EVEN_DIV_RATIO);
    write32(UHS_PLL_CFG1, (cfg1 & m) | UHS_PLL_CFG1_EVEN_DIV_EN | 28);
    let cfg1 = read32(UHS_PLL_CFG1);
    println!("PLL CFG1: {cfg1:08x}");

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
