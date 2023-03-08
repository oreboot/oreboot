#![feature(asm_const)]
#![no_std]

#[cfg(feature = "powerpc64")]
pub mod ppc64;
#[cfg(feature = "riscv64")]
pub mod riscv64;
