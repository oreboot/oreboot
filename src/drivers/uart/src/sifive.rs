/*
 * This file is part of the coreboot project.
 *
 * Copyright (C) 2018 Jonathan Neuschäfer
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
 * This is a driver for SiFive's own UART, documented in the FU540 manual:
 * https://www.sifive.com/documentation/chips/freedom-u540-c000-manual/
 */

// http://www.ti.com/lit/ds/symlink/pc16550d.pdf
use clock::ClockNode;
use core::ops;
use model::*;

use register::mmio::{ReadOnly, ReadWrite};
use register::register_bitfields;

const RETRY_COUNT: u32 = 100_000;

#[repr(C)]
pub struct RegisterBlock {
    td: ReadWrite<u32, TD::Register>,   /* Transmit data register */
    rd: ReadOnly<u32, RD::Register>,    /* Receive data register */
    txc: ReadWrite<u32, TXC::Register>, /* Transmit control register */
    rxc: ReadWrite<u32, RXC::Register>, /* Receive control register */
    ie: ReadWrite<u32, IE::Register>,   /* UART interrupt enable */
    ip: ReadWrite<u32, IP::Register>,   /* UART interrupt pending */
    div: ReadWrite<u32, DIV::Register>, /* Baud Rate Divisor */
}

pub struct SiFive {
    base: usize,
    // TODO: implement baudrate
    baudrate: u32,
}

impl ops::Deref for SiFive {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! {
    u32,
    TD [
        Data OFFSET(0) NUMBITS(8) [],
        Full OFFSET(31) NUMBITS(1) []
    ],
    RD [
        Data OFFSET(0) NUMBITS(8) [],
        Empty OFFSET(31) NUMBITS(1) []
    ],
    IE[
        TX OFFSET(0) NUMBITS(1) [],
        RX OFFSET(1) NUMBITS(1) []
    ],
    TXC [
        Enable OFFSET(0) NUMBITS(1) [],
        StopBits OFFSET(1) NUMBITS(1) []
     //   TXCnt OFFSET(16) NUMBITS(16) [],
    ],
    RXC [
        Enable OFFSET(0) NUMBITS(1) []
    //    TXCnt OFFSET(16) NUMBITS(16) [],
    ],
    IP [
        TXWM OFFSET(0) NUMBITS(1) [],
        RXWM OFFSET(1) NUMBITS(1) []
    ],
    DIV [
        DIV OFFSET(0) NUMBITS(16) []
    ]
}

impl SiFive {
    pub fn new(base: usize, baudrate: u32) -> SiFive {
        SiFive { base, baudrate }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for SiFive {
    fn init(&mut self) -> Result<()> {
        // Disable UART interrupts.
        self.ie.set(0 as u32);
        // Set clock rate to the default 33.33MHz.
        self.set_clock_rate(33330000);
        // Enable transmit.
        self.txc.modify(TXC::Enable.val(1));
        // Enable receive.
        self.rxc.modify(RXC::Enable.val(1));
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        'outer: for (read_count, c) in data.iter_mut().enumerate() {
            for _ in 0..RETRY_COUNT {
                // Create a copy of the rxdata register so that we don't
                // lose the Data field when we read the Empty field.
                let rd_copy = self.rd.extract();
                if !rd_copy.is_set(RD::Empty) {
                    *c = rd_copy.read(RD::Data) as u8;
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
                if !self.td.is_set(TD::Full) {
                    self.td.set(c.into());
                    continue 'outer;
                }
            }
            return Ok(sent_count);
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

impl ClockNode for SiFive {
    // This uses hfclk as the input rate.
    fn set_clock_rate(&mut self, rate: u32) {
        // For example, using the default clock rate of 33.33MHz:
        //   div = tlkclk / bbaud - 1
        //       = (33.33MHz / 2) / 115200 - 1
        //       = 144
        // Half the denominator is added to the numerator to round to closest int.
        let div = (rate + self.baudrate) / (2 * self.baudrate) - 1;
        self.div.modify(DIV::DIV.val(div));
    }
}

//
//
//
//static void sifive_uart_init(struct sifive_uart_registers *regs, int div)
//{
//	/* Configure the divisor */
//	write32(&regs->div, div);
//
//	/* Enable transmission, one stop bit, transmit watermark at 1 */
//	write32(&regs->txctrl, TXCTRL_TXEN|TXCTRL_NSTOP(1)|TXCTRL_TXCNT(1));
//
//	/* Enable reception, receive watermark at 0 */
//	write32(&regs->rxctrl, RXCTRL_RXEN|RXCTRL_RXCNT(0));
//}
//
//void uart_init(int idx)
//{
//	unsigned int div;
//	div = uart_baudrate_divisor(get_uart_baudrate(),
//		uart_platform_refclk(), uart_input_clock_divider());
//	sifive_uart_init(uart_platform_baseptr(idx), div);
//}
//
//static bool uart_can_tx(struct sifive_uart_registers *regs)
//{
//	return !(read32(&regs->txdata) & TXDATA_FULL);
//}
//
//void uart_tx_byte(int idx, unsigned char data)
//{
//	struct sifive_uart_registers *regs = uart_platform_baseptr(idx);
//
//	while (!uart_can_tx(regs))
//		; /* TODO: implement a timeout */
//
//	write32(&regs->txdata, data);
//}
//
//void uart_tx_flush(int idx)
//{
//	struct sifive_uart_registers *regs = uart_platform_baseptr(idx);
//	uint32_t ip;
//
//	/* Use the TX watermark bit to find out if the TX FIFO is empty */
//	do {
//		ip = read32(&regs->ip);
//	} while (!(ip & IP_TXWM));
//}
//
//unsigned char uart_rx_byte(int idx)
//{
//	struct sifive_uart_registers *regs = uart_platform_baseptr(idx);
//	uint32_t rxdata;
//
//	do {
//		rxdata = read32(&regs->rxdata);
//	} while (rxdata & RXDATA_EMPTY);
//
//	return rxdata & 0xff;
//}
//
//unsigned int uart_input_clock_divider(void)
//{
//	/*
//	 * The SiFive UART handles oversampling internally. The divided clock
//	 * is the baud clock.
//	 */
//	return 1;
//}
//
//#ifndef __PRE_RAM__
//void uart_fill_lb(void *data)
//{
//	/* TODO */
//}
//#endif
