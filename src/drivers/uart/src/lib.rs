#![no_std]
#![feature(asm)]

#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "ns16550")]
pub mod ns16550;
#[cfg(feature = "ns16550x32")]
pub mod ns16550x32;
#[cfg(feature = "opentitan")]
pub mod opentitan;
#[cfg(feature = "pl011")]
pub mod pl011;
#[cfg(feature = "sifive")]
pub mod sifive;
