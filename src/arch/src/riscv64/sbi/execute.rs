use super::feature;
use super::runtime::{MachineTrap, Runtime, SupervisorContext};
use core::{
    arch::asm,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};
use log::{print, println};
use riscv::register::{
    mip,
    mstatus::{self, MPP},
    scause::{Exception, Trap},
};
use rustsbi::spec::binary::SbiRet;
use sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR;

// This value is somewhat arbitrary. Taken from xv6:
// https://github.com/michaelengel/xv6-d1/blob/b1ffbd8930a10dfd616e0aa5543b40dd91a72b28/kernel/kernelvec.S#L104
const TIME_INC: u64 = 10_000_000;

const ECALL_OREBOOT: usize = 0x0A023B00;
const EBREAK: u16 = 0x9002;

const DEBUG: bool = true;
const DEBUG_ECALL: bool = false;
const DEBUG_MTIMER: bool = true;
const DEBUG_EBREAK: bool = true;
const DEBUG_EMULATE: bool = false;
const DEBUG_ILLEGAL: bool = true;
const DEBUG_MISALIGNED: bool = true;

fn ore_sbi(method: usize, args: [usize; 6]) -> SbiRet {
    match method {
        0x023A_DC52 => {
            let mut val = 0;
            let mut err = 0;
            let csr = args[0];
            if DEBUG {
                println!("[SBI] read CSR {:x}", csr);
            }
            match csr {
                0x7c0 => unsafe {
                    asm!("csrr {0}, 0x7c0", out(reg) val);
                },
                0x7c1 => unsafe {
                    asm!("csrr {0}, 0x7c1", out(reg) val);
                },
                0x7c2 => unsafe {
                    asm!("csrr {0}, 0x7c2", out(reg) val);
                },
                0x7c5 => unsafe {
                    asm!("csrr {0}, 0x7c5", out(reg) val);
                },
                _ => {
                    err = 1;
                }
            }
            if DEBUG {
                println!("[SBI] CSR {:x} is {:08x}, err {:x}", csr, val, err);
            }
            SbiRet {
                value: val,
                error: err,
            }
        }
        _ => SbiRet { value: 0, error: 1 },
    }
}

pub fn read32(reg: usize) -> u32 {
    unsafe { core::ptr::read_volatile(reg as *mut u32) }
}

pub fn write32(reg: usize, val: u32) {
    unsafe { core::ptr::write_volatile(reg as *mut u32, val) }
}

// TODO: Check newer specs on out this should work
fn putchar(_method: usize, args: [usize; 6]) -> SbiRet {
    let char = args[0] as u8 as char;
    print!("{char}");
    SbiRet { value: 0, error: 0 }
}

fn print_ecall_context(ctx: &mut SupervisorContext) {
    if DEBUG && DEBUG_ECALL {
        println!(
            "[SBI] ecall a6: {:x}, a7: {:x}, a0-a5: {:x} {:x} {:x} {:x} {:x} {:x}",
            ctx.a6, ctx.a7, ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5,
        );
    }
}

// FIXME: DO NOT HARDCODE; pass as parameter
const CLINT_BASE_JH7110: usize = 0x0200_0000;
const CLINT_BASE_D1: usize = 0x0400_0000;

// Machine Software Interrupt Pending registers are 32 bit (4 bytes)
const HART0_MSIP_OFFSET: usize = 0x0000;
// Machine Timer Compoare registers are 64 bit (8 bytes)
const HART0_MTIMECMP_OFFSET: usize = 0x4000;
// Machine Time is a 64-bit register
const MTIME_OFFSET: usize = 0xbff8;

pub fn execute_supervisor(
    supervisor_mepc: usize,
    hartid: usize,
    dtb_addr: usize,
) -> (usize, usize) {
    println!(
        "[SBI] Enter supervisor on hart {hartid} at {:x} with DTB from {:x}",
        supervisor_mepc, dtb_addr
    );
    let mut rt = Runtime::new_sbi_supervisor(supervisor_mepc, hartid, dtb_addr);
    // TODO: make a param
    let clint_base = CLINT_BASE_D1;
    let clint_base = CLINT_BASE_JH7110;
    let mtime: usize = clint_base + MTIME_OFFSET;
    let mtimecmp: usize = clint_base + HART0_MTIMECMP_OFFSET + 8 * hartid;
    let hart_msip: usize = clint_base + HART0_MSIP_OFFSET + 4 * hartid;
    loop {
        // NOTE: `resume()` drops into S-mode by calling `mret` (asm) eventually
        match Pin::new(&mut rt).resume(()) {
            CoroutineState::Yielded(MachineTrap::SbiCall()) => {
                let ctx = rt.context_mut();
                // specific for 1.9.1; see document for details
                feature::preprocess_supervisor_external(ctx);
                let param = [ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5];
                let ans = match ctx.a7 {
                    ECALL_OREBOOT => ore_sbi(ctx.a6, param),
                    LEGACY_CONSOLE_PUTCHAR => putchar(ctx.a6, param),
                    _ => {
                        print_ecall_context(ctx);
                        rustsbi::ecall(ctx.a7, ctx.a6, param)
                    }
                };
                if ans.error & (0xFFFFFFFF << 32) == 0x114514 << 32 {
                    // magic value to exit execution loop
                    break (ans.error & 0xFFFFFFFF, ans.value);
                }
                ctx.a0 = ans.error;
                ctx.a1 = ans.value;
                ctx.mepc = ctx.mepc.wrapping_add(4);
            }
            CoroutineState::Yielded(MachineTrap::IllegalInstruction()) => {
                let ctx = rt.context_mut();
                let ins = unsafe { get_vaddr_u32(ctx.mepc) } as usize;
                // NOTE: Not all instructions are 32 bit
                if ins as u16 == EBREAK {
                    // dump context on breakpoints for debugging
                    // TODO: how would we allow for "real" debugging?
                    if DEBUG_EBREAK {
                        println!("[SBI] Take an EBREAK! {ctx:#04X?}");
                    }
                    // skip instruction; this will likely cause the OS to crash
                    // use DEBUG to get actual information
                    ctx.mepc = ctx.mepc.wrapping_add(2);
                } else if !emulate_instruction(ctx, ins, mtime) {
                    if DEBUG_ILLEGAL {
                        println!("[SBI] Illegal instruction {ins:08x} not emulated {ctx:#04X?}");
                    }
                    unsafe {
                        if feature::should_transfer_trap(ctx) {
                            feature::do_transfer_trap(
                                ctx,
                                Trap::Exception(Exception::IllegalInstruction),
                            )
                        } else {
                            println!("[SBI] Na na na! {ctx:#04X?}");
                            fail_illegal_instruction(ctx, ins)
                        }
                    }
                }
            }
            CoroutineState::Yielded(MachineTrap::MachineTimer()) => {
                // TODO: Check if this actually works
                if DEBUG && DEBUG_MTIMER {
                    println!("[SBI] M-timer interrupt");
                }
                // Clear the mtimer interrupt by increasing the respective
                // hart's mtimecmp register.
                // Note that the MTIP bit in the MIP register is read-only.
                // mtimecmp is 64-bit, so stitch together high and low parts.
                let tl = read32(mtimecmp) as u64;
                let th = read32(mtimecmp + 4) as u64;
                let tv = th << 32 | tl;
                println!("[SBI] M-time cmp{hartid}: {tv}");
                // Increase whole the value to include overflow and write back.
                let tn = tv + TIME_INC;
                write32(mtimecmp, tn as u32);
                write32(mtimecmp + 4, (tn >> 32) as u32);
                // Yeet software interrupt pending to signal interrupt to S-mode
                // for this hart.
                // write32(hart_msip, 1);
                // TODO: There is also the Supervisor Timer Interrupt Pending
                // bit in the MIP register... why anyway?
                if false {
                    unsafe { mip::set_stimer() }
                }
            }
            CoroutineState::Yielded(MachineTrap::LoadMisaligned(_addr)) => {
                if DEBUG && DEBUG_MISALIGNED {
                    println!("[SBI] Load misaligned");
                }
            }
            CoroutineState::Yielded(MachineTrap::StoreMisaligned(_addr)) => {
                if DEBUG && DEBUG_MISALIGNED {
                    println!("[SBI] Store misaligned");
                }
            }
            // NOTE: These are all delegated.
            CoroutineState::Yielded(MachineTrap::LoadFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::StoreFault(addr)) => {
                if false {
                    println!("[SBI]   Attemped to store to ${addr:16x}");
                }
            }
            CoroutineState::Yielded(MachineTrap::ExternalInterrupt()) => {}
            CoroutineState::Yielded(MachineTrap::MachineSoft()) => {}
            CoroutineState::Yielded(MachineTrap::InstructionFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::LoadPageFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::StorePageFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::InstructionPageFault(_addr)) => {}
            CoroutineState::Complete(()) => unreachable!(),
        }
    }
}

#[inline]
unsafe fn get_vaddr_u32(vaddr: usize) -> u32 {
    get_vaddr_u16(vaddr) as u32 | ((get_vaddr_u16(vaddr.wrapping_add(2)) as u32) << 16)
}

#[inline]
#[allow(asm_sub_register)]
unsafe fn get_vaddr_u16(vaddr: usize) -> u16 {
    let mut ans: u16;
    asm!("
        li      {2}, (1 << 17)
        csrrs   {2}, mstatus, {2}
        lhu     {0}, 0({1})
        csrw    mstatus, {2}
    ", out(reg) ans, in(reg) vaddr, out(reg) _);
    ans
}

fn emulate_instruction(ctx: &mut SupervisorContext, ins: usize, mtimer: usize) -> bool {
    if DEBUG && DEBUG_EMULATE {
        println!("[SBI] Emulating instruction {ins:08x}, {ctx:#04X?}");
    }
    if feature::emulate_rdtime(ctx, ins, mtimer) {
        return true;
    }
    if feature::emulate_sfence_vma(ctx, ins) {
        return true;
    }
    false
}

// Real illegal instruction happening in M-mode
fn fail_illegal_instruction(ctx: &mut SupervisorContext, ins: usize) -> ! {
    let mepc = ctx.mepc;
    panic!("[SBI] invalid instruction from M-mode, mepc: {mepc:016x?}, instruction: {ins:016x?}, context: {ctx:016x?}");
}
