#![feature(asm_const)]
#![no_std]

#[cfg(feature = "armv7")]
pub mod armv7;
#[cfg(feature = "armv8")]
pub mod armv8;
#[cfg(feature = "powerpc64")]
pub mod ppc64;
#[cfg(feature = "riscv32")]
pub mod riscv32;
#[cfg(feature = "riscv64")]
pub mod riscv64;
#[cfg(feature = "x86_64")]
pub mod x86_64;
