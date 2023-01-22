#![feature(default_alloc_error_handler)]
#![feature(naked_functions, asm_const)]
#![feature(associated_type_bounds)]
#![no_std]
#![no_main]

use bl808_pac::Peripherals;
use core::{
    arch::asm,
    panic::PanicInfo,
    // ptr::slice_from_raw_parts,
};
#[macro_use]
extern crate log;
use riscv::register::{marchid, mhartid, mimpid, mvendorid};

mod init;

const BOARD_SOC: &str = "Bouffalo Lab BL808";
const BOARD_NAME: &str = "Sipeed M1S Dock";

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
        "csrw   mtvec, t0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        // 2. initialize programming language runtime
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
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
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        main       =   sym main,
        options(noreturn)
    )
}

fn sleep() {
    unsafe {
        for _ in 0..0x200000 {
            riscv::asm::nop();
        }
    }
}

fn riscv_plat_info() {
    let vid = mvendorid::read().map(|r| r.bits()).unwrap_or(0);
    let arch = marchid::read().map(|r| r.bits()).unwrap_or(0);
    let imp = mimpid::read().map(|r| r.bits()).unwrap_or(0);
    println!("RISC-V vendor {:x} arch {:x} imp {:x}", vid, arch, imp);
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {}", hart_id);
    println!("Board: {BOARD_NAME}");
    println!("SoC: {BOARD_SOC}");
}

/**
 * There are multiple UARTs available. We configure the first two to 115200
 * bauds.
 * We use UART0 like for classic POST codes via `Serial`'s `_debug` function.
 * The `print`/`println` macros output to UART1 for rich output.
 * Note that UART0 is really only for debugging here, and we want to use it for
 * the D0 core (64-bit "MM"/multimedia) otherwise.
 */
fn main() {
    let p = Peripherals::take().unwrap();
    let glb = p.GLB;
    init::gpio_uart_init(&glb);
    let serial = init::BSerial::new(p.UART0, p.UART1);
    log::set_logger(serial);

    // print to UART0
    log::_debug(42);

    // prints to UART1
    println!("oreboot 🦀");
    riscv_plat_info();
    println!("{}", glb.chip_inform.read().bits());

    for _ in 0..4 {
        println!("🐢");
        sleep();
    }
    init::resume_mm();
    init::reset_cpu();
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("panic {:?}", info);
    loop {
        core::hint::spin_loop();
    }
}
