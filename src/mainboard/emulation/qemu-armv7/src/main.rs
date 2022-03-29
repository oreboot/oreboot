#![no_std]
#![no_main]

mod romstage;
use core::arch::{asm, global_asm};
use core::fmt::Write;
use device_tree::print_fdt;
use oreboot_cpu::armltd::cortex_a9 as cpu;
use oreboot_drivers::{
    uart::pl011::PL011,
    wrappers::{DoD, Memory, SectionReader},
    Driver,
};

const DTFS_BASE: usize = 0x800000;
const DTFS_SIZE: usize = 0x80000;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut pl011 = PL011::new(0x09000000, 115200);
    let uart_driver: &mut dyn Driver = &mut pl011;
    // TODO: Handle error here and quit, rather than unwrapping.
    uart_driver.init().unwrap();
    uart_driver.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    let s = &mut [uart_driver];
    let console = &mut DoD::new(s);
    let mut w = print::WriteTo::new(console);

    // TODO: determine DTFS_BASE+SIZE based on layoutflash (or some other toolchain component)
    let dtfs = SectionReader::new(&Memory {}, DTFS_BASE, DTFS_SIZE);
    if let Err(err) = print_fdt(&dtfs, &mut w) {
        writeln!(w, "error: {}", err).expect(err);
    }

    cpu::init();
    let mut w = print::WriteTo::new(console);
    write!(w, "hi").expect("blame ryan");
    write!(w, "1").expect("blame ryan");
    write!(w, "2").expect("blame ryan");
    write!(w, "3").expect("blame ryan");
    write!(w, "4").expect("blame ryan");
    write!(w, "5").expect("blame ryan");
    write!(w, "6").expect("blame ryan");
    write!(w, "7").expect("blame ryan");
    writeln!(w, "{}7", 3).expect("blame ryan");
    romstage::romstage(&mut w)
}
use core::panic::PanicInfo;

pub fn halt() -> ! {
    loop {
        // Bug with LLVM marks empty loops as undefined behaviour.
        // See: https://github.com/rust-lang/rust/issues/28728
        unsafe { asm!("nop") }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

global_asm!(include_str!("vector_table.S"));
