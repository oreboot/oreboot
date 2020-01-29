#![feature(global_asm)]
#![no_std]
#![deny(warnings)]

pub mod lpc;
pub mod scu;
pub mod reg;

use scu::SCU;
use lpc::LPC;

// Bare minimal initialization to make the system usable (no DRAM e.g.)
pub fn init() {
    cpu::init();
    // This closes down e.g. CVE-2019-6260 from cold-boot, so this should be
    // done as early as possible
    SCU::new().init();
    LPC::new().init();
}

// TODO: Add UART routing helpers

global_asm!(include_str!("start.S"));
