#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
// TODO: remove when done debugging crap
#![allow(unused)]

#[macro_use]
extern crate log;

use core::{
    arch::{asm, global_asm},
    intrinsics::transmute,
    panic::PanicInfo,
    ptr::slice_from_raw_parts,
};
use embedded_hal_nb::serial::Write;
use riscv::register::{marchid, mhartid, mimpid, mvendorid};
use rustsbi::RustSBI;

mod uart;
use uart::JH71XXSerial;

pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

const STACK_SIZE: usize = 4 * 1024; // 4KiB

#[link_section = ".bss.uninit"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Set up stack and jump to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
#[allow(named_asm_labels)]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // Clear feature disable CSR
        "csrwi  0x7c1, 0",
        "csrw   mtvec, t0",
        "csrw   mie, zero",
        "csrw   mstatus, zero",
        // suspend non-boot hart
        "li     a1, 0",
        "csrr   a0, mhartid",
        "bne    a0, a1, .nonboothart",
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
        "1:",
        "bgeu   t0, t1, 1f",
        "sd     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "j .boothart",
        // wait for multihart to get back into the game
        ".nonboothart:",
        "csrw   mie, 8", // 1 << 3
        "wfi",
        "call   {payload}",
        ".boothart:",
        "call   {main}",
        stack      =   sym STACK,
        stack_size = const STACK_SIZE,
        payload    =   sym exec_payload,
        main       =   sym main,
        options(noreturn)
    )
}

fn init_logger(s: JH71XXSerial) {
    unsafe {
        static mut SERIAL: Option<JH71XXSerial> = None;
        SERIAL.replace(s);
        log::init(SERIAL.as_mut().unwrap());
    }
}

const DTB_ADDR: usize = 0x0;
const LOAD_ADDR: usize = 0x0;

struct Executor {
    ctx: SupervisorContext,
    /* other environment variables ... */
    sbi: RustSBI<Clint, Clint, MyPlatRfnc, MyPlatHsm, MyBoardPower, MyPlatPmu>,
    /* custom_1: CustomSBI<...> */
}

impl Executor {
    /// A function that runs the provided supervisor, uses `&mut self` for it
    /// modifies `SupervisorContext`.
    ///
    /// It returns for every Trap the supervisor produces. Its handler should read
    /// and modify `self.ctx` if necessary. After handled, `run()` this structure
    /// again or exit execution process.
    pub fn run(&mut self) -> Trap {
        todo!("fill in generic or platform specific trampoline procedure")
    }

    pub fn new() {}

    fn fill_sbi_return() {}
}

pub fn execute_supervisor() -> Operation {
    let mut exec = Executor::new();
    loop {
        let trap = exec.run();
        if let Trap::Exception(Exception::SupervisorEcall) = trap.cause() {
            let ans =
                exec.sbi
                    .handle_ecall(exec.sbi_extension(), exec.sbi_function(), exec.sbi_params());
            /*
            if ans.error == MY_SPECIAL_EXIT {
                break Operation::from(ans);
            }
            */
            // This line would also advance `sepc` with `4` to indicate the `ecall` is handled.
            exec.fill_sbi_return(ans);
        } else {
            // other trap types ...
        }
    }
}

fn exec_payload() {
    execute_supervisor();
    // TODO: remove ...
    let hart_id = mhartid::read();
    unsafe {
        // jump to payload
        let f = transmute::<usize, EntryPoint>(LOAD_ADDR);
        asm!("fence.i");
        f(hart_id, DTB_ADDR);
    }
}

fn main() {
    let serial = JH71XXSerial::new();
    init_logger(serial);
    println!("oreboot 🦀");
    exec_payload();
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
