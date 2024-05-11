use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[cfg(soc = "CV1800B")]
core::compile_error!("This is for SG200x only");

// see https://github.com/sophgo/fsbl
//
// Earlier SoCs (CV1800B, Milk-V Duo)
//
//  plat/cv180x/bl2/bl2.ld.S
//  19:    RAM (rwx): ORIGIN = BL2_BASE, LENGTH = BL2_SIZE
//
//  plat/cv180x/include/mmap.h
//  48:#define BL2_BASE (VC_RAM_BASE)
//
//  plat/cv180x/include/platform_def.h
//  289:    #define TPU_SRAM_SIZE 0x10000 // 64KiB
//  296:    #define VC_RAM_BASE 0x3BC00000 // Shadow_vc_mem
//
// Later SoCs (SG200x, Milk-V Duo 256M, Duo S)
//
//  plat/cv181x/include/platform_def.h
//          #define TPU_SRAM_ORIGIN_BASE 0x0C000000
//          #define TPU_SRAM_SIZE 0x40000 // 256KiB

const LINKERSCRIPT_FILENAME: &str = "link-duo_s-bt0a.ld";

// NOTE: The Arm core runs code at a different base address than the RISC-V.
const LINKERSCRIPT: &[u8] = b"
OUTPUT_ARCH(aarch64)
ENTRY(start)
MEMORY {
    SRAM : ORIGIN = 0x40100000, LENGTH = 64k
}
SECTIONS {
    .head : {
        *(.head.text)
    } > SRAM
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(8);
    } > SRAM
    .bss : {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > SRAM

    # https://docs.rust-embedded.org/embedonomicon/main.html
    .rodata : {
        *(.rodata .rodata.*);
    } > SRAM #FLASH
    .data : {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > SRAM
    _sidata = LOADADDR(.data);

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug_*)
        *(.comment*)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(LINKERSCRIPT_FILENAME))
        .unwrap()
        .write_all(LINKERSCRIPT)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
