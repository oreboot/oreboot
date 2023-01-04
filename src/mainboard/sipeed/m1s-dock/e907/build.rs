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
  RAM (rwx): ORIGIN = 0x22020000, LENGTH = 0x38000
}

SECTIONS {
  . = ORIGIN(RAM);

  .text . : ALIGN(16) {
    KEEP(*(.text.entry))
    KEEP(*(.text.main))
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.text*)))
    . = ALIGN(16);
  } > RAM

  .rodata . : ALIGN(16) {
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.rodata*)))
    . = ALIGN(16);
  } > RAM

  .data . : ALIGN(16) {
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.data*)))
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.sdata*)))
    . = ALIGN(16);
  } > RAM

  .bss . : ALIGN(16) {
    __bss_start = .;
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.sbss*)))
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.bss*)))
    . = ALIGN(16);
    __bss_end = .;
  } > RAM

  __stack_start = .;
  . += 0x1000;
  __stack_end = .;
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
