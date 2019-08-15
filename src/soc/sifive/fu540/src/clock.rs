#![feature(asm)]
#![feature(global_asm)]
/*
 * This file is part of the coreboot project.
 *
 * Copyright 2018 Philipp Hug <philipp@hug.cx>
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

// 33.33 Mhz after reset
const FU540_BASE_FQY: usize = 33330;

use core::ops;
use model::*;

use crate::reg;
use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

#[allow(non_snake_case)]
#[repr(C)]

pub struct RegisterBlock {
    Crystal: ReadWrite<u32, Crystal::Register>,   /* offset 0x00 */
    Core: ReadWrite<u32, PLLCfg0::Register>,      /* offset 0x04 */
    _reserved08: u32,                             /* offset 0x08 */
    DDR0: ReadWrite<u32, PLLCfg0::Register>,      /* offset 0x0c */
    DDR1: ReadWrite<u32, PLLCfg1::Register>,      /* offset 0x10 */
    _reserved14: u32,                             /* offset 0x14 */
    _reserved18: u32,                             /* offset 0x18 */
    GE0: ReadWrite<u32, PLLCfg0::Register>,       /* offset 0x1c */
    GE1: ReadWrite<u32, PLLCfg1::Register>,       /* offset 0x20 */
    ClkSel: ReadWrite<u32, ClkSel::Register>,     /* offset 0x24 */
    DevReset: ReadWrite<u32, ResetCtl::Register>, /* offset 0x28 */
}

register_bitfields! {
    u32,
    Crystal [
        Enable OFFSET(30) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        State OFFSET(29) NUMBITS(1) [
            Ready = 1
        ]
    ],
    PLLCfg0 [
        DivR OFFSET(0) NUMBITS(6) [],
        DivF OFFSET(6) NUMBITS(9) [],
        DivQ OFFSET(15) NUMBITS(3) [],
        Range OFFSET(18) NUMBITS(3) [],
        ByPass OFFSET(24) NUMBITS(1) [],
        FSE OFFSET(25) NUMBITS(1) [],
        LockRO OFFSET(31) NUMBITS(1) []
    ],
    PLLCfg1[
        Ctrl OFFSET(24) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ],
    ClkSel [
        Sel OFFSET(0) NUMBITS(1) [
            Core = 0,
            HF = 1
        ]
    ],
    ResetCtl [
        DDRCtl OFFSET(0) NUMBITS(1) [
            Reset = 0
        ],
        DDRAXI OFFSET(1) NUMBITS(1) [
            Reset = 0
        ],
        DDRAHB OFFSET(2) NUMBITS(1) [
            Reset = 0
        ],
        DDRPHY OFFSET(3) NUMBITS(1) [
            Reset = 0
        ],
        GE OFFSET(5) NUMBITS(1) [
            Reset = 0
        ] // bit 5. Not a typo.
        ]
}

// There has to be a better way but ...
// the nicest thing would be something likes this:
// fn ResetMask(m ...string)
// then match on strings like
// "ddr", "phy", whatever.
// Or possibly a varargs of enums. Whatever.
fn ResetMask(ddr: bool, axi: bool, ahb: bool, phy: bool, GE: bool) -> u32 {
    // The default is to reset nothing.
    let mut m = ReadWrite::<u32, ResetCtl::Register>::new(0x2f);
    if ddr {
        m.modify(ResetCtl::DDRCtl.val(0));
    }
    m.get()
}

fn DefaultCore() -> u32 {
    let mut r = ReadWrite::<u32, PLLCfg0::Register>::new(0);
    r.modify(PLLCfg0::DivR.val(0));
    r.modify(PLLCfg0::DivF.val(59));
    r.modify(PLLCfg0::DivQ.val(2));
    r.modify(PLLCfg0::Range.val(4));
    r.modify(PLLCfg0::ByPass.val(0));
    r.modify(PLLCfg0::FSE.val(1));
    r.get()
}
fn DefaultDDR() -> u32 {
    let mut r = ReadWrite::<u32, PLLCfg0::Register>::new(0);
    r.modify(PLLCfg0::DivR.val(0));
    r.modify(PLLCfg0::DivF.val(55));
    r.modify(PLLCfg0::DivQ.val(2));
    r.modify(PLLCfg0::Range.val(4));
    r.modify(PLLCfg0::ByPass.val(0));
    r.modify(PLLCfg0::FSE.val(1));
    r.get()
}
fn DefaultGE() -> u32 {
    let mut r = ReadWrite::<u32, PLLCfg0::Register>::new(0);
    r.modify(PLLCfg0::DivR.val(0));
    r.modify(PLLCfg0::DivF.val(59));
    r.modify(PLLCfg0::DivQ.val(5));
    r.modify(PLLCfg0::Range.val(4));
    r.modify(PLLCfg0::ByPass.val(0));
    r.modify(PLLCfg0::FSE.val(1));
    r.get()
}

pub struct Clock {
    base: usize,
}

impl ops::Deref for Clock {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl Driver for Clock {
    fn init(&mut self) {
        /* nothing to do. */
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        let r: usize;
        match data {
            b"on" => r = 1,
            _ => r = 0,
        }
        Ok(r)
    }

    fn shutdown(&mut self) {}
}

//static struct prci_ctlr *prci = (void *)FU540_PRCI;

const PRCI_CORECLK_MASK: u32 = 1;
const PRCI_CORECLK_CORE_PLL: u32 = 0;
const PRCI_CORECLK_HFCLK: u32 = 1;

const PRCI_PLLCFG_LOCK: u32 = (1 << 31);
const PRCI_PLLCFG_DIVR_SHIFT: u32 = 0;
const PRCI_PLLCFG_DIVF_SHIFT: u32 = 6;
const PRCI_PLLCFG_DIVQ_SHIFT: u32 = 15;
const PRCI_PLLCFG_RANGE_SHIFT: u32 = 18;
const PRCI_PLLCFG_BYPASS_SHIFT: u32 = 24;
const PRCI_PLLCFG_FSE_SHIFT: u32 = 25;
const PRCI_PLLCFG_DIVR_MASK: u32 = (0x03f << PRCI_PLLCFG_DIVR_SHIFT);
const PRCI_PLLCFG_DIVF_MASK: u32 = (0x1ff << PRCI_PLLCFG_DIVF_SHIFT);
const PRCI_PLLCFG_DIVQ_MASK: u32 = (0x007 << PRCI_PLLCFG_DIVQ_SHIFT);
const PRCI_PLLCFG_RANGE_MASK: u32 = (0x07 << PRCI_PLLCFG_RANGE_SHIFT);
const PRCI_PLLCFG_BYPASS_MASK: u32 = (0x1 << PRCI_PLLCFG_BYPASS_SHIFT);
const PRCI_PLLCFG_FSE_MASK: u32 = (0x1 << PRCI_PLLCFG_FSE_SHIFT);

const PRCI_DDRPLLCFG1_MASK: u32 = (1 << 31);

const PRCI_GEMGXLPPLCFG1_MASK: u32 = (1 << 31);

const PRCI_CORECLKSEL_CORECLKSEL: u32 = 1;

/* Clock initialization should only be done in romstage. */

// struct pll_settings {
// 	unsigned int divr:6;
// 	unsigned int divf:9;
// 	unsigned int divq:3;
// 	unsigned int range:3;
// 	unsigned int bypass:1;
// 	unsigned int fse:1;
// };

impl Clock {
    pub fn new() -> Clock {
        Clock { base: reg::PRCI as usize }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }

    // This is nasty but it will get us on the air and we can convert
    // these one by one to use Registers. I don't want to deal with
    // a language change AND a new way of doing things.
    fn configure_pll(r: ReadWrite<u32, PLLCfg0::Register>, v: ReadWrite<u32, PLLCfg0::Register>) {
        r.set(v.get());
        // Wait for PLL Lock.
        // TODO: timeout and panic message
        loop {
            if r.is_set(PLLCfg0::LockRO) {
                return;
            }
        }
    }

    // static void configure_pll(u32 *reg, const  struct pll_settings *s)
    // {
    // 	// Write the settings to the register
    // 	u32 c = read32(reg);
    // 	clrsetbits_le32(&c, PRCI_PLLCFG_DIVR_MASK
    // 		| PRCI_PLLCFG_DIVF_MASK | PRCI_PLLCFG_DIVQ_MASK
    // 		| PRCI_PLLCFG_RANGE_MASK | PRCI_PLLCFG_BYPASS_MASK
    // 		| PRCI_PLLCFG_FSE_MASK,
    // 		(s->divr << PRCI_PLLCFG_DIVR_SHIFT)
    // 		| (s->divf << PRCI_PLLCFG_DIVF_SHIFT)
    // 		| (s->divq << PRCI_PLLCFG_DIVQ_SHIFT)
    // 		| (s->range << PRCI_PLLCFG_RANGE_SHIFT)
    // 		| (s->bypass << PRCI_PLLCFG_BYPASS_SHIFT)
    // 		| (s->fse << PRCI_PLLCFG_FSE_SHIFT));
    // 	write32(reg, c);

    // 	// Wait for PLL lock
    // 	while (!(read32(reg) & PRCI_PLLCFG_LOCK))
    // 		; /* TODO: implement a timeout */
    // }

    /*
     * Set coreclk according to the SiFive FU540-C000 Manual
     * https://www.sifive.com/documentation/chips/freedom-u540-c000-manual/
     *
     * Section 7.1 recommends a frequency of 1.0 GHz (up to 1.5 Ghz is possible)
     *
     * Section 7.4.2 provides the necessary values:
     * For example, to setup COREPLL for 1 GHz operation, program divr = 0 (x1),
     * divf = 59 (4000 MHz VCO), divq = 2 (/4 Output divider)
     */
    // static const  struct pll_settings corepll_settings = {
    // 	.divr = 0,
    // 	.divf = 59,
    // 	.divq = 2,
    // 	.range = 4,
    // 	.bypass = 0,
    // 	.fse = 1,
    // };
    /*
     * Section 7.4.3: DDR and Ethernet Subsystem Clocking and Reset
     *
     * Unfortunately the documentation example doesn't match the HiFive
     * Unleashed board settings.
     * Configuration values taken from SiFive FSBL:
     * https://github.com/sifive/freedom-u540-c000-bootloader/blob/master/fsbl/main.c
     *
     * DDRPLL is set up for 933 MHz output frequency.
     * divr = 0, divf = 55 (3730 MHz VCO), divq = 2
     *
     * GEMGXLPLL is set up for 125 MHz output frequency.
     * divr = 0, divf = 59 (4000 MHz VCO), divq = 5
     */
    // static const  struct pll_settings ddrpll_settings = {
    // 	.divr = 0,
    // 	.divf = 55,
    // 	.divq = 2,
    // 	.range = 4,
    // 	.bypass = 0,
    // 	.fse = 1,
    // };

    // static const  struct pll_settings gemgxlpll_settings = {
    // 	.divr = 0,
    // 	.divf = 59,
    // 	.divq = 5,
    // 	.range = 4,
    // 	.bypass = 0,
    // 	.fse = 1,
    // };

    fn init_coreclk(&self) {
        // switch coreclk to input reference frequency before modifying PLL
        self.ClkSel.write(ClkSel::Sel::HF);
        //	clrsetbits_le32(&prci->coreclksel, PRCI_CORECLK_MASK,PRCI_CORECLK_HFCLK);

        self.Core.set(DefaultCore());
        //	configure_pll(&prci->corepllcfg0, &corepll_settings);

        // switch coreclk to use corepll
        //	clrsetbits_le32(&prci->coreclksel, PRCI_CORECLK_MASK, PRCI_CORECLK_CORE_PLL);
        self.ClkSel.write(ClkSel::Sel::Core);
    }

    fn init_pll_ddr(&self) {
        // disable ddr clock output before reconfiguring the PLL
        self.DDR1.write(PLLCfg1::Ctrl::Disable);

        self.DDR0.set(DefaultDDR());
        // configure_pll(&prci->ddrpllcfg0, &ddrpll_settings);

        self.DDR1.write(PLLCfg1::Ctrl::Enable);
        // // enable ddr clock output
        // setbits_le32(&cfg1, PRCI_DDRPLLCFG1_MASK);
        // write32(&prci->ddrpllcfg1, cfg1);
    }

    fn init_pll_ge(&self) {
        // disable ddr clock output before reconfiguring the PLL
        self.GE1.write(PLLCfg1::Ctrl::Disable);

        self.GE0.set(DefaultGE());
        // configure_pll(&prci->ddrpllcfg0, &ddrpll_settings);

        self.GE1.write(PLLCfg1::Ctrl::Enable);
        // // enable ddr clock output
        // setbits_le32(&cfg1, PRCI_DDRPLLCFG1_MASK);
        // write32(&prci->ddrpllcfg1, cfg1);
    }

    // static void init_gemgxlclk(void)
    // {
    // 	u32 cfg1 = read32(&prci->gemgxlpllcfg1);
    // 	clrbits_le32(&cfg1, PRCI_GEMGXLPPLCFG1_MASK);
    // 	write32(&prci->gemgxlpllcfg1, cfg1);

    // 	configure_pll(&prci->gemgxlpllcfg0, &gemgxlpll_settings);

    // 	setbits_le32(&cfg1, PRCI_GEMGXLPPLCFG1_MASK);
    // 	write32(&prci->gemgxlpllcfg1, cfg1);
    // }

    fn clock_init(&self) {
        /*
         * Update the peripheral clock dividers of UART, SPI and I2C to safe
         * values as we can't put them in reset before changing frequency.
         */
        update_peripheral_clock_dividers();

        self.init_coreclk();
        // put DDR and ethernet in reset
        // This jams them all.
        self.DevReset.set(ResetMask(true, true, true, true, true));

        self.init_pll_ddr();

        // The following code and its comments is mostly derived from the SiFive
        // u540 bootloader.
        // https://github.com/sifive/freedom-u540-c000-bootloader

        // get DDR out of reset
        // TODO: clean this up later
        self.DevReset.write(ResetCtl::DDRCtl::Reset);
        //write32(&prci->devicesresetreg, PRCI_DEVICESRESET_DDR_CTRL_RST_N(1));

        // TODO need a fence MACRO. Can't be a call to a function.
        // 	// HACK to get the '1 full controller clock cycle'.
        architecture::fence();
        // 	asm volatile ("fence");
        // YUCK.
        self.DevReset.set(ResetMask(false, false, false, false, true));
        // 	// get all DDR out of reset
        // 	write32(&prci->devicesresetreg,
        // 		PRCI_DEVICESRESET_DDR_CTRL_RST_N(1) |
        // 		PRCI_DEVICESRESET_DDR_AXI_RST_N(1) |
        // 		PRCI_DEVICESRESET_DDR_AHB_RST_N(1) |
        // 		PRCI_DEVICESRESET_DDR_PHY_RST_N(1));

        // 	// HACK to get the '1 full controller clock cycle'.
        architecture::fence();
        // 	asm volatile ("fence");

        // 	// These take like 16 cycles to actually propagate. We can't go sending
        // 	// stuff before they come out of reset. So wait.
        // 	// TODO: Add a register to read the current reset states, or DDR Control
        // 	// device?
        for i in 0..255 {
            architecture::nop();
        }
        // 	for (int i = 0; i < 256; i++)
        // 		asm volatile ("nop");
        // FIXME need a delay

        self.init_pll_ge();
        self.DevReset.set(ResetMask(false, false, false, false, false));
        // 	write32(&prci->devicesresetreg,
        // 		PRCI_DEVICESRESET_DDR_CTRL_RST_N(1) |
        // 		PRCI_DEVICESRESET_DDR_AXI_RST_N(1) |
        // 		PRCI_DEVICESRESET_DDR_AHB_RST_N(1) |
        // 		PRCI_DEVICESRESET_DDR_PHY_RST_N(1) |
        // 		PRCI_DEVICESRESET_GEMGXL_RST_N(1));

        // TODO fence
        // 	asm volatile ("fence");
    }
}

// NASTY. We'll live with it for now.
const UART_DEVICES: u32 = 2;
const UART_REG_DIV: u32 = 0x18;
const UART_DIV_VAL: u32 = 4;

const SPI_DIV: u32 = 0x00;
const SPI_DIV_VAL: u32 = 4;
use core::ptr;
fn poke(v: u32, a: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}
fn update_peripheral_clock_dividers() {
    poke((reg::QSPI0 + SPI_DIV), SPI_DIV_VAL);
    poke((reg::QSPI1 + SPI_DIV), SPI_DIV_VAL);
    poke((reg::QSPI2 + SPI_DIV), SPI_DIV_VAL);

    for i in 0..UART_DEVICES {
        poke((reg::uart(i) + UART_REG_DIV), UART_DIV_VAL);
    }
}

//fn clock_init(&self) {
// void clock_init(void)
// {
// 	/*
// 	 * Update the peripheral clock dividers of UART, SPI and I2C to safe
// 	 * values as we can't put them in reset before changing frequency.
// 	 */
// 	update_peripheral_clock_dividers();

// 	self.init_coreclk();

// 	// put DDR and ethernet in reset
// 	write32(&prci->devicesresetreg, 0);

// 	init_pll_ddr();

// 	// The following code and its comments is mostly derived from the SiFive
// 	// u540 bootloader.
// 	// https://github.com/sifive/freedom-u540-c000-bootloader

// 	// get DDR out of reset
// 	write32(&prci->devicesresetreg, PRCI_DEVICESRESET_DDR_CTRL_RST_N(1));

// 	// HACK to get the '1 full controller clock cycle'.
// 	asm volatile ("fence");

// 	// get DDR out of reset
// 	write32(&prci->devicesresetreg,
// 		PRCI_DEVICESRESET_DDR_CTRL_RST_N(1) |
// 		PRCI_DEVICESRESET_DDR_AXI_RST_N(1) |
// 		PRCI_DEVICESRESET_DDR_AHB_RST_N(1) |
// 		PRCI_DEVICESRESET_DDR_PHY_RST_N(1));

// 	// HACK to get the '1 full controller clock cycle'.
// 	asm volatile ("fence");

// 	// These take like 16 cycles to actually propagate. We can't go sending
// 	// stuff before they come out of reset. So wait.
// 	// TODO: Add a register to read the current reset states, or DDR Control
// 	// device?
// 	for (int i = 0; i < 256; i++)
// 		asm volatile ("nop");

// 	init_gemgxlclk();

// 	write32(&prci->devicesresetreg,
// 		PRCI_DEVICESRESET_DDR_CTRL_RST_N(1) |
// 		PRCI_DEVICESRESET_DDR_AXI_RST_N(1) |
// 		PRCI_DEVICESRESET_DDR_AHB_RST_N(1) |
// 		PRCI_DEVICESRESET_DDR_PHY_RST_N(1) |
// 		PRCI_DEVICESRESET_GEMGXL_RST_N(1));

// 	asm volatile ("fence");
// }

// /* Get the core clock's frequency, in KHz */
// int clock_get_coreclk_khz(void)
// {
// 	if (read32(&prci->coreclksel) & PRCI_CORECLK_MASK)
// 		return FU540_BASE_FQY;

// 	u32 cfg  = read32(&prci->corepllcfg0);
// 	u32 divr = (cfg & PRCI_PLLCFG_DIVR_MASK)
// 		>> PRCI_PLLCFG_DIVR_SHIFT;
// 	u32 divf = (cfg & PRCI_PLLCFG_DIVF_MASK)
// 		>> PRCI_PLLCFG_DIVF_SHIFT;
// 	u32 divq = (cfg & PRCI_PLLCFG_DIVQ_MASK)
// 		>> PRCI_PLLCFG_DIVQ_SHIFT;

// 	printk(BIOS_SPEW, "clk: r=%d f=%d q=%d\n", divr, divf, divq);
// 	return FU540_BASE_FQY
// 		* 2 * (divf + 1)
// 		/ (divr + 1)
// 		/ (1ul << divq);
// }

// /* Get the TileLink clock's frequency, in KHz */
// int clock_get_tlclk_khz(void)
// {
// 	/*
// 	 * The TileLink bus and most peripherals use tlclk, which is coreclk/2,
// 	 * as input.
// 	 */
// 	return clock_get_coreclk_khz() / 2;
// }

/* 7.3 Memory Map (0x1000_0000–0x1000_0FFF) */
/* This section presents an overview of the PRCI control and configuration registers. */
/* Copyright © 2018, SiFive Inc. All rights reserved. 40 */
/*  Crystal Input Control Register (hfxosccfg) */
/*  Register Offset */
/* 0x0 */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* [28:0] */
/*  Reserved */
/* 29 */
/* xosc_rdy */
/* RO */
/* 0x0 */
/* Crystal input ready */
/* 30 */
/* xosccfg_en */
/* RW */
/* 0x1 */
/* Crystal input enable */
/*            Table 12: */
/* Crystal Input Control Register */
/*  Core PLL Configuration Register (corepllcfg0) */
/*  Register Offset */
/* 0x4 */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* [5:0] */
/* divr */
/* RW */
/* 0x1 */
/* PLL reference divider value minus one */
/* [14:6] */
/* divf */
/* RW */
/* 0x1F */
/* PLL feedback divider value minus one */
/* [17:15] */
/* divq */
/* RW */
/* 0x3 */
/* Log2 of PLL output divider. Valid settings are 1, 2, 3, 4, 5, 6 */
/* [20:18] */
/* range */
/* RW */
/* 0x0 */
/* PLL filter range. 3’b100 = 33MHz */
/* [23:21] */
/*  Reserved */
/* 24 */
/* bypass */
/* RW */
/* 0x0 */
/* PLL bypass */
/* 25 */
/* fse */
/* RW */
/* 0x1 */
/* Internal or external input path. Valid setting is 1, internal feedback. */
/* [30:26] */
/*  Reserved */
/* 31 */
/* lock */
/* RO */
/* 0x0 */
/* PLL locked */
/*                  Table 13: */
/* Core PLL Configuration Register */
/*  DDR PLL Configuration Register (ddrpllcfg0) */
/*  Register Offset */
/* 0xC */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* [5:0] */
/* divr */
/* RW */
/* 0x1 */
/* PLL reference divider value minus one */
/* [14:6] */
/* divf */
/* RW */
/* 0x1F */
/* PLL feedback divider value minus one */
/* [17:15] */
/* divq */
/* RW */
/* 0x3 */
/* Log2 of PLL output divider. Valid settings are 1,2,3,4,5,6 */
/* [20:18] */
/* range */
/* RW */
/* 0x0 */
/* PLL filter range. 3’b100 = 33MHz */
/* [23:21] */
/*  Reserved */
/* 24 */
/* bypass */
/* RW */
/* 0x0 */
/* PLL bypass */
/* 25 */
/* fse */
/* RW */
/* 0x1 */
/* Internal or external input path. Valid settings is 1, inter- nal feedback. */
/* [30:26] */
/*  Reserved */
/* 31 */
/* lock */
/* RO */
/* 0x0 */
/* PLL locked */
/*                  Table 14: */
/* DDR PLL Configuration Register */

/* Copyright © 2018, SiFive Inc. All rights reserved. 41 */
/*  DDR PLL Configuration Register (ddrpllcfg1) */
/*  Register Offset */
/* 0x10 */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* [23:0] */
/*  Reserved */
/* 24 */
/* cke */
/* RW */
/* 0x0 */
/* PLL clock output enable. Glitch free clock gate after PLL output. 1 enables clock, 0 disables clock */
/*           Table 15: DDR PLL Configuration Register */
/*  Gigabit Ethernet PLL Configuration Register (gemgxlpllcfg0) */
/*  Register Offset */
/* 0x1C */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* [5:0] */
/* divr */
/* RW */
/* 0x1 */
/* PLL reference divider value minus one */
/* [14:6] */
/* divf */
/* RW */
/* 0x1F */
/* PLL feedback divider value minus one */
/* [17:15] */
/* divq */
/* RW */
/* 0x3 */
/* Log2 of PLL output divider. Valid settings are 1,2,3,4,5,6 */
/* [20:18] */
/* range */
/* RW */
/* 0x0 */
/* PLL filter range. 3’b100 = 33MHz */
/* [23:21] */
/*  Reserved */
/* 24 */
/* bypass */
/* RW */
/* 0x0 */
/* PLL bypass */
/* 25 */
/* fse */
/* RW */
/* 0x1 */
/* Internal or external input path. Valid settings is 1, inter- nal feedback. */
/* [30:26] */
/*  Reserved */
/* 31 */
/* lock */
/* RO */
/* 0x0 */
/* PLL locked */
/*                  Table 16: Gigabit Ethernet PLL Configuration Register */
/*  Gigabit Ethernet PLL Configuration Register (gemgxlpllcfg1) */
/*  Register Offset */
/* 0x20 */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* [23:0] */
/*  Reserved */
/* 24 */
/* cke */
/* RW */
/* 0x0 */
/* PLL clock output enable. Glitch free clock gate after PLL output. 1 enables clock, 0 disables clock */
/*           Table 17: Gigabit Ethernet PLL Configuration Register */
/* Table 18: CORECLK Source Selection Register */
/*  CORECLK Source Selection Register (coreclksel) */
/*  Register Offset */
/* 0x24 */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* 0 */
/* coreclksel */
/* RW */
/* 0x1 */
/* CORECLK select. 0 = CORE_PLL output 1 = HFCLK */
/* [31:1] */
/*  Reserved */

/* Copyright © 2018, SiFive Inc. All rights reserved. 42 */
/* Peripheral Devices Reset Control Register (devicesresetreg) */
/* Register Offset */
/* 0x28 */
/* Bits */
/* Field Name */
/* Attr. */
/* Rst. */
/* Description */
/* 0 */
/* DDR_CTRL_RST_N */
/* RW */
/* 0x0 */
/* DDR Controller reset (active low) */
/* 1 */
/* DDR_AXI_RST_N */
/* RW */
/* 0x0 */
/* DDR Controller AXI interface reset (active low) */
/* 2 */
/* DDR_AHB_RST_N */
/* RW */
/* 0x0 */
/* DDR Controller AHB interface reset (active low) */
/* 3 */
/* DDR_PHY_RST_N */
/* RW */
/* 0x0 */
/* DDR PHY reset (active low) */
/* 4 */
/* Reserved */
/* 5 */
/* GEMGXL_RST_N */
/* RW */
/* 0x0 */
/* Gigabit Ethernet Subsystem reset (active low) */
