#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

use clock::ClockNode;
use core::intrinsics::transmute;
use core::panic::PanicInfo;
use core::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};
use core::{fmt::Write, ptr};
use device_tree::{infer_type, Entry, FdtReader};
use model::Driver;
use payloads::payload;
use print;
use soc::clock::Clock;
use soc::ddr::DDR;
use soc::is_qemu;
use spi::SiFiveSpi;
use uart::sifive::SiFive;
use wrappers::{Memory, SectionReader, SliceReader};

global_asm!(include_str!("../../../../../src/soc/sifive/fu540/src/bootblock.S"));
global_asm!(include_str!("../../../../../src/soc/sifive/fu540/src/init.S"));

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
    clk.pwrite(b"on", 0).unwrap();
    uart0.pwrite(b"Done\r\n", 0).unwrap();

    // QEMU doesn't emulate the SPI controller interface
    if !is_qemu() {
        // Right now we execute out of the memory mapped by spi0, so we can't
        // reconfigure it. Show that the SPI init code works by initializing
        // spi1.
        uart0.pwrite(b"Initializing SPI1 controller...", 0).unwrap();
        spi1.init().unwrap();
        spi1.mmap(soc::spi::FU540_SPI_MMAP_CONFIG).unwrap();
        uart0.pwrite(b"Done\r\n", 0).unwrap();

        uart0.pwrite(b"Testing read from SPI1 mapped memory...", 0).unwrap();
        unsafe {
            // Perform a read at the base address of the SPI1 memory mapped space
            let mut _val: u32 = *(0x30000000 as *const u32);
            // Volatile `and` operation of the value with itself to try and make
            // sure that we don't optimize the read out.
            llvm_asm!("and $0, $1, $1" : "=r"(_val) : "r"(_val) :: "volatile");
        }
        uart0.pwrite(b"Done\r\n", 0).unwrap();
    }

    let w = &mut print::WriteTo::new(uart0);

    write!(w, "## ROM Device Tree\r\n").unwrap();

    // TODO: The fdt_address is garbage while running on hardware (but not in QEMU).
    write!(w, "ROM FDT address: 0x{:x}\n", fdt_address).unwrap();

    if is_qemu() {
        // TODO: With QEMU's device tree, our parses reaches an unexpected EOF.
        // We have no idea how long the FDT really is. This caps it to 1MiB.
        let rom_fdt = &mut SectionReader::new(&Memory {}, fdt_address, 8 * 1024 * 1024);
        if let Err(err) = print_fdt(rom_fdt, w) {
            write!(w, "error: {}\n", err).unwrap();
        }
    }

    write!(w, "## Oreboot Fixed Device Tree\r\n").unwrap();
    // Fixed DTFS is at offset 512KiB in flash. Max size 512Kib.
    let fixed_fdt = &mut SectionReader::new(&Memory {}, 0x20000000 + 512 * 1024, 512 * 1024);
    if let Err(err) = print_fdt(fixed_fdt, w) {
        write!(w, "error: {}\n", err).unwrap();
    }

    write!(w, "Initializing DDR...\r\n").unwrap();
    let mut ddr = DDR::new();

    let m =
        ddr.pwrite(b"on", 0).unwrap_or_else(|error| panic!("problem initalizing DDR: {:?}", error));

    write!(w, "Done\r\n").unwrap();

    write!(w, "Memory size is: {:x}\r\n", m).unwrap();

    write!(w, "Testing DDR...\r\n").unwrap();
    let mem = 0x80000000;
    match test_ddr(mem as *mut u32, m / 1024, w) {
        Err((a, v)) => {
            write!(w, "Unexpected read 0x{:x} at address 0x{:x}\r\n", v, a as usize,).unwrap()
        }
        _ => write!(w, "Passed\r\n").unwrap(),
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
    write!(w, "Loading payload\r\n").unwrap();
    payload.load();
    write!(w, "Running payload entry 0x{:x} dtb 0x{:x}\r\n", payload.entry, payload.dtb).unwrap();
    SPIN_LOCK.store(payload.entry, Ordering::Relaxed);
    payload.run();

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

// Returns Err((address, got)) or OK(()).
fn test_ddr(addr: *mut u32, size: usize, w: &mut print::WriteTo) -> Result<(), (*const u32, u32)> {
    write!(w, "Starting to fill with data\r\n").unwrap();
    // Fill with data.
    for i in 0..(size / 4) {
        unsafe { ptr::write(addr.add(i), (i + 1) as u32) };
    }

    write!(w, "Starting to read back data\r\n").unwrap();
    // Read back data.
    for i in 0..(size / 4) {
        let v = unsafe { ptr::read(addr.add(i)) };
        if v != i as u32 + 1 {
            return Err((unsafe { addr.add(i) }, v));
        }
    }
    Ok(())
}

// TODO: move out of mainboard
pub fn print_fdt(fdt: &mut dyn Driver, w: &mut print::WriteTo) -> Result<(), &'static str> {
    for entry in FdtReader::new(fdt)?.walk() {
        match entry {
            Entry::Node { path: p } => {
                write!(w, "{:depth$}{}\r\n", "", p.name(), depth = p.depth() * 2).unwrap();
            }
            Entry::Property { path: p, value: v } => {
                let buf = &mut [0; 1024];
                let len = v.pread(buf, 0)?;
                let val = infer_type(&buf[..len]);
                write!(w, "{:depth$}{} = {}\r\n", "", p.name(), val, depth = p.depth() * 2,)
                    .unwrap();
            }
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
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
