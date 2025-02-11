use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// NOTE: We omit 4 bytes here so that the same binary can be used for flashing.
// In flash, the first 4 bytes enode the size of the binary to load into SRAM.
const FLASH: &[u8] = b"
OUTPUT_ARCH(riscv)
OUTPUT_FORMAT(elf32-littleriscv)

ENTRY (start)

MEMORY {
  SRAM (rwx): ORIGIN = 0x22020000, LENGTH = 0x38000
}

SECTIONS {
    .head : {
        *(.head.text)
        KEEP(*(.debug))
        KEEP(*(.bootblock.boot))
    } > SRAM
    .text . : {
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
    .bss (NOLOAD) : ALIGN(4096) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}
";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link-bl808-e907.ld"))
        .unwrap()
        .write_all(FLASH)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
