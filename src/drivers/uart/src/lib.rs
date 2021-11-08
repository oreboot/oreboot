#![no_std]
//#![feature(asm)]

#[cfg(feature = "amdmmio")]
pub mod amdmmio;
#[cfg(feature = "debug_port")]
pub mod debug_port;
#[cfg(feature = "i8250")]
pub mod i8250;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "ns16550")]
pub mod ns16550;
#[cfg(feature = "null")]
pub mod null;
#[cfg(feature = "opentitan")]
pub mod opentitan;
#[cfg(feature = "pl011")]
pub mod pl011;
#[cfg(feature = "sifive")]
pub mod sifive;
#[cfg(feature = "sunxi")]
pub mod sunxi;

/* Calculate divisor. Do not floor but round to nearest integer. */
pub fn uart_baudrate_divisor(baudrate: usize, refclk: usize, oversample: usize) -> usize {
    (1 + (2 * refclk) / (baudrate * oversample)) / 2
}
