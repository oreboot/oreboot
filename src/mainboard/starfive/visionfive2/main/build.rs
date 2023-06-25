use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKERSCRIPT_FILENAME: &str = "link-visionfive2-main.ld";

const LINKERSCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    DRAM : ORIGIN = 0x40000000, LENGTH = 512k
}
SECTIONS {
    .head : {
        *(.head.text)
        KEEP(*(.debug))
        KEEP(*(.bootblock.boot))
    } > DRAM
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(8);
    } > DRAM
    .bss : {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > DRAM

    .rodata : ALIGN(4) {
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > DRAM
    .data : ALIGN(4) {
        _sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        _edata = .;
    } > DRAM
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
