#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![feature(int_abs_diff)]
#![no_std]
#![no_main]

use core::intrinsics::transmute;
use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};
use embedded_hal::digital::blocking::OutputPin;
use oreboot_soc::sunxi::d1::{
    ccu::Clocks,
    gpio::{portc, Function, Gpio, Pin},
    jtag::Jtag,
    pac::{Peripherals, SPI0},
    spi::{Spi, MODE_3},
    time::U32Ext,
    uart::{Config, Parity, Serial, StopBits, WordLength},
};

#[macro_use]
mod logging;
mod flash;
mod mctl;

#[cfg(feature = "nand")]
use flash::SpiNand;
#[cfg(feature = "nor")]
use flash::SpiNor;
use mctl::RAM_BASE;

// taken from oreboot
pub type EntryPoint = unsafe extern "C" fn(r0: usize, r1: usize);

const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[link_section = ".head.text"]
#[export_name = "head_jump"]
pub unsafe extern "C" fn head_jump() {
    asm!(
        ".option push",
        ".option rvc",
        "c.j    0x68", // 0x60: eGON.BT0 header; 0x08: FlashHead
        ".option pop",
        // sym start,
        options(noreturn)
    )
}

// todo: option(noreturn) generates an extra `unimp` insn

// eGON.BT0 header. This header is identified by D1 ROM code
// to copy BT0 stage bootloader into SRAM memory.
// This header takes 0x60 bytes.
#[repr(C)]
pub struct EgonHead {
    magic: [u8; 8],
    checksum: u32,
    length: u32,
    pub_head_size: u32,
    fel_script_address: u32,
    fel_uenv_length: u32,
    dt_name_offset: u32,
    dram_size: u32,
    boot_media: u32,
    string_pool: [u32; 13],
}

const STAMP_CHECKSUM: u32 = 0x5F0A6C39;

// TODO: determine offets/sizes at build time
// memory load addresses
const LIN_ADDR: usize = RAM_BASE + 0x0400_0000; // Linux will be decompressed in payloader
const DTB_ADDR: usize = RAM_BASE + 0x0120_0000; // dtb must be 2MB aligned and behind Linux
const ORE_ADDR: usize = RAM_BASE;
const ORE_SIZE: usize = 0x1_8000; // 96K
const DTF_SIZE: usize = 0x1_0000; // 64K
const LIN_SIZE: usize = 0x00fc_0000;
const DTB_SIZE: usize = 0x0001_0000;

// clobber used by KEEP(*(.head.egon)) in link script
#[link_section = ".head.egon"]
pub static EGON_HEAD: EgonHead = EgonHead {
    magic: *b"eGON.BT0",
    checksum: STAMP_CHECKSUM, // real checksum filled by blob generator
    length: 0,                // real size filled by blob generator
    pub_head_size: 0,
    fel_script_address: 0,
    fel_uenv_length: 0,
    dt_name_offset: 0,
    dram_size: 0,
    boot_media: 0,
    string_pool: [0; 13],
};

// Private use; not designed as conventional header structure.
// Real data filled by xtask.
// This header takes 0x8 bytes. When modifying this structure, make sure
// the offset in `head_jump` function is also modified.
#[repr(C)]
pub struct MainStageHead {
    offset: u32,
    length: u32,
}

// clobber used by KEEP(*(.head.main)) in link script
// To avoid optimization, always read from flash page. Do NOT use this
// variable directly.
#[link_section = ".head.main"]
pub static MAIN_STAGE_HEAD: MainStageHead = MainStageHead {
    offset: 0, // real offset filled by xtask
    length: 0, // real size filled by xtask
};

/*
see https://linux-sunxi.org/D1 for the C906 manual
original document is in Chinese, translated by Google here for reference

p19:

2. In all cases where virtual address and physical address conversion are
performed: when the authority is not in machine mode and the MMU is turned on,
there are two ways to configure the page properties of the address.
Type: sysmap.h file and page attributes expanded by C906 in PTE.
Which configuration method is used depends on the C906 extension register
MXSTATUS.
The value of the MAEE field in. If the value of the MAEE field is 1, the page
attribute of the address is determined by the extended page attribute in the
corresponding pte. If the value of the MAEE field is 0, the page attribute of
the address is determined by sysmap.h.

p52 (on mxstatus):

Flags – 63:59 bit page attributes

C906 extended page attribute, exists when the MAEE bit of the MXSTATUS register
is 1, and the function is as described in the MMU EntryLo register (SMEL).

p582:

MAEE - extended MMU address attributes:
• When MAEE is 0, the MMU address attribute is not extended.
• When MAEE is 1, the address attribute bit in the pte of the MMU is extended,
  and the user can configure the address attribute of the page.
This bit will be reset to 1’b0.

THEADISAEE - enable extended instruction set:
• When THEADISAEE is 0, an illegal instruction exception will be triggered when
  the C906 extended instruction is executed.
• When THEADISAEE is 1, the C906 extended instruction can be executed normally.
This bit will be reset to 1’b0.

// MAEE (Memory Attribute Extension Enable)
#define    MAEE             (0x1 << 21)
// EN_THEADISAEE (T-Head ISA Extension Enable)
#define    EN_THEADISAEE    (0x1 << 22)
*/

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
///
/// NOTE: `mxstatus` is a custom T-Head register. Do not confuse with `mstatus`.
/// It allows for configuring special eXtensions. See further below for details.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // 1. clear cache and processor states
        "csrw   mie, zero",
        // enable theadisaee and maee
        "li     t1, 0x1 << 22 | 0x1 << 21",
        "csrs   0x7c0, t1", // MXSTATUS
        // invalidate ICACHE/DCACHE/BTB/BHT
        "li     t2, 0x30013",
        "csrs   0x7c2, t2", // MCOR
        // 2. initialize programming langauge runtime
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
        "1:",
        "bgeu   t0, t1, 1f",
        "sd     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "la     a0, {egon_head}",
        "call   {main}",
        // function `main` returns address of next stage,
        // it drops all peripherals it holds when goes out of scope
        // now, jump to dram code
        "j      {finish}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        egon_head  =   sym EGON_HEAD,
        main       =   sym main,
        finish     =   sym finish,
        options(noreturn)
    )
}

#[cfg(feature = "nor")]
fn load(
    skip: usize,
    base: usize,
    size: usize,
    f: &mut SpiNor<
        SPI0,
        (
            Pin<'C', 2, Function<2>>,
            Pin<'C', 3, Function<2>>,
            Pin<'C', 4, Function<2>>,
            Pin<'C', 5, Function<2>>,
        ),
    >,
) {
    let chunks = 16;
    for i in 0..size / chunks {
        let off = skip + i * 4 * chunks;
        let buf = f.copy_into([(off >> 16) as u8, (off >> 8) as u8, off as u8]);

        for j in 0..chunks {
            let jw = 4 * j;
            let addr = base + i * 4 * chunks + jw;
            /*
            let s = [buf[jw], buf[jw + 1], buf[jw + 2], buf[jw + 3]];
            println!("{:x}\n\n", s[0]);
            println!(
                "a {:x} o {:08x} v {:02x}{:02x}{:02x}{:02x}",
                addr, off, s[0], s[1], s[2], s[3]
            );
            */
            let s = buf[jw..(jw + 4)].try_into().unwrap();
            // transform bytes from slice to u32
            let val = u32::from_le_bytes(s);
            // for debugging
            // println!("a {:x} o {:08x} v {:08x}", addr, off, val);
            unsafe { write_volatile(addr as *mut u32, val) };
        }
        // progress indicator each 2MB
        if (off - skip) % 0x10_0000 == 0 {
            print!("➡️");
            // for debugging
            // println!("a {:x} o {:08x} v {:08x}", addr, off, val);
        }
    }
    println!(".");
}

extern "C" fn main() -> usize {
    // there was configure_ccu_clocks, but ROM code have already done configuring for us
    let p = Peripherals::take().unwrap();
    // rom provided clock frequency, it's hard coded in bt0 stage
    let clocks = Clocks {
        psi: 600_000_000.hz(),
        apb1: 24_000_000.hz(),
    };
    let gpio = Gpio::new(p.GPIO);

    // configure jtag interface
    let tms = gpio.portf.pf0.into_function_4();
    let tck = gpio.portf.pf5.into_function_4();
    let tdi = gpio.portf.pf1.into_function_4();
    let tdo = gpio.portf.pf3.into_function_4();
    let _jtag = Jtag::new((tms, tck, tdi, tdo));

    // light up led
    let mut pb5 = gpio.portb.pb5.into_output();
    pb5.set_high().unwrap();
    let mut pc1 = gpio.portc.pc1.into_output();
    pc1.set_high().unwrap();

    /*
    // blinky
    for _ in 0..2 {
        for _ in 0..1000_0000 {
            core::hint::spin_loop();
        }
        pc1.set_low().unwrap();
        for _ in 0..1000_0000 {
            core::hint::spin_loop();
        }
        pc1.set_high().unwrap();
    }
    */

    // prepare serial port logger
    let tx = gpio.portb.pb8.into_function_6();
    let rx = gpio.portb.pb9.into_function_6();
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    let serial = Serial::new(p.UART0, (tx, rx), config, &clocks);
    crate::logging::set_logger(serial);

    println!("oreboot 🦀");

    let ram_size = mctl::init();
    println!("{}M 🐏", ram_size);

    #[cfg(feature = "nor")]
    let spi_speed = 24_000_000.hz();
    #[cfg(feature = "nand")]
    let spi_speed = 100_000_000.hz();

    // prepare spi interface to use in flash
    let sck = gpio.portc.pc2.into_function_2();
    let scs = gpio.portc.pc3.into_function_2();
    let mosi = gpio.portc.pc4.into_function_2();
    let miso = gpio.portc.pc5.into_function_2();
    let spi = Spi::new(p.SPI0, (sck, scs, mosi, miso), MODE_3, spi_speed, &clocks);

    #[cfg(feature = "nor")]
    {
        let mut flash = SpiNor::new(spi);

        // e.g., GigaDevice (GD) is 0xC8 and GD25Q128 is 0x4018
        // see flashrom/flashchips.h for details and more
        let id = flash.read_id();
        println!("NOR flash: {:x}/{:x}{:x}", id[0], id[1], id[2],);

        // TODO: Either read sizes from dtfs at runtime or at build time

        println!("Load... 💾");
        let skip = 0x1 << 15; // 32K, the size of boot0
        let size = ORE_SIZE >> 2;
        load(skip, ORE_ADDR, size, &mut flash);

        // 32K + oreboot + dtfs, see oreboot dtfs
        let skip = skip + ORE_SIZE + DTF_SIZE;
        let size = (LIN_SIZE) >> 2;
        load(skip, LIN_ADDR, size, &mut flash);

        // 32K + oreboot + dtfs + payload
        let skip = skip + LIN_SIZE;
        let size = (DTB_SIZE) >> 2;
        load(skip, DTB_ADDR, size, &mut flash);

        let _ = flash.free().free();
    }

    #[cfg(feature = "nand")]
    {
        let mut flash = SpiNand::new(spi);
        println!("NAND flash: {:?}", flash.read_id());

        // TODO: Either read sizes from dtfs at runtime or at build time

        let mut main_stage_head = [0u8; 8];
        flash.copy_into(0x60, &mut main_stage_head);
        let main_stage_head: MainStageHead = unsafe { core::mem::transmute(main_stage_head) };
        println!(
            "flash offset: {}, length: {}",
            main_stage_head.offset, main_stage_head.length
        );
        let ddr_buffer = unsafe {
            core::slice::from_raw_parts_mut(RAM_BASE as *mut u8, main_stage_head.length as usize)
        };
        flash.copy_into(main_stage_head.offset, ddr_buffer);
        // flash is freed when it goes out of scope
    }

    println!("Run payload at 0x{:x}", RAM_BASE);
    unsafe {
        let f: unsafe extern "C" fn() = transmute(RAM_BASE);
        f();
    }

    // returns an address of dram payload; now cpu would jump to this address
    // and run code inside
    RAM_BASE
}

// jump to dram
extern "C" fn finish(main_stage: extern "C" fn()) -> ! {
    main_stage();
    loop {
        unsafe { asm!("wfi") }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };
    loop {
        core::hint::spin_loop();
    }
}
