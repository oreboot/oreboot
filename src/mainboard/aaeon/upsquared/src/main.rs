#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use print;
use uart::debug_port::DebugPort;
use uart::i8250::I8250;

global_asm!(include_str!("../../../../arch/x86/x86_64/src/bootblock.S"));

#[no_mangle]
pub extern "C" fn _start(_fdt_address: usize) -> ! {
    let io = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    // Note: on real hardware, use port 0x80 instead for "POST" output
    let debug_io = &mut IOPort;
    let debug = &mut DebugPort::new(0xe9, debug_io);
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    debug.init().unwrap();
    debug.pwrite(b"Welcome to oreboot - debug port E9\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    // TODO: Get these values from the fdt
    let payload = &mut BzImage { low_mem_size: 0x80_000_000, high_mem_start: 0x1_000_000_000, high_mem_size: 0, rom_base: 0xff_000_000, rom_size: 0x1_000_000, load: 0x1_000_000, entry: 0x1_000_200 };

    payload.load(w).unwrap();

    write!(w, "Running payload\r\n").unwrap();
    payload.run(w);

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let io = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
