#![feature(global_asm)]
#![no_std]
#![deny(warnings)]

pub fn init() {
    arch::init()
}

extern {
    fn _start() -> !;
}

#[no_mangle]
// TODO: It would be nice to make this more Rust-y so that the calling convention
// is checked, and that we don't need e.g. no_mangle (and maybe even unsafe) on
// the target function
pub extern "C" fn _cpu_start() -> ! {
    unsafe {
        _start()
    }
}

#[no_mangle]
pub extern "C" fn _cpu_undefined_instr() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _cpu_svc() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _cpu_prefetch_abort() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _cpu_data_abort() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _cpu_reserved_vector() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _cpu_irq() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _cpu_fiq() -> ! {
    loop {}
}

global_asm!(include_str!("vector_table.S"));
