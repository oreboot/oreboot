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

pub const MSEL: u64 = 0x00001000;
pub const DTIM: u64 = 0x01000000;
pub const CLINT: u64 = 0x02000000;
pub const L2_LIM: u64 = 0x08000000;
pub const UART0: u64 = 0x10010000;
//pub const UART(x): u64 = (FU540_UART0 + 0x1000 * (x));
pub const PRCI: u64 = 0x10000000;
pub const QSPI0: u64 = 0x10040000;
pub const QSPI1: u64 = 0x10041000;
pub const QSPI2: u64 = 0x10050000;
pub const GPIO: u64 = 0x10060000;
pub const OTP: u64 = 0x10070000;
pub const PIN_CTRL: u64 = 0x10080000;
pub const ETH_MAC: u64 = 0x10090000;
pub const ETH_MGMT: u64 = 0x100a0000;
pub const DDR_CTRL: u64 = 0x100b0000;
pub const DDR_BUS_BLOCKER: u64 = 0x100b8000;
pub const DDR_MGMT: u64 = 0x100c0000;
pub const QSPI0_FLASH: u64 = 0x20000000;
pub const QSPI1_FLASH: u64 = 0x30000000;
pub const DRAM: u64 = 0x80000000;
pub const MAX_DRAM: u64 = 0x2000000000;
