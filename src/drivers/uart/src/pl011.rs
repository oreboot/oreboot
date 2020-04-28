#![allow(non_snake_case)]

use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

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
    UARTLCR_H: ReadWrite<u32, UARTLCR_H::Register>,
    UARTCR: ReadWrite<u32, UARTCR::Register>,
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
    pub fn new(base: usize, baudrate: u32) -> PL011 {
        PL011 { regs: base as *const RegisterBlock, baudrate: baudrate }
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

impl Driver for PL011 {
    fn init(&mut self) -> Result<()> {
        // Based on PL011 baud rate divisor table, set baud rate registers
        let (high, low) = match self.baudrate {
            230400 => (0x1, 0x5),
            115200 => (0x2, 0xB),
            76800 => (0x3, 0x10),
            38400 => (0x6, 0x21),
            14400 => (0x11, 0x17),
            2400 => (0x68, 0xB),
            110 => (0x8E0, 0x2F),
            _ => (0x0, 0x3), // Default values
        };
        unsafe {
            (*self.regs).UARTIBRD.set(high);
            (*self.regs).UARTFBRD.set(low);
            // Line control: set 8-bits, FIFO enable
            (*self.regs).UARTLCR_H.modify(UARTLCR_H::WLEN::WLEN_8);
            (*self.regs).UARTLCR_H.modify(UARTLCR_H::FEN.val(1));
            // Control register: RX/TX enable, UART enable
            (*self.regs).UARTCR.modify(UARTCR::RXE.val(1));
            (*self.regs).UARTCR.modify(UARTCR::TXE.val(1));
            (*self.regs).UARTCR.modify(UARTCR::UARTEN.val(1));
        }
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        for c in data.iter_mut() {
            while unsafe { (*self.regs).UARTFR.is_set(UARTFR::RXFE) } {}
            *c = unsafe { (*self.regs).UARTDR.read(UARTDR::DATA) as u8 };
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (i, &c) in data.iter().enumerate() {
            if !self.poll_status(UARTFR::TXFF, false) {
                return Ok(i);
            }
            unsafe { (*self.regs).UARTDR.set(c as u32) };
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {
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
    ],
    // Line control register
    UARTLCR_H [
        // Send break
        BRK OFFSET(0) NUMBITS(1) [],
        // Parity enable
        PEN OFFSET(1) NUMBITS(1) [],
        // Even parity select
        EPS OFFSET(2) NUMBITS(1) [],
        // Two stop bits select
        STP2 OFFSET(3) NUMBITS(1) [],
        // FIFOs Enable
        FEN OFFSET(4) NUMBITS(1) [],
        // Word length
        WLEN OFFSET(5) NUMBITS(2) [
            WLEN_5 = 0,
            WLEN_6 = 1,
            WLEN_7 = 2,
            WLEN_8 = 3
        ],
        // Stick Parity select
        SPS OFFSET(7) NUMBITS(1) []
    ],
    // Control register
    UARTCR [
        // UART enable
        UARTEN OFFSET(0) NUMBITS(1) [],
        // SIR enable
        SIREN OFFSET(1) NUMBITS(1) [],
        // IrDA SIR low power mode
        SIRLP OFFSET(2) NUMBITS(1) [],
        // Loop back enable
        LBE OFFSET(7) NUMBITS(1) [],
        // Transmit enable
        TXE OFFSET(8) NUMBITS(1) [],
        // Receive enable
        RXE OFFSET(9) NUMBITS(1) [],
        // Data transmit ready
        DTR OFFSET(10) NUMBITS(1) [],
        // Request to send
        RTS OFFSET(11) NUMBITS(1) [],
        // Data Carrier Detect
        Out1 OFFSET(12) NUMBITS(1) [],
        // Ring Indicator
        Out2 OFFSET(13) NUMBITS(1) [],
        // RTS hardware flow control enable
        RTSEn OFFSET(14) NUMBITS(1) [],
        // CTS hardware flow control enable
        CTSEn OFFSET(15) NUMBITS(1) []
    ]
}
