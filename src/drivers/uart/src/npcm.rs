// http://www.ti.com/lit/ds/symlink/pc16550d.pdf
use core::ops;
use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

#[repr(C)]
pub struct RegisterBlock {
    d: ReadWrite<u8, D::Register>,
    _p1: u8,
    _p2: u16,
    ie: ReadWrite<u8, IE::Register>,
    _p3: u8,
    _p4: u16,
    fc: ReadWrite<u8, FC::Register>,
    _p5: u8,
    _p6: u16,
    lc: ReadWrite<u8, LC::Register>,
    _p7: u8,
    _p8: u16,
    mc: ReadWrite<u8, MC::Register>,
    _p9: u8,
    _p10: u16,
    ls: ReadOnly<u8, LS::Register>,
    _p11: u8,
    _p12: u16,
}

pub struct NPCM {
    base: usize,
}

impl ops::Deref for NPCM {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl NPCM {
    pub fn new(base: usize, _baudrate: u32) -> NPCM {
        NPCM { base: base }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
    fn poll_status(&self, bit: Field<u8, LS::Register>, val: bool) -> bool {
        // Timeout after a few thousand cycles to prevent hanging forever.
        for _ in 0..100_000 {
            if self.ls.is_set(bit) == val {
                return true;
            }
        }
        return false;
    }
}

impl Driver for NPCM {
    fn init(&mut self) {
        /* disable all interrupts */
        self.ie.set(0u8);
        /* Enable dLAB */
        self.lc.write(LC::DivisorLatchAccessBit::BaudRate);
        // Until we know the clock rate the divisor values are kind of
        // impossible to know. Throw in a phony value.
        self.lc.write(LC::WLEN::WLEN_8);
        // TOdO: what are these bits. how do we write them.
        self.fc.set(0xc7);
        self.mc.set(0x0b);
        self.lc.write(LC::DivisorLatchAccessBit::Normal);
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        for c in data.iter_mut() {
            while self.ls.is_set(LS::IF) {}
            *c = self.d.read(D::DATA) as u8;
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (i, &c) in data.iter().enumerate() {
            if !self.poll_status(LS::OE, false) {
                return Ok(i);
            }
            self.d.set(c as u8);
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

// TODO: bitfields
register_bitfields! {
    u8,
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
