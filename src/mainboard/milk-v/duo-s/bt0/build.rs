use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// see https://github.com/sophgo/fsbl plat/cv181x/include/platform_def.h
// #define TPU_SRAM_ORIGIN_BASE 0x0C000000
// #define TPU_SRAM_SIZE 0x40000 // 256KiB

const LINKERSCRIPT_FILENAME: &str = "link-duo_s-bt0.ld";

const LINKERSCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    SRAM : ORIGIN = 0xc000000, LENGTH = 256k
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
