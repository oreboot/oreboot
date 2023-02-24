use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKERSCRIPT_FILENAME: &str = "link-visionfive2-bt0.ld";

// NOTE: The SRAM is called "L2 LIM", documented here:
// https://doc-en.rvspace.org/JH7110/TRM/JH7110_TRM/u74_memory_map.html
const FLASH: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    SRAM : ORIGIN = 0x08000000, LENGTH = 128k
    # DRAM : ORIGIN = 0x80000000, LENGTH = 8G
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
        .write_all(FLASH)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
