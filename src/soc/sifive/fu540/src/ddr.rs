use core::ops;
use model::*;

use crate::is_qemu;
use crate::reg;
use crate::ux00;
use core::convert::TryInto;
use register::mmio::ReadWrite;
use register::register_bitfields;

#[repr(C)]

pub struct BlockerRegister {
    blocker: ReadWrite<u64, Blocker::Register>,
}

// so what I'd really like to do, given that we can have some control over deref,
// is have this be 5 or so u32 and then, on deref, compute the correct address
// and use it. But one war at a time. That said, counting offsets is pretty 1979.
// For now, we won't really use this. We have working coreboot code and we'll transition
// one ugly bit at a time. DDR is very sensitive to simple errors.
pub struct RegisterBlock {
    _cr0: ReadWrite<u64, CR0::Register>,
    _2: [u32; 18],
    _cr19: ReadWrite<u64, CR19::Register>,
    _3: [u32; 1],
    _cr21: ReadWrite<u64, CR21::Register>,
    _4: [u32; 98],
    _cr120: ReadWrite<u64, CR120::Register>,
    _5: [u32; 11],
    _cr132: ReadWrite<u64, CR132::Register>,
    _6: [u32; 3],
    _cr136: ReadWrite<u64, CR136::Register>,
    _7: [u32; 0x800 - 127],
}

pub struct DDR {
    base: usize,
}

impl ops::Deref for DDR {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl DDR {
    pub fn new() -> DDR {
        DDR { base: reg::DDR_CTRL as usize }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Default for DDR {
    fn default() -> Self {
        DDR::new()
    }
}

impl Driver for DDR {
    fn init(&mut self) -> Result<()> {
        /* nothing to do. */
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        match data {
            b"on" => {
                sdram_init();
                Ok(mem_size().try_into().unwrap())
            }
            _ => Ok(0),
        }
    }

    fn shutdown(&mut self) {}
}

register_bitfields! {
    u64,
    Blocker [
        Address OFFSET(0) NUMBITS(54) [], // RST: 0, upper DDR address bits 55:2
        Enable OFFSET(54) NUMBITS(4) [] // RST: 0, 0xf to enable blocker
    ]
}

register_bitfields! {
    u32,
    CR0 [
       Start OFFSET(0) NUMBITS(1) [], // RST: 0
        Class OFFSET(8) NUMBITS(4) [] // RST: 0, ddr3: 6, ddr4: 0xa
    ],
    CR19[
        BurstLen OFFSET(16) NUMBITS(2) [] // RST: 2, BL1=0x1 BL2=0x2 BL4=0x3 BL8=3
    ],
    CR21 [
        Optimize OFFSET(0) NUMBITS(1) [] // RST: 0, Enables DDR controller optimized Read Modify Write logic
    ],
    CR120 [
        Interleave OFFSET(0) NUMBITS(1) [] // RST: 0, Disable read data interleaving. Set to 1 in FSBL for valid TileLink operation
    ],
    CR132 [
        TXWM OFFSET(0) NUMBITS(1) [],
        RXWM OFFSET(1) NUMBITS(1) []
    ],
    CR136 [
        InterruptMask OFFSET(0) NUMBITS(16) [] // RST: 0
    ]
}

// DDR Subsystem Bus Blocker Control Register 0
//     Base Address
//     DDR_BUS_BLOCKER: u64 = 0x100b8000;
// 0x100B_8000
// Bits
// Field Name
// Rst.
// Description
// [53:0]
// address [55:2]
// 0x0
// Upper DDR address bits [55:2]
// [59:56]
// enable_disable
// 0x0
// 0xF to enable Bus Blocker.
// This register can only be toggled once after reset.
// Copyright © 2018, SiFive Inc. All rights reserved. 122
//  DDR Controller Control Register 0
//  Base Address
// 0x100B_0000
// Bits
// Field Name
// Rst.
// Description
// 0
// start
// 0x0
// Start initialization of DDR Subsystem
// [11:8]
// dram_class
// 0x0
// DDR3:0x6 DDR4:0xA
//          Table 124:
// Table 125:
// Table 126:
// Table 127:
// DDR Controller Control Register 0
// DDR Controller Control Register 19
// DDR Controller Control Register 21
// DDR Controller Control Register 120
//  DDR Controller Control Register 19
//  Base Address
// 0x100B_004C
// Bits
// Field Name
// Rst.
// Description
// [18:16]
// bstlen
// 0x2
// Encoded burst length.
// BL1=0x1 BL2=0x2 BL4=0x3 BL8=3
//          DDR Controller Control Register 21
//  Base Address
// 0x100B_0054
// Bits
// Field Name
// Rst.
// Description
// 0
// optimal_rmodew_en
// 0
// Enables DDR controller optimized Read Modify Write logic
//          DDR Controller Control Register 120
//  Base Address
// 0x100B_01E0
// Bits
// Field Name
// Rst.
// Description
// 16
// diable_rd_interleave
// 0
// Disable read data interleaving. Set to 1 in FSBL for valid TileLink operation
//          DDR Controller Control Register 132
//  Base Address
// 0x100B_0210
// Bits
// Field Name
// Rst.
// Description
// 7
// int_status[7]
// 0
// An error has occured on the port com- mand channel
// 8
// int_status[8]
// 0
// The memory initialization has been completed
//          Table 128:
// DDR Controller Control Register 132

// Copyright © 2018, SiFive Inc. All rights reserved. 123
// DDR Controller Control Register 136
// Base Address
// 0x100B_0220
// Bits
// Field Name
// Rst.
// Description
// [31:0]
// int_mask
// 0
// MASK interrupt due to cause INT_STATUS [31:0]
fn sdram_init() {
    if is_qemu() {
        return;
    }

    ux00::ux00ddr_writeregmap();
    ux00::ux00ddr_disableaxireadinterleave();

    ux00::ux00ddr_disableoptimalrmodw();

    ux00::ux00ddr_enablewriteleveling();
    ux00::ux00ddr_enablereadleveling();
    ux00::ux00ddr_enablereadlevelinggate();
    if ux00::ux00ddr_getdramclass() == ux00::DRAM_CLASS_DDR4 {
        ux00::ux00ddr_enablevreftraining();
    }

    //mask off interrupts for leveling completion
    ux00::ux00ddr_mask_leveling_completed_interrupt();

    ux00::ux00ddr_mask_mc_init_complete_interrupt();
    ux00::ux00ddr_mask_outofrange_interrupts();
    let ddr_size: u64 = mem_size();
    ux00::ux00ddr_setuprangeprotection(ddr_size);
    ux00::ux00ddr_mask_port_command_error_interrupt();

    let ddr_end: u64 = reg::DRAM as u64 + ddr_size;
    let ddr_bus_blocker: u64 = reg::DDR_BUS_BLOCKER as u64;
    ux00::ux00ddr_start(ddr_bus_blocker, ddr_end);

    ux00::ux00ddr_phy_fixup();
}

pub fn mem_size() -> u64 {
    if is_qemu() {
        return 1024 * 1024 * 1024;
    }
    reg::DDR_SIZE
}
