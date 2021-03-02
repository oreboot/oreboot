#![no_std]
#![feature(llvm_asm)]

pub mod df;
pub mod hsmp;
pub mod pci;

pub fn soc_init(w: &mut impl core::fmt::Write) -> Result<(), &'static str> {
    let hsmp = hsmp::HSMP::new(0);
    unsafe {
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
    }
    Ok(())
}

/// Write 32 bits to port
unsafe fn outl(port: u16, val: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{al}"(val));
}

/// Read 32 bits from port
unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    llvm_asm!("inl %dx, %eax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    ret
}

pub fn smn_read(a: u32) -> u32 {
    // the smn device is at (0)
    unsafe {
        outl(0xcf8, 0x8000_00b8);
        outl(0xcfc, a);
        outl(0xcf8, 0x8000_00bc);
        inl(0xcfc)
    }
}

pub fn smn_write(a: u32, v: u32) {
    unsafe {
        outl(0xcf8, 0x800000b8);
        outl(0xcfc, a);
        outl(0xcf8, 0x800000bc);
        outl(0xcfc, v);
    }
}
