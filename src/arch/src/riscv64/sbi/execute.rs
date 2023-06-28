use super::feature;
use super::runtime::{MachineTrap, Runtime, SupervisorContext};
use core::{
    arch::asm,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};
use log::{print, println};
use riscv::register::scause::{Exception, Trap};
use riscv::register::{mie, mip};
use rustsbi::spec::binary::SbiRet;
use sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR;

const ECALL_OREBOOT: usize = 0x0A023B00;
const EBREAK: u16 = 0x9002;

const DEBUG: bool = true;
const DEBUG_MTIMER: bool = false;
const DEBUG_EBREAK: bool = true;
const DEBUG_EMULATE: bool = false;
const DEBUG_ILLEGAL: bool = true;

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

fn putchar(method: usize, args: [usize; 6]) -> SbiRet {
    let char = args[0] as u8 as char;
    print!("{char}");
    SbiRet { value: 0, error: 0 }
}

fn print_ecall_context(ctx: &mut SupervisorContext) {
    if DEBUG {
        println!(
            "[SBI] ecall a6: {:x}, a7: {:x}, a0-a5: {:x} {:x} {:x} {:x} {:x} {:x}",
            ctx.a6, ctx.a7, ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5,
        );
    }
}

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
                } else if !emulate_instruction(ctx, ins) {
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
                unsafe {
                    mip::set_stimer();
                }
            }
            // NOTE: These are all delegated.
            CoroutineState::Yielded(MachineTrap::ExternalInterrupt()) => {}
            CoroutineState::Yielded(MachineTrap::MachineSoft()) => {}
            CoroutineState::Yielded(MachineTrap::InstructionFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::LoadFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::LoadPageFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::StorePageFault(_addr)) => {}
            CoroutineState::Yielded(MachineTrap::StoreFault(_addr)) => {}
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

fn emulate_instruction(ctx: &mut SupervisorContext, ins: usize) -> bool {
    if DEBUG && DEBUG_EMULATE {
        println!("[SBI] Emulating instruction {ins:08x}, {ctx:#04X?}");
    }
    if feature::emulate_rdtime(ctx, ins) {
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
