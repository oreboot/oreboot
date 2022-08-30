pub const UART0_BASE: usize = 0x1000_0000;
pub const UART_THR: usize = 0;
pub const UART_RBR: usize = 0;
pub const UART_IER: usize = 0x1;
pub const UART_FCR: usize = 0x2;
pub const UART_LCR: usize = 0x3;
pub const UART_MCR: usize = 0x4;
pub const UART_LSR: usize = 0x5;
// pub const UART_USR: usize = 0x?;

pub const CLINT_BASE: usize = 0x0200_0000;
pub const MSIP0: usize = 0;
pub const MTIMECMPL: usize = 0x4000;
// pub const MTIMECMPH: usize = 0x4004;
// pub const SSIP: usize = 0xC000;
