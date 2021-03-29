use mp::pcie::PCIeTraining;
use mp::usb::USBTraining;
use mp::mpmailbox::MPMailbox;
use mp::mpmailbox::Result;

pub struct MP1 {
    mailbox: MPMailbox<6>,
}

impl MP1 {
    pub fn new() -> Self {
        Self {
            mailbox: MPMailbox::<6>::new(0x3B1_0528, 0x3B1_0564, 0x3B1_0998),
        }
    }
    pub fn test(&self, v: u32) -> Result<u32> {
        self.mailbox.test(v)
    }
    pub fn smu_version(&self) -> Result<u32> {
        self.mailbox.smu_version()
    }
}

impl PCIeTraining for MP1 {
}

impl USBTraining for MP1 {
}
