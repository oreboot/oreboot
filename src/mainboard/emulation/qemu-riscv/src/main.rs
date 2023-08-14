#![doc = include_str!("README.md")]
#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

mod serial;

use core::arch::global_asm;
use core::panic::PanicInfo;
use log::println;
use ns16550a::*;
use serial::VirtSerial;

global_asm!(include_str!("bootblock.S"));
global_asm!(include_str!("init.S"));

const SERIAL_PORT_BASE_ADDRESS: usize = 0x1000_0000;

// MySerial implements embedded_hal_nb::serial::Write
fn init_logger(s: VirtSerial) {
    static ONCE: spin::Once<()> = spin::Once::new();

    ONCE.call_once(|| unsafe {
        static mut SERIAL: Option<VirtSerial> = None;
        SERIAL.replace(s);
        log::init(SERIAL.as_mut().unwrap());
    });
}

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    let uart = Uart::new(SERIAL_PORT_BASE_ADDRESS);
    uart.init(
        WordLength::EIGHT,
        StopBits::ONE,
        ParityBit::DISABLE,
        ParitySelect::EVEN,
        StickParity::DISABLE,
        Break::DISABLE,
        DMAMode::MODE0,
        100,
    );
    for c in "Rust oreboot\n".chars() {
        uart.put(c as u8);
    }
    init_logger(uart.into());
    println!("Hello world");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
