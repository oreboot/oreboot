#![feature(fn_align)]
#![feature(coroutine_trait)]
#![no_std]

#[cfg(feature = "riscv64")]
pub mod riscv64;
