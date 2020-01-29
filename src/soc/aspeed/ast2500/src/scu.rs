#![deny(warnings)]

use core::ops;

use register::mmio::{ReadWrite};
use register::{register_bitfields, register_structs};

register_bitfields! {
    u32,
    // Protection Key Register
    SCU0 [
        Lock OFFSET(0) NUMBITS(32) [
            Lock = 0x0,
            Unlock = 0x1688_A8A8
        ]
    ],
    // Misc. Control Register
    SCU2C [
        DisableUARTDebug OFFSET(10) NUMBITS(1) [],
        UARTEnableDiv13 OFFSET(12) NUMBITS(1) [],
        P2AAccessDisable OFFSET(22) NUMBITS(4) [
            All = 0b1111
        ]
    ],
    // PCI-Express Configuration Setting Control Register
    SCU180 [
        // This register contains a bucket of functions that can be enabled
        // but currently we do not want any of them enabled, just be sure they
        // are disabled.
        DUMMY OFFSET(0) NUMBITS(32) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub SCUBlock {
        (0x000 => scu0: ReadWrite<u32, SCU0::Register>),
        (0x02C => scu2c: ReadWrite<u32, SCU2C::Register>),
        (0x180 => scu180: ReadWrite<u32, SCU180::Register>),
        (0x030 => @END),
    }
}

pub struct SCU {
}

impl ops::Deref for SCU {
    type Target = SCUBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl SCU {
    pub fn new() -> Self {
        SCU {}
    }

    fn ptr(&self) -> *const SCUBlock {
        0x1e6e_2000 as *const _
    }

    pub fn init(&self) {
        // The intent of this initialization is to make the SOC as secure as
        // possible, and let us or the payload activate functionality when it
        // is ready to be used.
        // This is *a bit* challenging on the AST25x0, if you know of anything
        // else that should be disabled, please add it.

        self.unlock();

        // Disable UART debug access
        self.scu2c.write(SCU2C::DisableUARTDebug::SET);

        // Disable everything related to PCI-Express
        self.scu180.set(0u32);

        // Disable all memory access to the BMC over PCI
        self.scu2c.write(SCU2C::P2AAccessDisable::All);

        // Disable div13 for UART to make baud calculation predictable
        self.scu2c.write(SCU2C::UARTEnableDiv13::CLEAR);

        // TODO: Watchdog?

        self.lock();
    }

    fn unlock(&self) {
        self.scu0.write(SCU0::Lock::Unlock);
    }

    fn lock(&self) {
        self.scu0.write(SCU0::Lock::Lock);
    }
}
