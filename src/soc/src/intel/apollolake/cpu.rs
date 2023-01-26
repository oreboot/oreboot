use crate::intel::apollolake::msr::{ENABLE_IA_UNTRUSTED, MSR_POWER_MISC};
use oreboot_cpu::x86::msr_access::rdmsr;

pub fn cpu_soc_is_in_untrusted_mode() -> bool {
    let msr = unsafe { rdmsr(MSR_POWER_MISC) };
    (msr.lo & ENABLE_IA_UNTRUSTED) != 0
}
