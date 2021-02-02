/*
 * This file is part of the coreboot project.
 *
 * Copyright (C) 2020 Google Inc.
 * Copyright (C) 2021 DATACOM Electronics GmbH
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

#![allow(non_upper_case_globals)]

use clock::ClockNode;
use core::ptr;
use model::*;
use x86_64::registers::model_specific::Msr;

const SMB_UART_CONFIG: u32 = 0xfed8_00fc;
const SMB_UART_1_8M_SHIFT: u8 = 28;
const SMB_UART_CONFIG_UART0_1_8M: u32 = 1 << SMB_UART_1_8M_SHIFT;
const SMB_UART_CONFIG_UART1_1_8M: u32 = 1 << (SMB_UART_1_8M_SHIFT + 1);

const FCH_UART_LEGACY_DECODE: u32 = 0xfedc0020;
const FCH_LEGACY_3F8_SH: u16 = 1 << 3;
//const FCH_LEGACY_2F8_SH: u16 = 1 << 1;

// See coreboot:src/soc/amd/common/block/include/amdblocks/acpimmio_map.h

const ACPI_MMIO_BASE: u32 = 0xfed8_0000;
const IOMUX_BASE: u32 = ACPI_MMIO_BASE + 0xd00; // Note: 256 u8 block--one u8 per pinctrl
const AOAC_BASE: u32 = ACPI_MMIO_BASE + 0x1e00;

// See coreboot:src/soc/amd/common/block/include/amdblocks/aoac.h

const FCH_AOACx40_D3_CONTROL: u32 = AOAC_BASE + 0x40; // Note: 32 times (control: u8, status: u8) block
const AOAC_PWR_ON_DEV: u8 = 1 << 3;

/* TODO: Use D3 State:
FCH_AOAC_PWR_RST_STATE        BIT(0)
FCH_AOAC_RST_CLK_OK_STATE     BIT(1)
FCH_AOAC_RST_B_STATE          BIT(2)
FCH_AOAC_DEV_OFF_GATING_STATE BIT(3)
FCH_AOAC_D3COLD               BIT(4)
FCH_AOAC_CLK_OK_STATE         BIT(5)
FCH_AOAC_STAT0                BIT(6)
FCH_AOAC_STAT1                BIT(7) */

// See coreboot:src/soc/amd/picasso/include/soc/southbridge.h

const FCH_AOAC_DEV_UART0: u8 = 11;
const FCH_AOAC_DEV_UART1: u8 = 12;
const FCH_AOAC_DEV_AMBA: u8 = 17;

// It's kind of a shame, but every single pci crate I've looked at is
// basically close to useless. Unless I'm missing something,
// which is likely. They really should get all the various authors
// and a room and just DEFINE ONE THING. It's not rocket science.
// I'm not going to attempt to write one because:
// 1. I suck at it.
// 2. It would be JUST ONE MORE.
// SMN

fn smn_read(a: u32) -> u32 {
    // the smn device is at (0)
    unsafe {
        outl(0xcf8, 0x8000_00b8);
        outl(0xcfc, a);
        outl(0xcf8, 0x8000_00bc);
        inl(0xcfc)
    }
}

fn smn_write(a: u32, v: u32) {
    unsafe {
        outl(0xcf8, 0x800000b8);
        outl(0xcfc, a);
        outl(0xcf8, 0x800000bc);
        outl(0xcfc, v);
    }
}

// end SMN
fn poke16(a: u32, v: u16) -> () {
    let y = a as *mut u16;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn poke32(a: u32, v: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek32(a: u32) -> u32 {
    let y = a as *mut u32;
    unsafe {
        return ptr::read_volatile(y);
    }
}

// Read u8 from address A, reset bits from RESET_MASK, set bits from SET_MASK, write result back to address A
fn pokers8(a: u32, reset_mask: u8, set_mask: u8) -> () {
    let y = a as *mut u8;
    unsafe {
        ptr::write_volatile(y, (ptr::read_volatile(y) & !reset_mask) | set_mask);
    }
}

/// Write 32 bits to port
unsafe fn outl(port: u16, val: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{al}"(val));
}

/// Read 32 bits from port
unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    llvm_asm!("inl %dx, %eax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    return ret;
}

// WIP: mainboard driver. I mean the concept is a WIP.
pub struct MainBoard {}

impl MainBoard {
    pub fn new() -> MainBoard {
        MainBoard {}
    }
}

fn d3_control_power_on(n: u8) -> () {
    if n < 32 {
        pokers8(FCH_AOACx40_D3_CONTROL + u32::from(n * 2), 0, AOAC_PWR_ON_DEV);
    }
}

fn pinctrl(pin: u8, mode: u8) -> () {
    // IOMUX (IOMUX total: 256 registers; one u8 per GPIO)
    pokers8(IOMUX_BASE + u32::from(pin), 3, mode);
}

impl Driver for MainBoard {
    fn init(&mut self) -> Result<()> {
        // Knowledge from coreboot to get minimal serials working.
        // clock defaults are NOT fine.
        //uart_ctrl = sm_pci_read32(SMB_UART_CONFIG);
        //uart_ctrl |= 1 << (SMB_UART_1_8M_SHIFT + idx);
        //sm_pci_write32(SMB_UART_CONFIG, uart_ctrl);
        // we want 1.8m. They made oddball 48m default. Stupid.
        let mut uc = peek32(SMB_UART_CONFIG);
        uc = uc | SMB_UART_CONFIG_UART0_1_8M;
        uc = uc | SMB_UART_CONFIG_UART1_1_8M;
        poke32(SMB_UART_CONFIG, uc);

        d3_control_power_on(FCH_AOAC_DEV_AMBA); // Note: It's on by default
        d3_control_power_on(FCH_AOAC_DEV_UART0); // Note: It's on by default
        d3_control_power_on(FCH_AOAC_DEV_UART1); // Note: It's on by default

        // UART 0

        // See coreboot:src/soc/amd/picasso/include/soc/gpio.h
        pinctrl(135, 0); // [UART0_CTS_L, UART2_RXD, EGPIO135][0]; Note: The reset default is 0
        pinctrl(136, 0); // [UART0_RXD, EGPIO136][0]; Note: The reset default is 0
        pinctrl(137, 0); // [UART0_RTS_L, EGPIO137][0]
        pinctrl(138, 0); // [UART0_TXD, EGPIO138][0]
        pinctrl(139, 0); // [UART0_INTR, AGPIO139][0]; Note: The reset default is 0

        // UART 1

        // See coreboot:src/soc/amd/picasso/include/soc/gpio.h
        pinctrl(140, 0); // [UART1_CTS_L_UART3_TXD, EGPIO140][0]; Note: The reset default is 0
        pinctrl(141, 0); // [UART1_RXD, EGPIO141][0]; Note: The reset default is 0
        pinctrl(142, 0); // [UART1_RTS_L, EGPIO142][0]
        pinctrl(143, 0); // [UART1_TXD, EGPIO143][0]
        pinctrl(144, 0); // [UART1_INTR, AGPIO144][0]; Note: The reset default is 0

        // Set up the legacy decode for UART 0.
        poke16(FCH_UART_LEGACY_DECODE, FCH_LEGACY_3F8_SH);
        let mut msr0 = Msr::new(0x1b);
        unsafe {
            let v = msr0.read() | 0x900;
            msr0.write(v);
            //let v = msr.read() | 0xd00;
            //write!(w, "NOT ENABLING x2apic!!!\n\r");
            //msr.write(v);
        }
        // IOAPIC
        //     wmem fed80300 e3070b77
        //    wmem fed00010 3
        poke32(0xfed80300, 0xe3070b77); //FCH PM DECODE EN
        poke32(0xfed00010, 3); //HPETCONFIG
        let i = peek32(0xfed00010);
        poke32(0xfed00010, i | 8);
        // THis is likely not needed but.
        //poke32(0xfed00108, 0x5b03d997);

        // enable ioapic redirection
        // IOHC::IOAPIC_BASE_ADDR_LO
        smn_write(0x13B1_02f0, 0xFEC0_0001);

        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        return Ok(0);
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

impl ClockNode for MainBoard {
    // This uses hfclk as the input rate.
    fn set_clock_rate(&mut self, _rate: u32) {}
}
