/* SPDX-License-Identifier: GPL-2.0-only */

// Paging constants
pub const PG_P: u64 = 1;
pub const PG_RW: u64 = 1 << 1;
pub const PG_PS: u64 = 1 << 7;

pub const PAGE_SIZE: usize = 4096;

/*
 * EFLAGS bits
 */
pub const X86_EFLAGS_CF: u64 = 0x00000001; /* Carry Flag */
pub const X86_EFLAGS_BIT1: u64 = 0x00000002; /* Bit 1 - always on */
pub const X86_EFLAGS_PF: u64 = 0x00000004; /* Parity Flag */
pub const X86_EFLAGS_AF: u64 = 0x00000010; /* Auxiliary carry Flag */
pub const X86_EFLAGS_ZF: u64 = 0x00000040; /* Zero Flag */
pub const X86_EFLAGS_SF: u64 = 0x00000080; /* Sign Flag */
pub const X86_EFLAGS_TF: u64 = 0x00000100; /* Trap Flag */
pub const X86_EFLAGS_IF: u64 = 0x00000200; /* Interrupt Flag */
pub const X86_EFLAGS_DF: u64 = 0x00000400; /* Direction Flag */
pub const X86_EFLAGS_OF: u64 = 0x00000800; /* Overflow Flag */
pub const X86_EFLAGS_IOPL: u64 = 0x00003000; /* IOPL mask */
pub const X86_EFLAGS_NT: u64 = 0x00004000; /* Nested Task */
pub const X86_EFLAGS_RF: u64 = 0x00010000; /* Resume Flag */
pub const X86_EFLAGS_VM: u64 = 0x00020000; /* Virtual Mode */
pub const X86_EFLAGS_AC: u64 = 0x00040000; /* Alignment Check */
pub const X86_EFLAGS_VIF: u64 = 0x00080000; /* Virtual Interrupt Flag */
pub const X86_EFLAGS_VIP: u64 = 0x00100000; /* Virtual Interrupt Pending */
pub const X86_EFLAGS_ID: u64 = 0x00200000; /* CPUID detection flag */

pub const FL_RSVD_1: u64 = 0x00000002; // These 1s must be 1, rflags |= this
pub const FL_RSVD_0: u64 = 0x003f7fd7; // These 0s must be 0, rflags &= this
pub const FL_IF: u64 = 1 << 9;

/*
 * Basic CPU control in CR0
 */
pub const X86_CR0_PE: u64 = 0x00000001; /* Protection Enable */
pub const X86_CR0_MP: u64 = 0x00000002; /* Monitor Coprocessor */
pub const X86_CR0_EM: u64 = 0x00000004; /* Emulation */
pub const X86_CR0_TS: u64 = 0x00000008; /* Task Switched */
pub const X86_CR0_ET: u64 = 0x00000010; /* Extension Type */
pub const X86_CR0_NE: u64 = 0x00000020; /* Numeric Error */
pub const X86_CR0_WP: u64 = 0x00010000; /* Write Protect */
pub const X86_CR0_AM: u64 = 0x00040000; /* Alignment Mask */
pub const X86_CR0_NW: u64 = 0x20000000; /* Not Write-through */
pub const X86_CR0_CD: u64 = 0x40000000; /* Cache Disable */
pub const X86_CR0_PG: u64 = 0x80000000; /* Paging */

/*
 * Paging options in CR3
 */
pub const X86_CR3_PWT: u64 = 0x00000008; /* Page Write Through */
pub const X86_CR3_PCD: u64 = 0x00000010; /* Page Cache Disable */
pub const X86_CR3_PCID_MASK: u64 = 0x00000fff; /* PCID Mask */

/*
 * Intel CPU features in CR4
 */
pub const X86_CR4_VME: u64 = 0x00000001; /* enable vm86 extensions */
pub const X86_CR4_PVI: u64 = 0x00000002; /* virtual interrupts flag enable */
pub const X86_CR4_TSD: u64 = 0x00000004; /* disable time stamp at ipl 3 */
pub const X86_CR4_DE: u64 = 0x00000008; /* enable debugging extensions */
pub const X86_CR4_PSE: u64 = 0x00000010; /* enable page size extensions */
pub const X86_CR4_PAE: u64 = 0x00000020; /* enable physical address extensions */
pub const X86_CR4_MCE: u64 = 0x00000040; /* Machine check enable */
pub const X86_CR4_PGE: u64 = 0x00000080; /* enable global pages */
pub const X86_CR4_PCE: u64 = 0x00000100; /* enable performance counters at ipl 3 */
pub const X86_CR4_OSFXSR: u64 = 0x00000200; /* enable fast FPU save and restore */
pub const X86_CR4_OSXMMEXCPT: u64 = 0x00000400; /* enable unmasked SSE exceptions */
pub const X86_CR4_VMXE: u64 = 0x00002000; /* enable VMX virtualization */
pub const X86_CR4_RDWRGSFS: u64 = 0x00010000; /* enable RDWRGSFS support */
pub const X86_CR4_PCIDE: u64 = 0x00020000; /* enable PCID support */
pub const X86_CR4_OSXSAVE: u64 = 0x00040000; /* enable xsave and xrestore */
pub const X86_CR4_SMEP: u64 = 0x00100000; /* enable SMEP support */
pub const X86_CR4_SMAP: u64 = 0x00200000; /* enable SMAP support */

// APIC base
pub const IO_APIC_BASE: usize = 0xfec00000;
pub const APIC_BASE: usize = 0xfee00000;

/*
 * CPUID bits
 */
// eax = 0x1
pub const CPUID_MONITOR: u32 = 1 << 3;
pub const CPUID_VMX: u32 = 1 << 5;
pub const CPUID_PDCM: u32 = 1 << 15;
pub const CPUID_XSAVE: u32 = 1 << 26;
pub const CPUID_OSXSAVE: u32 = 1 << 27;
pub const CPUID_HV: u32 = 1 << 31;
pub const CPUID_TSC_DL: u32 = 1 << 24;

// eax = 0x7
pub const CPUID_TSC_ADJUST: u32 = 1 << 1;
