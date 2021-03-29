#![no_std]
#![feature(llvm_asm)]

pub mod hsmp;

use hsmp::HSMP;
use mp::mpmailbox::MPMailbox;

pub fn soc_init(w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
    let hsmp = HSMP::new();
    match hsmp.test(42) {
        Ok(v) => {
            write!(w, "HSMP test(42) result: {:x?}\r\n", v).unwrap();
        }
        Err(e) => {
            write!(w, "HSMP test(42) error: {:x?}\r\n", e).unwrap();
        }
    }
    match hsmp.smu_version() {
        Ok(v) => {
            write!(w, "HSMP smu version result: {:x?}\r\n", v).unwrap();
        }
        Err(e) => {
            write!(w, "HSMP smu version error: {:x?}\r\n", e).unwrap();
        }
    }
    match hsmp.interface_version() {
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
