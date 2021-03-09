#![no_std]
#![feature(llvm_asm)]

pub mod hsmp;

pub fn soc_init(w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
    let hsmp = hsmp::HSMP::new(0);
    match hsmp.test(42) {
        Ok(v) => {
            write!(w, "HSMP test(42) result: {}\r\n", v).unwrap();
        }
        Err(e) => {
            write!(w, "HSMP test(42) error: {}\r\n", e).unwrap();
        }
    }
    match hsmp.smu_version() {
        Ok((major, minor)) => {
            write!(w, "HSMP smu version result: {}.{}\r\n", major, minor).unwrap();
        }
        Err(e) => {
            write!(w, "HSMP smu version error: {}\r\n", e).unwrap();
        }
    }
    match hsmp.interface_version() {
        Ok((major, minor)) => {
            write!(w, "HSMP interface version result: {}.{}\r\n", major, minor).unwrap();
        }
        Err(e) => {
            write!(w, "HSMP interface version error: {}\r\n", e).unwrap();
        }
    }
    Ok(())
}
