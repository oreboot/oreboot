use core::{
    arch::asm,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};
use riscv::register::{mcause, mepc, mie, mip, mtval};
use rustsbi::spec::binary::SbiRet;
use rustsbi::RustSBI;
use sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR;

use super::feature;
use super::runtime::{Runtime, SupervisorContext, Trap, ILLEGAL_INSTRUCTION, INSTRUCTION_FAULT};
use log::{print, println};

const EBREAK: u16 = 0x9002;

const DEBUG: bool = false;
const DEBUG_ECALL: bool = false;
const DEBUG_MTIMER: bool = false;
const DEBUG_EBREAK: bool = true;
const DEBUG_EMULATE: bool = false;
const DEBUG_ILLEGAL: bool = true;

const ECALL_OREBOOT: usize = 0x0A02_3B00;
const ORE_SBI_DUMP_CSR: usize = 0x023A_DC52;

// These are our custom SBI calls for debugging.
fn ore_sbi(ctx: &SupervisorContext) -> SbiRet {
    let method = ctx.a6;
    match method {
        ORE_SBI_DUMP_CSR => {
            let mut value = 0;
            let mut error = 0;
            let csr = ctx.a0;
            if DEBUG {
                println!("[SBI] read CSR {csr:x}");
            }
            match csr {
                0x7c0 => unsafe {
                    asm!("csrr {0}, 0x7c0", out(reg) value);
                },
                0x7c1 => unsafe {
                    asm!("csrr {0}, 0x7c1", out(reg) value);
                },
                0x7c2 => unsafe {
                    asm!("csrr {0}, 0x7c2", out(reg) value);
                },
                0x7c5 => unsafe {
                    asm!("csrr {0}, 0x7c5", out(reg) value);
                },
                _ => {
                    error = 1;
                }
            }
            if DEBUG {
                println!("[SBI] CSR {csr:02x} is {value:08x}, error {error:x}");
            }
            SbiRet { value, error }
        }
        _ => SbiRet { value: 0, error: 1 },
    }
}

fn print_ecall_context(ctx: &mut SupervisorContext) {
    if DEBUG && DEBUG_ECALL {
        println!(
            "[SBI] ecall a6: {:x}, a7: {:x}, a0-a5: {:x} {:x} {:x} {:x} {:x} {:x}",
            ctx.a6, ctx.a7, ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5,
        );
    }
}

fn dump_mstate() {
    println!(
        "[SBI] 0x{:x} 0x{:x} {:?}",
        mtval::read(),
        mepc::read(),
        mcause::read().cause()
    );
}

pub fn execute_supervisor<S: RustSBI>(
    sbi: S,
    supervisor_mepc: usize,
    hartid: usize,
    dtb_addr: usize,
) -> (usize, usize) {
    println!(
        "[SBI] Prepare supervisor on hart {hartid} at {:x} with DTB from {:x}",
        supervisor_mepc, dtb_addr
    );
    let mut rt = Runtime::new(supervisor_mepc, hartid, dtb_addr);
    println!("[SBI] Enter loop...");
    loop {
        // NOTE: `resume()` drops into S-mode by calling `mret` (asm) eventually
        match Pin::new(&mut rt).resume(()) {
            CoroutineState::Yielded(Trap::SbiCall) => {
                let ctx = rt.context_mut();
                // specific for 1.9.1; see document for details
                feature::preprocess_supervisor_external(ctx);
                let param = [ctx.a0, ctx.a1, ctx.a2, ctx.a3, ctx.a4, ctx.a5];
                let ans = match ctx.a7 {
                    ECALL_OREBOOT => ore_sbi(ctx),
                    LEGACY_CONSOLE_PUTCHAR => {
                        let char = ctx.a0 as u8 as char;
                        print!("{char}");
                        SbiRet { value: 0, error: 0 }
                    }
                    _ => {
                        print_ecall_context(ctx);
                        sbi.handle_ecall(ctx.a7, ctx.a6, param)
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
            CoroutineState::Yielded(Trap::IllegalInstruction) => {
                let ctx = rt.context_mut();
                let ins = unsafe { get_vaddr_u32(ctx.mepc) } as usize;
                // NOTE: Not all instructions are 32 bit
                if ins as u16 == EBREAK {
                    // dump context on breakpoints for debugging
                    // TODO: how would we allow for "real" debugging?
                    if DEBUG_EBREAK {
                        println!("[SBI] Take an EBREAK!");
                        dump_mstate();
                        panic!("{ctx:#04X?}");
                    }
                    // skip instruction; this will likely cause the OS to crash
                    // use DEBUG to get actual information
                    ctx.mepc = ctx.mepc.wrapping_add(2);
                } else if !emulate_instruction(ctx, ins) {
                    if DEBUG_ILLEGAL {
                        println!("[SBI] Illegal instruction {ins:08x} not emulated");
                        dump_mstate();
                        println!("{ctx:#04X?}");
                    }
                    unsafe {
                        if feature::should_transfer_trap(ctx) {
                            feature::do_transfer_trap(ctx, ILLEGAL_INSTRUCTION)
                        } else {
                            fail_illegal_instruction(ctx, ins)
                        }
                    }
                }
            }
            CoroutineState::Yielded(Trap::InstructionFault) => {
                let ctx = rt.context_mut();
                unsafe {
                    if feature::should_transfer_trap(ctx) {
                        feature::do_transfer_trap(ctx, INSTRUCTION_FAULT)
                    } else {
                        println!("[SBI] Instruction fault");
                        dump_mstate();
                        panic!("{ctx:#04X?}");
                    }
                }
            }
            CoroutineState::Yielded(Trap::MachineExternal) => {
                // TODO
            }
            CoroutineState::Yielded(Trap::MachineSoft) => {
                // TODO
            }
            CoroutineState::Yielded(Trap::MachineTimer) => {
                if DEBUG && DEBUG_MTIMER {
                    println!("[SBI] M-timer interrupt");
                }
                // NOTE: The ECALL handler enables the interrupt.
                unsafe { mie::clear_mtimer() }
                // Yeet timer interrupt pending to signal interrupt to S-mode
                // for this hart.
                unsafe { mip::set_stimer() }
            }
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
    println!("[SBI] Invalid instruction from M-mode");
    println!("  mepc:        {mepc:016x?}");
    println!("  instruction: {ins:04x?}");
    panic!("{ctx:04x?}");
}
