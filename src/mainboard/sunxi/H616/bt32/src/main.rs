#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::{arch::asm, panic::PanicInfo};

use util::mmio::write32;

/*
// FIXME: The compiler would add a `BRK` (aarch64) instruction hereafter.
// FIXME: The compiler would add a `UDF` (aarch32) instruction hereafter.
// No clue why, so just add `jump` as an inline word to eGON header instead.
/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
#[unsafe(naked)]
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
    jump32: u32,
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
    jump32: 0xea000016, // b 0x60
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
#[unsafe(naked)]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    naked_asm!(
        // hack to include eGON header; we skip this via the jump from header
        "bl   {main}",
        ".word 0x140001e7", // b #0x490 in Aarch64
        "ldr  r0, {egon_head}",
        egon_head  =   sym EGON_HEAD,
        main       =   sym main
    )
}

// H0 is TX, H1 is RX

// p695
const GPIO_BASE: usize = 0x0300_B000;
const GPIO_PORTC_CFG1: usize = GPIO_BASE + 0x004C; // PC8-15
const GPIO_PORTC_DATA: usize = GPIO_BASE + 0x0058;
const PC13_OUT: u32 = 0b001 << 20;
const PC13_HIGH: u32 = 1 << 13;

// See U-Boot arch/arm/mach-sunxi/Kconfig
//   under config MACH_SUN50I_H6(16), there is select SUN50I_GEN_H6
// for H6 targets
const RVBAR: usize = 0x0901_0040;
// non-H6 targets
// const RVBAR: usize = 0x0170_00a0;

const RVBAR_ALT: usize = 0x0810_0040;
// for non-H6
// const RVBAR_ALT: usize = RVBAR;

const START_AARCH64: u32 = 0x0002_0000 + 2048;

fn blink(delay: u32) {
    let cycs = delay * 0x10000;
    write32(GPIO_PORTC_DATA, PC13_HIGH);
    for _ in 0..cycs {
        core::hint::spin_loop();
    }
    write32(GPIO_PORTC_DATA, 0);
    for _ in 0..cycs {
        core::hint::spin_loop();
    }
}

#[inline]
fn save_regs() {
    #[allow(named_asm_labels)]
    unsafe {
        asm!(
            "b .code",
            // leave space to store information
            ".fel_stash: ",
            ".word 0x00000000", // SP
            ".word 0x00000000", // LR
            ".word 0x00000000", // CPSR
            ".word 0x00000000", // SCTLR
            ".word 0x00000000", // VBAR
            ".word 0x00000000", // SP_IRQ
            ".word 0x00000000", // ICC_PMR
            ".word 0x00000000", // ICC_IGRPEN1
            ".code:",
            "adr     r0, .fel_stash",
            "ldr     r1, .fel_stash",
            "add     r0, r0, r1",
            "str     sp, [r0]",
            "str     lr, [r0, #4]",
            "mrs     lr, CPSR",
            "str     lr, [r0, #8]",
            "mrc     p15, 0, lr, cr1, cr0, 0", // SCTLR
            "str     lr, [r0, #12]",
            "mrc     p15, 0, lr, cr12, cr0, 0", // VBAR
            "str     lr, [r0, #16]",
            "mrc     p15, 0, lr, cr12, cr12, 5", // ICC_SRE
            "tst     lr, #1",
            "beq     2f",
            "mrc     p15, 0, lr, c4, c6, 0", // ICC_PMR
            "str     lr, [r0, #24]",
            "mrc     p15, 0, lr, c12, c12, 7", // ICC_IGRPEN1
            "str     lr, [r0, #28]",
            "2:"
        );
    }
}

#[inline]
fn reset64() {
    if false {
        write32(RVBAR, START_AARCH64);
    } else {
        write32(RVBAR_ALT, START_AARCH64);
    }
    unsafe {
        asm!(
            "dsb	sy",
            "isb	sy",
            "mrc	p15, 0, r0, cr12, cr0, 2", // read RMR register
            "orr	r0, r0, #3",               // request reset in AArch64
            "mcr	p15, 0, r0, cr12, cr0, 2", // write RMR register
            "isb	sy",
        );
    }
}

// see also https://iitd-plos.github.io/col718/ref/arm-instructionset.pdf
#[no_mangle]
pub extern "C" fn main() -> ! {
    // set PC13 (status LED) to output
    write32(GPIO_PORTC_CFG1, PC13_OUT);
    // first sign of life
    blink(23);
    save_regs();
    blink(23);
    reset64();
    loop {
        blink(10);
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
