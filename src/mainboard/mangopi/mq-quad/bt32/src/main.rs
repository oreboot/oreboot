#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};

/*
// FIXME: The compiler would add a `BRK` (aarch64) instruction hereafter.
// FIXME: The compiler would add a `UDF` (aarch32) instruction hereafter.
// No clue why, so just add `jump` as an inline word to eGON header instead.
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
        "b .+0x64", // 0x60: eGON.BT0 header; 0x08: FlashHead
        // ".word 0x54000341", // this is the result...
        options(noreturn)
    )
}
*/
// eGON.BT0 header. This header is identified by D1 ROM code
// to copy BT0 stage bootloader into SRAM memory.
// This header takes 0x60 bytes.
#[repr(C)]
pub struct EgonHead {
    jump: u32,
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

// clobber used by KEEP(*(.head.egon)) in link script
#[link_section = ".head.egon"]
pub static EGON_HEAD: EgonHead = EgonHead {
    jump: 0xea000017, // b 0x64
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

/// All 64-bit capable Allwinner SoCs reset in AArch32 (and continue to
/// exectute the Boot ROM in this state), so we need to switch to AArch64
/// at some point.
/// https://github.com/u-boot/u-boot/blob/master/arch/arm/mach-sunxi/rmr_switch.S
///
/// Clear stuff and jump to main.
/// Kudos to Azeria \o/
/// https://azeria-labs.com/memory-instructions-load-and-store-part-4/
/// Xn registers are 64-bit, general purpose; X31 aka Xzr is always 0
/// Wn registers are 32-bit and aliases of lower half of Xn
/// https://linux-sunxi.org/Arm64
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // hack to include eGON header
        "ldr     r0, {egon_head}",
        "bl   {main}",
        egon_head  =   sym EGON_HEAD,
        main       =   sym main,
        options(noreturn)
    )
}

// H0 is TX, H1 is RX

// p695
const GPIO_BASE: usize = 0x0300_B000;
const GPIO_PORTC_CFG1: usize = GPIO_BASE + 0x004C; // PC8-15
const GPIO_PORTC_DATA: usize = GPIO_BASE + 0x0058;
const PC13_OUT: u32 = 0b001 << 20;
const PC13_HIGH: u32 = 1 << 13;

// SUN50I GEN H6
const RVBAR: usize = 0x0170_00a0;

// const RVBAR: usize = 0x0901_0040;

const START_AARCH64: u32 = 0x0020_0300;

extern "C" fn main() -> ! {
    unsafe {
        // set PC13 high (status LED)
        write_volatile(GPIO_PORTC_CFG1 as *mut u32, PC13_OUT); // set to out
        let new_addr = START_AARCH64;
        write_volatile(RVBAR as *mut u32, new_addr);
        for _ in 0..3 {
            write_volatile(GPIO_PORTC_DATA as *mut u32, PC13_HIGH);
            for _ in 0..0x1f0000 {
                core::hint::spin_loop();
            }
            write_volatile(GPIO_PORTC_DATA as *mut u32, 0);
            for _ in 0..0x1f0000 {
                core::hint::spin_loop();
            }
        }
        asm!(
            "dsb	sy",
            "isb	sy",
            "mrc	p15, 0, r0, cr12, cr0, 2", // read RMR register
            "orr	r0, r0, #3",               // request reset in AArch64
            "mcr	p15, 0, r0, cr12, cr0, 2", // write RMR register
            "isb	sy",
        );
        loop {
            asm!("wfi");
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
