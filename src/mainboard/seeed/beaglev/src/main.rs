#![no_std]
#![no_main]

use core::arch::global_asm;
//use core::fmt::Write;
use core::hint::spin_loop;
use core::intrinsics::transmute;
use core::panic::PanicInfo;
use core::ptr;
use core::ptr::slice_from_raw_parts;
use core::sync::atomic::{AtomicUsize, Ordering};
use oreboot_arch::riscv64 as arch;
use oreboot_drivers::Driver;
use oreboot_soc::starfive::jh7100::{
    clock::Clock,
    //  iopad::IOpad,
    iopadctl::IOpadctl,
    rstgen::RSTgen,
    syscon::Syscon,
};
use payloads::payload;
pub mod uart;
use crate::uart::UART;

global_asm!(include_str!(
    "../../../../../src/soc/src/starfive/jh7100/start.S"
));

// All the non-boot harts spin on this lock.
static SPIN_LOCK: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn _start_nonboot_hart(hart_id: usize, fdt_address: usize) -> ! {
    spin_loop();
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

    // Note that in future, we will make a DoD of all the parts in .. the mainboard?
    // and then call write with appropriate strings to enable stuff.
    // FIXME: breaks when running on VisionFive from SRAM / loaded by mask ROM
    let mut clk = Clock::new(&mut clks);
    clk.pwrite(b"on", 0).unwrap();

    // todo: use base
    let mut iopadctl = IOpadctl::new(0);
    // you might argue this is getting ridiculous.
    // plan 9 is not for everywhere.
    // I might agree.
    iopadctl.pwrite(b"early", 0).unwrap();
    let mut rstgen = RSTgen::new();
    rstgen.pwrite(b"on", 0).unwrap();
    // FIXME: breaks when running on VisionFive from SRAM / loaded by mask ROM
    // iopadctl.pwrite(b"on", 0).unwrap();

    //        let mut syscon = Syscon::new();
    //        let mut iopad = IOpad::new();

    //    syscon.finish();
    //      iopad.finish();

    // Let's try some serial out now.
    let mut uart = UART::new();
    // NOTE: In mask ROM mode, the UART is already set up for 9600 baud
    uart.init().unwrap();
    uart.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    uart.pwrite(b"\r\nsyscon start\r\n", 0).unwrap();

    let mut syscon = Syscon::new(0); // todo: use base
    syscon.pwrite(b"on", 0).unwrap();
    uart.pwrite(b"\r\nsyscon done\r\n", 0).unwrap();

    // Now, dump a bunch of memory ranges to check on
    // NOTE: When run via mask ROM from SRAM, we do not see the SPI flash
    // which would be mapped to memory on regular boot, only get ffffffffff.

    // NOTE: First SRAM: We are here!
    uart.pwrite(b"\r\nRead from 0x1800_0000: ", 0).unwrap();
    let slice = slice_from_raw_parts(0x1800_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    // NOTE: Stock firmware "second boot" starts here
    uart.pwrite(b"\r\nRead from 0x2000_0000: ", 0).unwrap();
    let slice = slice_from_raw_parts(0x2000_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    // NOTE: Offset 256K in stock firmware is U-Boot
    uart.pwrite(b"\r\nRead from 0x2004_0000: ", 0).unwrap();
    let slice = slice_from_raw_parts(0x2004_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    // NOTE: Offset 64K in stock firmware is DRAM init
    uart.pwrite(b"\r\nRead from 0x2001_0000: ", 0).unwrap();
    let slice = slice_from_raw_parts(0x2001_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    // NOTE: Second SRAM (0x1808_0000 - 0x1809_FFFF)
    uart.pwrite(b"\r\nRead from 0x1808_0000: ", 0).unwrap();
    let slice = slice_from_raw_parts(0x1808_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    // NOTE: The first 32bits are the DRAM blob size
    let dram_blob_size = peek32(0x2001_0000) as u32;
    // Copy the actual DRAM init blob (starting at byte 4) to second SRAM
    for addr in (0usize..dram_blob_size as usize).step_by(4) {
        // uart.pwrite(b".", 0).unwrap();
        let d = peek32(0x2001_0004 + addr as u32);
        poke32(0x1808_0000 + addr as u32, d);
    }

    // Dump to check if the DRAM init blob was copied
    uart.pwrite(b"\r\n0x1808_0000", 0).unwrap();
    let slice = slice_from_raw_parts(0x1808_0000 as *const u8, 32);
    uart.pwrite(unsafe { &*slice }, 1).unwrap();

    /*
    // call ddr
    pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

    // this is SRAM space
    unsafe {
        let f = transmute::<usize, EntryPoint>(0x1808_0000);
        uart.pwrite(b"Jump to 1808_0000", 0).unwrap();
        // NOTE: first argument would be the hart ID, so why 1 and not 0?
        f(1, 0x1804_0000);
    }
    */
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
