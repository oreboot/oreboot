/* SPDX-License-Identifier: GPL-2.0-only */

// Paging constants
pub mod pg {
    pub const P: u64 = 1 << 0;
    pub const RW: u64 = 1 << 1;
    pub const US: u64 = 1 << 2;
    pub const PWT: u64 = 1 << 3;
    pub const PCD: u64 = 1 << 4;
    pub const A: u64 = 1 << 5;
    pub const D: u64 = 1 << 6;
    pub const PS: u64 = 1 << 7;
    pub const G: u64 = 1 << 8;
    pub const PAT: u64 = 1 << 12;
    pub const XD: u64 = 1 << 63;

    pub const SIZE: usize = 4096;
}
/*
 * EFLAGS bits
 */
pub mod eflags {
    pub const CF: u64 = 0x00000001; /* Carry Flag */
    pub const BIT1: u64 = 0x00000002; /* Bit 1 - always on */
    pub const PF: u64 = 0x00000004; /* Parity Flag */
    pub const AF: u64 = 0x00000010; /* Auxiliary carry Flag */
    pub const ZF: u64 = 0x00000040; /* Zero Flag */
    pub const SF: u64 = 0x00000080; /* Sign Flag */
    pub const TF: u64 = 0x00000100; /* Trap Flag */
    pub const IF: u64 = 0x00000200; /* Interrupt Flag */
    pub const DF: u64 = 0x00000400; /* Direction Flag */
    pub const OF: u64 = 0x00000800; /* Overflow Flag */
    pub const IOPL: u64 = 0x00003000; /* IOPL mask */
    pub const NT: u64 = 0x00004000; /* Nested Task */
    pub const RF: u64 = 0x00010000; /* Resume Flag */
    pub const VM: u64 = 0x00020000; /* Virtual Mode */
    pub const AC: u64 = 0x00040000; /* Alignment Check */
    pub const VIF: u64 = 0x00080000; /* Virtual Interrupt Flag */
    pub const VIP: u64 = 0x00100000; /* Virtual Interrupt Pending */
    pub const ID: u64 = 0x00200000; /* CPUID detection flag */

    pub const RSVD_1: u64 = 0x00000002; // These 1s must be 1, rflags |= this
    pub const RSVD_0: u64 = 0x003f7fd7; // These 0s must be 0, rflags &= this
}

/*
 * Basic CPU control in CR0
 */
pub mod cr0 {
    pub const PE: u64 = 0x00000001; /* Protection Enable */
    pub const MP: u64 = 0x00000002; /* Monitor Coprocessor */
    pub const EM: u64 = 0x00000004; /* Emulation */
    pub const TS: u64 = 0x00000008; /* Task Switched */
    pub const ET: u64 = 0x00000010; /* Extension Type */
    pub const NE: u64 = 0x00000020; /* Numeric Error */
    pub const WP: u64 = 0x00010000; /* Write Protect */
    pub const AM: u64 = 0x00040000; /* Alignment Mask */
    pub const NW: u64 = 0x20000000; /* Not Write-through */
    pub const CD: u64 = 0x40000000; /* Cache Disable */
    pub const PG: u64 = 0x80000000; /* Paging */
}

/*
 * Paging options in CR3
 */
pub mod cr3 {
    pub const PWT: u64 = 0x00000008; /* Page Write Through */
    pub const PCD: u64 = 0x00000010; /* Page Cache Disable */
    pub const PCID_MASK: u64 = 0x00000fff; /* PCID Mask */
}

/*
 * Intel CPU features in CR4
 */
pub mod cr4 {
    pub const VME: u64 = 0x00000001; /* enable vm86 extensions */
    pub const PVI: u64 = 0x00000002; /* virtual interrupts flag enable */
    pub const TSD: u64 = 0x00000004; /* disable time stamp at ipl 3 */
    pub const DE: u64 = 0x00000008; /* enable debugging extensions */
    pub const PSE: u64 = 0x00000010; /* enable page size extensions */
    pub const PAE: u64 = 0x00000020; /* enable physical address extensions */
    pub const MCE: u64 = 0x00000040; /* Machine check enable */
    pub const PGE: u64 = 0x00000080; /* enable global pages */
    pub const PCE: u64 = 0x00000100; /* enable performance counters at ipl 3 */
    pub const OSFXSR: u64 = 0x00000200; /* enable fast FPU save and restore */
    pub const OSXMMEXCPT: u64 = 0x00000400; /* enable unmasked SSE exceptions */
    pub const VMXE: u64 = 0x00002000; /* enable VMX virtualization */
    pub const RDWRGSFS: u64 = 0x00010000; /* enable RDWRGSFS support */
    pub const PCIDE: u64 = 0x00020000; /* enable PCID support */
    pub const OSXSAVE: u64 = 0x00040000; /* enable xsave and xrestore */
    pub const SMEP: u64 = 0x00100000; /* enable SMEP support */
    pub const SMAP: u64 = 0x00200000; /* enable SMAP support */
}

// APIC base
pub const IO_APIC_BASE: usize = 0xfec00000;
pub const APIC_BASE: usize = 0xfee00000;

/*
 * CPUID bits
 */
pub mod cpuid {
    // eax = 0x1
    pub const MONITOR: u32 = 1 << 3;
    pub const VMX: u32 = 1 << 5;
    pub const PDCM: u32 = 1 << 15;
    pub const XSAVE: u32 = 1 << 26;
    pub const OSXSAVE: u32 = 1 << 27;
    pub const HV: u32 = 1 << 31;
    pub const TSC_DL: u32 = 1 << 24;

    // eax = 0x7
    pub const TSC_ADJUST: u32 = 1 << 1;
}
