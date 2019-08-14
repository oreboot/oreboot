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
use model::*;
use core::ops;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields};

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    TD: ReadWrite<u32, TD::Register>,	/* Transmit data register */
    RD: ReadOnly<u32, RD::Register>,	/* Receive data register */
    TXC: ReadWrite<u32, TXC::Register>,	/* Transmit control register */
    RXC: ReadWrite<u32, RXC::Register>,	/* Receive control register */
    IE: ReadWrite<u32, IE::Register>,	/* UART interrupt enable */
    IP: ReadWrite<u32, IP::Register>,	/* UART interrupt pending */
    BR: ReadOnly<u32, BR::Register>,	/* Baud rate */
}

pub struct SiFive {
    base: usize,
    // TODO: implement baudrate
    _baudrate: u32,
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
    BR [
        BaudRate OFFSET(0) NUMBITS(32) []
    ]

}

impl SiFive {
    pub fn new(base: usize, baudrate: u32) -> SiFive {
        SiFive{base: base, _baudrate: baudrate}
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for SiFive {
    fn init(&mut self) {
        self.IE.set(0 as u32);
        // TODO: set baudrate
//        self.BR.write(LC::DivisorLatchAccessBit::BaudRate);
        // Until we know the clock rate the divisor values are kind of
        // impossible to know. Throw in a phony value.
        self.TXC.modify(TXC::Enable.val(1));
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
            while self.TD.is_set(TD::Full) {}
                //return Ok(i);
            //}
            self.TD.set(c.into());
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
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
