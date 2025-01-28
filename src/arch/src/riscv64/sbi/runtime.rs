use core::{
    arch::naked_asm,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};
use log::println;
use riscv::interrupt::{Exception, Interrupt, Trap};
use riscv::register::{
    mcause, medeleg, mepc, mideleg, mie,
    mstatus::{self, Mstatus, MPP},
    mtval,
    mtvec::{self, TrapMode},
};

fn delegate_interrupt_exception() {
    unsafe {
        mideleg::set_sext();
        mideleg::set_stimer();
        mideleg::set_ssoft();
        // p 35, table 3.6
        medeleg::set_instruction_misaligned();
        medeleg::set_instruction_fault();
        // Do not medeleg::set_illegal_instruction();
        // We need to handle sfence.VMA and timer access in SBI, i.e., rdtime.
        // medeleg::set_breakpoint();
        medeleg::set_load_misaligned();
        medeleg::set_load_fault(); // PMP violation, shouldn't be hit
        medeleg::set_store_misaligned();
        medeleg::set_store_fault();
        medeleg::set_user_env_call();
        // Do not delegate env call from S-mode nor M-mode; we handle it :)
        medeleg::set_instruction_page_fault();
        medeleg::set_load_page_fault();
        medeleg::set_store_page_fault();
        mie::set_mext();
        mie::set_mtimer();
        mie::set_msoft();
        mie::set_sext();
        mie::set_stimer();
        mie::set_ssoft();
    }
}

// Set up the trap mode and entry point (vector) for the M-mode trap handler.
// Then delegate most interrupts and exceptions to S-mode (the OS).
pub fn init() {
    // NOTE: This must be aligned to 4 bytes, asserted via repr() directive.
    let addr = supervisor_save as usize;
    unsafe { mtvec::write(addr, TrapMode::Direct) };
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

// best debugging function on the planet
fn crap() {
    println!(
        "[rustsbi] 0x{:x} 0x{:x} {:?}\n",
        mtval::read(),
        mepc::read(),
        mcause::read().cause()
    );
}

impl Coroutine for Runtime {
    type Yield = MachineTrap;
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        unsafe { do_resume(&mut self.context as *mut _) };
        let mtval = mtval::read();
        let t: Trap<Interrupt, Exception> = mcause::read().cause().try_into().unwrap();
        let trap = match t {
            Trap::Exception(Exception::SupervisorEnvCall) => MachineTrap::SbiCall(),
            Trap::Exception(Exception::IllegalInstruction) => MachineTrap::IllegalInstruction(),
            Trap::Exception(Exception::InstructionFault) => MachineTrap::InstructionFault(mtval),
            Trap::Exception(Exception::Breakpoint) => MachineTrap::IllegalInstruction(),
            Trap::Exception(Exception::LoadFault) => MachineTrap::LoadFault(mtval),
            Trap::Exception(Exception::StoreFault) => MachineTrap::StoreFault(mtval),
            Trap::Interrupt(Interrupt::MachineExternal) => MachineTrap::ExternalInterrupt(),
            Trap::Interrupt(Interrupt::MachineTimer) => {
                println!("mtimer traaaaap");
                crap();
                MachineTrap::MachineTimer()
            }
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

/// # Safety
///
/// This is the entry point to get back to S-mode.
/// Before resuming S-mode, save the M-mode state and restore the S-mode state.
/// Handle with care.
#[naked]
#[link_section = ".text"]
unsafe extern "C" fn do_resume(_supervisor_context: *mut SupervisorContext) {
    naked_asm!(
        "j     {machine_save}",
        machine_save = sym machine_save,
    )
}

/// # Safety
///
/// NOTE: Before entering this function, the caller's registers have been saved.
/// This is the reverse of machine_restore. Store current M-mode state.
/// a0 holds the supervisor context.
/// Handle with care.
#[naked]
#[link_section = ".text"]
unsafe extern "C" fn machine_save(_supervisor_context: *mut SupervisorContext) -> ! {
    naked_asm!(
        // Top of the stack
        "addi   sp, sp, -15*8",
        "sd     ra,  0*8(sp)
         sd     gp,  1*8(sp)
         sd     tp,  2*8(sp)
         sd     s0,  3*8(sp)
         sd     s1,  4*8(sp)
         sd     s2,  5*8(sp)
         sd     s3,  6*8(sp)
         sd     s4,  7*8(sp)
         sd     s5,  8*8(sp)
         sd     s6,  9*8(sp)
         sd     s7, 10*8(sp)
         sd     s8, 11*8(sp)
         sd     s9, 12*8(sp)
         sd    s10, 13*8(sp)
         sd    s11, 14*8(sp)",
        "j     {supervisor_restore}",
        supervisor_restore = sym supervisor_restore,
    )
}

/// # Safety
///
/// Restore S-mode state and return to S-mode.
/// This is the reverse of supervisor_save.
/// Handle with care.
#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn supervisor_restore(_supervisor_context: *mut SupervisorContext) -> ! {
    naked_asm!(
        // Save top of stack
        "sd     sp,  33*8(a0)",
        // Save a0, the supervisor context, in mscratch for later
        "csrw   mscratch, a0",
        // Restore the stack from supervisor context
        "mv     sp,  a0",
        // Restore mepc and mstatus
        "ld     t0,  31*8(sp)
         ld     t1,  32*8(sp)
         csrw   mstatus, t0
         csrw   mepc, t1",
        "ld     ra,  0*8(sp)
         ld     gp,  2*8(sp)
         ld     tp,  3*8(sp)
         ld     t0,  4*8(sp)
         ld     t1,  5*8(sp)
         ld     t2,  6*8(sp)
         ld     s0,  7*8(sp)
         ld     s1,  8*8(sp)
         ld     a0,  9*8(sp)
         ld     a1, 10*8(sp)
         ld     a2, 11*8(sp)
         ld     a3, 12*8(sp)
         ld     a4, 13*8(sp)
         ld     a5, 14*8(sp)
         ld     a6, 15*8(sp)
         ld     a7, 16*8(sp)
         ld     s2, 17*8(sp)
         ld     s3, 18*8(sp)
         ld     s4, 19*8(sp)
         ld     s5, 20*8(sp)
         ld     s6, 21*8(sp)
         ld     s7, 22*8(sp)
         ld     s8, 23*8(sp)
         ld     s9, 24*8(sp)
         ld    s10, 25*8(sp)
         ld    s11, 26*8(sp)
         ld     t3, 27*8(sp)
         ld     t4, 28*8(sp)
         ld     t5, 29*8(sp)
         ld     t6, 30*8(sp)",
        "ld     sp,  1*8(sp)",
        // Return to S-mode
        "mret",
    )
}

/// # Safety
///
/// This is the start of the interrupt handler. Handle with care.
/// Store the S-mode state for later resumption.
///       csrrw rd, csr, rs1
///       copy csr to rd and initial value of rs1 to csr, atomically
/// NOTE: must be 4-byte aligned
#[naked]
#[repr(align(4))]
#[link_section = ".text"]
pub unsafe extern "C" fn supervisor_save() -> ! {
    naked_asm!(
        // Swap this sp (stack pointer) with mscratch (M-mode scratch register).
        ".p2align 2",
        "csrrw  sp, mscratch, sp",
        "sd     ra,  0*8(sp)
         sd     gp,  2*8(sp)
         sd     tp,  3*8(sp)
         sd     t0,  4*8(sp)
         sd     t1,  5*8(sp)
         sd     t2,  6*8(sp)
         sd     s0,  7*8(sp)
         sd     s1,  8*8(sp)
         sd     a0,  9*8(sp)
         sd     a1, 10*8(sp)
         sd     a2, 11*8(sp)
         sd     a3, 12*8(sp)
         sd     a4, 13*8(sp)
         sd     a5, 14*8(sp)
         sd     a6, 15*8(sp)
         sd     a7, 16*8(sp)
         sd     s2, 17*8(sp)
         sd     s3, 18*8(sp)
         sd     s4, 19*8(sp)
         sd     s5, 20*8(sp)
         sd     s6, 21*8(sp)
         sd     s7, 22*8(sp)
         sd     s8, 23*8(sp)
         sd     s9, 24*8(sp)
         sd    s10, 25*8(sp)
         sd    s11, 26*8(sp)
         sd     t3, 27*8(sp)
         sd     t4, 28*8(sp)
         sd     t5, 29*8(sp)
         sd     t6, 30*8(sp)",
        "csrr   t0, mstatus
         sd     t0, 31*8(sp)",
        "csrr   t1, mepc
         sd     t1, 32*8(sp)",
        // Take back sp from mscratch via t2 and save current sp in mscratch.
        "csrrw  t2, mscratch, sp",
        // Store the sp on the stack.
        "sd     t2,  1*8(sp)",
        "j      {machine_restore}",
        machine_restore = sym machine_restore,
    )
}
/// # Safety
///
/// Restore M-mode state from stack.
/// Handle with care.
#[naked]
#[link_section = ".text"]
unsafe extern "C" fn machine_restore() -> ! {
    naked_asm!(
        // Restore M-mode / SBI runtime sp from mscratch.
        "csrr   sp, mscratch",
        "ld     sp, 33*8(sp)",
        "ld     ra,  0*8(sp)
         ld     gp,  1*8(sp)
         ld     tp,  2*8(sp)
         ld     s0,  3*8(sp)
         ld     s1,  4*8(sp)
         ld     s2,  5*8(sp)
         ld     s3,  6*8(sp)
         ld     s4,  7*8(sp)
         ld     s5,  8*8(sp)
         ld     s6,  9*8(sp)
         ld     s7, 10*8(sp)
         ld     s8, 11*8(sp)
         ld     s9, 12*8(sp)
         ld    s10, 13*8(sp)
         ld    s11, 14*8(sp)",
        // Top of stack
        "addi   sp, sp, 15*8",
        // Return back to M-mode runtime.
        "jr     ra",
    )
}
