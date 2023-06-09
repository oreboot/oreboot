use super::feature;
use super::runtime::{MachineTrap, Runtime, SupervisorContext};
use core::{
    arch::asm,
    ops::{Generator, GeneratorState},
    pin::Pin,
};
use log::{print, println};
use riscv::register::mip;
use riscv::register::scause::{Exception, Trap};
use rustsbi::spec::binary::SbiRet;
use sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR;

const ECALL_OREBOOT: usize = 0x0A023B00;
const EBREAK: u16 = 0x9002;
const DEBUG: bool = false;

fn ore_sbi(method: usize, args: [usize; 6]) -> SbiRet {
    let dbg = true;
    match method {
        0x023A_DC52 => {
            let mut val = 0;
            let mut err = 0;
            let csr = args[0];
            if dbg {
                println!("[rustsbi] read CSR {:x}", csr);
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
            if dbg {
                println!("[rustsbi] CSR {:x} is {:08x}, err {:x}", csr, val, err);
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

pub fn execute_supervisor(supervisor_mepc: usize, a0: usize, a1: usize) -> (usize, usize) {
    let mut rt = Runtime::new_sbi_supervisor(supervisor_mepc, a0, a1);
    loop {
        // NOTE: `resume()` drops into S-mode by calling `mret` (asm) eventually
        match Pin::new(&mut rt).resume(()) {
            GeneratorState::Yielded(MachineTrap::SbiCall()) => {
                let ctx = rt.context_mut();
                // specific for 1.9.1; see document for details
                feature::preprocess_supervisor_external(ctx);
                let param = [ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5];
                let ans = match ctx.a7 {
                    ECALL_OREBOOT => ore_sbi(ctx.a6, param),
                    LEGACY_CONSOLE_PUTCHAR => putchar(ctx.a6, param),
                    _ => {
                        if ctx.a7 != LEGACY_CONSOLE_PUTCHAR && DEBUG {
                            println!("[rustsbi] ecall {:x}", ctx.a6);
                        }
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
            GeneratorState::Yielded(MachineTrap::IllegalInstruction()) => {
                let ctx = rt.context_mut();
                let ins = unsafe { get_vaddr_u32(ctx.mepc) } as usize;
                // NOTE: Not all instructions are 32 bit
                if ins as u16 == EBREAK {
                    // dump context on breakpoints for debugging
                    // TODO: how would we allow for "real" debugging?
                    if DEBUG {
                        println!("[rustsbi] Take an EBREAK!\r {:#04X?}", ctx);
                    }
                    // skip instruction; this will likely cause the OS to crash
                    // use DEBUG to get actual information
                    ctx.mepc = ctx.mepc.wrapping_add(2);
                } else if !emulate_illegal_instruction(ctx, ins) {
                    unsafe {
                        if feature::should_transfer_trap(ctx) {
                            feature::do_transfer_trap(
                                ctx,
                                Trap::Exception(Exception::IllegalInstruction),
                            )
                        } else {
                            println!("[rustsbi] Na na na! {:#04X?}", ctx);
                            fail_illegal_instruction(ctx, ins)
                        }
                    }
                }
            }
            // NOTE: These are all delegated.
            GeneratorState::Yielded(MachineTrap::ExternalInterrupt()) => {}
            GeneratorState::Yielded(MachineTrap::MachineTimer()) => {
                // TODO: Check if this actually works
                if DEBUG {
                    println!("M timer int");
                }
                unsafe {
                    mip::set_stimer();
                }
            }
            GeneratorState::Yielded(MachineTrap::MachineSoft()) => {}
            GeneratorState::Yielded(MachineTrap::InstructionFault(_addr)) => {}
            GeneratorState::Yielded(MachineTrap::LoadFault(_addr)) => {}
            GeneratorState::Yielded(MachineTrap::LoadPageFault(_addr)) => {}
            GeneratorState::Yielded(MachineTrap::StorePageFault(_addr)) => {}
            GeneratorState::Yielded(MachineTrap::StoreFault(_addr)) => {}
            GeneratorState::Yielded(MachineTrap::InstructionPageFault(_addr)) => {}
            GeneratorState::Complete(()) => unreachable!(),
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

fn emulate_illegal_instruction(ctx: &mut SupervisorContext, ins: usize) -> bool {
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
    panic!("invalid instruction from machine level, mepc: {:016x?}, instruction: {:016x?}, context: {:016x?}", ctx.mepc, ins, ctx);
}
