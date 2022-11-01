#![doc = include_str!("README.md")]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uart_16550::MmioSerialPort;

const SERIAL_PORT_BASE_ADDRESS: usize = 0x1000_0000;

// To receive a byte:
//let data = serial_port.receive();

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    loop {
        let mut serial_port = unsafe { MmioSerialPort::new(SERIAL_PORT_BASE_ADDRESS) };
        serial_port.init();

        // Now the serial port is ready to be used. To send a byte:
        serial_port.send(42);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
