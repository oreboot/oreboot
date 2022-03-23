/*
 * This file is part of the oreboot project.
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

use core::ops;

use tock_registers::interfaces::ReadWriteable;
use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;
use tock_registers::registers::{ReadOnly, ReadWrite};
use vcell::VolatileCell;

const HPET0: usize = 0xfed0_0000;

// We fill out as little of this as possible.
// We're firmware and should never plan to use it
// with all its features.
#[repr(C)]
pub struct RegisterBlock {
    hpet_id: ReadOnly<u32, HPETID::Register>,
    clk_period: VolatileCell<u32>, // fs
    _8: u32,
    _12: u32,
    config: ReadWrite<u32, CONFIG::Register>,
    _20: u32,
    _24: u32,
    _28: u32,
    _int_stat: u32, // all reserved
    _36: [u32; 51],
    main_ctr: VolatileCell<u64>, // Note: in 32 bit mode only lower 32 bit are valid
}

pub struct HPET {
    base: usize,
}

impl ops::Deref for HPET {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! {
    u32,
    HPETID [
        VENDORID OFFSET(16) NUMBITS(16) [],
        LEGACYCAP OFFSET(15) NUMBITS(1) [],
        COUNTERSIZECAP OFFSET(13) NUMBITS(1) [
            B32 = 0,
            B64 = 1
        ]
    ],
    CONFIG [
        LEGACYEN OFFSET(1) NUMBITS(1) [],
        TMREN OFFSET(0) NUMBITS(1) []
    ]
}

impl HPET {
    /// # Safety
    /// use at own risk
    pub unsafe fn new(base: usize) -> HPET {
        HPET { base }
    }

    // Note: Caller needs to ensure that PMx00000000[HpetEn]=1 (where PM=FED8_0300h)
    pub fn hpet0() -> HPET {
        HPET { base: HPET0 }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }

    pub fn enable(&self) -> Result<(), &str> {
        // set enable bit for HPET timer as per 2.3.5
        // https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf
        self.config.modify(CONFIG::TMREN::SET);
        match self.config.is_set(CONFIG::TMREN) {
            true => Ok(()),
            false => Err("HPET.enable() failed to set timer_enable."),
        }
    }

    // Sleeps at least the given amount of time (in fs).
    // Note: That means that DELAY cannot be much longer than 4 us.
    pub fn sleep(&self, delay: u32) {
        let clk_period = self.clk_period.get() as u64; // in fs
        let counter_mask = if self.hpet_id.is_set(HPETID::COUNTERSIZECAP) {
            !0u64
        } else {
            0xffff_ffffu64
        };
        let ticks = (delay as u64 + clk_period - 1) / clk_period;
        let starting_value = self.main_ctr.get() & counter_mask;
        loop {
            let value = self.main_ctr.get() & counter_mask;
            if value - starting_value >= ticks as u64 {
                break;
            }
            // TODO: asm nop nop nop
        }
    }
}
