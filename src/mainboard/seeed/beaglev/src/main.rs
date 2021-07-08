#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

//use core::fmt::Write;
use core::intrinsics::transmute;
use core::panic::PanicInfo;
use core::ptr;
use core::ptr::slice_from_raw_parts;
use core::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};
use model::Driver;
use payloads::payload;
use soc::clock::Clock;
//use soc::syscon::Syscon;
//use soc::iopad::IOpad;
use soc::iopadctl::IOpadctl;
use soc::rstgen::RSTgen;
//use uart::sifive::SiFive;
pub mod uart;
use crate::uart::UART;

global_asm!(include_str!(
    "../../../../../src/soc/starfive/jh7100/src/start.S"
));

// TODO: For some reason, on hardware, a1 is not the address of the dtb, so we hard-code the device
// tree here. TODO: The kernel ebreaks when given this device tree.
//const DTB: &'static [u8] = include_bytes!("hifive.dtb");

// All the non-boot harts spin on this lock.
static SPIN_LOCK: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn _start_nonboot_hart(hart_id: usize, fdt_address: usize) -> ! {
    spin_loop_hint();
    loop {
        // NOPs prevent thrashing the bus.
        for _ in 0..128 {
            arch::nop();
        }
        match SPIN_LOCK.load(Ordering::Relaxed) {
            0 => {}
            entrypoint => unsafe {
                let entrypoint = transmute::<usize, payload::EntryPoint>(entrypoint);
                // TODO: fdt_address might different from boot hart
                entrypoint(hart_id, fdt_address);
                // TODO: panic if returned from entrypoint
            },
        };
    }
}

#[no_mangle]
pub extern "C" fn _start_boot_hart(_hart_id: usize, _fdt_address: usize) -> ! {
    // Have to do a ton of stuff just to get uart.
    // Why do companies always get this wrong ...
    // I mean, even a working pin we could shuffle bits on would work.

    let mut clks = [
        //spi0 as &mut dyn ClockNode,
        //spi1 as &mut dyn ClockNode,
        //spi2 as &mut dyn ClockNode,
	//        uart0 as &mut dyn ClockNode,
    ];
    let mut clk = Clock::new(&mut clks);
    clk.pwrite(b"on", 0).unwrap();
    let mut iopadctl = IOpadctl::new();
    iopadctl.pwrite(b"early", 0).unwrap(); // you might argue this is getting ridiculous.
                                           // plan 9 is not for everywhere.
                                           // I might agree.
    let mut rstgen = RSTgen::new();
    rstgen.pwrite(b"on", 0).unwrap();

    iopadctl.pwrite(b"on", 0).unwrap();

    //        let mut syscon = Syscon::new();
    //        let mut iopad = IOpad::new();

    //    syscon.finish();
    //      iopad.finish();
    let mut uart = UART::new();
    uart.init().unwrap();
    uart.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    uart.pwrite(b"\r\n0x2000_0000", 0).unwrap();
    let slice = slice_from_raw_parts(0x2000_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();
    uart.pwrite(b"\r\n0x2004_0000", 0).unwrap();
    let slice = slice_from_raw_parts(0x2004_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();
    uart.pwrite(b"\r\n0x2001_0000", 0).unwrap();
    let slice = slice_from_raw_parts(0x2001_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();
    uart.pwrite(b"\r\n0x1808_0000", 0).unwrap();
    let slice = slice_from_raw_parts(0x1808_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();
    let sz = peek32(0x2001_0000) as u32;
    //	let ddr = slice_from_raw_parts(0x2001_0000 as *const u8, sz as usize);

    for addr in (0usize..sz as usize).step_by(4) {
        uart.pwrite(b".", 0).unwrap();
        let d = peek32(0x2001_0004 + addr as u32);
        poke32(0x1808_0000 + addr as u32, d);
    }
    uart.pwrite(b"\r\n0x1808_0000", 0).unwrap();
    let slice = slice_from_raw_parts(0x1808_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    // Let's try some serial out now.

    // call ddr
    pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

    unsafe {
        let f = transmute::<usize, EntryPoint>(0x1808_0000);
        uart.pwrite(b"Jump to 1808_0000", 0).unwrap();
        f(1, 0x1804_0000);
    }
    uart.pwrite(b"That escalated quickly", 0).unwrap();
    arch::halt();
}

fn poke32(a: u32, v: u32) {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek32(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}

// Returns Err((address, got)) or OK(()).
//
/*
fn test_ddr(
    addr: *mut u32,
    size: usize,
    w: &mut impl core::fmt::Write,
) -> Result<(), (*const u32, u32)> {
    writeln!(w, "Starting to fill with data\r").unwrap();
    // Fill with data.
    for i in 0..(size / 4) {
        unsafe { ptr::write(addr.add(i), (i + 1) as u32) };
    }

    writeln!(w, "Starting to read back data\r").unwrap();
    // Read back data.
    for i in 0..(size / 4) {
        let v = unsafe { ptr::read(addr.add(i)) };
        if v != i as u32 + 1 {
            return Err((unsafe { addr.add(i) }, v));
        }
    }
    Ok(())
}
*/

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    arch::halt()
}
#[no_mangle]
pub extern "C" fn trap_entry(_hart_id: usize) -> ! {
    arch::halt();
}
