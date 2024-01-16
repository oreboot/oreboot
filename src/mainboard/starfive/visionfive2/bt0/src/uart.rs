use log::{Error, Serial};

use crate::pac;

pub type JH71XXSerial = jh71xx_hal::uart::Uart<pac::UART0>;

// UART0 Clock = clk_osc (24Mhz)
pub const UART_CLK: usize = 24_000_000;

pub(crate) fn uart0_divisor() -> u16 {
    let uart0 = pac::uart0_reg();

    // Clear FIFOs to set UART0 to idle
    uart0
        .fcr()
        .modify(|_, w| w.rfifor().set_bit().xfifor().set_bit());
    while uart0.usr().read().busy().bit_is_set() {}

    uart0.lcr().modify(|_, w| w.dlab().set_bit());
    let div = uart0.dll().read().bits() | (uart0.dlh().read().bits() << 8);
    uart0.lcr().modify(|_, w| w.dlab().clear_bit());

    div as u16
}
