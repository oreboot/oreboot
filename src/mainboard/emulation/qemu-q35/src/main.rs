#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use payloads::payload;
use print;
use uart::null::Null;
//use core::ptr;

global_asm!(include_str!("../../../../arch/x86/x86_64/src/bootblock.S"));

//global_asm!(include_str!("init.S"));
//fn poke(v: u32, a: u32) -> () {
//    let y = a as *mut u32;
//    unsafe {
//        ptr::write_volatile(y, v);
//    }
//}

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let uart0 = &mut Null;
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    let payload = &mut payload::StreamPayload {
        typ: payload::ftype::CBFS_TYPE_SELF,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: 0x1000020 as usize,
        rom_len: 0 as usize,
        mem_len: 0 as usize,
        dtb: 0,
        rom: 0xff000000,
    };

    write!(w, "loading payload with fdt_address {}\r\n", fdt_address).unwrap();
    payload.load(w);
    write!(w, "Running payload\r\n").unwrap();
    payload.run(w);

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut Null;
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
