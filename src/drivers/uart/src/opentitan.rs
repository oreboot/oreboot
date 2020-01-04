/*
 * This file is part of the coreboot project.
 *
 * Copyright (C) 2018 Jonathan Neuschäfer
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
 * This is a driver for SiFive's own UART, documented in the FU540 manual:
 * https://www.sifive.com/documentation/chips/freedom-u540-c000-manual/
 */

// http://www.ti.com/lit/ds/symlink/pc16550d.pdf
use clock::ClockNode;
use core::ops;
use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::register_bitfields;

#[repr(C)]
pub struct RegisterBlock {
    // TODO: start using the new register crate which lets us set offsets.
    // We'll do that when the HJSON is right and we can just generate it.
    // tracer shows we need padding?
    interrupts: [u32; 3],
    ctrl: ReadWrite<u32, CTRL::Register>, /* UART control register */
    // 0x10
    status: ReadOnly<u32, STATUS::Register>, /* UART live status register */
    rdata: ReadOnly<u32, RDATA::Register>,   /* UART read data */
    // needs to be at 0x18
    wdata: ReadWrite<u32, WDATA::Register>, /* UART write data */
    fifo_ctrl: ReadWrite<u32, FIFO_CTRL::Register>, /* UART FIFO control register */
    fifo_status: ReadOnly<u32, FIFO_STATUS::Register>, /* UART FIFO status register */
    ovrd: ReadWrite<u32, OVRD::Register>, /* TX pin override control. Gives direct SW control over TX pin state */
    val: ReadOnly<u32, VAL::Register>,    /* UART oversampled values */
    timeout_ctrl: ReadWrite<u32, TIMEOUT_CTRL::Register>, /* UART RX timeout control */
}
register_bitfields! {
    u32,
    CTRL [
        TX OFFSET(0) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* TX enable */
        RX OFFSET(1) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* RX enable */
        NF OFFSET(2) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* RX noise filter enable.
If the noise filter is enabled, RX line goes through the 3-tap
repetition code. It ignores single IP clock period noise. */
        SLPBK OFFSET(4) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* System loopback enable.

If this bit is turned on, any outgoing bits to TX are received through RX.
See Block Diagram. Note that the TX line goes 1 if System loopback is enabled. */
        LLPBK OFFSET(5) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* Line loopback enable.

If this bit is turned on, incoming bits are forwarded to TX for testing purpose.
See Block Diagram. Note that the internal design sees RX value as 1 always if line
loopback is enabled. */
        PARITY_EN OFFSET(6) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* If true, parity is enabled in both RX and TX directions. */
        PARITY_ODD OFFSET(7) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* If PARITY_EN is true, this determines the type, 1 for odd parity, 0 for even. */
        RXBLVL OFFSET(9) NUMBITS(2) [ ],/* Trigger level for RX break detection. Sets the number of character
times the line must be low to detect a break. */
        NCO OFFSET(16) NUMBITS(16) []/* BAUD clock rate control. */
    ],
    STATUS [
        TXFULL OFFSET(0) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* TX buffer is full */
        RXFULL OFFSET(1) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* RX buffer is full */
        TXEMPTY OFFSET(2) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* TX FIFO is empty */
        TXIDLE OFFSET(3) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* TX is idle */
        RXIDLE OFFSET(4) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* RX is idle */
        RXEMPTY OFFSET(5) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/* RX FIFO is empty */
    ],
    RDATA [
        DATA OFFSET(7) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/*  */
    ],
    WDATA [
        DATA OFFSET(7) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/*  */
    ],
    FIFO_CTRL [
        RXRST OFFSET(0) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* RX fifo reset. Write 1 to the register resets RX_FIFO. Read returns 0 */
        TXRST OFFSET(1) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* TX fifo reset. Write 1 to the register resets TX_FIFO. Read returns 0 */
        RXILVL OFFSET(4) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* Trigger level for RX interrupts. If the FIFO depth is greater than or equal to
the setting, it raises rx_watermark interrupt. */
        TXILVL OFFSET(6) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/* Trigger level for TX interrupts. If the FIFO depth is greater than or equal to
the setting, it raises tx_watermark interrupt. */
    ],
    FIFO_STATUS [
        TXLVL OFFSET(5) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* Current fill level of TX fifo */
        RXLVL OFFSET(21) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/* Current fill level of RX fifo */
    ],
    OVRD [
        TXEN OFFSET(0) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* Enable TX pin override control */
        TXVAL OFFSET(1) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/* Write to set the value of the TX pin */
    ],
    VAL [
        RX OFFSET(15) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/* Last 16 oversampled values of RX. Most recent bit is bit 0, oldest 15. */
    ],
    TIMEOUT_CTRL [
        VAL OFFSET(23) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ],/* RX timeout value in UART bit times */
        EN OFFSET(31) NUMBITS(1) [
            OFF = 0,
            ON = 1
        ]/* Enable RX timeout feature */
    ]
}

// UART specific constants

const CLK_FIXED_FREQ_HZ: u32 = 500 * 1000;

pub struct OpenTitanUART {
    base: usize,
    baudrate: u32,
}

impl ops::Deref for OpenTitanUART {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl OpenTitanUART {
    pub fn new(base: usize, baudrate: u32) -> OpenTitanUART {
        OpenTitanUART { base: base, baudrate: baudrate }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for OpenTitanUART {
    fn init(&mut self) -> Result<()> {
        // nco = 2^20 * baud / fclk
        let uart_ctrl_nco = 0x4ea4; //(self.baudrate << 20) / CLK_FIXED_FREQ_HZ;
        let ctrl_val = uart_ctrl_nco;

        self.ctrl.modify(CTRL::NCO.val(ctrl_val));
        self.ctrl.modify(CTRL::RX::ON);
        self.ctrl.modify(CTRL::TX::ON);

        // reset RX/TX FIFOs
        self.fifo_ctrl.modify(FIFO_CTRL::RXRST::ON);
        self.fifo_ctrl.modify(FIFO_CTRL::TXRST::ON);

        // disable interrupts
        // TODO: set UART_INTR_ENABLE to 0
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        return Ok(0);
        // TODO
        //for c in data.iter_mut() {
        //    while ! self.RD.is_set(RD::Empty) {}
        //    *c = self.RD.read(RD::Data) as u8;
        //}
        //Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (_, &c) in data.iter().enumerate() {
            // TODO: give up after 100k tries.
            while self.status.is_set(STATUS::TXFULL) {
                // TODO: This is an extra safety precaution to prevent LLVM from possibly removing
                //       this loop. Remove if we deem it not necessary.
                unsafe { asm!("" :::: "volatile") }
            }
            //return Ok(i);
            //}
            self.wdata.set(c.into());
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

impl ClockNode for OpenTitanUART {
    // This uses hfclk as the input rate.
    fn set_clock_rate(&mut self, _rate: u32) {
        // For example, using the default clock rate of 33.33MHz:
        //   div = tlkclk / bbaud - 1
        //       = (33.33MHz / 2) / 115200 - 1
        //       = 144
        // Half the denominator is added to the numerator to round to closest int.
    }
}
