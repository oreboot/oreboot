use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const FLASH: &[u8] = b"
OUTPUT_ARCH(riscv)
OUTPUT_FORMAT(elf64-littleriscv)

ENTRY (start)

MEMORY {
  DRAM (rwx): ORIGIN = 0x3EF80000, LENGTH = 512k
  CODE (rwx): ORIGIN = 0x3EFF0000, LENGTH = 64k
  VRAM (rwx): ORIGIN = 0x3F000000, LENGTH = 32k
}

SECTIONS {
  . = ORIGIN(DRAM);

  .text . : ALIGN(16) {
    KEEP(*(.text.entry))
    KEEP(*(.text.main))
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.text*)))
    . = ALIGN(16);
  } > CODE

  .rodata . : ALIGN(16) {
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.rodata*)))
    . = ALIGN(16);
  } > CODE

  .data . : ALIGN(16) {
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.data*)))
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.sdata*)))
    . = ALIGN(16);
  } > CODE

  .bss . : ALIGN(16) {
    __bss_start = .;
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.sbss*)))
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.bss*)))
    . = ALIGN(16);
    __bss_end = .;
  } > CODE

  __stack_start = .;
  . += 0x1000;
  __stack_end = .;
}
";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link-bl808-c906.ld"))
        .unwrap()
        .write_all(FLASH)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
