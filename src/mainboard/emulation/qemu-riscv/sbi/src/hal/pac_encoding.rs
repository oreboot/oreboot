pub const UART0_BASE: usize = 0x1000_0000;
pub const UART_THR: usize = 0;
pub const UART_RBR: usize = 0;
pub const UART_IER: usize = 0x8;
pub const UART_FCR: usize = 0x10;
pub const UART_LCR: usize = 0x18;
pub const UART_MCR: usize = 0x20;
pub const UART_LSR: usize = 0x28;
pub const UART_USR: usize = 0x7c;

pub const CLINT_BASE: usize = 0x0400_0000;
pub const MSIP0: usize = 0;
pub const MTIMECMPL: usize = 0x4000;
// pub const MTIMECMPH: usize = 0x4004;
// pub const SSIP: usize = 0xC000;
