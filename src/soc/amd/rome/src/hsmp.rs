#![allow(non_snake_case)]

use mp::mpmailbox::MPMailbox;
use mp::mpmailbox::Result;

pub struct HSMP {
    mailbox: MPMailbox<8>,
}

impl HSMP {
    pub fn new() -> HSMP {
        Self { mailbox: MPMailbox::<8>::new(0x3B1_0534, 0x3B1_0980, 0x3B1_09E0) }
    }
    pub fn test(&self, v: u32) -> Result<u32> {
        self.mailbox.test(v)
    }
    pub fn smu_version(&self) -> Result<u32> {
        self.mailbox.smu_version()
    }
    pub fn interface_version(&self) -> Result<u32> {
        let result = self.mailbox.call1(3, 0)?;
        Ok(result)
    }
}

impl Default for HSMP {
    fn default() -> Self {
        Self::new()
    }
}
