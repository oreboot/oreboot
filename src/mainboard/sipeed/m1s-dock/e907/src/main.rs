#![feature(naked_functions, asm_const)]
#![feature(associated_type_bounds)]
#![no_std]
#![no_main]

use bl808_pac::Peripherals;
use core::{
    arch::asm,
    panic::PanicInfo,
    // ptr::slice_from_raw_parts,
    slice,
};
#[macro_use]
extern crate log;
use riscv::register::{marchid, mhartid, mimpid, mvendorid};

mod init;
mod uart;

const BOARD_SOC: &str = "Bouffalo Lab BL808";
const BOARD_NAME: &str = "Sipeed M1S Dock";

const STACK_SIZE: usize = 4 * 1024; // 4KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Set up stack and jump to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // 1. disable and clear interrupts
        "csrw   mtvec, t0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        // 2. initialize programming language runtime
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
        "1:",
        "bgeu   t0, t1, 1f",
        "sw     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "call   {main}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        main       =   sym main,
        options(noreturn)
    )
}

fn sleep() {
    unsafe {
        for _ in 0..0x200000 {
            riscv::asm::nop();
        }
    }
}

fn riscv_plat_info() {
    let vid = mvendorid::read().map(|r| r.bits()).unwrap_or(0);
    let arch = marchid::read().map(|r| r.bits()).unwrap_or(0);
    let imp = mimpid::read().map(|r| r.bits()).unwrap_or(0);
    println!("RISC-V vendor {:x} arch {:x} imp {:x}", vid, arch, imp);
    let hart_id = mhartid::read();
    println!("RISC-V hart ID {}", hart_id);
    println!("Board: {BOARD_NAME}");
    println!("SoC: {BOARD_SOC}");
}

fn dump(addr: usize, length: usize) {
    let s = unsafe { slice::from_raw_parts(addr as *const u8, length) };
    println!("dump {length} bytes @{addr:x}");
    for w in s.iter() {
        print!("{:02x}", w);
    }
    println!();
}

fn init_logger(u: uart::BSerial) {
    static ONCE: spin::Once<()> = spin::Once::new();

    ONCE.call_once(|| unsafe {
        static mut SERIAL: Option<uart::BSerial> = None;
        SERIAL.replace(u);
        log::init(SERIAL.as_mut().unwrap());
    });
}

const PSRAM_CONFIGURE: usize = 0x2005_2000;

const PSRAM_BASE: usize = 0x5000_0000;

/**
 * There are multiple UARTs available. We configure the first two to 115200
 * bauds.
 * We use UART0 like for classic POST codes via `Serial`'s `debug` function.
 * The `print`/`println` macros output to UART1 for rich output.
 * Note that UART0 is really only for debugging here, and we want to use it for
 * the D0 core (64-bit "MM"/multimedia) otherwise.
 */
fn main() {
    let p = Peripherals::take().unwrap();
    let glb = p.GLB;
    init::gpio_uart_init(&glb);
    let serial = uart::BSerial::new(p.UART0, p.UART1);
    init_logger(serial);

    // print to UART0
    log::debug('*' as u8);

    // prints to UART1
    println!("oreboot 🦀");
    riscv_plat_info();
    println!("{}", glb.chip_inform.read().bits());

    for _ in 0..4 {
        println!("🐢");
        sleep();
    }

    const MM_ENTRY: usize = 0x3eff_0000;
    dump(MM_ENTRY, 32);

    use core::ptr::{read_volatile, write_volatile};

    let psram_cfg = unsafe { read_volatile(PSRAM_CONFIGURE as *mut u32) };
    println!("psram_cfg {:x}", psram_cfg);

    const TZC_SEC_BASE: usize = 0x2000_5000;
    const TZC_SEC_TZC_PSRAMA_TZSRG_CTRL: usize = TZC_SEC_BASE + 0x0380;
    const TZC_SEC_TZC_PSRAMA_TZSRG_R0: usize = TZC_SEC_TZC_PSRAMA_TZSRG_CTRL;
    const TZC_SEC_TZC_PSRAMB_TZSRG_CTRL: usize = TZC_SEC_BASE + 0x03A8;
    const TZC_SEC_TZC_PSRAMB_TZSRG_R0: usize = TZC_SEC_TZC_PSRAMA_TZSRG_CTRL;

    const PSRAM_SIZE: u32 = 64 * 1024 * 1024;
    let start_addr = 0;
    let end_addr = PSRAM_SIZE;
    let region = 0;

    /*
     * CPU Prefetching barrier; see Bl808 SDK
     * see drivers/bl808_driver/startup/m0/source/system_bl808.c
     */
    unsafe {
        let v = read_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_CTRL as *mut u32);
        println!("TZC_SEC_TZC_PSRAMA_TZSRG_CTRL {v:x}");
        // prepare?
        write_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_CTRL as *mut u32, 0xffff_ffff);

        let v = read_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_R0 as *mut u32);
        println!("TZC_SEC_TZC_PSRAMA_TZSRG_R0 {v:x}");
        // lose the lower 10 bits (22 bits remain), then drop higher 6 remaining bits
        let start_addr_high = (start_addr >> 10) & 0xffff;
        let v_start = (start_addr_high) << 16; // xxxx_0000
        let end_addr_high = (end_addr >> 10) & 0xffff;
        let v_end = (end_addr_high - 1) & 0xffff; // 0000_xxxx
        let v = v_start | v_end;
        write_volatile((TZC_SEC_TZC_PSRAMA_TZSRG_R0 + region * 4) as *mut u32, v);
        let v = read_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_R0 as *mut u32);
        println!("TZC_SEC_TZC_PSRAMA_TZSRG_R0 {v:x}");

        /* set enable but not lock */
        let ctrl_mask = 1 << (region + 16);
        let v = read_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_CTRL as *mut u32);
        write_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_CTRL as *mut u32, v | ctrl_mask);
        let v = read_volatile(TZC_SEC_TZC_PSRAMA_TZSRG_CTRL as *mut u32);
        println!("TZC_SEC_TZC_PSRAMA_TZSRG_CTRL {v:x}");
    }

    // NOTE: before using PSRAM, also implement PHY init; see Bl808 SDK
    // drivers/bl808_driver/std_drv/src/bl808_uhs_phy.c

    unsafe { write_volatile(PSRAM_BASE as *mut u32, 0x1234_5678) }
    dump(PSRAM_BASE, 8);

    init::resume_mm(MM_ENTRY as u32);
    if false {
        init::reset_cpu();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("panic {:?}", info);
    loop {
        core::hint::spin_loop();
    }
}
