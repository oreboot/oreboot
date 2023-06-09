#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(generator_trait)]
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::{arch::asm, ptr::read_volatile};
use embedded_hal::digital::OutputPin;
use log::{print, println};
use oreboot_compression::decompress;
use oreboot_soc::sunxi::d1::{
    ccu::Clocks,
    gpio::Gpio,
    pac::{Peripherals, UART0},
    time::U32Ext,
    uart::{self, Config, D1Serial, Parity, StopBits, WordLength},
};
use spin;

mod sbi_platform;

const MEM: usize = 0x4000_0000;

// see ../fixed-dtfs.dts
// const PAYLOAD_OFFSET: usize = 0x2_0000;
const PAYLOAD_SIZE: usize = 0x20_0000; // 2 MB
const PAYLOAD_ADDR: usize = MEM + 0x20_0000;

// compressed image
const LINUXBOOT_TMP_OFFSET: usize = 0x0400_0000;
const LINUXBOOT_TMP_ADDR: usize = MEM + LINUXBOOT_TMP_OFFSET;

// target location for decompressed image
const LINUXBOOT_OFFSET: usize = 0x0020_0000;
const LINUXBOOT_ADDR: usize = MEM + LINUXBOOT_OFFSET;
const LINUXBOOT_SIZE: usize = 0x0180_0000;
// DTB_OFFSET should be >=LINUXBOOT_OFFSET+LINUXBOOT_SIZE and match bt0
// TODO: Should we just copy it to a higher address before decompressing Linux?
const DTB_OFFSET: usize = 0x01a0_0000;
const DTB_ADDR: usize = MEM + DTB_OFFSET;

struct Standout;
impl core::fmt::Write for Standout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print!("{s}");
        Ok(())
    }
}

fn decompress_lb() {
    // check for Device Tree header, d00dfeed
    let dtb = unsafe { read_volatile(DTB_ADDR as *const u32) };
    if dtb != 0xedfe0dd0 {
        panic!("DTB looks wrong: {:08x}\n", dtb);
    } else {
        print!("DTB looks fine, yay!\n");
    }

    decompress(Standout, LINUXBOOT_TMP_ADDR, LINUXBOOT_ADDR, LINUXBOOT_SIZE);

    // check for kernel to be okay
    let a = LINUXBOOT_ADDR + 0x30;
    let r = unsafe { read_volatile(a as *mut u32) };
    if r == u32::from_le_bytes(*b"RISC") {
        print!("Payload looks like Linux Image, yay!\n");
    } else {
        panic!("Payload does not look like Linux Image: {:x}\n", r);
    }
    // Recheck on DTB, kernel should not run into it
    let dtb = unsafe { read_volatile(DTB_ADDR as *mut u32) };
    if dtb != 0xedfe0dd0 {
        panic!("DTB looks wrong: {:08x} - was it overridden?\n", dtb);
    } else {
        print!("DTB still fine, yay!\n");
    }
}

// MHCR - Machine mode hardware configuration register
// MCOR - Machine mode cache operation register
//
// Cache Enable and Mode Configuration: The Machine Mode Hardware Configuration
// Register (mhcr) enables switching of instruction and data caches and
// configuration of write allocation and writeback modes. The supervisor mode
// hardware configuration register (shcr) is a map of mhcr and is read-only.
//
// Dirty Entry Clear and Invalidate Operations: The Machine Mode Cache Operation
// Register (mcor) can perform dirty entry and invalidation operations on the
// instruction and data caches.
//
// Cache read operation: machine mode cache access instruction register (mcins),
// cache access index register (mcindex) and cache access data register  0/1
// (mcdata0/1). and data cache read operations. The specific control register
// description can refer to the machine mode processor control and status
// extension register group.
//
// p 583:
// 16.1.7.2 Machine Mode Hardware Configuration Register (MHCR)
//
// The Machine Mode Hardware Configuration Register (MHCR) is used to configure
// the processor, including capabilities and functionality. The bit length of
// this register is 64 bits, and the read and write permissions of the register
// are readable and writable in machine mode, that is, non-machine mode access
// will result in illegal instruction exceptions.
//
// 0 IE - Icache enable bit:
// • When IE=0, Icache is closed;
// • When IE=1, Icache is turned on.
// This bit will be set to 1’b0 by reset.
//
// 1 DE - Dcache enable bit:
// • When DE=0, Dcache is closed;
// • When DE=1, Dcache is on.
// This bit will be set to 1’b0 by reset.
//
// 2 WA - Cache Write Allocation Set Bit:
// • When WA=0, the data cache is in write non-allocate mode;
// • When WA=1, the data cache is in write allocate mode.
// This bit will be set to 1’b0 by reset.
//
// 3 WB - Cache Write Back Set Bits:
// • When WB=0, the data cache is in write through mode.
// • When WB=1, the data cache is in write back mode.
// C906 only supports write back mode, and WB is fixed to 1.
//
// 4 RS - Address Return Stack Set Bit:
// • When RS=0, the return stack is closed;
// • When RS=1, the return stack is turned on.
// This bit will be set to 1’b0 by reset.
//
//
// 5 BPE - Allow Predictive Jump Set bit:
// • When BPE=0, predictive jumps are turned off;
// • When BPE=1, predictive jumps are turned on.
// This bit will be set to 1’b0 by reset.
//
// 6 BTB - Branch Target Prediction Enable Bit:
// • When BTB=0, branch target prediction is turned off.
// • When BTB=1, branch target prediction is on.
// This bit will be set to 1’b0 by reset.
//
// 8 WBR - Write Burst Enable Bit:
// • When WBR=0, write burst transfers are not supported.
// • When WBR=1, write burst transfers are supported.
// WBR is fixed to 1 in C906.

// NOTE: D-cache b0rks things

// 16.1.7.4 Machine Mode Implicit Operation Register (MHINT)
// The Machine Mode Implicit Operation Register (MHINT) is used to cache various
// function switch controls.
// The bit length of this register is 64 bits, and the read and write
// permissions of the register are readable and writable in machine mode, that
// is, non-machine mode access will result in illegal instruction exceptions.
//
// DPLD - DCACHE Prefetch Enable Bit:
// • When DPLD is 0, DCACHE prefetch is disabled;
// • When DPLD is 1, DCACHE prefetch is on.
// This bit will be set to 1’b0 by reset.
//
// AMR - L1 DCache Write Allocation Policy Auto Adjust Enable Bits:
// • When AMR is 0, the write allocation strategy is determined by the page
//   attribute WA of the access address;
// • When AMR is 1, when a storage operation of three consecutive cache lines
//   occurs, subsequent storage operations of consecutive addresses are no
//   longer written to the L1 Cache;
// • When AMR is 2, when a storage operation of 64 consecutive cache lines
//   occurs, subsequent storage operations of consecutive addresses are no
//   longer written to the L1 Cache;
// • When AMR is 3, when a store operation of 128 consecutive cache lines
//   occurs, subsequent store operations of consecutive addresses are no longer
//   written to the L1 Cache.
// These bits will be reset to 2’b0.
//
// IPLD - ICACHE Prefetch Enable Bit:
// • When IPLD is 0, ICACHE prefetching is disabled;
// • When IPLD is 1, ICACHE prefetch is on.
// This bit will be reset to 1’b0.
//
// IWPE - ICACHE Road Prediction Enable Bit:
// • When IWPE is 0, ICACHE road prediction is turned off;
// • When IWPE is 1, ICACHE road prediction is on.
// This bit will be set to 1’b0 by reset.
//
// D_DIS - DCACHE Number of prefetch cache lines:
// • When DPLD is 0, prefetch 2 cache lines;
// • When DPLD is 1, prefetch 4 cache lines;
// • When DPLD is 2, prefetch 8 cache lines;
// • When DPLD is 3, 16 cache lines are prefetched.
// These bits will be reset to 2’b10.

// when handled from BT0 stage, DDR is prepared.
// this code runs from DDR start
#[naked]
#[export_name = "_start"]
#[link_section = ".text.entry"]
unsafe extern "C" fn start() -> ! {
    asm!(
        // MCOR: disable caches
        "li     t1, 0x22",
        "csrw   0x7c2, t1",
        // 1. clear cache and processor states
        // BT0 stage already handled MXSTATUS for us
        // 2. initialize programming language runtime
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
        "1:",
        "bgeu   t0, t1, 1f",
        "sd     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "call   {main}",
        // Function `main` returns with hardware power operation type
        // which may be reboot or shutdown. Function `finish` would
        // perform these operations.
        "j      {finish}",
        stack      =   sym ENV_STACK,
        stack_size = const STACK_SIZE,
        main       =   sym main,
        finish     =   sym finish,
        options(noreturn)
    )
}

// stack which the bootloader environment would make use of.
#[link_section = ".bss.uninit"]
static mut ENV_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
const STACK_SIZE: usize = 8 * 1024; // 8KiB

static PLATFORM: &str = "T-HEAD Xuantie Platform";
static VERSION: &str = env!("CARGO_PKG_VERSION");

fn dump_csrs() {
    let mut v: usize;
    unsafe {
        print!("==== platform CSRs ====\r\n");
        asm!("csrr {}, 0x7c0", out(reg) v);
        print!("   MXSTATUS  {:08x}\r\n", v);
        asm!("csrr {}, 0x7c1", out(reg) v);
        print!("   MHCR      {:08x}\r\n", v);
        asm!("csrr {}, 0x7c2", out(reg) v);
        print!("   MCOR      {:08x}\r\n", v);
        asm!("csrr {}, 0x7c5", out(reg) v);
        print!("   MHINT     {:08x}\r\n", v);
        print!("see C906 manual p581 ff\r\n");
        print!("=======================\r\n");
    }
}

fn init_csrs() {
    print!("Set up extension CSRs\n");
    dump_csrs();
    unsafe {
        // MXSTATUS: T-Head ISA extension enable, MAEE, MM, UCME, CLINTEE
        // NOTE: Linux relies on detecting errata via mvendorid, marchid and
        // mipmid. If that detection fails, and we enable MAEE, Linux won't come
        // up. When D-cache is enabled, and the detection fails, we run into
        // cache coherency issues. Welcome to the minefield! :)
        // NOTE: We already set part of this in bt0, but it seems to get lost?
        asm!("csrs 0x7c0, {}", in(reg) 0x00638000);
        // MCOR: invalidate ICACHE/DCACHE/BTB/BHT
        asm!("csrw 0x7c2, {}", in(reg) 0x00070013);
        // MHCR
        asm!("csrw 0x7c1, {}", in(reg) 0x000011ff);
        // MHINT
        asm!("csrw 0x7c5, {}", in(reg) 0x0016e30c);
    }
    dump_csrs();
}

type Serial = D1Serial<UART0, uart::Pins_B8_B9>;

fn init_logger(s: Serial) {
    static ONCE: spin::Once<()> = spin::Once::new();

    ONCE.call_once(|| unsafe {
        static mut SERIAL: Option<Serial> = None;
        SERIAL.replace(s);
        log::init(SERIAL.as_mut().unwrap());
    });
}

// Function `main`. It would initialize an environment for the kernel.
// The environment does not exit when bootloading stage is finished;
// it remains in background to provide environment features which the
// kernel would make use of.
// Those features would include RISC-V SBI calls, instruction emulations,
// misaligned and so on.
extern "C" fn main() -> usize {
    let p = Peripherals::take().unwrap();
    let clocks = Clocks {
        psi: 600_000_000.hz(),
        apb1: 24_000_000.hz(),
    };
    let gpio = Gpio::new(p.GPIO);
    // turn off led
    let mut pb5 = gpio.portb.pb5.into_output();
    pb5.set_low().unwrap();

    // prepare serial port logger
    let tx = gpio.portb.pb8.into_function_6();
    let rx = gpio.portb.pb9.into_function_6();
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };

    let serial = D1Serial::new(p.UART0, (tx, rx), config, &clocks);
    init_logger(serial);

    print!("oreboot: serial uart0 initialized\n");

    // how we figured out https://github.com/rust-embedded/riscv/pull/107
    if true {
        use riscv::register::{marchid, mimpid, mvendorid};
        let vid = mvendorid::read().map(|r| r.bits()).unwrap_or(0);
        let arch = marchid::read().map(|r| r.bits()).unwrap_or(0);
        let imp = mimpid::read().map(|r| r.bits()).unwrap_or(0);
        print!("RISC-V vendor {:x} arch {:x} imp {:x}\r\n", vid, arch, imp);
    }

    let use_sbi = cfg!(feature = "supervisor");
    if use_sbi {
        use oreboot_arch::riscv64::sbi;
        sbi_platform::init();
        init_csrs();

        sbi::runtime::init();
        sbi::info::print_info(PLATFORM, VERSION);

        decompress_lb();
        println!(
            "Enter supervisor at {:x} with DTB from {:x}",
            LINUXBOOT_ADDR, DTB_ADDR
        );
        let (reset_type, reset_reason) =
            sbi::execute::execute_supervisor(LINUXBOOT_ADDR, 0, DTB_ADDR);
        print!("oreboot: reset reason = {}", reset_reason);
        reset_type
    } else {
        // TODO: Do we need more stuff here?
        unsafe {
            asm!("csrs 0x7c0, {}", in(reg) 0x00018000);
        }
        println!("You are NOT MY SUPERVISOR!");
        decompress(Standout, LINUXBOOT_TMP_ADDR, PAYLOAD_ADDR, PAYLOAD_SIZE);
        println!("Running payload at 0x{:x}", PAYLOAD_ADDR);
        unsafe {
            let f: unsafe extern "C" fn() = core::mem::transmute(PAYLOAD_ADDR);
            f();
        }
        println!("Unexpected return from payload");
        0
    }
}

extern "C" fn finish(reset_type: u32) -> ! {
    use sbi_spec::srst::*;
    match reset_type {
        RESET_TYPE_SHUTDOWN => loop {
            println!("🦀");
            // unsafe { asm!("wfi") }
        },
        /*
        RESET_TYPE_COLD_REBOOT => todo!(),
        RESET_TYPE_WARM_REBOOT => todo!(),
        */
        _ => unimplemented!(),
    }
}

/// This function is called on panic.
#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
        print!("{:?}", info.message());
    } else {
        println!("panic at unknown location");
    };
    loop {
        core::hint::spin_loop();
    }
}
