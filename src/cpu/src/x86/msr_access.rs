use core::arch::asm;

#[repr(C)]
pub struct Msr {
    pub hi: u32,
    pub lo: u32,
}

impl Msr {
    pub const fn new() -> Self {
        Self { hi: 0, lo: 0 }
    }
}

pub unsafe fn rdmsr(addr: u32) -> Msr {
    let mut result = Msr::new();

    asm!("rdmsr", out("eax") result.lo, out("edx") result.hi, in("ecx") addr, options(nomem, nostack));

    result
}
