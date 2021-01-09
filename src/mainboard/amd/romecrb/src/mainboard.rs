/*
 * This file is part of the coreboot project.
 *
 * Copyright (C) 2020 Google Inc.
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

use clock::ClockNode;
use core::ptr;
use model::*;
use x86_64::registers::model_specific::Msr;

const FCH_UART_LEGACY_DECODE: u32 = 0xfedc0020;
const FCH_LEGACY_3F8_SH: u16 = 1 << 3;

// It's kind of a shame, but every single pci crate I've looked at is
// basically close to useless. Unless I'm missing something,
// which is likely. They really should get all the various authors
// and a room and just DEFINE ONE THING. It's not rocket science.
// I'm not going to attempt to write one because:
// 1. I suck at it.
// 2. It would be JUST ONE MORE.
// SMN

fn snmr(a: u32) -> u32 {
    // the smn device is at (0)
    unsafe {
        outl(0xcf8, 0x800000b8);
        outl(0xcfc, a);
        outl(0xcf8, 0x800000bc);
        inl(0xcfc)
    }
}

fn snmw(a: u32, v: u32) {
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

impl Driver for MainBoard {
    fn init(&mut self) -> Result<()> {
        // Knowledge from coreboot to get minimal serial working.
        // GPIO defaults are fine.
        // clock default is NOT fine.
        // Need to set it to 8 mhz.
        // this should fuck up uart output but we'll see.
        //uart_ctrl = sm_pci_read32(SMB_UART_CONFIG);
        //uart_ctrl |= 1 << (SMB_UART_1_8M_SHIFT + idx);
        //sm_pci_write32(SMB_UART_CONFIG, uart_ctrl);
        // FED8000 is the basic MMIO space.
        // fed800fc is the uart control reg.
        // bit 28 is the bit which sets it between 48m and 1.8m
        // we want 1.8m. They made oddball 48m default. Stupid.
        let mut uc = peek32(0xfed800fc);
        uc = uc | (1 << 28);
        poke32(0xfed800fc, uc);
        // Set up the legacy decode.
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
        poke32(0xfed80300, 0xe3070b77);
        poke32(0xfed00010, 3);
        let i = peek32(0xfed00010);
        poke32(0xfed00010, i | 8);
        // THis is likely not needed but.
        //poke32(0xfed00108, 0x5b03d997);

        // enable ioapic.
        snmw(0x13b102f0, 0xfec00001);
        // TOM2
        snmw(0x13b1_0064, 0x5000_0001);
        snmw(0x13b1_0068, 0x4);
        snmw(0x13c1_0064, 0x5000_0001);
        snmw(0x13c1_0068, 0x4);
        snmw(0x13d1_0064, 0x5000_0001);
        snmw(0x13d1_0068, 0x4);
        snmw(0x13e1_0064, 0x5000_0001);
        snmw(0x13e1_0068, 0x4);

        //118
        snmw(0x13b1_0118, 0x2);

        snmw(0x13b1_0190, 0x00007f60);
        snmw(0x13b1_0194, 0x0000_0000);
        snmw(0x13b1_0198, 0x0000_0000);
        snmw(0x13b1_019c, 0x0000_0000);
        snmw(0x13b1_01a0, 0x0000_0000);
        snmw(0x13b1_01a4, 0x8000_5f40);
        snmw(0x13b1_01a8, 0x0000_0000);
        snmw(0x13b1_01ac, 0x0000_0000);
        snmw(0x13b1_01b0, 0x0000_0000);
        snmw(0x13b1_01b4, 0x0000_0000);
        snmw(0x13b1_01b8, 0x8000_3f20);
        snmw(0x13b1_01bc, 0x0000_0000);
        snmw(0x13b1_01c0, 0x0000_0000);
        snmw(0x13b1_01c4, 0x0000_0000);
        snmw(0x13b1_01c8, 0x0000_0000);
        snmw(0x13b1_01cc, 0x8000_1f00);
        snmw(0x13b1_01d0, 0x0000_0000);
        snmw(0x13b1_01d4, 0x0000_0000);
        snmw(0x13b1_01d8, 0x0000_0000);
        snmw(0x13b1_01dc, 0x0000_0000);
        snmw(0x13b1_01e0, 0x8000_ffe0);
        snmw(0x13b1_01e4, 0x0000_0000);
        snmw(0x13b1_01e8, 0x0000_0000);
        snmw(0x13b1_01ec, 0x0000_0000);
        snmw(0x13b1_01f0, 0x0000_0000);
        snmw(0x13b1_01f4, 0x8000_dfc0);
        snmw(0x13b1_01f8, 0x0000_0000);
        snmw(0x13b1_01fc, 0x0000_0000);
        snmw(0x13b1_0200, 0x0000_0000);
        snmw(0x13b1_0204, 0x0000_0000);
        snmw(0x13b1_0208, 0x8000_bfa0);
        snmw(0x13b1_020c, 0x0000_0000);
        snmw(0x13b1_0210, 0x0000_0000);
        snmw(0x13b1_0214, 0x0000_0000);
        snmw(0x13b1_0218, 0x0000_0000);
        snmw(0x13b1_021c, 0x8000_9f80);
        snmw(0x13c1_0190, 0x8000_7f60);
        snmw(0x13c1_0194, 0x0000_0000);
        snmw(0x13c1_0198, 0x0000_0000);
        snmw(0x13c1_019c, 0x0000_0000);
        snmw(0x13c1_01a0, 0x0000_0000);
        snmw(0x13c1_01a4, 0x0000_5f40);
        snmw(0x13c1_01a8, 0x0000_0000);
        snmw(0x13c1_01ac, 0x0000_0000);
        snmw(0x13c1_01b0, 0x0000_0000);
        snmw(0x13c1_01b4, 0x0000_0000);
        snmw(0x13c1_01b8, 0x8000_3f20);
        snmw(0x13c1_01bc, 0x0000_0000);
        snmw(0x13c1_01c0, 0x0000_0000);
        snmw(0x13c1_01c4, 0x0000_0000);
        snmw(0x13c1_01c8, 0x0000_0000);
        snmw(0x13c1_01cc, 0x8000_1f00);
        snmw(0x13c1_01d0, 0x0000_0000);
        snmw(0x13c1_01d4, 0x0000_0000);
        snmw(0x13c1_01d8, 0x0000_0000);
        snmw(0x13c1_01dc, 0x0000_0000);
        snmw(0x13c1_01e0, 0x8000_ffe0);
        snmw(0x13c1_01e4, 0x0000_0000);
        snmw(0x13c1_01e8, 0x0000_0000);
        snmw(0x13c1_01ec, 0x0000_0000);
        snmw(0x13c1_01f0, 0x0000_0000);
        snmw(0x13c1_01f4, 0x8000_dfc0);
        snmw(0x13c1_01f8, 0x0000_0000);
        snmw(0x13c1_01fc, 0x0000_0000);
        snmw(0x13c1_0200, 0x0000_0000);
        snmw(0x13c1_0204, 0x0000_0000);
        snmw(0x13c1_0208, 0x8000_bfa0);
        snmw(0x13c1_020c, 0x0000_0000);
        snmw(0x13c1_0210, 0x0000_0000);
        snmw(0x13c1_0214, 0x0000_0000);
        snmw(0x13c1_0218, 0x0000_0000);
        snmw(0x13c1_021c, 0x8000_9f80);
        snmw(0x13d1_0190, 0x8000_7f60);
        snmw(0x13d1_0194, 0x0000_0000);
        snmw(0x13d1_0198, 0x0000_0000);
        snmw(0x13d1_019c, 0x0000_0000);
        snmw(0x13d1_01a0, 0x0000_0000);
        snmw(0x13d1_01a4, 0x8000_5f40);
        snmw(0x13d1_01a8, 0x0000_0000);
        snmw(0x13d1_01ac, 0x0000_0000);
        snmw(0x13d1_01b0, 0x0000_0000);
        snmw(0x13d1_01b4, 0x0000_0000);
        snmw(0x13d1_01b8, 0x0000_3f20);
        snmw(0x13d1_01bc, 0x0000_0000);
        snmw(0x13d1_01c0, 0x0000_0000);
        snmw(0x13d1_01c4, 0x0000_0000);
        snmw(0x13d1_01c8, 0x0000_0000);
        snmw(0x13d1_01cc, 0x8000_1f00);
        snmw(0x13d1_01d0, 0x0000_0000);
        snmw(0x13d1_01d4, 0x0000_0000);
        snmw(0x13d1_01d8, 0x0000_0000);
        snmw(0x13d1_01dc, 0x0000_0000);
        snmw(0x13d1_01e0, 0x8000_ffe0);
        snmw(0x13d1_01e4, 0x0000_0000);
        snmw(0x13d1_01e8, 0x0000_0000);
        snmw(0x13d1_01ec, 0x0000_0000);
        snmw(0x13d1_01f0, 0x0000_0000);
        snmw(0x13d1_01f4, 0x8000_dfc0);
        snmw(0x13d1_01f8, 0x0000_0000);
        snmw(0x13d1_01fc, 0x0000_0000);
        snmw(0x13d1_0200, 0x0000_0000);
        snmw(0x13d1_0204, 0x0000_0000);
        snmw(0x13d1_0208, 0x8000_bfa0);
        snmw(0x13d1_020c, 0x0000_0000);
        snmw(0x13d1_0210, 0x0000_0000);
        snmw(0x13d1_0214, 0x0000_0000);
        snmw(0x13d1_0218, 0x0000_0000);
        snmw(0x13d1_021c, 0x8000_9f80);
        snmw(0x13e1_0190, 0x8000_7f60);
        snmw(0x13e1_0194, 0x0000_0000);
        snmw(0x13e1_0198, 0x0000_0000);
        snmw(0x13e1_019c, 0x0000_0000);
        snmw(0x13e1_01a0, 0x0000_0000);
        snmw(0x13e1_01a4, 0x8000_5f40);
        snmw(0x13e1_01a8, 0x0000_0000);
        snmw(0x13e1_01ac, 0x0000_0000);
        snmw(0x13e1_01b0, 0x0000_0000);
        snmw(0x13e1_01b4, 0x0000_0000);
        snmw(0x13e1_01b8, 0x8000_3f20);
        snmw(0x13e1_01bc, 0x0000_0000);
        snmw(0x13e1_01c0, 0x0000_0000);
        snmw(0x13e1_01c4, 0x0000_0000);
        snmw(0x13e1_01c8, 0x0000_0000);
        snmw(0x13e1_01cc, 0x0000_1f00);
        snmw(0x13e1_01d0, 0x0000_0000);
        snmw(0x13e1_01d4, 0x0000_0000);
        snmw(0x13e1_01d8, 0x0000_0000);
        snmw(0x13e1_01dc, 0x0000_0000);
        snmw(0x13e1_01e0, 0x8000_ffe0);
        snmw(0x13e1_01e4, 0x0000_0000);
        snmw(0x13e1_01e8, 0x0000_0000);
        snmw(0x13e1_01ec, 0x0000_0000);
        snmw(0x13e1_01f0, 0x0000_0000);
        snmw(0x13e1_01f4, 0x8000_dfc0);
        snmw(0x13e1_01f8, 0x0000_0000);
        snmw(0x13e1_01fc, 0x0000_0000);
        snmw(0x13e1_0200, 0x0000_0000);
        snmw(0x13e1_0204, 0x0000_0000);
        snmw(0x13e1_0208, 0x8000_bfa0);
        snmw(0x13e1_020c, 0x0000_0000);
        snmw(0x13e1_0210, 0x0000_0000);
        snmw(0x13e1_0214, 0x0000_0000);
        snmw(0x13e1_021c, 0x8000_9f80);

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
