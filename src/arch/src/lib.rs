#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(generator_trait)]
#![no_std]

#[cfg(feature = "riscv64")]
pub mod riscv64;
