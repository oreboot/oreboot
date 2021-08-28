#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

const PAGE_SIZE: usize = 4096;
use rpp_procedural::preprocess_asm;

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

global_asm!(preprocess_asm!("src/mode_switch.S"));

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { llvm_asm!("hlt" :::: "volatile") }
    }
}

pub fn fence() {
    unsafe { llvm_asm!("nop" :::: "volatile") }
}

pub fn nop() {
    unsafe { llvm_asm!("nop" :::: "volatile") }
}

pub fn enable_sse() {
    unsafe {
        llvm_asm!(
            r#"
            movq %cr0, %rax
            /* CR0.EM=0: disable emulation, otherwise SSE instruction cause #UD */
            andw $$0xFFFB, %ax
            /* CR0.MP=1: enable monitoring coprocessor */
            orw $$0x0002, %ax
            movq %rax, %cr0

            movq %cr4, %rax
            /* CR4.OSFXSR=1: Operating System Support for FXSAVE and FXRSTOR instructions */
            /* CR4.OSXMMEXCPT=1: Operating System Support for Unmasked SIMD Floating-Point Exceptions */
            orw $$0x0600, %ax
            movq %rax, %cr4"#
            ::: "rax" : "volatile")
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
