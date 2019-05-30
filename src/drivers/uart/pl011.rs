#![no_std]

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    UARTDR: ReadWrite<u32, UARTDR::Register>,
    UARTRSR: ReadWrite<u32, UARTRSR::Register>,
    RESERVED0: [u32; 3],
    UARTFR: ReadOnly<u32, UARTFR::Register>,
    RESERVED1: [u32; 1],
    UARTILPR: ReadWrite<u32>,
    UARTIBRD: ReadWrite<u32>,
    UARTFBRD: ReadWrite<u32>,
    UARTLCR_H: ReadWrite<u32>,
    UARTCR: ReadWrite<u32>,
    UARTIFLS: ReadWrite<u32>,
    UARTIMSC: ReadWrite<u32>,
    UARTRIS: ReadWrite<u32>,
    UARTMIS: ReadWrite<u32>,
    UARTICR: ReadWrite<u32>,
    UARTDMACR: ReadWrite<u32>,
}

pub struct PL011 {
    regs: *const RegisterBlock,
    baudrate: u32,
}

impl PL011 {
    pub fn new(base : usize, baudrate : u32) -> PL011 {
        PL011 {
            regs: base as *const RegisterBlock,
            baudrate: baudrate,
        }
    }

    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
    fn poll_status(&self, bit: Field<u32, UARTFR::Register>, val: bool) -> bool {
        // Timeout after a few thousand cycles to prevent hanging forever.
        for _ in 0..100_000 {
            if unsafe { (*self.regs).UARTFR.is_set(bit) == val } {
                return true;
            }
        }
        return false;
    }
}

impl driver::Driver for PL011 {
    fn init(&self) {
        // TODO: actually use the given baud rate
        // 115200
        unsafe {
            (*self.regs).UARTIBRD.set(0x0);
            (*self.regs).UARTFBRD.set(0x3);
        }
        // 2400
        //(*self.regs).HIGH = 0x0;
        //(*self.regs).LOW = 0xBF;
    }

    fn read(&self, data: &mut [u8]) -> usize {
        for c in data.iter_mut() {
            while unsafe { (*self.regs).UARTFR.is_set(UARTFR::RXFE) } {}
            *c = unsafe { (*self.regs).UARTDR.read(UARTDR::DATA) as u8 };
        }
        data.len()
    }

    fn write(&self, data: &[u8]) -> usize {
        for (i, &c) in data.iter().enumerate() {
            if !self.poll_status(UARTFR::TXFF, false) {
                return i;
            }
            unsafe { (*self.regs).UARTDR.set(c as u32) };
        }
        data.len()
    }

    fn close(&self) {
        // flush the fifo
        self.poll_status(UARTFR::RXFF, false);
    }
}

register_bitfields! {
    u32,
    // Data register
    UARTDR [
        OE OFFSET(11) NUMBITS(1) [],
        BE OFFSET(10) NUMBITS(1) [],
        PE OFFSET(9) NUMBITS(1) [],
        FE OFFSET(8) NUMBITS(1) [],
        DATA OFFSET(0) NUMBITS(8) []
    ],
    // Receive status register / error clear register
    UARTRSR [
        OE OFFSET(3) NUMBITS(1) [],
        BE OFFSET(2) NUMBITS(1) [],
        PE OFFSET(1) NUMBITS(1) [],
        FE OFFSET(0) NUMBITS(1) []
    ],
    // Flag register
    UARTFR [
        // Ring indicator
        RI OFFSET(8) NUMBITS(1) [],
        // Transmit FIFO empty.
        TXFE OFFSET(7) NUMBITS(1) [],
        // Receive FIFO full.
        RXFF OFFSET(6) NUMBITS(1) [],
        // Transmit FIFO full.
        TXFF OFFSET(5) NUMBITS(1) [],
        // Receive FIFO empty
        RXFE OFFSET(4) NUMBITS(1) [],
        // UART busy
        BUSY OFFSET(3) NUMBITS(1) [],
        // Data carrier detect
        DCD OFFSET(2) NUMBITS(1) [],
        // Data set ready
        DSR OFFSET(1) NUMBITS(1) [],
        // Clear to send
        CTS OFFSET(0) NUMBITS(1) []
    ]
    // TODO: Integer baud rate register
    //UARTIBRD,
    // TODO: Fractional baud rate register
    //UARTFBRD,
    // TODO: Control register
    //UARTCR,
}
