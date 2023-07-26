use core::{
    arch::asm,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};
use log::println;
use riscv::register::{
    mcause::{self, Exception, Interrupt, Trap},
    medeleg, mepc, mideleg, mie, mip,
    mstatus::{self, Mstatus, MPP},
    mtval,
    mtvec::{self, TrapMode},
};

// mideleg: 0x222
// medeleg: 0xb151
// OpenSBI medeleg: 0xb109
fn delegate_interrupt_exception() {
    unsafe {
        mip::clear_stimer();
        mip::clear_sext();
        mip::clear_ssoft();
        mip::clear_utimer();
        mip::clear_uext();
        mip::clear_usoft();

        mideleg::set_sext();
        mideleg::set_stimer();
        mideleg::set_ssoft();
        // p 35, table 3.6
        medeleg::set_instruction_misaligned();
        medeleg::set_instruction_fault();
        // Do not medeleg::set_illegal_instruction();
        // We need to handle sfence.VMA and timer access in SBI, i.e., rdtime.
        medeleg::set_breakpoint();
        // medeleg::set_load_misaligned();
        medeleg::clear_load_misaligned();
        // load fault means PMP violation, shouldn't be hit
        medeleg::set_load_fault();
        // medeleg::set_store_misaligned();
        medeleg::set_store_fault();
        medeleg::set_user_env_call();
        // Do not delegate env call from S-mode nor M-mode; we handle it :)
        medeleg::set_instruction_page_fault();
        medeleg::set_load_page_fault();
        medeleg::set_store_page_fault();
        if true {
            // mie::set_mext();
            // mie::set_mtimer();
            mie::set_msoft();
            mie::set_sext();
            mie::set_stimer();
            mie::set_ssoft();
        }
    }
}

pub fn init() {
    let mut addr = from_supervisor_save as usize;
    // Must be aligned to 4 bytes
    if addr & 0x2 != 0 {
        addr += 0x2;
    }
    println!("[SBI] set mtvec: {addr:x}");
    unsafe { mtvec::write(addr, TrapMode::Direct) };
    println!("[SBI] delegate interrupts and exceptions");
    delegate_interrupt_exception();
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

const DEBUG: bool = false;
const DEBUG_MTIMER: bool = false;

// best debugging function on the planet
fn print_exception_interrupt() {
    if DEBUG {
        let cause = mcause::read().cause();
        let epc = mepc::read();
        match epc {
            0xffffffff80406e1e => {}
            0xffffffff80406e2e => {}
            0xffffffff80357fc4 => {}
            0xffffffff803583d8 => {}
            0xffffffff803583d0 => {}
            0xffffffff803583c8 => {}
            0xffffffff803583c0 => {}
            0xffffffff8026abf0 => {}
            0xffffffff80358130 => {}
            _ => {
                println!("[SBI] DEBUG: {cause:?} @ 0x{epc:016x}");
            }
        }
    }
}

/*

[    0.000000] rcu: Hierarchical RCU implementation.
[    0.000000] rcu:     RCU restricting CPUs from NR_CPUS=64 to nr_cpu_ids=4.
[    0.000000] rcu:     RCU debug extended QS entry/exit.
[    0.000000] rcu: RCU calculated value of scheduler-enlistment delay is 25 jiffies.
[    0.000000] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=4
[    0.000000] NR_IRQS: 64, nr_irqs: 64, preallocated irqs: 0
[    0.000000] CPU with hartid=0 is not available
[    0.000000] riscv-intc: unable to find hart id for /cpus/cpu@0/interrupt-controller
[    0.000000] riscv-intc: 64 local interrupts mapped
[    0.000000] plic: interrupt-controller@c000000: mapped 136 interrupts with 4 handlers for 9 contexts.
[    0.000000] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[    0.000000] riscv-timer: riscv_timer_init_dt: Registering clocksource cpuid [0] hartid [1]
[    0.000000] clocksource: riscv_clocksource: mask: 0xffffffffffffffff max_cycles: 0x1d854df40, max_idle_ns: 881590404240 ns
[    0.000000] Oops - illegal instruction [#1]
[    0.000000] Modules linked in:
[    0.000000] CPU: 0 PID: 0 Comm: swapper/0 Not tainted 6.3.0-rc3-cyrevolt-g853b23029090 #20
[    0.000000] Hardware name: StarFive VisionFive 2 v1.3B (DT)
[    0.000000] epc : riscv_sched_clock+0x6/0x10
[    0.000000]  ra : sched_clock_register+0xa4/0x1ba
[    0.000000] epc : ffffffff80406e2e ra : ffffffff8060bd5e sp : ffffffff81203e00
[    0.000000]  gp : ffffffff812dec80 tp : ffffffff8120dc80 t0 : 756f736b636f6c63
[    0.000000]  t1 : 00000000003d0900 t2 : 72756f736b636f6c s0 : ffffffff81203e10
[    0.000000]  s1 : 00000000003d0900 a0 : ffffffff81203e20 a1 : ffffffff81288df0
[    0.000000]  a2 : 0000000000000028 a3 : ffffffff81288df0 a4 : 0000000000000000
[    0.000000]  a5 : 0000000000000000 a6 : 000000003e800000 a7 : 0000000000000000
[    0.000000]  s2 : ffffffff81288dc0 s3 : 0000000000000040 s4 : 0000000200000100
[    0.000000]  s5 : 000001ffffffffcc s6 : ffffffff80406e28 s7 : ffffffffffffffff
[    0.000000]  s8 : 0000000000000000 s9 : 0000000000000000 s10: 0000000000000000
[    0.000000]  s11: 0000000000000000 t3 : ffffffff812f0e0f t4 : ffffffff812f0e0f
[    0.000000]  t5 : ffffffff812f0e10 t6 : ffffffff81203e48
[    0.000000] status: 0000000200000100 badaddr: 00000000c0102573 cause: 0000000000000002
[    0.000000] [<ffffffff80406e2e>] riscv_sched_clock+0x6/0x10
[    0.000000] Code: e422 0800 2573 c010 6422 0141 8082 1141 e422 0800 (2573) c010
[    0.000000] ---[ end trace 0000000000000000 ]---
[    0.000000] Kernel panic - not syncing: Attempted to kill the idle task!
[    0.000000] ---[ end Kernel panic - not syncing: Attempted to kill the idle task! ]---

*/

impl Coroutine for Runtime {
    type Yield = MachineTrap;
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        unsafe { do_resume(&mut self.context as *mut _) };
        let cause = mcause::read().cause();
        if cause != Trap::Interrupt(Interrupt::MachineTimer) {
            print_exception_interrupt();
        }
        let mtval = mtval::read();
        let trap = match cause {
            Trap::Exception(Exception::SupervisorEnvCall) => MachineTrap::SbiCall(),
            Trap::Exception(Exception::IllegalInstruction) => MachineTrap::IllegalInstruction(),
            Trap::Exception(Exception::InstructionFault) => MachineTrap::InstructionFault(mtval),
            Trap::Exception(Exception::Breakpoint) => MachineTrap::IllegalInstruction(),
            Trap::Exception(Exception::LoadFault) => MachineTrap::LoadFault(mtval),
            Trap::Exception(Exception::StoreFault) => MachineTrap::StoreFault(mtval),
            Trap::Interrupt(Interrupt::MachineExternal) => MachineTrap::ExternalInterrupt(),
            Trap::Interrupt(Interrupt::MachineTimer) => {
                if DEBUG_MTIMER {
                    print_exception_interrupt();
                }
                MachineTrap::MachineTimer()
            }
            Trap::Interrupt(Interrupt::MachineSoft) => MachineTrap::MachineSoft(),
            Trap::Exception(Exception::InstructionPageFault) => {
                MachineTrap::InstructionPageFault(mtval)
            }
            Trap::Exception(Exception::LoadPageFault) => MachineTrap::LoadPageFault(mtval),
            Trap::Exception(Exception::StorePageFault) => MachineTrap::StorePageFault(mtval),
            Trap::Exception(Exception::StoreMisaligned) => MachineTrap::StoreFault(mtval),
            Trap::Exception(Exception::LoadMisaligned) => MachineTrap::LoadFault(mtval),
            e => panic!(
                "[SBI] unhandled: {e:?}! mtval: {mtval:08x}, ctx: {:#x?}",
                self.context
            ),
        };
        CoroutineState::Yielded(trap)
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
    asm!(
        "j      {from_machine_save}",
        from_machine_save = sym from_machine_save,
        options(noreturn),
    )
}

#[naked]
#[link_section = ".text"]
unsafe extern "C" fn from_machine_save(_supervisor_context: *mut SupervisorContext) -> ! {
    asm!(
        // sp: top of the stack
        "addi   sp, sp, -15*8",
        // Before entering the function, the caller's register has been saved,
        // and the callee's register should be saved
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
        // a0: privileged context
        "j      {to_supervisor_restore}",
        to_supervisor_restore = sym to_supervisor_restore,
        options(noreturn)
    )
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn to_supervisor_restore(_supervisor_context: *mut SupervisorContext) -> ! {
    asm!(
        // a0: privileged context
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
