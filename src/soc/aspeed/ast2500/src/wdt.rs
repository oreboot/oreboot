#![deny(warnings)]

use core::ops;

use register::mmio::{ReadWrite};
use register::{register_bitfields, register_structs};

register_bitfields! {
    u32,
    // Watchdog 2 (WDT2) Control Register
    WDT2C [
        Enable OFFSET(0) NUMBITS(1) []
    ],
    // Watchdog 3 (WDT3) Control Register
    WDT4C [
        Enable OFFSET(0) NUMBITS(1) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub WDTBlock {
        (0x02C => wdt2c: ReadWrite<u32, WDT2C::Register>),
        (0x04C => wdt4c: ReadWrite<u32, WDT4C::Register>),
        (0x050 => @END),
    }
}

pub struct WDT {
}

impl ops::Deref for WDT {
    type Target = WDTBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl WDT {
    pub fn new() -> Self {
        WDT {}
    }

    fn ptr(&self) -> *const WDTBlock {
        0x1e78_5000 as *const _
    }

    pub fn init(&self) {
        // WDT3 is used to detect 24-bit/32-bit SPI support. If we are executing
        // code we are good. It needs to be disabled within one second of boot
        // so do it here in the initialization.
        self.wdt4c.write(WDT4C::Enable::CLEAR);
    }

    pub fn successful_bootup(&self) {
        // Disable watchdog 2, used to reset in case of broken bootloader
        self.wdt2c.write(WDT2C::Enable::CLEAR);
    }
}
