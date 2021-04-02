#![no_std]
#![feature(llvm_asm)]

pub mod hsmp;
use hsmp::HSMP;

pub struct SOC {
    hsmp: HSMP,
}

impl SOC {
    pub fn new() -> SOC {
        let hsmp = HSMP::new();
        Self { hsmp }
    }

    pub fn init(&mut self, w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
        match self.hsmp.test(42) {
            Ok(v) => {
                write!(w, "HSMP test(42) result: {:x?}\r\n", v).unwrap();
            }
            Err(e) => {
                write!(w, "HSMP test(42) error: {:x?}\r\n", e).unwrap();
            }
        }
        match self.hsmp.smu_version() {
            Ok(v) => {
                write!(w, "HSMP smu version result: {:x?}\r\n", v).unwrap();
            }
            Err(e) => {
                write!(w, "HSMP smu version error: {:x?}\r\n", e).unwrap();
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
        let topology = df::FabricTopology::new();
        write!(w, "Topology: {:?}\r\n", topology).unwrap();
        Ok(())
    }
}

impl Default for SOC {
    fn default() -> Self {
        Self::new()
    }
}
