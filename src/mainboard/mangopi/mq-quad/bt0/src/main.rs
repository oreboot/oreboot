#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};

const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

// FIXME: The compiler would add a `BRK` (breakpoint) instruction hereafter.
// No clue why, so just add `jump` as an inline word to eGON header instead.
/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
/*
#[naked]
#[link_section = ".head.text"]
#[export_name = "head_jump"]
pub unsafe extern "C" fn head_jump() {
    asm!(
        // "b.ne .+0x68", // 0x60: eGON.BT0 header; 0x08: FlashHead
        ".word 0x54000341",
        options(noreturn)
    )
}
*/
// todo: option(noreturn) generates an extra `unimp` insn

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
    jump: 0x54000341, // b.ne 0x68
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

// TODO: Arm asm
/// Clear stuff and jump to main.
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // 2. initialize programming language runtime
        // clear bss segment
        "ldr     x1, sbss",
        "ldr     x2, ebss",
        "1:",
        // jump out of loop once x2 reaches x1
        "sub     x3, x2, x1",
        "cbz     x3, 1f",
        // clear out the respective address in memory
        "str     x0, [x2], #0",
        "sub     x2, x2, 4",
        "bl      1b",
        "1:",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "ldr     x1, {stack}",
        "mov     sp, x1",
        "ldr     x1, {stack_size}",
        "add     sp, sp, x1",
        // hack to include eGON header
        "ldr     xzr, {egon_head}",
        "bl   {main}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        egon_head  =   sym EGON_HEAD,
        main       =   sym main,
        options(noreturn)
    )
}

extern "C" fn main() -> usize {
    // TODO: code.....
    0
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    /*
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };
    */
    loop {
        core::hint::spin_loop();
    }
}
