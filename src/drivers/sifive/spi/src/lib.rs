#![no_std]
/// This is a driver for SiFive's SPI master, documented in the FU540 manual:

use model::*;
use core::ops;
use clock::ClockNode;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields};

#[repr(C)]
pub struct RegisterBlock {
    /// Serial clock divisor
    sckdiv: ReadWrite<u32, SCKDIV::Register>,
    /// Serial clock mode
    sckmode: ReadWrite<u32>,
    _reserved0: u32,
    _reserved1: u32,
    /// Chip select ID
    csid: ReadWrite<u32>,
    /// Chip select default
    csdef: ReadWrite<u32>,
    /// Chip select mode
    csmode: ReadWrite<u32>,
    _reserved2: u32,
    _reserved3: u32,
    _reserved4: u32,
    /// Delay control 0
    delay0: ReadWrite<u32>,
    /// Delay control 1
    delay1: ReadWrite<u32>,
    _reserved5: u32,
    _reserved6: u32,
    _reserved7: u32,
    _reserved8: u32,
    /// Frame format
    fmt: ReadWrite<u32>,
    _reserved9: u32,
    /// Tx FIFO Data
    txdata: ReadWrite<u32>,
    /// Rx FIFO data
    rxdata: ReadWrite<u32>,
    /// Tx FIFO watermark
    txmark: ReadWrite<u32>,
    /// Rx FIFO watermark
    rxmark: ReadWrite<u32>,
    _reserved10: u32,
    _reserved11: u32,
    /// SPI flash interface control
    fctrl: ReadWrite<u32>,
    /// SPI flash instruction format
    ffmt: ReadWrite<u32>,
    _reserved12: u32,
    _reserved13: u32,
    /// SPI interrupt enable
    ie: ReadWrite<u32>,
    /// SPI interrupt pending
    ip: ReadOnly<u32>,
}

pub struct SiFiveSpi {
    base: usize,
    serial_rate: u32,
}

impl ops::Deref for SiFiveSpi {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! {
    u32,
    SCKDIV [
        DIV OFFSET(0) NUMBITS(12) []
    ]
}

impl SiFiveSpi {
    pub fn new(base: usize, serial_rate: u32) -> SiFiveSpi {
        SiFiveSpi{base: base, serial_rate: serial_rate}
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for SiFiveSpi {
    fn init(&mut self) {
        // TODO: Implement init
        self.set_clock_rate(33_330_000);
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        // TODO: Implement pread
        NOT_IMPLEMENTED
    }

    fn pwrite(&mut self, _data: &[u8], _offset: usize) -> Result<usize> {
        // TODO: Implement wread
        NOT_IMPLEMENTED
    }

    fn shutdown(&mut self) {}
}

impl ClockNode for SiFiveSpi {
    // This uses hfclk as the input rate.
    fn set_clock_rate(&mut self, rate: u32) {
        // Since this is a SPI master, the serial rate is fairly flexible as long as it is not
        // faster than the SPI slave's specification. For this reason, the divisor rounds up.
        let denominator = 4 * self.serial_rate;
        let div = (rate + denominator - 1) / denominator;
        self.sckdiv.modify(SCKDIV::DIV.val(div));
    }
}
