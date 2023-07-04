use core::arch::asm;
use log::println;
use riscv::register::{self as reg, mhartid, mie, mip};
use rustsbi::spec::binary::SbiRet;
use rustsbi::HartMask;
use starfive_visionfive2_lib::{clear_ipi, get_mtime, set_ipi, set_mtimecmp};

const DEBUG: bool = false;
const DEBUG_IPI: bool = true;
const DEBUG_FENCE: bool = true;
const DEBUG_TIMER: bool = false;

pub fn init() {
    init_pmp();
    // init_plic();
    let hartid = mhartid::read();

    if hartid == 1 {
        println!("[SBI] ipi init");
    }
    rustsbi::init_ipi(&Ipi);

    if hartid == 1 {
        println!("[SBI] rfence init");
    }
    rustsbi::init_remote_fence(&Rfence);

    if hartid == 1 {
        println!("[SBI] timer init");
    }
    rustsbi::init_timer(&Timer);

    if hartid == 1 {
        println!("[SBI] reset init");
    }
    rustsbi::init_reset(&Reset);
}

// TODO: move out to SBI lib?
fn init_pmp() {
    let cfg = 0x0000_0000_000f_0f0f;
    reg::pmpaddr0::write(0x0000_0000_4000_0000 >> 2);
    reg::pmpaddr1::write(0x0000_0000_4020_0000 >> 2);
    reg::pmpaddr2::write(0x00ff_ffff_ffff_ffff >> 2);
    reg::pmpaddr3::write(0);
    reg::pmpaddr4::write(0);
    reg::pmpaddr5::write(0);
    reg::pmpaddr6::write(0);
    reg::pmpaddr7::write(0);
    reg::pmpaddr8::write(0);
    reg::pmpcfg0::write(cfg);
    reg::pmpcfg2::write(0); // nothing active here
}

// FIXME: copied from nezha, does this work similar for U7/JH7110?!
// At least this hangs / crash as it is, can't even read the CSR.
fn init_plic() {
    let mut addr: usize;
    unsafe {
        asm!("csrr {}, 0xfc1", out(reg) addr); // 0x1000_0000, RISC-V PLIC
        let a = addr + 0x001ffffc; // 0x101f_fffc
        if true {
            println!("BADADDR {addr:x} SOME ADDR {a:x}");
        }
        // allow S-mode to access PLIC regs, D1 manual p210
        // core::ptr::write_volatile(a as *mut u8, 0x1);
    }
}

struct Ipi;
impl rustsbi::Ipi for Ipi {
    fn send_ipi(&self, hart_mask: HartMask) -> SbiRet {
        let hartid = mhartid::read();
        if DEBUG && DEBUG_IPI && hartid == 1 {
            println!("[SBI] IPI {hart_mask:?}");
        }
        for i in 0..=4 {
            if hart_mask.has_bit(i) {
                set_ipi(i);
                clear_ipi(i);
            }
        }
        SbiRet::success(0)
    }
}

struct Rfence;
impl rustsbi::Fence for Rfence {
    fn remote_fence_i(&self, hart_mask: HartMask) -> SbiRet {
        let hartid = mhartid::read();
        if DEBUG && DEBUG_FENCE && hartid == 1 {
            println!("[SBI] remote_fence_i {hart_mask:?}");
        }
        unsafe {
            asm!(
                "sfence.vma",  // TLB flush
                "fence.i",     // local hart
                "fence   w,w", // whatever..?
            );
        }
        for i in 0..=4 {
            if hart_mask.has_bit(i) {
                set_ipi(i);
                clear_ipi(i);
            }
        }
        SbiRet::success(0)
    }

    fn remote_sfence_vma_asid(
        &self,
        hart_mask: HartMask,
        start_addr: usize,
        size: usize,
        asid: usize,
    ) -> SbiRet {
        let hartid = mhartid::read();
        if DEBUG && DEBUG_FENCE && hartid == 1 {
            println!("[SBI] remote_sfence_vma_asid {hart_mask:?}");
        }
        SbiRet::success(0)
    }

    fn remote_sfence_vma(&self, hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet {
        let hartid = mhartid::read();
        if DEBUG && DEBUG_FENCE && hartid == 1 {
            println!("[SBI] remote_sfence_vma {hart_mask:?}");
        }
        SbiRet::success(0)
    }
}

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, stime_value: u64) {
        let hartid = mhartid::read();
        if DEBUG && DEBUG_TIMER && hartid == 1 {
            println!("[SBI] setTimer {stime_value}");
        }
        set_mtimecmp(hartid, stime_value);
        if DEBUG && DEBUG_TIMER && hartid == 1 {
            println!("[SBI] timer is set...");
        }
        // let time = riscv::register::time::read64();
        let time = get_mtime();
        unsafe {
            if time > stime_value {
                mie::clear_stimer();
                mip::set_stimer();
            } else {
                // clear any pending timer and reenable the interrupt
                mip::clear_stimer();
                mie::set_stimer();
                // mie::set_mtimer();
            }
        };
    }
}

// magic value to exit execution loop
const RESET_MAGIC: usize = 0x114514 << 32;

struct Reset;
impl rustsbi::Reset for Reset {
    fn system_reset(&self, reset_type: u32, reset_reason: u32) -> SbiRet {
        SbiRet {
            error: reset_type as usize | RESET_MAGIC,
            value: reset_reason as usize,
        }
    }
}
