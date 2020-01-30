#![feature(global_asm)]
#![no_std]
#![deny(warnings)]

pub mod lpc;
pub mod scu;
pub mod reg;
pub mod wdt;

use scu::SCU;
use lpc::LPC;
use wdt::WDT;

// Bare minimal initialization to make the system usable (no DRAM e.g.)
pub fn init() {
    cpu::init();
    // This closes down e.g. CVE-2019-6260 from cold-boot, so this should be
    // done as early as possible
    SCU::new().init();
    LPC::new().init();

    let w = WDT::new();
    w.init();

    w.successful_bootup();
}

// TODO: Add UART routing helpers

global_asm!(include_str!("start.S"));
