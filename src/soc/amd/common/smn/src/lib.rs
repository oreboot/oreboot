#![feature(llvm_asm)]
#![no_std]

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
        outl(0xcf8, 0x8000_00b8);
        outl(0xcfc, a);
        outl(0xcf8, 0x8000_00bc);
        outl(0xcfc, v);
    }
}
