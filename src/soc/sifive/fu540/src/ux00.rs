/* Copyright (c) 2018 SiFive, Inc */
/* SPDX-License-Identifier: Apache-2.0 */
/* SPDX-License-Identifier: GPL-2.0-or-later */
/* See the file LICENSE for further information */

use crate::ddrregs;
use crate::reg;
use core::ptr;
use core::fmt;
use model::Driver;
use uart::sifive::SiFive;

pub struct WriteTo<'a> {
    drv: &'a mut dyn Driver,
}

impl<'a> WriteTo<'a> {
    pub fn new(drv: &'a mut dyn Driver) -> Self {
        WriteTo { drv: drv }
    }
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.drv.pwrite(s.as_bytes(), 0) {
            Err(_) => Err(fmt::Error),
            _ => Ok(()),
        }
    }
}

// #define _REG32((reg:DDR_CTRL,p, i) (*(volatile u32 *)((p) + (i)))
pub const DRAM_CLASS_OFFSET: u32 = 8;
pub const DRAM_CLASS_DDR4: u32 = 0xA;
pub const OPTIMAL_RMODW_EN_OFFSET: u32 = 0;
pub const DISABLE_RD_INTERLEAVE_OFFSET: u32 = 16;
pub const OUT_OF_RANGE_OFFSET: u32 = 1;
pub const MULTIPLE_OUT_OF_RANGE_OFFSET: u32 = 2;
pub const PORT_COMMAND_CHANNEL_ERROR_OFFSET: u32 = 7;
pub const MC_INIT_COMPLETE_OFFSET: u32 = 8;
pub const LEVELING_OPERATION_COMPLETED_OFFSET: u32 = 22;
pub const DFI_PHY_WRLELV_MODE_OFFSET: u32 = 24;
pub const DFI_PHY_RDLVL_MODE_OFFSET: u32 = 24;
pub const DFI_PHY_RDLVL_GATE_MODE_OFFSET: u32 = 0;
pub const VREF_EN_OFFSET: u32 = 24;
pub const PORT_ADDR_PROTECTION_EN_OFFSET: u32 = 0;
pub const AXI0_ADDRESS_RANGE_ENABLE: u32 = 8;
pub const AXI0_RANGE_PROT_BITS_0_OFFSET: u32 = 24;
pub const RDLVL_EN_OFFSET: u32 = 16;
pub const RDLVL_GATE_EN_OFFSET: u32 = 24;
pub const WRLVL_EN_OFFSET: u32 = 0;

pub const PHY_RX_CAL_DQ0_0_OFFSET: u64 = 0;
pub const PHY_RX_CAL_DQ1_0_OFFSET: u64 = 16;

// This is nasty but at the same time, the way the code is written, we need to
// make this change slowly.

// This is a 64-bit machine but all this action seems to be on 32-bit values.
// No idea why this is.
// Index is a word offset.
fn poke(Pointer: u32, Index: u32, Value: u32) -> () {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    let w = &mut WriteTo::new(uart0);
    fmt::write(w, format_args!("poke {:x} {:x} {:x}\r\n", Pointer, Index, Value)).unwrap();

    let addr = (Pointer + (Index << 2)) as *mut u32;
    unsafe {
        ptr::write_volatile(addr, Value);
    }
    fmt::write(w, format_args!("done\r\n")).unwrap();
}

fn poke64(Pointer: u32, Index: u32, Value: u64) -> () {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    let w = &mut WriteTo::new(uart0);
    fmt::write(w, format_args!("poke64 {:x} {:x} {:x}\r\n", Pointer, Index, Value)).unwrap();

    let addr = (Pointer + (Index << 2)) as *mut u32;
    let addr1 = (Pointer + (Index << 2) + 4) as *mut u32;
    unsafe {
        let v1: u32 = (Value >> 32) as u32;
        ptr::write_volatile(addr, v1);
        let v2: u32 = (Value) as u32;
        ptr::write_volatile(addr1, v2);
    }
    fmt::write(w, format_args!("done\r\n")).unwrap();
}

fn set(Pointer: u32, Index: u32, Value: u32) -> () {
    let v = peek(Pointer, Index);
    poke(Pointer, Index, v | Value);
}

fn clr(Pointer: u32, Index: u32, Value: u32) -> () {
    let v = peek(Pointer, Index);
    poke(Pointer, Index, v & Value);
}

fn peek(Pointer: u32, Index: u32) -> u32 {
    let uart0 = &mut SiFive::new(/*soc::UART0*/ 0x10010000, 115200);
    let w = &mut WriteTo::new(uart0);
    fmt::write(w, format_args!("peek {:x} {:x}\r\n", Pointer, Index)).unwrap();

    let addr = (Pointer + (Index << 2)) as *const u32;
    let v = unsafe { ptr::read_volatile(addr) };
    fmt::write(w, format_args!("done {:x}\r\n", v)).unwrap();
    v
}

// #define _REG32((reg::DDR_CTRL,p, i) (*(volatile u32 *)((p) + (i)))

pub fn phy_reset() {
    for i in 1152..=1214 {
        poke(reg::DDR_PHY, i as u32, ddrregs::DenaliPhyData[i]);
        //u32 physet = DenaliPhyData[i];
        // /*if (physet!=0)*/ DDR_PHY[i] = physet;
    }
    for i in 0..=1151 {
        poke(reg::DDR_PHY, i as u32, ddrregs::DenaliPhyData[i]);
        //for (i=0;i<=1151;i++) {
        //    u32 physet = DenaliPhyData[i];
        //if (physet!=0)*/ DDR_PHY[i] = physet;
    }
}

pub fn ux00ddr_writeregmap() {
    for i in 0..=264 {
        //  for (i=0;i<=264;i++) {
        poke(reg::DDR_CTRL, i as u32, ddrregs::DenaliCtlData[i]);
        // u32 ctlset = DenaliCtlData[i];
        // /*if (ctlset!=0)*/ DDR_CTRL[i] = ctlset;
    }

    phy_reset();
}

pub fn ux00ddr_start(filteraddr: u64, ddrend: u64) {
    //   // START register at ddrctl register base offset 0
    let regdata = peek(reg::DDR_CTRL, 0) | 1;
    poke(reg::DDR_CTRL, 0, regdata);
    //peek(reg::DDR_CTRL, 0);
    // WAIT for initialization complete : bit 8 of INT_STATUS (DENALI_CTL_132) 0x210
    // 132 * 4
    loop {
        let r = peek(reg::DDR_CTRL, 132);
        let m = 1 << MC_INIT_COMPLETE_OFFSET;
        if (r & m) != 0 {
            break;
        }
    }

    // Disable the BusBlocker in front of the controller AXI slave ports
    let freg = peek(filteraddr as u32, 0);
    poke64(freg, 0, (0x0f00000000000000 | (ddrend >> 2)) as u64);
    //         volatile u64 *filterreg = (volatile uint64_t *)filteraddr;
    //   filterreg[0] = 0x0f00000000000000UL | (ddrend >> 2);
    //   //                ^^ RWX + TOR
}

pub fn ux00ddr_mask_mc_init_complete_interrupt() {
    // Mask off Bit 8 of Interrupt Status
    // Bit [8] The MC initialization has been completed
    set(reg::DDR_CTRL, 136, 1 << MC_INIT_COMPLETE_OFFSET);
}

pub fn ux00ddr_mask_outofrange_interrupts() {
    // Mask off Bit 8, Bit 2 and Bit 1 of Interrupt Status
    // Bit [2] Multiple accesses outside the defined PHYSICAL memory space have occured
    // Bit [1] A memory access outside the defined PHYSICAL memory space has occured
    set(reg::DDR_CTRL, 136, (1 << OUT_OF_RANGE_OFFSET) | (1 << MULTIPLE_OUT_OF_RANGE_OFFSET));
}

pub fn ux00ddr_mask_port_command_error_interrupt() {
    // Mask off Bit 7 of Interrupt Status
    // Bit [7] An error occured on the port command channel
    set(reg::DDR_CTRL, 136, 1 << PORT_COMMAND_CHANNEL_ERROR_OFFSET);
}

pub fn ux00ddr_mask_leveling_completed_interrupt() {
    // Mask off Bit 22 of Interrupt Status
    // Bit [22] The leveling operation has completed
    set(reg::DDR_CTRL, 136, 1 << LEVELING_OPERATION_COMPLETED_OFFSET);
}

pub fn ux00ddr_setuprangeprotection(end_addr: u64) {
    poke(reg::DDR_CTRL, 209, 0x0);
    let end_addr_16Kblocks: u32 = (((end_addr >> 14) & 0x7FFFFF) - 1) as u32;
    poke(reg::DDR_CTRL, 210, end_addr_16Kblocks);
    poke(reg::DDR_CTRL, 212, 0x0);
    poke(reg::DDR_CTRL, 214, 0x0);
    poke(reg::DDR_CTRL, 216, 0x0);
    set(reg::DDR_CTRL, 224, 0x3 << AXI0_RANGE_PROT_BITS_0_OFFSET);
    poke(reg::DDR_CTRL, 225, 0xFFFFFFFF);
    set(reg::DDR_CTRL, 208, 1 << AXI0_ADDRESS_RANGE_ENABLE);
    set(reg::DDR_CTRL, 208, 1 << PORT_ADDR_PROTECTION_EN_OFFSET);
}

pub fn ux00ddr_disableaxireadinterleave() {
    set(reg::DDR_CTRL, 120, 1 << DISABLE_RD_INTERLEAVE_OFFSET);
}

pub fn ux00ddr_disableoptimalrmodw() {
    clr(reg::DDR_CTRL, 21, !(1 << OPTIMAL_RMODW_EN_OFFSET));
}

pub fn ux00ddr_enablewriteleveling() {
    set(reg::DDR_CTRL, 170, (1 << WRLVL_EN_OFFSET) | (1 << DFI_PHY_WRLELV_MODE_OFFSET));
}

pub fn ux00ddr_enablereadleveling() {
    set(reg::DDR_CTRL, 181, 1 << DFI_PHY_RDLVL_MODE_OFFSET);
    set(reg::DDR_CTRL, 260, 1 << RDLVL_EN_OFFSET);
}

pub fn ux00ddr_enablereadlevelinggate() {
    set(reg::DDR_CTRL, 260, 1 << RDLVL_GATE_EN_OFFSET);
    set(reg::DDR_CTRL, 182, 1 << DFI_PHY_RDLVL_GATE_MODE_OFFSET);
}

pub fn ux00ddr_enablevreftraining() {
    set(reg::DDR_CTRL, 184, 1 << VREF_EN_OFFSET);
}

pub fn ux00ddr_getdramclass() -> u32 {
    (peek(reg::DDR_CTRL, 0) >> DRAM_CLASS_OFFSET) & 0xF
}

// TODO: scrub this mess. coreboot had some bugs.
pub fn ux00ddr_phy_fixup() -> u64 {
    // return bitmask of failed lanes

    let ddrphyreg: u32 = reg::DDR_CTRL + 0x2000;

    let mut fails: u64 = 0;
    let mut slicebase: u32 = 0;
    let mut dq: u32 = 0;
    for slice in 0..8 {
        // check errata condition
        let regbase: u32 = slicebase;
        for reg in 0..4 {
            // what the hell?
            let updownreg: u32 = peek((regbase + reg) << 2, ddrphyreg);
            for bit in 0..2 {
                let mut phy_rx_cal_dqn_0_offset: u64 = 0;
                if bit == 0 {
                    phy_rx_cal_dqn_0_offset = PHY_RX_CAL_DQ0_0_OFFSET;
                } else {
                    phy_rx_cal_dqn_0_offset = PHY_RX_CAL_DQ1_0_OFFSET;
                }
                let down: u32 = (updownreg >> phy_rx_cal_dqn_0_offset) & 0x3F;
                let up: u32 = (updownreg >> (phy_rx_cal_dqn_0_offset + 6)) & 0x3F;
                let failc0: bool = ((down == 0) && (up == 0x3F));
                let failc1: bool = ((up == 0) && (down == 0x3F));

                // print error message on failure
                if failc0 || failc1 {
                    // good news: we can use fmt; skip this nonsense.
                    //if (fails==0) uart_puts((void*) UART0_CTRL_ADDR, "DDR error in fixing up \n");
                    fails |= (1 << dq);
                    let mut slicelsc: u8 = 48;
                    let mut slicemsc: u8 = 48;
                    slicelsc += (dq % 10) as u8;
                    slicemsc += (dq / 10) as u8;
                    //uart_puts((void*) UART0_CTRL_ADDR, "S ");
                    //uart_puts((void*) UART0_CTRL_ADDR, &slicemsc);
                    //uart_puts((void*) UART0_CTRL_ADDR, &slicelsc);
                    //if (failc0) uart_puts((void*) UART0_CTRL_ADDR, "U");
                    //else uart_puts((void*) UART0_CTRL_ADDR, "D");
                    //uart_puts((void*) UART0_CTRL_ADDR, "\n");
                }
                dq = dq + 1;
            }
        }
        slicebase += 128;
    }
    0
}
