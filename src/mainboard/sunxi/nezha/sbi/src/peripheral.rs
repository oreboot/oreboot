use riscv::register::mip;
use rustsbi::println;

use crate::hal::{msip, pac_encoding::UART0_BASE, Serial};

pub fn init_peripheral() {
    // serial is used for both println and SBI console
    let serial = Serial::new(UART0_BASE);
    rustsbi::legacy_stdio::init_legacy_stdio_embedded_hal(serial);
    rustsbi::init_timer(Timer);
    rustsbi::init_reset(Reset);
    rustsbi::init_ipi(Ipi);
}
struct Ipi;

impl rustsbi::Ipi for Ipi {
    fn max_hart_id(&self) -> usize {
        1
    }
    fn send_ipi_many(&self, hart_mask: rustsbi::HartMask) -> rustsbi::SbiRet {
        for i in 0..=self.max_hart_id() {
            if hart_mask.has_bit(i) {
                msip::set_ipi(i);
                msip::clear_ipi(i);
            }
        }
        rustsbi::SbiRet::ok(0)
    }
}

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, stime_value: u64) {
        println!("[rustsbi] setTimer");
        use crate::hal::clint::mtimecmp;
        mtimecmp::write(stime_value);
        unsafe {
            // clear the pending timer interrupt bit as well.
            mip::clear_mtimer();
            mip::set_mtimer()
        };
    }
}
pub struct Reset;

impl rustsbi::Reset for Reset {
    fn system_reset(&self, reset_type: usize, reset_reason: usize) -> rustsbi::SbiRet {
        // TODO: shut down all harts
        println!(
            "[rustsbi] Reset triggered. Program halt. Type: {}, reason: {}",
            reset_type, reset_reason
        );
        #[allow(clippy::empty_loop)]
        loop {}
    }
}
