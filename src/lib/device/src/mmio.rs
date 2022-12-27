#[cfg(feature = "x86_64")]
use arch::x86_64::mmio::write32;

#[cfg(not(any(
    feature = "armv7",
    feature = "armv8",
    feature = "powerpc64",
    feature = "riscv32",
    feature = "riscv64",
    feature = "x86_64"
)))]
pub unsafe fn write32(addr: usize, value: u32) {
    core::ptr::write_volatile(addr as *mut u32, value);
}

pub unsafe fn write32p(addr: usize, value: u32) {
    write32(addr, value);
}
