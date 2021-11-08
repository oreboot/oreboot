#![no_std]
//#![feature(global_asm)]

global_asm!(include_str!("bootblock.S"));

// There is no earlygrey-specific way of halting yet.
pub use cpu::halt;
