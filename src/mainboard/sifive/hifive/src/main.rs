#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![deny(warnings)]

use clock::ClockNode;
use consts::DeviceCtl;
use core::arch::{asm, global_asm};
use core::hint::spin_loop;
use core::intrinsics::transmute;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{fmt::Write, ptr};
use device_tree::print_fdt;
use model::Driver;
use payloads::payload;
use soc::clock::Clock;
use soc::ddr::DDR;
use spi::SiFiveSpi;
use uart::sifive::SiFive;
use wrappers::{Memory, SectionReader, SliceReader};

global_asm!(include_str!(
    "../../../../../src/soc/sifive/fu540/src/bootblock.S"
));
global_asm!(include_str!(
    "../../../../../src/soc/sifive/fu540/src/init.S"
));

// TODO: For some reason, on hardware, a1 is not the address of the dtb, so we hard-code the device
// tree here. TODO: The kernel ebreaks when given this device tree.
//const DTB: &'static [u8] = include_bytes!("hifive.dtb");

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
pub extern "C" fn _start_boot_hart(_hart_id: usize, fdt_address: usize) -> ! {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    uart0.init().unwrap();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    // Set SPIs to 50MHZ clock rate.
    let spi0 = &mut SiFiveSpi::new(0x10040000, 50_000_000);
    let spi1 = &mut SiFiveSpi::new(0x10041000, 50_000_000);
    let spi2 = &mut SiFiveSpi::new(0x10050000, 50_000_000);

    uart0.pwrite(b"Initializing clocks...\r\n", 0).unwrap();
    // Peripheral clocks get their dividers updated when the PLL initializes.
    let mut clks = [
        spi0 as &mut dyn ClockNode,
        spi1 as &mut dyn ClockNode,
        spi2 as &mut dyn ClockNode,
        uart0 as &mut dyn ClockNode,
    ];
    let mut clk = Clock::new(&mut clks);
    clk.ctl(DeviceCtl::On).unwrap();
    uart0.pwrite(b"Done\r\n", 0).unwrap();

    // Right now we execute out of the memory mapped by spi0, so we can't
    // reconfigure it. Show that the SPI init code works by initializing
    // spi1.
    uart0.pwrite(b"Initializing SPI1 controller...", 0).unwrap();
    spi1.init().unwrap();
    spi1.mmap(soc::spi::FU540_SPI_MMAP_CONFIG).unwrap();
    uart0.pwrite(b"Done\r\n", 0).unwrap();

    uart0
        .pwrite(b"Testing read from SPI1 mapped memory...", 0)
        .unwrap();
    unsafe {
        // Perform a read at the base address of the SPI1 memory mapped space
        let mut _val: u32 = *(0x30000000 as *const u32);
        // Volatile `and` operation of the value with itself to try and make
        // sure that we don't optimize the read out.
        asm!("and {0}, {1}, {1}", in(reg) _val, out(reg) _val);
    }
    uart0.pwrite(b"Done\r\n", 0).unwrap();

    let w = &mut print::WriteTo::new(uart0);

    writeln!(w, "## ROM Device Tree\r").unwrap();

    // TODO: The fdt_address is garbage while running on hardware (but not in QEMU).
    writeln!(w, "ROM FDT address: 0x{:x}", fdt_address).unwrap();

    // We have no idea how long the FDT really is. This caps it to 1MiB.
    let rom_fdt = &mut SectionReader::new(&Memory {}, fdt_address, 1024 * 1024);
    if let Err(err) = print_fdt(rom_fdt, w) {
        writeln!(w, "error: {}", err).unwrap();
    }

    writeln!(w, "## Oreboot Fixed Device Tree\r").unwrap();
    // Fixed DTFS is at offset 512KiB in flash. Max size 512Kib.
    let fixed_fdt = &mut SectionReader::new(&Memory {}, 0x20000000 + 512 * 1024, 512 * 1024);
    if let Err(err) = print_fdt(fixed_fdt, w) {
        writeln!(w, "error: {}", err).unwrap();
    }

    writeln!(w, "Initializing DDR...\r").unwrap();
    let mut ddr = DDR::new();

    let m = ddr
        .ctl(DeviceCtl::On)
        .unwrap_or_else(|error| panic!("problem initalizing DDR: {:?}", error));

    writeln!(w, "Done\r").unwrap();

    writeln!(w, "Memory size is: {:x}\r", m).unwrap();

    writeln!(w, "Testing DDR...\r").unwrap();
    let mem = 0x80000000;
    match test_ddr(mem as *mut u32, m / 1024, w) {
        Err((a, v)) => writeln!(
            w,
            "Unexpected read 0x{:x} at address 0x{:x}\r",
            v, a as usize,
        )
        .unwrap(),
        _ => writeln!(w, "Passed\r").unwrap(),
    }

    // TODO; This payload structure should be loaded from SPI rather than hardcoded.
    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: mem,
            data: &mut SectionReader::new(&Memory {}, 0x20000000 + 0x100000, 0x600000),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: fdt_address, /*mem + 10*1024*1024*/
            data: &mut SliceReader::new(&[0u8; 0]),
        },
    ];
    let mut payload: payload::Payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: 0,
        dtb: 0,
        // TODO: These two length fields are not used.
        rom_len: 0,
        mem_len: 0,
        segs: kernel_segs,
    };
    writeln!(w, "Loading payload\r").unwrap();
    payload.load();
    writeln!(
        w,
        "Running payload entry 0x{:x} dtb 0x{:x}\r",
        payload.entry, payload.dtb
    )
    .unwrap();
    SPIN_LOCK.store(payload.entry, Ordering::Relaxed);
    payload.run();

    writeln!(w, "Unexpected return from payload\r").unwrap();
    arch::halt()
}

// Returns Err((address, got)) or OK(()).
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

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = writeln!(w, "PANIC: {}\r", info);
    arch::halt()
}
