/*
 * This file is part of the oreboot project.
 *
 * Copyright (C) 2020 SiFive Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

/*
 * This is a driver for the SiFive SPI Controller, documented in the FU540 manual:
 * https://www.sifive.com/documentation/chips/freedom-u540-c000-manual/
 */
#![no_std]
#![deny(warnings)]

use clock::ClockNode;
use core::ops;
use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::register_bitfields;

const RETRY_COUNT: u32 = 100_000;

pub enum SiFiveSpiPhase {
    SampleLeading,
    SampleTrailing,
}

pub enum SiFiveSpiPolarity {
    InactiveLow,
    InactiveHigh,
}

pub enum SiFiveSpiProtocol {
    Single,
    Dual,
    Quad,
}

pub enum SiFiveSpiEndianness {
    BigEndian,
    LittleEndian,
}

pub struct SiFiveSpiConfig {
    pub freq: u32,
    pub phase: SiFiveSpiPhase,
    pub polarity: SiFiveSpiPolarity,
    pub protocol: SiFiveSpiProtocol,
    pub endianness: SiFiveSpiEndianness,
    pub bits_per_frame: u8,
}

pub struct SiFiveSpiMmapConfig {
    pub command_enable: bool,
    pub address_len: u8,
    pub pad_count: u8,
    pub command_protocol: SiFiveSpiProtocol,
    pub address_protocol: SiFiveSpiProtocol,
    pub data_protocol: SiFiveSpiProtocol,
    pub command_code: u8,
    pub pad_code: u8,
}

#[repr(C)]
pub struct RegisterBlock {
    /// Serial clock divisor
    sckdiv: ReadWrite<u32, ClockDivider::Register>,
    /// Serial clock mode
    sckmode: ReadWrite<u32, ClockMode::Register>,
    _reserved0: u32,
    _reserved1: u32,
    /// Chip select ID
    csid: ReadWrite<u32, ActiveChipSelect::Register>,
    /// Chip select default
    csdef: ReadWrite<u32, ChipSelectDefault::Register>,
    /// Chip select mode
    csmode: ReadWrite<u32, ChipSelectMode::Register>,
    _reserved2: u32,
    _reserved3: u32,
    _reserved4: u32,
    /// Delay control 0
    delay0: ReadWrite<u32, DelayControl0::Register>,
    /// Delay control 1
    delay1: ReadWrite<u32, DelayControl1::Register>,
    _reserved5: u32,
    _reserved6: u32,
    _reserved7: u32,
    _reserved8: u32,
    /// Frame format
    fmt: ReadWrite<u32, Format::Register>,
    _reserved9: u32,
    /// Tx FIFO Data
    txdata: ReadWrite<u32, TransmitData::Register>,
    /// Rx FIFO data
    rxdata: ReadOnly<u32, ReceiveData::Register>,
    /// Tx FIFO watermark
    txmark: ReadWrite<u32, TransmitMark::Register>,
    /// Rx FIFO watermark
    rxmark: ReadWrite<u32, ReceiveMark::Register>,
    _reserved10: u32,
    _reserved11: u32,
    /// SPI flash interface control
    fctrl: ReadWrite<u32, FlashControl::Register>,
    /// SPI flash instruction format
    ffmt: ReadWrite<u32, FlashFormat::Register>,
    _reserved12: u32,
    _reserved13: u32,
    /// SPI interrupt enable
    ie: ReadWrite<u32, InterruptEnable::Register>,
    /// SPI interrupt pending
    ip: ReadOnly<u32, InterruptPending::Register>,
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
    ClockDivider [
        DIV OFFSET(0) NUMBITS(12) []
    ],
    ClockMode [
        PHA OFFSET(0) NUMBITS(1) [
            SampleLeading  = 0,
            SampleTrailing = 1
        ],
        POL OFFSET(1) NUMBITS(1) [
            InactiveLow  = 0,
            InactiveHigh = 1
        ]
    ],
    ActiveChipSelect [
        ID OFFSET(0) NUMBITS(32) []
    ],
    ChipSelectDefault [
        DEF0 OFFSET(0) NUMBITS(1) [],
        DEF1 OFFSET(0) NUMBITS(1) [],
        DEF2 OFFSET(0) NUMBITS(1) [],
        DEF3 OFFSET(0) NUMBITS(1) [],
        DEF4 OFFSET(0) NUMBITS(1) [],
        DEF5 OFFSET(0) NUMBITS(1) [],
        DEF6 OFFSET(0) NUMBITS(1) [],
        DEF7 OFFSET(0) NUMBITS(1) [],
        DEF8 OFFSET(0) NUMBITS(1) [],
        DEF9 OFFSET(0) NUMBITS(1) [],
        DEF10 OFFSET(0) NUMBITS(1) [],
        DEF11 OFFSET(0) NUMBITS(1) [],
        DEF12 OFFSET(0) NUMBITS(1) [],
        DEF13 OFFSET(0) NUMBITS(1) [],
        DEF14 OFFSET(0) NUMBITS(1) [],
        DEF15 OFFSET(0) NUMBITS(1) [],
        DEF16 OFFSET(0) NUMBITS(1) [],
        DEF17 OFFSET(0) NUMBITS(1) [],
        DEF18 OFFSET(0) NUMBITS(1) [],
        DEF19 OFFSET(0) NUMBITS(1) [],
        DEF20 OFFSET(0) NUMBITS(1) [],
        DEF21 OFFSET(0) NUMBITS(1) [],
        DEF22 OFFSET(0) NUMBITS(1) [],
        DEF23 OFFSET(0) NUMBITS(1) [],
        DEF24 OFFSET(0) NUMBITS(1) [],
        DEF25 OFFSET(0) NUMBITS(1) [],
        DEF26 OFFSET(0) NUMBITS(1) [],
        DEF27 OFFSET(0) NUMBITS(1) [],
        DEF28 OFFSET(0) NUMBITS(1) [],
        DEF29 OFFSET(0) NUMBITS(1) [],
        DEF30 OFFSET(0) NUMBITS(1) [],
        DEF31 OFFSET(0) NUMBITS(1) []
    ],
    ChipSelectMode [
        MODE OFFSET(0) NUMBITS(2) [
            Auto = 0b00,
            Hold = 0b10,
            Off  = 0b11
        ]
    ],
    DelayControl0 [
        CSSCK OFFSET(0) NUMBITS(8) [],
        SCKCS OFFSET(16) NUMBITS(8) []
    ],
    DelayControl1 [
        InterCS OFFSET(0) NUMBITS(8) [],
        InterXfr OFFSET(16) NUMBITS(8) []
    ],
    Format [
        Protocol OFFSET(0) NUMBITS(2) [
            Single = 0b00,
            Dual   = 0b01,
            Quad   = 0b10
        ],
        Endianness OFFSET(2) NUMBITS(1) [
            MSB = 0b0,
            LSB = 0b1
        ],
        Direction OFFSET(3) NUMBITS(1) [
            Rx = 0b0,
            Tx = 0b1
        ],
        Length OFFSET(16) NUMBITS(4) []
    ],
    TransmitData [
        Data OFFSET(0) NUMBITS(8) [],
        Full OFFSET(31) NUMBITS(1) []
    ],
    ReceiveData [
        Data OFFSET(0) NUMBITS(8) [],
        Empty OFFSET(31) NUMBITS(1) []
    ],
    TransmitMark [
        TXWM OFFSET(0) NUMBITS(3) []
    ],
    ReceiveMark [
        RXWM OFFSET(0) NUMBITS(3) []
    ],
    InterruptEnable [
        TXWMIE OFFSET(0) NUMBITS(1) [],
        RXWMIE OFFSET(1) NUMBITS(1) []
    ],
    InterruptPending [
        TXWMIP OFFSET(0) NUMBITS(1) [],
        RXWMIP OFFSET(1) NUMBITS(1) []
    ],
    FlashControl [
        MMAPEN OFFSET(0) NUMBITS(1) []
    ],
    FlashFormat [
        CMD_EN OFFSET(0) NUMBITS(1) [],
        ADDR_LEN OFFSET(1) NUMBITS(3) [],
        PAD_CNT OFFSET(4) NUMBITS(4) [],
        CMD_PROTO OFFSET(8) NUMBITS(2) [
            Single = 0b00,
            Dual   = 0b01,
            Quad   = 0b10
        ],
        ADDR_PROTO OFFSET(10) NUMBITS(2) [
            Single = 0b00,
            Dual   = 0b01,
            Quad   = 0b10
        ],
        DATA_PROTO OFFSET(12) NUMBITS(2) [
            Single = 0b00,
            Dual   = 0b01,
            Quad   = 0b10
        ],
        CMD_CODE OFFSET(16) NUMBITS(8) [],
        PAD_CODE OFFSET(24) NUMBITS(8) []
    ]
}

impl SiFiveSpi {
    pub fn new(base: usize, serial_rate: u32) -> SiFiveSpi {
        SiFiveSpi { base, serial_rate }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }

    pub fn setup(&mut self, cs: u32, config: SiFiveSpiConfig) -> Result<()> {
        self.set_clock_rate(config.freq);
        self.csid.write(ActiveChipSelect::ID.val(cs));
        match config.phase {
            SiFiveSpiPhase::SampleLeading => self.sckmode.modify(ClockMode::PHA::SampleLeading),
            SiFiveSpiPhase::SampleTrailing => self.sckmode.modify(ClockMode::PHA::SampleTrailing),
        }
        match config.polarity {
            SiFiveSpiPolarity::InactiveLow => self.sckmode.modify(ClockMode::POL::InactiveLow),
            SiFiveSpiPolarity::InactiveHigh => self.sckmode.modify(ClockMode::POL::InactiveHigh),
        }
        match config.protocol {
            SiFiveSpiProtocol::Single => self.fmt.modify(Format::Protocol::Single),
            SiFiveSpiProtocol::Dual => self.fmt.modify(Format::Protocol::Dual),
            SiFiveSpiProtocol::Quad => self.fmt.modify(Format::Protocol::Quad),
        }
        match config.endianness {
            SiFiveSpiEndianness::LittleEndian => self.fmt.modify(Format::Endianness::LSB),
            SiFiveSpiEndianness::BigEndian => self.fmt.modify(Format::Endianness::MSB),
        }
        self.fmt.modify(Format::Direction::Tx);
        self.fmt.modify(Format::Length.val(config.bits_per_frame.into()));

        Ok(())
    }

    pub fn mmap(&mut self, config: SiFiveSpiMmapConfig) -> Result<()> {
        // Disable memory mapped mode before configuring
        self.fctrl.modify(FlashControl::MMAPEN.val(0));

        // Reset SPI Flash chip
        self.pwrite(&[0x66, 0x99], 0).unwrap();

        self.ffmt.modify(FlashFormat::CMD_EN.val(config.command_enable.into()));
        self.ffmt.modify(FlashFormat::ADDR_LEN.val(config.address_len.into()));
        self.ffmt.modify(FlashFormat::PAD_CNT.val(config.pad_count.into()));
        match config.command_protocol {
            SiFiveSpiProtocol::Single => self.ffmt.modify(FlashFormat::CMD_PROTO::Single),
            SiFiveSpiProtocol::Dual => self.ffmt.modify(FlashFormat::CMD_PROTO::Dual),
            SiFiveSpiProtocol::Quad => self.ffmt.modify(FlashFormat::CMD_PROTO::Quad),
        }
        match config.address_protocol {
            SiFiveSpiProtocol::Single => self.ffmt.modify(FlashFormat::ADDR_PROTO::Single),
            SiFiveSpiProtocol::Dual => self.ffmt.modify(FlashFormat::ADDR_PROTO::Dual),
            SiFiveSpiProtocol::Quad => self.ffmt.modify(FlashFormat::ADDR_PROTO::Quad),
        }
        match config.data_protocol {
            SiFiveSpiProtocol::Single => self.ffmt.modify(FlashFormat::DATA_PROTO::Single),
            SiFiveSpiProtocol::Dual => self.ffmt.modify(FlashFormat::DATA_PROTO::Dual),
            SiFiveSpiProtocol::Quad => self.ffmt.modify(FlashFormat::DATA_PROTO::Quad),
        }
        self.ffmt.modify(FlashFormat::CMD_CODE.val(config.command_code.into()));
        self.ffmt.modify(FlashFormat::PAD_CODE.val(config.pad_code.into()));

        // Re-enable memory mapped mode
        self.fctrl.modify(FlashControl::MMAPEN.val(1));

        Ok(())
    }
}

impl Driver for SiFiveSpi {
    fn init(&mut self) -> Result<()> {
        // Disable Interrupts
        self.ie.set(0 as u32);
        // Set rate to 33.33 MHz
        self.set_clock_rate(33_330_000);

        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        'outer: for (read_count, c) in data.iter_mut().enumerate() {
            for _ in 0..RETRY_COUNT {
                let rxdata_copy = self.rxdata.extract();
                if !rxdata_copy.is_set(ReceiveData::Empty) {
                    *c = rxdata_copy.read(ReceiveData::Data) as u8;
                    continue 'outer;
                }
            }
            return Ok(read_count);
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        'outer: for (sent_count, &c) in data.iter().enumerate() {
            for _ in 0..RETRY_COUNT {
                if !self.txdata.is_set(TransmitData::Full) {
                    self.txdata.set(c.into());
                    continue 'outer;
                }
            }
            return Ok(sent_count);
        }
        Ok(data.len())
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
        self.sckdiv.modify(ClockDivider::DIV.val(div));
    }
}
