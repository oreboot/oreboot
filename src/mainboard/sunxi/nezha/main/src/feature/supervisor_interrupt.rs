use crate::runtime::SupervisorContext;
use riscv::register::{mie, mip};

static mut DEVINTRENTRY: usize = 0;

// Due to legacy 1.9.1 version of privileged spec, if we are in S-level
// timer handler (delegated from M mode), and we call SBI's `set_timer`,
// a M-level external interrupt may be triggered. This may try to obtain
// data structures locked previously by S-level interrupt handler, which
// results in a deadlock.
// Ref: https://github.com/luojia65/rustsbi/pull/5
pub fn preprocess_supervisor_external(ctx: &mut SupervisorContext) {
    if ctx.a7 == 0x0 {
        unsafe {
            let mtip = mip::read().mtimer();
            if mtip && DEVINTRENTRY != 0 {
                mie::set_mext();
            }
        }
    }
}
