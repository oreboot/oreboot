use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKER_SCRIPT_FILE: &str = "link-allwinner-h616-bt32.ld";

// NOTE: The first 60 bytes are for the header, which we prepend via xtask.
const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(armv7a)
ENTRY(head_jump)
MEMORY {
    SRAM : ORIGIN = 0x00020060, LENGTH = 16288
}
SECTIONS {
    .head : {
        *(.head.text)
        KEEP(*(.head.main))
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
        *(.ARM.exidx)
        *(.eh_frame)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(LINKER_SCRIPT_FILE))
        .unwrap()
        .write_all(LINKER_SCRIPT)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
