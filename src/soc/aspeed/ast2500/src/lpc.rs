#![deny(warnings)]

use core::ops;

use register::mmio::{ReadWrite};
use register::{register_bitfields, register_structs};

register_bitfields! {
    u32,
    // Host Interface Control Register 5
    HICR5 [
        EnableFWH OFFSET(10) NUMBITS(1) []
    ],
    // Host Interface Control Register B
    HICRB [
        DisableLPC2AHB OFFSET(6) NUMBITS(1) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub LPCBlock {
        (0x080 => hicr5: ReadWrite<u32, HICR5::Register>),
        (0x100 => hicrb: ReadWrite<u32, HICRB::Register>),
        (0x104 => @END),
    }
}

pub struct LPC {
}

impl ops::Deref for LPC {
    type Target = LPCBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl LPC {
    pub fn new() -> Self {
        LPC {}
    }

    fn ptr(&self) -> *const LPCBlock {
        0x1e78_9000 as *const _
    }

    pub fn init(&self) {
        // Also see comments in SCU::init for context

        // Disable LPC FWH cycles
        self.hicr5.write(HICR5::EnableFWH::CLEAR);
        // Disable LPC2AHB function
        self.hicrb.write(HICRB::DisableLPC2AHB::SET);
    }
}
