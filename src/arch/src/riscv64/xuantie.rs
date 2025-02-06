// XuanTie aka T-Head designs processing cores for use in SoCs and MCUs.
// General information and manuals are on the vendor website:
// https://www.xrvm.com/product/list/xuantie
// https://www.xrvm.com/product/xuantie/C906
// https://www.xrvm.com/product/xuantie/C908
// Some open RTL and manuals are also at https://github.com/XUANTIE-RV.
// A copy of the C906 manual in English is publicly available:
// https://occ-intl-prod.oss-ap-southeast-1.aliyuncs.com/resource/XuanTie-OpenC906-UserManual.pdf
// See also https://github.com/XUANTIE-RV/thead-extension-spec

use core::arch::asm;
use xuantie::register as reg;

use log::println;

// relative to PLIC base, see C906/C908 manuals
const PLIC_PERMISSION_CONTROL: usize = 0x001f_fffc;

// relative to PLIC base, see C906/C908 manuals
const CLINT_BASE_OFFSET: usize = 0x0400_0000;

// these are relative to the CLINT base, see also
// https://github.com/riscv/riscv-aclint/blob/main/riscv-aclint.adoc
const MTIME_COMPARE_OFFSET: usize = 0x4000;
const MTIME_OFFSET: usize = 0xBFF8;

// The XuanTie _MAPBADDR_ (M-mode accessible only, APB address) holds the base
// address of the MMIO based PLIC, which the CLINT base address is relative to.
pub fn get_plic_base() -> usize {
    reg::mapbaddr::read()
}

pub fn get_plic_perm() -> usize {
    get_plic_base() + PLIC_PERMISSION_CONTROL
}

pub fn get_clint_base() -> usize {
    get_plic_base() + CLINT_BASE_OFFSET
}

pub fn get_mtime_compare_reg() -> usize {
    get_clint_base() + MTIME_COMPARE_OFFSET
}

pub fn get_mtime_reg() -> usize {
    get_clint_base() + MTIME_OFFSET
}

/// Grant S-mode access to the PLIC.
pub fn init_plic() {
    util::mmio::write8(get_plic_perm(), 1);
}

pub fn dump_csrs() {
    println!("=== XuanTie CSRs ===");
    let v = reg::mxstatus::read();
    println!("   MXSTATUS  {v:08x?}");
    let v = reg::mhcr::read();
    println!("   MHCR      {v:08x?}");
    println!("====================");
}

struct Cpuid;

impl Cpuid {
    pub fn read() -> usize {
        riscv::read_csr!(0xfc0);
        unsafe { _read() }
    }
}

/// The machine-mode _MCPUID_ register stores the processor model information.
/// Continuous reads of the register yield up to 7 different return values.
/// The reset value is defined by each product itself in compliance with the
/// XuanTie (aka T-Head) product definition specifications to facilitate
/// software identification.
/// For decoding, see https://github.com/platform-system-interface/thead_cpuinfo
pub fn print_cpuid() {
    for i in 0..7 {
        let id = Cpuid::read();
        println!("MCPUID {i}: {id:08x}");
    }
}
