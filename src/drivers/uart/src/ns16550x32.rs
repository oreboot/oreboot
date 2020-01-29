// This is the same as NS16550 but using 32-bit registers
use core::ops;
use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

#[repr(C)]
pub struct RegisterBlock {
    // TODO: DLAB
    d: ReadWrite<u32, D::Register>,
    ie: ReadWrite<u32, IE::Register>,
    fc: ReadWrite<u32, FC::Register>,
    lc: ReadWrite<u32, LC::Register>,
    mc: ReadWrite<u32, MC::Register>,
    ls: ReadOnly<u32, LS::Register>,
}

pub struct NS16550x32 {
    base: usize,
    baudrate: u32,
    clk: u32,
}

impl ops::Deref for NS16550x32 {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl NS16550x32 {
    pub fn new(base: usize, baudrate: u32, clk: u32) -> NS16550x32 {
        NS16550x32 { base, baudrate, clk }
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
            if self.ls.is_set(bit) == val {
                return true;
            }
        }
        return false;
    }
}

impl Driver for NS16550x32 {
    fn init(&mut self) -> Result<()> {
        self.lc.write(LC::DivisorLatchAccessBit::Normal);
        /* disable all interrupts */
        self.ie.set(0u32);
        /* Enable dLAB */
        self.lc.write(LC::DivisorLatchAccessBit::BaudRate);

        // TODO: Implement DLAB handling. DLAB is overlaid on the D/IE fields
        // and we need some sort of union {}.
        // However, in simulator the baud-rate is kind of ignored.
        self.lc.write(LC::DivisorLatchAccessBit::Normal);
        Ok(())
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
            self.d.set(c as u32);
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

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
