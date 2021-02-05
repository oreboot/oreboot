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

        // enable ioapic.
        smn_write(0x13b102f0, 0xfec00001);
        // TOM2
        smn_write(0x13b1_0064, 0x5000_0001);
        smn_write(0x13b1_0068, 0x4);
        smn_write(0x13c1_0064, 0x5000_0001);
        smn_write(0x13c1_0068, 0x4);
        smn_write(0x13d1_0064, 0x5000_0001);
        smn_write(0x13d1_0068, 0x4);
        smn_write(0x13e1_0064, 0x5000_0001);
        smn_write(0x13e1_0068, 0x4);

        //118
        // snmw(0x13b1_0118, 0x2);

        smn_write(0x13B1_0004, 0x00000800); //0x13B1_0004 
        smn_write(0x13B1_0020, 0x00000001); //0x13B1_0020
        smn_write(0x13B1_0028, 0x02620006); //0x13B1_0028
        smn_write(0x13B1_0034, 0x00000001); //0x13B1_0034
        smn_write(0x13B1_0044, 0x00000160); //0x13B1_0044
        smn_write(0x13B1_005C, 0x01000000); //0x13B1_005C
        smn_write(0x13B1_0060, 0x7fffffff); //0x13B1_0060
        smn_write(0x13B1_0064, 0x50000001); //0x13B1_0064
        smn_write(0x13B1_0068, 0x00000004); //0x13B1_0068
        smn_write(0x13B1_0070, 0x00000001); //0x13B1_0070
        smn_write(0x13B1_0080, 0x00010001); //0x13B1_0080
        smn_write(0x13B1_0088, 0x00000100); //0x13B1_0088
        smn_write(0x13B1_00B8, 0x00000009); //0x13B1_00B8
        smn_write(0x13B1_00BC, 0x0000000a); //0x13B1_00BC
        smn_write(0x13B1_00C0, 0x0000000b); //0x13B1_00C0
        smn_write(0x13B1_00C4, 0x0000000c); //0x13B1_00C4
        smn_write(0x13B1_00C8, 0x0000000d); //0x13B1_00C8
        smn_write(0x13B1_00CC, 0x0000000e); //0x13B1_00CC
        smn_write(0x13B1_00D0, 0x0000000f); //0x13B1_00D0
        smn_write(0x13B1_00D4, 0x00000011); //0x13B1_00D4
        smn_write(0x13B1_00D8, 0x00000019); //0x13B1_00D8
        smn_write(0x13B1_00DC, 0x0000001a); //0x13B1_00DC
        smn_write(0x13B1_00E0, 0x0000001b); //0x13B1_00E0
        smn_write(0x13B1_00E4, 0x0000001c); //0x13B1_00E4
        smn_write(0x13B1_00E8, 0x0000001d); //0x13B1_00E8
        smn_write(0x13B1_00EC, 0x0000001e); //0x13B1_00EC
        smn_write(0x13B1_00F0, 0x0000001f); //0x13B1_00F0
        smn_write(0x13B1_00F4, 0x00000021); //0x13B1_00F4
        smn_write(0x13B1_00F8, 0x00000029); //0x13B1_00F8
        smn_write(0x13B1_00FC, 0x0000002a); //0x13B1_00FC
        smn_write(0x13B1_0100, 0x00000039); //0x13B1_0100
        smn_write(0x13B1_0104, 0x00000041); //0x13B1_0104
        smn_write(0x13B1_0118, 0x00000002); //0x13B1_0118
        smn_write(0x13B1_012C, 0x0000ff00); //0x13B1_012C
        smn_write(0x13B1_0180, 0xf931b07f); //0x13B1_0180
        smn_write(0x13B1_0184, 0xfffffffd); //0x13B1_0184
        smn_write(0x13B1_0188, 0x17fff040); //0x13B1_0188
        smn_write(0x13B1_018C, 0xfffffffe); //0x13B1_018C
        smn_write(0x13B1_0190, 0x00007f60); //0x13B1_0190
        smn_write(0x13B1_01A4, 0x80005f40); //0x13B1_01A4
        smn_write(0x13B1_01B8, 0x80003f20); //0x13B1_01B8
        smn_write(0x13B1_01CC, 0x80001f00); //0x13B1_01CC
        smn_write(0x13B1_01E0, 0x8000ffe0); //0x13B1_01E0
        smn_write(0x13B1_01F4, 0x8000dfc0); //0x13B1_01F4
        smn_write(0x13B1_0208, 0x8000bfa0); //0x13B1_0208
        smn_write(0x13B1_021C, 0x80009f80); //0x13B1_021C
        smn_write(0x13B1_0230, 0x00000001); //0x13B1_0230
        smn_write(0x13B1_02C4, 0x00040000); //0x13B1_02C4
        smn_write(0x13B1_02E0, 0xc9000101); //0x13B1_02E0
        smn_write(0x13B1_02E8, 0xc9100003); //0x13B1_02E8
        smn_write(0x13B1_02F0, 0xc9280001); //0x13B1_02F0
        smn_write(0x13B1_0340, 0x0000000f); //0x13B1_0340
        smn_write(0x13B1_0344, 0x0000c810); //0x13B1_0344
        smn_write(0x13B1_0348, 0x00000001); //0x13B1_0348
        smn_write(0x13B1_0350, 0x00000004); //0x13B1_0350
        smn_write(0x13B1_0354, 0x00020000); //0x13B1_0354
        smn_write(0x13B1_0358, 0x00020000); //0x13B1_0358

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
