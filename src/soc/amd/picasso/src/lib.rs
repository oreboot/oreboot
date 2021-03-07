#![no_std]
#![feature(llvm_asm)]

pub mod mp1;

pub fn soc_init(w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
    let mp1 = mp1::MP1::new();
    match mp1.test(42) {
        Ok(v) => {
            write!(w, "MP1 test(42) result: {}\r\n", v).unwrap();
        }
        Err(e) => {
            write!(w, "MP1 test(42) error: {}\r\n", e).unwrap();
        }
    }
    match mp1.smu_version() {
        Ok((major, minor)) => {
            write!(w, "MP1 smu version result: {}.{}\r\n", major, minor).unwrap();
        }
        Err(e) => {
            write!(w, "MP1 smu version error: {}\r\n", e).unwrap();
        }
    }
    match mp1.interface_version() {
        Ok((major, minor)) => {
            write!(w, "MP1 interface version result: {}.{}\r\n", major, minor).unwrap();
        }
        Err(e) => {
            write!(w, "MP1 interface version error: {}\r\n", e).unwrap();
        }
    }
    Ok(())
}
