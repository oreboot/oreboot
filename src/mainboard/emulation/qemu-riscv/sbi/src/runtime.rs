use core::{
    arch::asm,
    ops::{Generator, GeneratorState},
    pin::Pin,
};
use riscv::register::{
    mcause::{self, Exception, Interrupt, Trap},
    mstatus::{self, Mstatus, MPP},
    mtval,
    mtvec::{self, TrapMode},
};

pub fn init() {
    let mut addr = from_supervisor_save as usize;
    // Must be aligned to 4 bytes
    if addr & 0x2 != 0 {
        addr += 0x2;
    }
    unsafe { mtvec::write(addr, TrapMode::Direct) };
}

pub struct Runtime {
    context: SupervisorContext,
}

impl Runtime {
    pub fn new_sbi_supervisor(supervisor_mepc: usize, a0: usize, a1: usize) -> Self {
        let context: SupervisorContext = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
        let mut ans = Runtime { context };
        ans.prepare_supervisor(supervisor_mepc);
        ans.context.a0 = a0;
        ans.context.a1 = a1;
        ans
    }

    fn reset(&mut self) {
        unsafe { mstatus::set_mpp(MPP::Supervisor) };
        self.context.mstatus = mstatus::read();
        // NOTE: `machine_stack` will be overridden by the resume function
        self.context.machine_stack = 0x2333333366666666;
    }

    // When handling exceptions, use `context_mut` to get the context of the
    // current user at runtime, which can change the content of the context.
    pub fn context_mut(&mut self) -> &mut SupervisorContext {
        &mut self.context
    }

    pub fn prepare_supervisor(&mut self, new_mepc: usize) {
        self.reset();
        self.context.mepc = new_mepc;
    }
}

impl Generator for Runtime {
    type Yield = MachineTrap;
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> GeneratorState<Self::Yield, Self::Return> {
        unsafe { do_resume(&mut self.context as *mut _) };
        let mtval = mtval::read();
        // if mcause::read().cause() != Trap::Exception(Exception::SupervisorEnvCall){
        //     println!("[rustsbi] 0x{:x} 0x{:x} {:?}",mtval,mepc::read(),mcause::read().cause());
        // }
        let trap = match mcause::read().cause() {
            Trap::Exception(Exception::SupervisorEnvCall) => MachineTrap::SbiCall(),
            Trap::Exception(Exception::IllegalInstruction) => MachineTrap::IllegalInstruction(),
            Trap::Exception(Exception::InstructionFault) => MachineTrap::InstructionFault(mtval),
            Trap::Exception(Exception::LoadFault) => MachineTrap::LoadFault(mtval),
            Trap::Exception(Exception::StoreFault) => MachineTrap::StoreFault(mtval),
            Trap::Interrupt(Interrupt::MachineExternal) => MachineTrap::ExternalInterrupt(),
            Trap::Interrupt(Interrupt::MachineTimer) => MachineTrap::MachineTimer(),
            Trap::Interrupt(Interrupt::MachineSoft) => MachineTrap::MachineSoft(),
            Trap::Exception(Exception::InstructionPageFault) => {
                MachineTrap::InstructionPageFault(mtval)
            }
            Trap::Exception(Exception::LoadPageFault) => MachineTrap::LoadPageFault(mtval),
            Trap::Exception(Exception::StorePageFault) => MachineTrap::StorePageFault(mtval),
            e => panic!(
                "unhandled exception: {:?}! mtval: {:#x?}, ctx: {:#x?}",
                e, mtval, self.context
            ),
        };
        GeneratorState::Yielded(trap)
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum MachineTrap {
    SbiCall(),
    IllegalInstruction(),
    ExternalInterrupt(),
    MachineTimer(),
    MachineSoft(),
    InstructionFault(usize),
    LoadFault(usize),
    StoreFault(usize),
    InstructionPageFault(usize),
    LoadPageFault(usize),
    StorePageFault(usize),
}

#[derive(Debug)]
#[repr(C)]
pub struct SupervisorContext {
    pub ra: usize, // 0
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,            // 30
    pub mstatus: Mstatus,     // 31
    pub mepc: usize,          // 32
    pub machine_stack: usize, // 33
}

#[naked]
#[link_section = ".text"]
unsafe extern "C" fn do_resume(_supervisor_context: *mut SupervisorContext) {
    asm!("j     {from_machine_save}", from_machine_save = sym from_machine_save, options(noreturn))
}

#[naked]
#[link_section = ".text"]
unsafe extern "C" fn from_machine_save(_supervisor_context: *mut SupervisorContext) -> ! {
    asm!( // sp:机器栈顶
        "addi   sp, sp, -15*8", // sp:机器栈顶
        // 进入函数之前，已经保存了调用者寄存器，应当保存被调用者寄存器
        "sd     ra, 0*8(sp)
        sd      gp, 1*8(sp)
        sd      tp, 2*8(sp)
        sd      s0, 3*8(sp)
        sd      s1, 4*8(sp)
        sd      s2, 5*8(sp)
        sd      s3, 6*8(sp)
        sd      s4, 7*8(sp)
        sd      s5, 8*8(sp)
        sd      s6, 9*8(sp)
        sd      s7, 10*8(sp)
        sd      s8, 11*8(sp)
        sd      s9, 12*8(sp)
        sd      s10, 13*8(sp)
        sd      s11, 14*8(sp)",
        // a0:特权级上下文
        "j      {to_supervisor_restore}",
        to_supervisor_restore = sym to_supervisor_restore,
        options(noreturn)
    )
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn to_supervisor_restore(_supervisor_context: *mut SupervisorContext) -> ! {
    asm!(
        // a0:特权级上下文
        "sd     sp, 33*8(a0)", // 机器栈顶放进特权级上下文
        "csrw   mscratch, a0", // 新mscratch:特权级上下文
        // mscratch:特权级上下文
        "mv     sp, a0", // 新sp:特权级上下文
        "ld     t0, 31*8(sp)
        ld      t1, 32*8(sp)
        csrw    mstatus, t0
        csrw    mepc, t1",
        "ld     ra, 0*8(sp)
        ld      gp, 2*8(sp)
        ld      tp, 3*8(sp)
        ld      t0, 4*8(sp)
        ld      t1, 5*8(sp)
        ld      t2, 6*8(sp)
        ld      s0, 7*8(sp)
        ld      s1, 8*8(sp)
        ld      a0, 9*8(sp)
        ld      a1, 10*8(sp)
        ld      a2, 11*8(sp)
        ld      a3, 12*8(sp)
        ld      a4, 13*8(sp)
        ld      a5, 14*8(sp)
        ld      a6, 15*8(sp)
        ld      a7, 16*8(sp)
        ld      s2, 17*8(sp)
        ld      s3, 18*8(sp)
        ld      s4, 19*8(sp)
        ld      s5, 20*8(sp)
        ld      s6, 21*8(sp)
        ld      s7, 22*8(sp)
        ld      s8, 23*8(sp)
        ld      s9, 24*8(sp)
        ld     s10, 25*8(sp)
        ld     s11, 26*8(sp)
        ld      t3, 27*8(sp)
        ld      t4, 28*8(sp)
        ld      t5, 29*8(sp)
        ld      t6, 30*8(sp)",
        "ld     sp, 1*8(sp)", // 新sp:特权级栈
        // sp:特权级栈, mscratch:特权级上下文
        "mret",
        options(noreturn)
    )
}

// 中断开始

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn from_supervisor_save() -> ! {
    asm!( // sp:特权级栈,mscratch:特权级上下文
        ".p2align 2",
        "csrrw  sp, mscratch, sp", // 新mscratch:特权级栈, 新sp:特权级上下文
        "sd     ra, 0*8(sp)
        sd      gp, 2*8(sp)
        sd      tp, 3*8(sp)
        sd      t0, 4*8(sp)
        sd      t1, 5*8(sp)
        sd      t2, 6*8(sp)
        sd      s0, 7*8(sp)
        sd      s1, 8*8(sp)
        sd      a0, 9*8(sp)
        sd      a1, 10*8(sp)
        sd      a2, 11*8(sp)
        sd      a3, 12*8(sp)
        sd      a4, 13*8(sp)
        sd      a5, 14*8(sp)
        sd      a6, 15*8(sp)
        sd      a7, 16*8(sp)
        sd      s2, 17*8(sp)
        sd      s3, 18*8(sp)
        sd      s4, 19*8(sp)
        sd      s5, 20*8(sp)
        sd      s6, 21*8(sp)
        sd      s7, 22*8(sp)
        sd      s8, 23*8(sp)
        sd      s9, 24*8(sp)
        sd     s10, 25*8(sp)
        sd     s11, 26*8(sp)
        sd      t3, 27*8(sp)
        sd      t4, 28*8(sp)
        sd      t5, 29*8(sp)
        sd      t6, 30*8(sp)",
        "csrr   t0, mstatus
        sd      t0, 31*8(sp)",
        "csrr   t1, mepc
        sd      t1, 32*8(sp)",
        // mscratch:特权级栈,sp:特权级上下文
        "csrrw  t2, mscratch, sp", // 新mscratch:特权级上下文,t2:特权级栈
        "sd     t2, 1*8(sp)", // 保存特权级栈
        "j      {to_machine_restore}",
        to_machine_restore = sym to_machine_restore,
        options(noreturn)
    )
}

#[naked]
#[link_section = ".text"]
unsafe extern "C" fn to_machine_restore() -> ! {
    asm!(
        // mscratch:特权级上下文
        "csrr   sp, mscratch", // sp:特权级上下文
        "ld     sp, 33*8(sp)", // sp:机器栈
        "ld     ra, 0*8(sp)
        ld      gp, 1*8(sp)
        ld      tp, 2*8(sp)
        ld      s0, 3*8(sp)
        ld      s1, 4*8(sp)
        ld      s2, 5*8(sp)
        ld      s3, 6*8(sp)
        ld      s4, 7*8(sp)
        ld      s5, 8*8(sp)
        ld      s6, 9*8(sp)
        ld      s7, 10*8(sp)
        ld      s8, 11*8(sp)
        ld      s9, 12*8(sp)
        ld      s10, 13*8(sp)
        ld      s11, 14*8(sp)",
        "addi   sp, sp, 15*8", // sp:机器栈顶
        "jr     ra",           // 其实就是ret
        options(noreturn)
    )
}
