// http://www.ti.com/lit/ds/symlink/pc16550d.pdf
use model::*;
use core::ops;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

#[repr(C)]
pub struct RegisterBlock {
    D: ReadWrite<u32, D::Register>,
    IE: ReadWrite<u32, IE::Register>,
    FC: ReadWrite<u32, FC::Register>,
    LC: ReadWrite<u32, LC::Register>,
    MC: ReadWrite<u32, MC::Register>,
    LS: ReadOnly<u32, LS::Register>,
}

pub struct NS16550 {
    base: usize,
    baudrate: u32,
}

impl ops::Deref for NS16550 {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl NS16550 {
    pub fn new(base: usize, baudrate: u32) -> NS16550 {
        NS16550 { base: base, baudrate: baudrate }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
    fn poll_status(&self, bit: Field<u32, LS::Register>, val: bool) -> bool {
        // Timeout after a few thousand cycles to prevent hanging forever.
        for _ in 0..100_000 {
            if self.LS.is_set(bit) == val {
                return true;
            }
        }
        return false;
    }
}

impl Driver for NS16550 {
    fn init(&mut self) {
        // Based on PL011 baud rate divisor table, set baud rate registers
        let (_high, _low) = match self.baudrate {
            230400 => (0x1, 0x5),
            115200 => (0x2, 0xB),
            76800 => (0x3, 0x10),
            38400 => (0x6, 0x21),
            14400 => (0x11, 0x17),
            2400 => (0x68, 0xB),
            110 => (0x8E0, 0x2F),
            _ => (0x0, 0x3), // Default values
        };
        self.IE.set(0u32);
        self.LC.write(LC::DivisorLatchAccessBit::BaudRate);
        // Until we know the clock rate the divisor values are kind of
        // impossible to know. Throw in a phony value.
        self.LC.write(LC::WLEN::WLEN_8);
        // TODO: what are these bits. how do we write them.
        self.FC.set(0xc7);
        self.MC.set(0x0b);
        self.LC.write(LC::DivisorLatchAccessBit::Normal);
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        for c in data.iter_mut() {
            while self.LS.is_set(LS::IF) {}
            *c = self.D.read(D::DATA) as u8;
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (i, &c) in data.iter().enumerate() {
            if !self.poll_status(LS::OE, false) {
                return Ok(i);
            }
            self.D.set(c as u32);
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

// TODO: bitfields
register_bitfields! {
    u32,
    // Data register
    D [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    IE [
        RX OFFSET(0) NUMBITS(1) [],
        TX OFFSET(1) NUMBITS(1) [],
        Error OFFSET(2) NUMBITS(1) [],
        StatusChange OFFSET(3) NUMBITS(1) []
    ],
    FC [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    LC [
        WLEN OFFSET(0) NUMBITS(2) [
            WLEN_5 = 0,
            WLEN_6 = 1,
            WLEN_7 = 2,
            WLEN_8 = 3
        ],
        StopBits OFFSET(3) NUMBITS(1) [],
        ParityEnable OFFSET(4) NUMBITS(1) [],
        EvenParity OFFSET(5) NUMBITS (1) [],
        StickParity OFFSET(6) NUMBITS (1) [],
        DivisorLatchAccessBit OFFSET(7) NUMBITS (1) [
            Normal = 0,
            BaudRate = 1
        ]
    ],
    MC [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    LS [
        IF OFFSET(0) NUMBITS(1) [],
        OE OFFSET(1) NUMBITS(1) []
    ]
}
