use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const FLASH: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    SRAM : ORIGIN = 0x40000000, LENGTH = 128k
}
SECTIONS {
    .head : {
        *(.head.text)
        KEEP(*(.debug))
        KEEP(*(.bootblock.boot))
    } > SRAM
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > SRAM
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > SRAM
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > SRAM
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link-visionfive2-main.ld"))
        .unwrap()
        .write_all(FLASH)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
