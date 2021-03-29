#![no_std]

mod mp1;
use mp1::MP1;

pub fn soc_init(w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
    let mp1 = MP1::new();
    match mp1.test(42) {
        Ok(v) => {
            write!(w, "MP1 test(42) result: {:x?}\r\n", v).unwrap();
        }
        Err(e) => {
            write!(w, "MP1 test(42) error: {:x?}\r\n", e).unwrap();
        }
    }
    match mp1.smu_version() {
        Ok(version) => {
            write!(w, "MP1 smu version result: {:x?}\r\n", version).unwrap();
        }
        Err(e) => {
            write!(w, "MP1 smu version error: {:x?}\r\n", e).unwrap();
        }
    }
    Ok(())
}
