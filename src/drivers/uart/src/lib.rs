#![no_std]
#![feature(asm)]
#![deny(warnings)]

#[cfg(feature = "ns16550")]
pub mod ns16550;
#[cfg(feature = "pl011")]
pub mod pl011;
#[cfg(feature = "sifive")]
pub mod sifive;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "opentitan")]
pub mod opentitan;
