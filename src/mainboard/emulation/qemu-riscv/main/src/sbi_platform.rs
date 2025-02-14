use riscv::register::{self as reg, mhartid, mie, mip};
use rustsbi::spec::binary::SbiRet;
use rustsbi::{HartMask, RustSBI};

const DEBUG: bool = false;
const DEBUG_TIMER: bool = true;

#[derive(RustSBI)]
pub struct PlatSbi {
    timer: Timer,
}

pub fn init() -> PlatSbi {
    init_pmp();
    PlatSbi { timer: Timer }
}

fn init_pmp() {
    let cfg = 0x0f0f0f0f0fusize; // pmpaddr0-1 and pmpaddr2-3 are read-only
    reg::pmpcfg0::write(cfg);
    reg::pmpcfg2::write(0); // nothing active here
    reg::pmpaddr0::write(0x80000000usize >> 2);
    reg::pmpaddr1::write(0x80200000usize >> 2);
}

use core::ptr::{read_volatile, write_volatile};

pub fn write32(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

pub fn write64(reg: usize, val: u64) {
    write32(reg, val as u32);
    write32(reg + 4, (val >> 32) as u32);
}

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, stime_value: u64) {
        if DEBUG && DEBUG_TIMER {
            println!("[SBI] setTimer {stime_value}");
        }
        // Clear any pending timer
        unsafe { mip::clear_stimer() };
        // Set new value for this hart
        let hartid = mhartid::read();
        write64(crate::mem_map::MTIME_COMPARE + 4 * hartid, stime_value);
        // Reenable the interrupt
        unsafe { mie::set_mtimer() }
    }
}
