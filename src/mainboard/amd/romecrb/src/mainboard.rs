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

use uart::amdmmio::AMDMMIO;
use uart::debug_port::DebugPort;

use crate::c00::c00;
use crate::msr::msrs;
use arch::ioport::IOPort;
use clock::ClockNode;
use core::ops::BitAnd;
use core::ops::BitOr;
use core::ops::Not;
use core::ptr;
use model::*;
use smn::{smn_read, smn_write};
use uart::i8250::I8250;
use vcell::VolatileCell;
use x86_64::registers::model_specific::Msr;

const SMB_UART_CONFIG: *const VolatileCell<u32> = 0xfed8_00fc as *const _;
const SMB_UART_1_8M_SHIFT: u8 = 28;
const SMB_UART_CONFIG_UART0_1_8M: u32 = 1 << SMB_UART_1_8M_SHIFT;
const SMB_UART_CONFIG_UART1_1_8M: u32 = 1 << (SMB_UART_1_8M_SHIFT + 1);

const FCH_UART_LEGACY_DECODE: *const VolatileCell<u16> = 0xfedc_0020 as *const _;
const FCH_LEGACY_3F8_SH: u16 = 3;
//const FCH_LEGACY_2F8_SH: u16 = 1 << 1;

// See coreboot:src/soc/amd/common/block/include/amdblocks/acpimmio_map.h

const ACPI_MMIO_BASE: usize = 0xfed8_0000;
//const IOMUX_BASE: *mut u8 = (ACPI_MMIO_BASE + 0xd00) as *mut _;
const IOMUX_BASE: *const [VolatileCell<u8>; 256] = (ACPI_MMIO_BASE + 0xd00) as *const _; // Note: 256 u8 block--one u8 per pinctrl
const AOAC_BASE: usize = ACPI_MMIO_BASE + 0x1e00;

// See coreboot:src/soc/amd/common/block/include/amdblocks/aoac.h

const FCH_AOACx40_D3_CONTROL: *const [VolatileCell<u8>; 64] = (AOAC_BASE + 0x40) as *const _; // Note: 32 times (control: u8, status: u8) block
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

unsafe fn poke<T>(a: *mut T, v: T) -> () {
    ptr::write_volatile(a, v);
}

// Read T from VolatileCell A, reset bits from RESET_MASK, set bits from SET_MASK, write result back to address A
unsafe fn pokers32(a: *mut u32, reset_mask: u32, set_mask: u32) -> () {
    ptr::write_volatile(a, (ptr::read_volatile(a) & !reset_mask) | set_mask);
}

unsafe fn pokers<T>(a: *const VolatileCell<T>, reset_mask: T, set_mask: T) -> ()
where
    T: Copy + Not<Output = T> + BitAnd<Output = T> + BitOr<Output = T>,
{
    (*a).set(((*a).get() & !reset_mask) | set_mask);
}

// WIP: mainboard driver. I mean the concept is a WIP.
pub struct MainBoard {
    //debug_io: IOPort,
    debug: DebugPort,
    //uart0_io: IOPort,
    uart0: I8250,
    p0: AMDMMIO,
    //pub text_outputs: [&'a mut dyn Driver; 3],
}

impl MainBoard {
    pub fn new() -> Self {
        let mut result = Self { uart0: I8250::new(0x3f8, 0, IOPort {}), debug: DebugPort::new(0x80, IOPort {}), p0: AMDMMIO::com2() };
        result
    }
    pub fn text_outputs(&self) -> [&mut dyn Driver; 3] {
        [&mut self.uart0, &mut self.debug, &mut self.p0]
    }
}

fn d3_control_power_on(n: u8) -> () {
    if n < 32 {
        unsafe {
            pokers(&(*FCH_AOACx40_D3_CONTROL)[usize::from(n * 2)], 0, AOAC_PWR_ON_DEV);
        }
    }
}

fn pinctrl(pin: u8, mode: u8) -> () {
    // IOMUX (IOMUX total: 256 registers; one u8 per GPIO)
    unsafe {
        pokers(&(*IOMUX_BASE)[usize::from(pin)], 3, mode);
    }
}

fn smnhack(w: &mut dyn core::fmt::Write, reg: u32, want: u32) -> () {
    let got = smn_read(reg);
    write!(w, "{:x}: got {:x}, want {:x}\r\n", reg, got, want).unwrap();
    if got == want {
        return;
    }
    smn_write(reg, want);
    let got = smn_read(reg);
    write!(w, "Try 2: {:x}: got {:x}, want {:x}\r\n", reg, got, want).unwrap();
}

impl Driver for MainBoard {
    fn init(&mut self) -> Result<()> {
        unsafe {
            // FCH PM DECODE EN
            poke(0xfed80300 as *mut u32, 0xe3070b77);

            // Knowledge from coreboot to get minimal serials working.
            // clock defaults are NOT fine.
            //uart_ctrl = sm_pci_read32(SMB_UART_CONFIG);
            //uart_ctrl |= 1 << (SMB_UART_1_8M_SHIFT + idx);
            //sm_pci_write32(SMB_UART_CONFIG, uart_ctrl);
            // we want 1.8m. They made oddball 48m default. Stupid.
            pokers(SMB_UART_CONFIG, 0, SMB_UART_CONFIG_UART0_1_8M | SMB_UART_CONFIG_UART1_1_8M);

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
            (*FCH_UART_LEGACY_DECODE).set(1 << FCH_LEGACY_3F8_SH);
            let mut msr0 = Msr::new(0x1b);
            /*unsafe*/
            {
                let v = msr0.read() | 0x900;
                msr0.write(v);
                //let v = msr.read() | 0xd00;
                //write!(w, "NOT ENABLING x2apic!!!\n\r");
                //msr.write(v);
            }
            // IOAPIC
            //     wmem fed80300 e3070b77
            //    wmem fed00010 3
            poke(0xfed00010 as *mut u32, 3); //HPETCONFIG
            pokers32(0xfed00010 as *mut u32, 0, 8);
            // THis is likely not needed but.
            //poke32(0xfed00108, 0x5b03d997);

            // enable ioapic redirection
            // IOHC::IOAPIC_BASE_ADDR_LO
            smn_write(0x13B1_02f0, 0xFEC0_0001);

            self.uart0.init().unwrap();
            self.uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
            self.debug.init().unwrap();
            self.debug.pwrite(b"Welcome to oreboot - debug port 80\r\n", 0).unwrap();
            self.p0.init().unwrap();
            self.p0.pwrite(b"Welcome to oreboot - com2\r\n", 0).unwrap();
            //self.text_outputs = [self.debug as &mut dyn Driver, uart0 as &mut dyn Driver, p0 as &mut dyn Driver];
            //let mut p: [u8; 1] = [0xf0; 1];
            //post.pwrite(&p, 0x80).unwrap();

            let r: &mut dyn Driver = self.text_outputs()[0];
            let w = &mut print::WriteTo::new(r);

            // Logging.
            smnhack(w, 0x13B1_02F4, 0x00000000u32);
            /* FIXME
                smnhack(w, 0x13B1_02F0, 0xc9280001u32);
                smnhack(w, 0x13C1_02F4, 0x00000000u32);
                smnhack(w, 0x13C1_02F0, 0xf4180001u32);
                smnhack(w, 0x13D1_02F4, 0x00000000u32);
                smnhack(w, 0x13D1_02F0, 0xc8180001u32);
                smnhack(w, 0x13E1_02F4, 0x00000000u32);
                smnhack(w, 0x13E1_02F0, 0xf5180001u32);
                // IOMMU on    smnhack(w, 0x13F0_0044, 0xc9200001u32);
                smnhack(w, 0x13F0_0044, 0xc9200000u32);
                smnhack(w, 0x13F0_0048, 0x00000000u32);
                // IOMMU on smnhack(w, 0x1400_0044, 0xf4100001u32);
                smnhack(w, 0x1400_0044, 0xf4100000u32);
                smnhack(w, 0x1400_0048, 0x00000000u32);
                // IOMMU on smnhack(w, 0x1410_0044, 0xc8100001u32);
                smnhack(w, 0x1410_0044, 0xc8100000u32);
                smnhack(w, 0x1410_0048, 0x00000000u32);
                // IOMMU on smnhack(w, 0x1420_0044, 0xf5100001u32);
                smnhack(w, 0x1420_0044, 0xf5100000u32);
                smnhack(w, 0x1420_0048, 0x00000000u32);
                smnhack(w, 0x1094_2014, 0x00000000u32);
                smnhack(w, 0x1094_2010, 0x0000000cu32);
                smnhack(w, 0x10A4_2014, 0x00000000u32);
                smnhack(w, 0x10A4_2010, 0x0000000cu32);
                smnhack(w, 0x1074_1014, 0x00000000u32);
                smnhack(w, 0x10A4_2010, 0x0000000cu32);
                smnhack(w, 0x1074_1014, 0x00000000u32);
                smnhack(w, 0x1074_1010, 0x00000000u32);
                smnhack(w, 0x1074_2014, 0x00000000u32);
                smnhack(w, 0x1074_2010, 0x00000000u32);
                smnhack(w, 0x1074_3014, 0x00000000u32);
                smnhack(w, 0x1074_3010, 0xc6000004u32);
                smnhack(w, 0x1074_4014, 0x00000000u32);
                smnhack(w, 0x1074_4010, 0x00000000u32);
                smnhack(w, 0x10B4_2014, 0x00000000u32);
                smnhack(w, 0x10B4_2010, 0x0000000cu32);
                smnhack(w, 0x1084_3014, 0x00000000u32);
                smnhack(w, 0x1084_3010, 0xf8000004u32);
                smnhack(w, 0x10C4_2014, 0x00000000u32);
                smnhack(w, 0x10C4_2010, 0x0000000cu32);
                smnhack(w, 0x13B1_0044, 0x00000160u32);
                smnhack(w, 0x13C1_0044, 0x00000140u32);
                smnhack(w, 0x13D1_0044, 0x00000120u32);
                smnhack(w, 0x13E1_0044, 0x00000100u32);
                smnhack(w, 0x1010_0018, 0x00636360u32);
                smnhack(w, 0x1050_0018, 0x00646460u32);
                smnhack(w, 0x1020_0018, 0x00414140u32);
                smnhack(w, 0x1060_0018, 0x00424240u32);
                smnhack(w, 0x1060_1018, 0x00000000u32);
                smnhack(w, 0x1060_2018, 0x00000000u32);
                smnhack(w, 0x1030_0018, 0x00212120u32);
                smnhack(w, 0x1070_0018, 0x00222220u32);
                smnhack(w, 0x1070_1018, 0x00000000u32);
                smnhack(w, 0x1070_2018, 0x00000000u32);
                smnhack(w, 0x1040_0018, 0x00020200u32);
                smnhack(w, 0x1080_0018, 0x00030300u32);
                smnhack(w, 0x1090_0018, 0x00000000u32);
                smnhack(w, 0x10A0_0018, 0x00000000u32);
                smnhack(w, 0x10B0_0018, 0x00000000u32);
                smnhack(w, 0x10C0_0018, 0x00000000u32);
                smnhack(w, 0x1110_0018, 0x00000000u32);
                smnhack(w, 0x1120_0018, 0x00000000u32);
                smnhack(w, 0x1130_0018, 0x00000000u32);
                smnhack(w, 0x1120_0018, 0x00000000u32);
                smnhack(w, 0x1130_0018, 0x00000000u32);
                smnhack(w, 0x1140_0018, 0x00010100u32);
                smnhack(w, 0x1110_1018, 0x00000000u32);
                smnhack(w, 0x1120_1018, 0x00000000u32);
                smnhack(w, 0x1130_1018, 0x00000000u32);
                smnhack(w, 0x1140_1018, 0x00000000u32);
                smnhack(w, 0x1110_2018, 0x00000000u32);
                smnhack(w, 0x1120_2018, 0x00000000u32);
                smnhack(w, 0x1130_2018, 0x00000000u32);
                smnhack(w, 0x1140_2018, 0x00000000u32);
                smnhack(w, 0x1110_3018, 0x00000000u32);
                smnhack(w, 0x1120_3018, 0x00000000u32);
                smnhack(w, 0x1130_3018, 0x00000000u32);
                smnhack(w, 0x1140_3018, 0x00000000u32);
                smnhack(w, 0x1110_4018, 0x00000000u32);
                smnhack(w, 0x1120_4018, 0x00000000u32);
                smnhack(w, 0x1130_4018, 0x00000000u32);
                smnhack(w, 0x1140_4018, 0x00000000u32);
                smnhack(w, 0x1110_5018, 0x00000000u32);
                smnhack(w, 0x1120_5018, 0x00000000u32);
                smnhack(w, 0x1130_5018, 0x00000000u32);
                smnhack(w, 0x1140_5018, 0x00000000u32);
                smnhack(w, 0x1110_6018, 0x00000000u32);
                smnhack(w, 0x1120_6018, 0x00000000u32);
                smnhack(w, 0x1130_6018, 0x00000000u32);
                smnhack(w, 0x1140_6018, 0x00000000u32);
                smnhack(w, 0x1110_7018, 0x00000000u32);
                smnhack(w, 0x1120_7018, 0x00000000u32);
                smnhack(w, 0x1130_7018, 0x00000000u32);
                smnhack(w, 0x1140_7018, 0x00000000u32);

                // end logging
                smnhack(w, 0x13b1_0030, 0x00000001u32 << 11);
                smnhack(w, 0x13c1_0030, 0x00000001u32 << 11);
                smnhack(w, 0x13d1_0030, 0x00000001u32 << 11);
                smnhack(w, 0x13e1_0030, 0x00000001u32 << 11);
                smnhack(w, 0x13e1_0030, 0x00000001u32 << 11);

                // PCIE crs count
                smnhack(w, 0x13b1_0028, 0x02620006u32);
                smnhack(w, 0x13c1_0028, 0x02620006u32);
                smnhack(w, 0x13d1_0028, 0x02620006u32);
                smnhack(w, 0x13e1_0028, 0x02620006u32);

                // PCIE 100 mhz
                smnhack(w, 0x13b1_0020, 0x00000001u32);
                smnhack(w, 0x13c1_0020, 0x00000001u32);
                smnhack(w, 0x13d1_0020, 0x00000001u32);
                smnhack(w, 0x13e1_0020, 0x00000001u32);

                // lovely bridges
                smnhack(w, 0x13b3_C004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13b3_9804, 0x00040007 | 0x100u32);
                smnhack(w, 0x13b3_9404, 0x00040007 | 0x100u32);
                smnhack(w, 0x13b3_9004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13b3_8004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13b3_5404, 0x00040000 | 0x100u32);
                smnhack(w, 0x13b3_5004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_4C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_4804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_4404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_4004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_3C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_3804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_3404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_3004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_2C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_2804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_2404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_2004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_1C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_1804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_1404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_1004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_1404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13b3_1004, 0x00040005 | 0x100u32);

                smnhack(w, 0x13c3_C004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13c3_9804, 0x00040007 | 0x100u32);
                smnhack(w, 0x13c3_9404, 0x00040007 | 0x100u32);
                smnhack(w, 0x13c3_9004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13c3_8004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13c3_5404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_5004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_4C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_4804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_4404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_4004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_3C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_3804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_3404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_3004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_2C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_2804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_2404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_2004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_1C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_1804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_1404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13c3_1004, 0x00040005 | 0x100u32);

                smnhack(w, 0x13d3_C004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13d3_9804, 0x00040007 | 0x100u32);
                smnhack(w, 0x13d3_9404, 0x00040007 | 0x100u32);
                smnhack(w, 0x13d3_9004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13d3_8004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13d3_5404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_5004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_4C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_4804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_4404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_4004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_4404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_4004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_3C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_3804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_3404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_3004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_2C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_2804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_2404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_2004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_1C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_1804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_1404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13d3_1004, 0x00040005 | 0x100u32);

                smnhack(w, 0x13e3_C004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13e3_9804, 0x00040007 | 0x100u32);
                smnhack(w, 0x13e3_9404, 0x00040007 | 0x100u32);
                smnhack(w, 0x13e3_9004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13e3_8004, 0x00040000 | 0x100u32);
                smnhack(w, 0x13e3_5404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_5004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_4C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_4804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_4404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_4004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_3C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_3804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_3404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_3004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_2C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_2804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_2404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_2004, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_1C04, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_1804, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_1404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_1004, 0x00040001 | 0x100u32);
                smnhack(w, 0x13e3_1404, 0x00040005 | 0x100u32);
                smnhack(w, 0x13e3_1004, 0x00040001 | 0x100u32);
            */

            if true {
                msrs(w);
            }
            c00(w);

            Ok(())
        }
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
