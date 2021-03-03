#![no_std]
#![feature(llvm_asm)]

pub mod smu;

pub fn soc_init(w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
    let smu = smu::SMU::new();
    match smu.test(42) {
        Ok(v) => {
            write!(w, "SMU test(42) result: {}\r\n", v).unwrap();
        }
        Err(e) => {
            write!(w, "SMU test(42) error: {}\r\n", e).unwrap();
        }
    }
    match smu.smu_version() {
        Ok((major, minor)) => {
            write!(w, "SMU smu version result: {}.{}\r\n", major, minor).unwrap();
        }
        Err(e) => {
            write!(w, "SMU smu version error: {}\r\n", e).unwrap();
        }
    }
    match smu.interface_version() {
        Ok((major, minor)) => {
            write!(w, "SMU interface version result: {}.{}\r\n", major, minor).unwrap();
        }
        Err(e) => {
            write!(w, "SMU interface version error: {}\r\n", e).unwrap();
        }
    }
}
