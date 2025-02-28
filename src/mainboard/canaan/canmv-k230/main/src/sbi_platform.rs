use core::arch::asm;
use riscv::register::{mhartid, mie, mip};
use rustsbi::spec::binary::SbiRet;
use rustsbi::{HartMask, RustSBI};

use crate::util::{read32, read64, write32, write64};

#[derive(RustSBI)]
pub struct PlatSbi {
    //  ipi: Ipi,
    //  reset: Reset,
    timer: Timer,
}

pub fn init() -> PlatSbi {
    enable_mtimer_clock();
    init_pmp();
    PlatSbi {
        //    ipi: Ipi,
        //    reset: Reset,
        timer: Timer,
    }
}

// TODO: move to SoC lib crate
const CPU0_X: usize = crate::mem_map::HDI_BASE + 0x0020;
const CPU1_X: usize = crate::mem_map::HDI_BASE + 0x0030;
// We don't know why, but this enables the mtimer clock.
// see https://github.com/kendryte/k230_linux_sdk
// buildroot-overlay/boot/uboot/u-boot-2022.10-overlay/arch/riscv/cpu/k230/cpu.c
// harts_early_init
fn enable_mtimer_clock() {
    write32(CPU0_X, 1);
    write32(CPU1_X, 1);
}

fn init_pmp() {
    use riscv::register::*;
    let cfg = 0x0f090f0f0fusize; // pmpaddr0-1 is read-only
    pmpcfg0::write(cfg);
    pmpcfg2::write(0); // nothing active here
    pmpaddr0::write(0x00000000usize >> 2);
    pmpaddr1::write(0x00200000usize >> 2);
    pmpaddr4::write(0xffffffffusize >> 2);
}

// see C908 manual
const CLINT_BASE_OFFSET: usize = 0x0400_0000;
// these are relative to the CLINT base
const MTIME_COMPARE_OFFSET: usize = 0x4000;
const MTIME_OFFSET: usize = 0xBFF8;
// XuanTie specific second mapping for S-mode
const STIME_OFFSET: usize = 0xFFF8;

pub fn get_clint_base() -> usize {
    let mut plic_base: usize;
    unsafe {
        // 0xfc1 is MAPBADDR (M-mode APB address) as per C908 manual
        // reflects the base address of on-chip registers (CLINT, PLIC)
        asm!("csrr {}, 0xfc1", out(reg) plic_base);
    }
    plic_base + CLINT_BASE_OFFSET
}

pub fn get_mtime_compare_reg() -> usize {
    get_clint_base() + MTIME_COMPARE_OFFSET
}

pub fn get_mtime_reg() -> usize {
    get_clint_base() + MTIME_OFFSET
}

fn get_time() -> u64 {
    let mtime: u64;
    unsafe {
        asm!("csrr {}, time", out(reg) mtime);
    }
    mtime
}

const DEBUG: bool = false;

struct Timer;
impl rustsbi::Timer for Timer {
    fn set_timer(&self, value: u64) {
        if DEBUG {
            println!("[SBI] set timer: {value:016x}");
        }
        // Clear any pending timer
        unsafe { mip::clear_stimer() };

        // Set new value for this hart
        let hartid = mhartid::read();
        let mtime_cmp = get_mtime_compare_reg() + 4 * hartid;
        write64(mtime_cmp, value);

        // Reenable the interrupt
        unsafe { mie::set_mtimer() }
    }
}
