use crate::runtime::SupervisorContext;
use riscv::register::{
    mstatus::{self, MPP, SPP},
    mtval, scause, sepc, stval, stvec,
};
use rustsbi::println;

pub unsafe fn should_transfer_trap(ctx: &mut SupervisorContext) -> bool {
    ctx.mstatus.mpp() != MPP::Machine
}

pub unsafe fn do_transfer_trap(ctx: &mut SupervisorContext, cause: scause::Trap) {
    // 设置S层异常原因为：非法指令
    // The reason for setting S-layer exception is: illegal instruction
    scause::set(cause);
    // 填写异常指令的指令内容
    // Fill in the instruction content of the abnormal instruction
    let ins = mtval::read();
    println!("[rustsbi] It's a trap! SCAUSE: {:x?}\r", cause);
    println!("[rustsbi] INSTRUCTION: 0x{:x?}\r", ins);
    // println!("[rustsbi] STATE: {:#04X?}\r", ctx);
    stval::write(ins);
    // 填写S层需要返回到的地址，这里的mepc会被随后的代码覆盖掉
    // Fill in the address that the S layer needs to return to, the mepc here
    // will be overwritten by the subsequent code.
    sepc::write(ctx.mepc);
    // 设置中断位
    // Set the interrupt bit
    mstatus::set_mpp(MPP::Supervisor);
    mstatus::set_spp(SPP::Supervisor);
    if mstatus::read().sie() {
        mstatus::set_spie()
    }
    mstatus::clear_sie();
    // mstatus::set_sum();
    ctx.mstatus = mstatus::read();
    // 设置返回地址，返回到S层
    // Set the return address and return to the S layer
    // 注意，无论是Direct还是Vectored模式，所有异常的向量偏移都是0，不需要处理中断向量，跳转到入口地址即可
    // Note that regardless of whether it is in Direct or Vectored mode, the
    // vector offset of all exceptions is 0, and there is no need to process
    // the interrupt vector, just jump to the entry address.
    ctx.mepc = stvec::read().address();
}
