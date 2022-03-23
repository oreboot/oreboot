#[cfg(feature = "uart_amdmmio")]
pub mod amdmmio;
#[cfg(feature = "uart_debug_port")]
pub mod debug_port;
#[cfg(feature = "uart_i8250")]
pub mod i8250;
#[cfg(feature = "uart_log")]
pub mod log;
#[cfg(feature = "uart_ns16550")]
pub mod ns16550;
#[cfg(feature = "uart_null")]
pub mod null;
#[cfg(feature = "uart_opentitan")]
pub mod opentitan;
#[cfg(feature = "uart_pl011")]
pub mod pl011;
#[cfg(feature = "uart_sifive")]
pub mod sifive;
#[cfg(feature = "uart_spi")]
pub mod spi;
#[cfg(feature = "uart_sunxi")]
pub mod sunxi;

/* Calculate divisor. Do not floor but round to nearest integer. */
pub fn uart_baudrate_divisor(baudrate: usize, refclk: usize, oversample: usize) -> usize {
    (1 + (2 * refclk) / (baudrate * oversample)) / 2
}
