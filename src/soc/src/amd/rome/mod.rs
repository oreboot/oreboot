pub mod hsmp;
use hsmp::HSMP;

use crate::amd::common::{df::FabricTopology, mp::mpmailbox::MPMailbox};

pub struct SOC {
    hsmp: HSMP,
    // TODO: rename these when we know what they do
    mp0: MPMailbox<8>,
    mp1: MPMailbox<8>,
    mp2: MPMailbox<8>,
    mp3: MPMailbox<8>,
}

impl SOC {
    pub fn new() -> SOC {
        let hsmp = HSMP::new();
        let mp0 = MPMailbox::<8>::new(0x3B1_0520, 0x3B1_002C, 0x3B1_09B8);
        let mp1 = MPMailbox::<8>::new(0x3B1_0524, 0x3B1_0570, 0x3B1_0A40);
        let mp2 = MPMailbox::<8>::new(0x3B1_0528, 0x3B1_0574, 0x3B1_0960);
        let mp3 = MPMailbox::<8>::new(0x3B1_0530, 0x3B1_057C, 0x3B1_09C4);
        Self {
            hsmp,
            mp0,
            mp1,
            mp2,
            mp3,
        }
    }

    pub fn init(&mut self, w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
        let mps = [
            &self.mp0,
            &self.mp1,
            &self.mp2,
            &self.mp3,
            &self.hsmp.mailbox,
        ];
        for mp in mps.iter() {
            match mp.test(42) {
                Ok(v) => {
                    write!(w, "mp result: {:x?}\r\n", v).unwrap();
                }
                Err(e) => {
                    write!(w, "mp test(42) error: {:x?}\r\n", e).unwrap();
                }
            }
            match mp.smu_version() {
                Ok(v) => {
                    write!(w, "mp smu version result: {:x?}\r\n", v).unwrap();
                }
                Err(e) => {
                    write!(w, "mp smu version error: {:x?}\r\n", e).unwrap();
                }
            }
        }

        match self.hsmp.interface_version() {
            Ok(v) => {
                write!(w, "HSMP interface version result: {:x?}\r\n", v).unwrap();
            }
            Err(e) => {
                write!(w, "HSMP interface version error: {:x?}\r\n", e).unwrap();
            }
        }
        let topology = FabricTopology::new();
        write!(w, "Topology: {:?}\r\n", topology).unwrap();
        Ok(())
    }
}

impl Default for SOC {
    fn default() -> Self {
        Self::new()
    }
}
