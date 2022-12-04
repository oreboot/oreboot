use cpu::x86::msr_access::rdmsr;

pub const MSR_POWER_MISC: u32 = 0x120;
pub const MSR_POWER_CTL: u32 = 0x1fc;

// FIXME: rdmsr must be run inside the kernel/bootloader,
// so this test needs ring 0 or equivalent permission
// Running in userspace results in SIGSEV
#[ignore]
#[test]
fn test_rdmsr() {
    let msr = unsafe { rdmsr(MSR_POWER_MISC) };
    println!("msr: lo: {}, hi: {}", msr.lo, msr.hi);
}
