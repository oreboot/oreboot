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
    // pmpcfg0 packs eight 8-bit entries (RV64: entries 0..=7). Each byte:
    //
    //   bit    7   6   5   4   3   2   1   0
    //        +---+---+---+---+---+---+---+---+
    //        | L | - | - | A | A | X | W | R |
    //        +---+---+---+---+---+---+---+---+
    //          |             |     |   |   |
    //          |             |     |   |   +--> R : read
    //          |             |     |   +------> W : write
    //          |             |     +----------> X : execute
    //          |             +----------------> A : 0=OFF, 1=TOR, 2=NA4, 3=NAPOT
    //          +------------------------------> L : lock
    //
    //   0x0f = R|W|X | TOR      0x09 = R | TOR      0x1f = R|W|X | NAPOT
    //
    //   entry 0: TOR   [0x0, 0x8000_0000)            RWX  MMIO / low memory
    //   entry 1: TOR   [0x8000_0000, 0x8020_0000)    R    oreboot/SBI firmware
    //   entry 2: NAPOT [0x0, 0xffff_ffff_ffff_ffff]  RWX  payload RAM
    //
    // Note: `entry 2` uses NAPOT with pmpaddr = all-ones, the spec's encoding for the
    // entire address space. Being the highest-numbered entry it is lowest priority,
    // so entries 0 and 1 still protect their ranges;
    // NAPOT (unlike TOR) can also cover the uppermost word.
    let cfg = 0x1f090fusize;
    reg::pmpcfg0::write(cfg);
    reg::pmpcfg2::write(0); // nothing active here
    reg::pmpaddr0::write(0x80000000usize >> 2);
    reg::pmpaddr1::write(0x80200000usize >> 2);
    reg::pmpaddr2::write(usize::MAX);
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
