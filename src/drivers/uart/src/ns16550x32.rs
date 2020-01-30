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
        // Timeout after a few million cycles to prevent hanging forever.
        for _ in 0..10_000_000 {
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
        // Disable interrupts
        self.ie.set(0u32);
        // Enable DLAB to set baud rate
        self.lc.write(LC::DivisorLatchAccessBit::BaudRate);

        // The baud rate is set using the 16550 standard formula:
        //
        //                 uart_clk
        //   baudrate =  ------------
        //                 16 * div
        //
        // Divisors are set in 2x 8-bit registers;
        // Divisor Latch Low (DLL), and Divisor Latch High (DLH).
        let div = self.clk / (16 * self.baudrate);

        // Since we're in DLAB these registers are DLL and DLH
        self.d.set(div & 0xff);
        self.ie.set((div >> 8) & 0xff);

        // Go back to normal mode
        self.lc.write(LC::DivisorLatchAccessBit::Normal);

        // Reset and enable FIFOs
        self.fc.write(FC::Enable::SET);
        // Set 8n1
        self.lc.write(LC::WLEN::WLEN_8 + LC::ParityEnable::SET);
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        for c in data.iter_mut() {
            while self.ls.is_set(LS::DR) {}
            *c = self.d.read(D::DATA) as u8;
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (i, &c) in data.iter().enumerate() {
            if !self.poll_status(LS::THRE, true) {
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
        Enable OFFSET(0) NUMBITS(1) [],
        RXReset OFFSET(1) NUMBITS(1) [],
        TXReset OFFSET(2) NUMBITS(1) [],
        TXTriggerLevel OFFSET(4) NUMBITS(2) [
            TX_FIFO_Empty = 0,
            TX_FIFO_2B = 1,
            TX_FIFO_Quarter_Full = 2,
            TX_FIFO_Half_Full = 3
        ],
        RXTriggerLevel OFFSET(6) NUMBITS(2) [
            RX_FIFO_1B = 0,
            RX_FIFO_4B = 1,
            RX_FIFO_8B = 2,
            RX_FIFO_14B = 3
        ]
    ],
    LC [
        WLEN OFFSET(0) NUMBITS(2) [
            WLEN_5 = 0,
            WLEN_6 = 1,
            WLEN_7 = 2,
            WLEN_8 = 3
        ],
        StopBits OFFSET(2) NUMBITS(1) [],
        ParityEnable OFFSET(3) NUMBITS(1) [],
        EvenParity OFFSET(4) NUMBITS(1) [],
        DivisorLatchAccessBit OFFSET(7) NUMBITS(1) [
            Normal = 0,
            BaudRate = 1
        ]
    ],
    MC [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    LS [
        DR   OFFSET(0) NUMBITS(1) [],
        OE   OFFSET(1) NUMBITS(1) [],
        THRE OFFSET(5) NUMBITS(1) [],
        TEMT OFFSET(6) NUMBITS(1) []
    ]
}
