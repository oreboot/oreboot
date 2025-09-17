#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};

const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Clear stuff and jump to main.
/// Kudos to Azeria \o/
/// https://azeria-labs.com/memory-instructions-load-and-store-part-4/
/// Xn registers are 64-bit, general purpose; X31 aka Xzr is always 0
/// Wn registers are 32-bit and aliases of lower half of Xn
/// https://linux-sunxi.org/Arm64
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        "bl   {main}", // TODO: remove
        // 2. initialize programming language runtime
        // clear bss segment
        "ldr     w1, sbss",
        "ldr     w2, ebss",
        "1:",
        // jump out of loop once x2 reaches x1
        "sub     w3, w2, w1",
        "cbz     w3, 1f",
        // clear out the respective address in memory
        "str     w0, [x2], #0",
        "sub     w2, w2, 4",
        "bl      1b",
        "1:",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "ldr     x1, {stack}",
        "mov     sp, x1",
        "ldr     w1, {stack_size}",
        "add     sp, sp, x1",
        // jump to main :)
        "bl   {main}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        main       =   sym main,
        options(noreturn)
    )
}

// H0 is TX, H1 is RX
// PC13 is status LED

// p695
const GPIO_BASE: usize = 0x0300_B000;
const GPIO_PORTC_CFG0: usize = GPIO_BASE + 0x0048;
const GPIO_PORTC_CFG1: usize = GPIO_BASE + 0x004C; // PC8-15
const GPIO_PORTC_CFG2: usize = GPIO_BASE + 0x0050;
const GPIO_PORTC_CFG3: usize = GPIO_BASE + 0x0054;
const GPIO_PORTC_DATA: usize = GPIO_BASE + 0x0058;
const PC13_OUT: u32 = 0b001 << 20;
const PC13_HIGH: u32 = 1 << 13;

extern "C" fn main() -> usize {
    unsafe {
        // set PC13 high (status LED)
        write_volatile(GPIO_PORTC_CFG1 as *mut u32, PC13_OUT); // set to out
        for _ in 0..3 {
            write_volatile(GPIO_PORTC_DATA as *mut u32, PC13_HIGH);
            for _ in 0..0x1f0000 {
                core::hint::spin_loop();
            }
            write_volatile(GPIO_PORTC_DATA as *mut u32, 0);
            for _ in 0..0x1f0000 {
                core::hint::spin_loop();
            }
        }
    }
    // TODO: code.....
    0
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    /*
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };
    */
    loop {
        core::hint::spin_loop();
    }
}
