use core::arch::asm;
use oreboot_soc::sunxi::d1::clint::{msip, mtimecmp};
use riscv::register::{mie, mip};
use rustsbi::spec::binary::SbiRet;
use rustsbi::{print, HartMask};

pub fn init_peripheral() {
    print!("timer init\n");
    rustsbi::init_timer(&Timer);
    print!("reset init\n");
    rustsbi::init_reset(&Reset);
    print!("ipi init\n");
    rustsbi::init_ipi(&Ipi);
}
struct Ipi;

impl rustsbi::Ipi for Ipi {
    fn send_ipi(&self, hart_mask: HartMask) -> SbiRet {
        // TODO: This was a member function in previous RustSBI
        fn max_hart_id() -> usize {
            1
        }
        for i in 0..=max_hart_id() {
            if hart_mask.has_bit(i) {
                msip::set_ipi(i);
                msip::clear_ipi(i);
            }
        }
        SbiRet::success(0)
    }
}

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, stime_value: u64) {
        let time: u64;
        unsafe {
            asm!("csrr {}, time", out(reg) time);
        }
        // print!("[rustsbi] setTimer {}\n", stime_value);
        mtimecmp::write(stime_value);
        unsafe {
            if time > stime_value {
                mip::set_stimer();
            } else {
                // clear any pending timer and reenable the interrupt
                mip::clear_stimer();
                mie::set_mtimer();
            }
        };
    }
}
pub struct Reset;

// magic value to exit execution loop
const RESET_MAGIC: usize = 0x114514 << 32;

impl rustsbi::Reset for Reset {
    fn system_reset(&self, reset_type: u32, reset_reason: u32) -> SbiRet {
        SbiRet {
            error: reset_type as usize | RESET_MAGIC,
            value: reset_reason as usize,
        }
    }
}
