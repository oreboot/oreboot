/* Copyright (c) 2018 SiFive, Inc */
/* SPDX-License-Identifier: Apache-2.0 */
/* SPDX-License-Identifier: GPL-2.0-or-later */
/* See the file LICENSE for further information */

use super::ddrregs;
use super::reg;
use core::ptr;

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
// index is a word offset.
fn poke(pointer: u32, index: u32, value: u32) {
    let addr = (pointer + (index << 2)) as *mut u32;
    unsafe {
        ptr::write_volatile(addr, value);
    }
}

fn poke64(pointer: u32, index: u32, value: u64) {
    let addr = (pointer + (index << 2)) as *mut u64;
    //let addr1 = (pointer + (index << 2) + 4) as *mut u32;
    unsafe {
        ptr::write_volatile(addr, value);
        //let v2: u32 = (value) as u32;
        //ptr::write_volatile(addr1, v2);
        //let v1: u32 = (value >> 32) as u32;
        //ptr::write_volatile(addr, v1);
    }
}

fn set(pointer: u32, index: u32, value: u32) {
    let v = peek(pointer, index);
    poke(pointer, index, v | value);
}

fn clr(pointer: u32, index: u32, value: u32) {
    let v = peek(pointer, index);
    poke(pointer, index, v & value);
}

fn peek(pointer: u32, index: u32) -> u32 {
    let addr = (pointer + (index << 2)) as *const u32;
    unsafe { ptr::read_volatile(addr) }
}

// #define _REG32((reg::DDR_CTRL,p, i) (*(volatile u32 *)((p) + (i)))

pub fn phy_reset() {
    for i in 1152..=1214 {
        poke(reg::DDR_PHY, i as u32, ddrregs::DENALI_PHY_DATA[i]);
        //u32 physet = DENALI_PHY_DATA[i];
        // /*if (physet!=0)*/ DDR_PHY[i] = physet;
    }
    for i in 0..=1151 {
        poke(reg::DDR_PHY, i as u32, ddrregs::DENALI_PHY_DATA[i]);
        //for (i=0;i<=1151;i++) {
        //    u32 physet = DENALI_PHY_DATA[i];
        //if (physet!=0)*/ DDR_PHY[i] = physet;
    }
}

pub fn ux00ddr_writeregmap() {
    for i in 0..=264 {
        //  for (i=0;i<=264;i++) {
        poke(reg::DDR_CTRL, i as u32, ddrregs::DENALI_CTL_DATA[i]);
        // u32 ctlset = DENALI_CTL_DATA[i];
        // /*if (ctlset!=0)*/ DDR_CTRL[i] = ctlset;
    }

    phy_reset();
}

pub fn ux00ddr_start(filteraddr: u64, ddrend: u64) {
    //   // START register at ddrctl register base offset 0
    let regdata = peek(reg::DDR_CTRL, 0) | 1;
    poke(reg::DDR_CTRL, 0, regdata);
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
    let freg = filteraddr as u32;
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
    set(
        reg::DDR_CTRL,
        136,
        (1 << OUT_OF_RANGE_OFFSET) | (1 << MULTIPLE_OUT_OF_RANGE_OFFSET),
    );
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
    let end_addr_16k_blocks: u32 = (((end_addr >> 14) & 0x7FFFFF) - 1) as u32;
    poke(reg::DDR_CTRL, 210, end_addr_16k_blocks);
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
    set(
        reg::DDR_CTRL,
        170,
        (1 << WRLVL_EN_OFFSET) | (1 << DFI_PHY_WRLELV_MODE_OFFSET),
    );
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

    //let mut fails: u64 = 0;
    let mut slicebase: u32 = 0;
    //let mut dq: u32 = 0;
    for _ in 0..8 {
        // check errata condition
        let regbase: u32 = slicebase + 34;
        for reg in 0..4 {
            // what the hell?
            let updownreg: u32 = peek(ddrphyreg, regbase + reg);
            for bit in 0..2 {
                let phy_rx_cal_dqn_0_offset = if bit == 0 {
                    PHY_RX_CAL_DQ0_0_OFFSET
                } else {
                    PHY_RX_CAL_DQ1_0_OFFSET
                };
                let down: u32 = (updownreg >> phy_rx_cal_dqn_0_offset) & 0x3F;
                let up: u32 = (updownreg >> (phy_rx_cal_dqn_0_offset + 6)) & 0x3F;
                let failc0: bool = (down == 0) && (up == 0x3F);
                let failc1: bool = (up == 0) && (down == 0x3F);

                // print error message on failure
                if failc0 || failc1 {
                    // good news: we can use fmt; skip this nonsense.
                    //if (fails==0) uart_puts((void*) UART0_CTRL_ADDR, "DDR error in fixing up \n");
                    //fails |= (1 << dq);
                    //let mut slicelsc: u8 = 48;
                    //let mut slicemsc: u8 = 48;
                    //slicelsc += (dq % 10) as u8;
                    //slicemsc += (dq / 10) as u8;
                    //uart_puts((void*) UART0_CTRL_ADDR, "S ");
                    //uart_puts((void*) UART0_CTRL_ADDR, &slicemsc);
                    //uart_puts((void*) UART0_CTRL_ADDR, &slicelsc);
                    //if (failc0) uart_puts((void*) UART0_CTRL_ADDR, "U");
                    //else uart_puts((void*) UART0_CTRL_ADDR, "D");
                    //uart_puts((void*) UART0_CTRL_ADDR, "\n");
                }
                //dq += 1;
            }
        }
        slicebase += 128;
    }
    0
}
