#![no_std]
#![feature(asm)]
#![feature(generator_trait)]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]

mod execute;
mod feature;
mod hal;
mod hart_csr_utils;
mod peripheral;
mod runtime;

extern crate alloc;
extern crate bitflags;

use crate::{hal::write_reg, hart_csr_utils::print_hart_pmp};
use buddy_system_allocator::LockedHeap;
use riscv::register::{medeleg, mideleg, mie};
use rustsbi::println;

const SBI_HEAP_SIZE: usize = 8 * 1024; // 8KiB

static mut HEAP_SPACE: [u8; SBI_HEAP_SIZE] = [0; SBI_HEAP_SIZE];
static PLATFORM: &str = "T-HEAD Xuantie Platform";
#[global_allocator]
static SBI_HEAP: LockedHeap<32> = LockedHeap::empty();

pub fn sbi_init(payload_offset: usize, dtb_offset: usize) -> ! {
    let hartid = riscv::register::mhartid::read();
    if hartid == 0 {
        init_bss();
    }
    init_pmp();
    runtime::init();
    if hartid == 0 {
        init_heap();
        unsafe {
            init_plic();
        }
        peripheral::init_peripheral();
        println!("[rustsbi] RustSBI version {}\r", rustsbi::VERSION);
        println!("{}", rustsbi::LOGO);
        println!("[rustsbi] Platform Name: {}\r", PLATFORM);
        println!(
            "[rustsbi] Implementation: RustSBI-NeZha Version {}\r",
            env!("CARGO_PKG_VERSION")
        );
    }
    unsafe {
        delegate_interrupt_exception();
    }
    if hartid == 0 {
        hart_csr_utils::print_hart_csrs();
        println!("[rustsbi] enter supervisor 0x{:x}\r", payload_offset);
        println!("[rustsbi] dtb handed over from 0x{:x}\r", dtb_offset);
        print_hart_pmp();
    }
    execute::execute_supervisor(payload_offset, hartid, dtb_offset)
}

fn init_bss() {
    extern "C" {
        // static mut __bss_start: u32;
        // static mut __bss_end: u32;
        static mut edata: u32;
        static mut sdata: u32;
        static sidata: u32;
    }
    unsafe {
        // r0::zero_bss(&mut __bss_start, &mut __bss_end);
        r0::init_data(&mut sdata, &mut edata, &sidata);
    }
}

/**
 * from stock vendor OpenSBI:
 * PMP0    : 0x0000000040000000-0x000000004001ffff (A)
 * PMP1    : 0x0000000040000000-0x000000007fffffff (A,R,W,X)
 * PMP2    : 0x0000000000000000-0x0000000007ffffff (A,R,W)
 * PMP3    : 0x0000000009000000-0x000000000901ffff (
 */
// TODO: protect oreboot; this is an all-accessible config
fn init_pmp() {
    use riscv::register::*;
    let cfg = 0x0f0f0f0f0fusize;
    pmpcfg0::write(cfg);
    // pmpcfg2::write(0);
    pmpaddr0::write(0x40000000usize >> 2);
    pmpaddr1::write(0x40200000usize >> 2);
    pmpaddr2::write(0x80000000usize >> 2);
    pmpaddr3::write(0xc0000000usize >> 2);
    pmpaddr4::write(0xffffffffusize >> 2);
}

unsafe fn init_plic() {
    let mut addr: usize;
    asm!("csrr {}, 0xfc1", out(reg) addr);
    write_reg(addr, 0x001ffffc, 0x1)
}

/*
 * From stock Nezha OpenSBI:
 *
 * MIDELEG : 0x0000000000000222
 * MEDELEG : 0x000000000000b1ff
 *
 * QEMU OpenSBI 0.9:
 *
 * Boot HART MIDELEG         : 0x0000000000000222
 * Boot HART MEDELEG         : 0x000000000000b109
 */
// see riscv-privileged spec v1.10
unsafe fn delegate_interrupt_exception() {
    mideleg::set_sext();
    mideleg::set_stimer();
    mideleg::set_ssoft();
    // p 35, table 3.6
    medeleg::set_instruction_misaligned();
    medeleg::set_instruction_fault();
    // Do not medeleg::set_illegal_instruction();
    // We need to handle sfence.VMA and timer access in SBI.
    medeleg::set_breakpoint();
    medeleg::set_load_misaligned();
    medeleg::set_load_fault(); // PMP violation, shouldn't be hit
    medeleg::set_store_misaligned();
    medeleg::set_store_fault();
    medeleg::set_user_env_call();
    // Do not delegate env call from S-mode nor M-mode
    medeleg::set_instruction_page_fault();
    medeleg::set_load_page_fault();
    medeleg::set_store_page_fault();
    mie::set_msoft();
}

fn init_heap() {
    unsafe {
        SBI_HEAP
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, SBI_HEAP_SIZE)
    }
}
