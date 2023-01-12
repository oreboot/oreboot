#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

use core::{
    arch::asm,
    panic::PanicInfo,
    ptr::write_volatile,
    // ptr::slice_from_raw_parts,
};
use riscv::register::{marchid, mhartid, mimpid, mvendorid};

mod init;
use init::SWRST_CFG2;
#[macro_use]
mod log;

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
}

fn main() {
    unsafe {
        init::gpio_uart_init();
        init::uart_init();
        let serial = init::Serial::new();
        log::set_logger(serial);

        println!("oreboot 🦀");
        riscv_plat_info();

        // AAAAAA.... 🐢....
        loop {
            write_volatile(init::UART0_FIFO_WDATA as *mut u32, 'A' as u32);
            println!("🐢");
            sleep();
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        write_volatile(init::UART0_FIFO_WDATA as *mut u32, 'X' as u32);
    }
    loop {
        core::hint::spin_loop();
    }
}
