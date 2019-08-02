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

const MSel: u64 =	0x00001000;
const Dtim: u64 =	0x01000000;
const CLINT: u64 =	0x02000000;
const L2Lim: u64 =	0x08000000;
const UART0: u64 =	0x10010000;
//const UART(x): u64 =	(FU540_UART0 + 0x1000 * (x));
const PRCI: u64 =	0x10000000;
const QSPI0: u64 =	0x10040000;
const QSPI1: u64 =	0x10041000;
const QSPI2: u64 =	0x10050000;
const GPIO: u64 =	0x10060000;
const OTP: u64 =	0x10070000;
const pinctrl: u64 =	0x10080000;
const EthMAC: u64 =	0x10090000;
const Ethmgmt: u64 =	0x100a0000;
const DDRctrl: u64 =	0x100b0000;
const DDRBusBlocker: u64 =0x100b8000;
const DDRMGMT: u64 =	0x100c0000;
const QSPI0Flash: u64 =0x20000000;
const QSPI1Flash: u64 =0x30000000;
const DRAM: u64 =	0x80000000;
const MaxDRAM: u64 =	0x2000000000;
