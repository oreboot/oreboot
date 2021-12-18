#![feature(asm, lang_items, start)]
#![feature(asm_const)]
#![no_std]
#![feature(global_asm)]

const PAGE_SIZE: usize = 4096;

pub mod acpi;
pub mod bzimage;
pub mod consts;
pub mod ioport;

// NOTE: The ROM page table is defined by a symbol in the bootblock. It
// will be populated at runtime in new_rom_util.
const ROM_DAT32: u32 = 0x20;
const ROM_CODE32: u32 = 0x18;
const ROM_CODE64: u32 = 0x28;

const RAM_DAT32: u32 = 0x10;
const RAM_CODE32: u32 = 0x08;
const RAM_CODE64: u32 = 0x18;
const RAM_PAGE_TABLE_ADDR: u32 = 0x7e000;

pub struct X86Util {
    page_table_addr: u32,
    code64_seg: u32,
    code32_seg: u32,
    data32_seg: u32,
}

impl X86Util {
    // TODO: Refactor this so each boot block has a function
    // to create an X86Util object.
    pub fn new_ram_util() -> Self {
        X86Util {
            page_table_addr: RAM_PAGE_TABLE_ADDR,
            code64_seg: RAM_CODE64,
            code32_seg: RAM_CODE32,
            data32_seg: RAM_DAT32,
        }
    }

    pub fn new_rom_util() -> Self {
        let page_table = unsafe { &pml4 as *const _ as u32 };

        X86Util {
            page_table_addr: page_table,
            code64_seg: ROM_CODE64,
            code32_seg: ROM_CODE32,
            data32_seg: ROM_DAT32,
        }
    }

    /// TODO: Make parameters and return value more rust-y?
    pub fn protected_mode_call(&self, func_ptr: u32, arg1: u32, arg2: u32) -> u32 {
        unsafe {
            let mut info = BootBlockInfo {
                code64_seg: self.code64_seg,
                code32_seg: self.code32_seg,
                data32_seg: self.data32_seg,
                page_table_addr: self.page_table_addr,
            };

            protected_mode_call_impl(func_ptr, arg1, arg2, &mut info)
        }
    }
}

use self::consts::*;
global_asm!(
    include_str!("mode_switch.S"),
    CD = const x86::cr0::CD,
    NW = const x86::cr0::NW,
    TS = const x86::cr0::TS,
    MP = const x86::cr0::MP,
    PG = const x86::cr0::PG,
    WP = const x86::cr0::WP,
    PSE = const x86::cr4::PSE,
    PAE = const x86::cr4::PAE,
    EFER = const msr::EFER,
    LME = const msr::efer::LME,
    options(att_syntax)
);
/*
self::const_asm!(include_str!("mode_switch.S"), options(att_syntax));
*/

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("hlt") }
    }
}

pub fn fence() {
    unsafe { asm!("nop") }
}

pub fn nop() {
    unsafe { asm!("nop") }
}

pub fn enable_sse() {
    let mut cr0: u64;
    let mut cr4: u64;
    unsafe {
        asm!("movq %cr0, %rax", out("rax") cr0, options(att_syntax));
        /* CR0.EM=0: disable emulation, otherwise SSE instruction cause #UD */
        cr0 &= 0xFFFF_FFFB;
        cr0 |= 0x2; /* CR0.MP=1: enable monitoring coprocessor */
        asm!("movq %rax, %cr0", in("rax") cr0, options(att_syntax));

        asm!("movq %cr4, %rax", out("rax") cr4, options(att_syntax));
        /* CR4.OSFXSR=1: Operating System Support for FXSAVE and FXRSTOR instructions */
        /* CR4.OSXMMEXCPT=1: Operating System Support for Unmasked SIMD Floating-Point Exceptions */
        cr4 |= 0x0600;
        asm!("movq %rax, %cr4", in("rax") cr4, options(att_syntax));
    }
}

#[repr(C)]
struct BootBlockInfo {
    code64_seg: u32,
    code32_seg: u32,
    data32_seg: u32,
    page_table_addr: u32,
}

extern "C" {

    static pml4: u8;

    fn protected_mode_call_impl(
        func_ptr: u32,
        arg1: u32,
        arg2: u32,
        info: *mut BootBlockInfo,
    ) -> u32;

}
