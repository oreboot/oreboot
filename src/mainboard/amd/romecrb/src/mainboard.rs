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

use arch::ioport::IOPort;
use clock::ClockNode;
use core::ops::BitAnd;
use core::ops::BitOr;
use core::ops::Not;
use core::ptr;
use model::*;
use smn::smn_write;
use uart::amdmmio::UART;
use uart::debug_port::DebugPort;
use uart::i8250::I8250;
use vcell::VolatileCell;
use x86_64::registers::model_specific::Msr;

const SMB_UART_CONFIG: *const VolatileCell<u32> = 0xfed8_00fc as *const _;
const SMB_UART_1_8M_SHIFT: u8 = 28;
const SMB_UART_CONFIG_UART0_1_8M: u32 = 1 << SMB_UART_1_8M_SHIFT;
const SMB_UART_CONFIG_UART1_1_8M: u32 = 1 << (SMB_UART_1_8M_SHIFT + 1);

// Nothing from these three lines is documented anywhere that we can find.
// FCH_UART_LEGACY_DECODE is in coreboot.
// Some idea of the proper values was obtained using the u-root io command,
// coupled with dumping ACPI to get some idea what the values might mean.
// The rest is further interpolation.
const FCH_UART_LEGACY_DECODE: *const VolatileCell<u16> = 0xfedc_0020 as *const _;
// ACPI calls this IER3, which we assume means Io Enable Redirect 3?
const FCH_LEGACY_3F8_SH: u16 = 1 << 3;
// ACPI calls this WUR3, which means ... ?
// This is two-bit field for bits 15:14.
const FCH_WUR3: u16 = 1 << 14;
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
    com1: I8250<IOPort>,
    debug: DebugPort<IOPort>,
    uart0: UART,
}

impl MainBoard {
    pub fn new() -> MainBoard {
        Self { com1: I8250::new(0x3f8, 0, IOPort {}), debug: DebugPort::new(0x80, IOPort {}), uart0: UART::uart0() }
    }
    pub fn text_output_drivers(&mut self) -> [&mut dyn Driver; 3] {
        [&mut self.com1, &mut self.debug, &mut self.uart0]
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
            (*FCH_UART_LEGACY_DECODE).set(FCH_LEGACY_3F8_SH | FCH_WUR3);
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
            pokers32(0xfed00010 as *mut u32, 0, 3); //HPETCONFIG

            // THis is likely not needed but.
            //poke32(0xfed00108, 0x5b03d997);

            // enable ioapic redirection
            // IOHC::IOAPIC_BASE_ADDR_LO
            smn_write(0x13B1_02f0, 0xFEC0_0001);

            for driver in self.text_output_drivers().iter_mut() {
                driver.init()?;
            }

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
