#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![feature(global_asm)]
#![deny(warnings)]

const PAGE_SIZE: usize = 4096;
pub mod acpi;
pub mod bzimage;
pub mod consts;
pub mod ioport;

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
