use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const NEZHA_FLASH: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    DDR : ORIGIN = 0x40000000, LENGTH = 1M
}
SECTIONS {
    .text : {
        *(.text.entry)
        *(.text .text.*)
    } > DDR
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > DDR
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > DDR
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > DDR
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link-nezha-main.ld"))
        .unwrap()
        .write_all(NEZHA_FLASH)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
