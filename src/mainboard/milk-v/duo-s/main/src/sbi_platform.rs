use core::arch::asm;
use riscv::register::{self as reg, mie, mip};
use rustsbi::spec::binary::SbiRet;
use rustsbi::HartMask;

pub fn init() {
    // TODO
    init_pmp();
    println!("[SBI] PLIC init");
    init_plic();
    println!("[SBI] ipi init");
    rustsbi::init_ipi(&Ipi);
    println!("[SBI] rfence init");
    rustsbi::init_remote_fence(&Rfence);
    println!("[SBI] timer init");
    rustsbi::init_timer(&Timer);
    println!("[SBI] reset init");
    rustsbi::init_reset(&Reset);
}

/**
 * from stock vendor OpenSBI:
 * PMP0    : 0x0000000040000000-0x000000004001ffff (A)
 * PMP1    : 0x0000000040000000-0x000000007fffffff (A,R,W,X)
 * PMP2    : 0x0000000000000000-0x0000000007ffffff (A,R,W)
 * PMP3    : 0x0000000009000000-0x000000000901ffff (
 */
// see privileged spec v1.10 p44 ff
// https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
fn init_pmp() {
    if false {
        let cfg = 0x0f0f0f0f0fusize; // pmpaddr0-1 and pmpaddr2-3 are read-only
        reg::pmpcfg0::write(cfg);
        reg::pmpcfg2::write(0); // nothing active here
        reg::pmpaddr0::write(0x40000000usize >> 2);
        reg::pmpaddr1::write(0x40200000usize >> 2);
        reg::pmpaddr2::write(0x80000000usize >> 2);
        reg::pmpaddr3::write(0x80200000usize >> 2);
    }
    // A: address matching; 0x01 means TOR (Top of range)
    // [ L  x  x  A1   A0  X  W  R ]
    // let cfg = 0x0000_0000_0f08_0f0f;
    let cfg = 0x0000_0000_0f0f_0f0f;
    reg::pmpaddr0::write(0x0);
    reg::pmpaddr1::write(0x0000_0000_4000_0000 >> 2);
    reg::pmpaddr2::write(0x0000_0000_4020_0000 >> 2);
    reg::pmpaddr3::write(0x00ff_ffff_ffff_ffff >> 2);
    reg::pmpaddr4::write(0);
    reg::pmpaddr5::write(0);
    reg::pmpaddr6::write(0);
    reg::pmpaddr7::write(0);
    reg::pmpaddr8::write(0);
    reg::pmpcfg0::write(cfg);
    reg::pmpcfg2::write(0); // nothing active here
}

fn init_plic() {
    let mut addr: usize;
    unsafe {
        // What? 0xfc1 is BADADDR as per C906 manual; this seems to work though
        asm!("csrr {}, 0xfc1", out(reg) addr); // 0x1000_0000, RISC-V PLIC
        let a = addr + 0x001ffffc; // 0x101f_fffc
        if false {
            println!("BADADDR {addr:x} SOME ADDR {a:x}");
        }
        // allow S-mode to access PLIC regs, D1 manual p210
        core::ptr::write_volatile(a as *mut u8, 0x1);
    }
}

struct Ipi;
impl rustsbi::Ipi for Ipi {
    fn send_ipi(&self, hart_mask: HartMask) -> SbiRet {
        // TODO
        SbiRet::success(0)
    }
}

struct Rfence;
impl rustsbi::Fence for Rfence {
    fn remote_fence_i(&self, hart_mask: HartMask) -> SbiRet {
        // TODO
        SbiRet::success(0)
    }

    fn remote_sfence_vma_asid(
        &self,
        hart_mask: HartMask,
        start_addr: usize,
        size: usize,
        asid: usize,
    ) -> SbiRet {
        // TODO
        SbiRet::success(0)
    }

    fn remote_sfence_vma(&self, hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet {
        // TODO
        SbiRet::success(0)
    }
}

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, stime_value: u64) {
        // TODO
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
