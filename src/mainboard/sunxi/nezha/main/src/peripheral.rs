use core::arch::asm;
use riscv::register::{mie, mip};
use rustsbi::{print, HartMask, SbiRet};

use oreboot_soc::sunxi::d1::clint::{msip, mtimecmp};

pub fn init_peripheral() {
    print!("timer init\n");
    rustsbi::init_timer(Timer);
    print!("reset init\n");
    rustsbi::init_reset(Reset);
    print!("ipi init\n");
    rustsbi::init_ipi(Ipi);
}
struct Ipi;

impl rustsbi::Ipi for Ipi {
    fn max_hart_id(&self) -> usize {
        1
    }
    fn send_ipi_many(&self, hart_mask: HartMask) -> SbiRet {
        for i in 0..=self.max_hart_id() {
            if hart_mask.has_bit(i) {
                msip::set_ipi(i);
                msip::clear_ipi(i);
            }
        }
        SbiRet::ok(0)
    }
}

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, stime_value: u64) {
        let time: u64;
        unsafe {
            asm!("csrr {}, time", out(reg) time);
        }
        // FIXME: This is an attempt to see if the timer is an issue; remove!
        print!("[rustsbi] setTimer {}\n", stime_value);
        mtimecmp::write(stime_value);
        unsafe {
            // clear the pending timer interrupt bit as well.
            mip::set_mtimer();
            if time > stime_value {
                mip::set_stimer();
            } else {
                mip::clear_stimer();
                mie::set_mtimer();
            }
        };
    }
}
pub struct Reset;

impl rustsbi::Reset for Reset {
    fn system_reset(&self, reset_type: usize, reset_reason: usize) -> rustsbi::SbiRet {
        SbiRet {
            error: reset_type | (0x114514 << 32), // magic value to exit execution loop
            value: reset_reason,
        }
    }
}
