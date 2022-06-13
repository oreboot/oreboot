use crate::feature;
use crate::runtime::{MachineTrap, Runtime, SupervisorContext};
use core::{
    arch::asm,
    ops::{Generator, GeneratorState},
    pin::Pin,
};
use riscv::register::scause::{Exception, Trap};
use rustsbi::println;

pub fn execute_supervisor(supervisor_mepc: usize, a0: usize, a1: usize) -> (usize, usize) {
    let mut rt = Runtime::new_sbi_supervisor(supervisor_mepc, a0, a1);
    loop {
        match Pin::new(&mut rt).resume(()) {
            GeneratorState::Yielded(MachineTrap::SbiCall()) => {
                let ctx = rt.context_mut();
                // specific for 1.9.1; see document for details
                feature::preprocess_supervisor_external(ctx);
                let param = [ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5];
                let ans = rustsbi::ecall(ctx.a7, ctx.a6, param);
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
                if !emulate_illegal_instruction(ctx, ins) {
                    unsafe {
                        if feature::should_transfer_trap(ctx) {
                            feature::do_transfer_trap(
                                ctx,
                                Trap::Exception(Exception::IllegalInstruction),
                            )
                        } else {
                            println!("[rustsbi] Na na na! {:#04X?}\r", ctx);
                            fail_illegal_instruction(ctx, ins)
                        }
                    }
                }
            }
            // NOTE: These are all delegated.
            GeneratorState::Yielded(MachineTrap::ExternalInterrupt()) => {}
            GeneratorState::Yielded(MachineTrap::MachineTimer()) => {}
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
