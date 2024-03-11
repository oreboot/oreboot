#![doc = include_str!("README.md")]
#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use ns16550a::*;
global_asm!(include_str!("bootblock.S"));
global_asm!(include_str!("init.S"));

const SERIAL_PORT_BASE_ADDRESS: usize = 0x1000_0000;

// To receive a byte:
//let data = serial_port.receive();

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
    loop {
        uart.put(uart.get().unwrap_or_default());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
