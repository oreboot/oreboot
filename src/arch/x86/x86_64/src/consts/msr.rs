/* SPDX-License-Identifier: GPL-2.0-only */

// imported from Akaros/kern/arch/x86/ros/msr-index.h

/* CPU model specific register (MSR) numbers */

/* x86-64 specific MSRs */
pub const MSR_EFER: u32 = 0xc0000080; /* extended feature register */
pub const MSR_STAR: u32 = 0xc0000081; /* legacy mode SYSCALL target */
pub const MSR_LSTAR: u32 = 0xc0000082; /* long mode SYSCALL target */
pub const MSR_CSTAR: u32 = 0xc0000083; /* compat mode SYSCALL target */
pub const MSR_SYSCALL_MASK: u32 = 0xc0000084; /* EFLAGS mask for syscall */
pub const MSR_FS_BASE: u32 = 0xc0000100; /* 64bit FS base */
pub const MSR_GS_BASE: u32 = 0xc0000101; /* 64bit GS base */
pub const MSR_KERNEL_GS_BASE: u32 = 0xc0000102; /* SwapGS GS shadow */
pub const MSR_TSC_AUX: u32 = 0xc0000103; /* Auxiliary TSC */

/* EFER bits: */

pub const EFER_LME: u64 = 1 << 8; /* Long mode enable */
pub const EFER_LMA: u64 = 1 << 10; /* Long mode active (read-only) */

pub const MSR_IA32_SYSENTER_CS: u32 = 0x00000174;
pub const MSR_IA32_SYSENTER_ESP: u32 = 0x00000175;
pub const MSR_IA32_SYSENTER_EIP: u32 = 0x00000176;

pub const MSR_IA32_TSC: u32 = 0x00000010;

pub const MSR_IA32_MISC_ENABLE: u32 = 0x000001a0;
pub const MSR_IA32_BIOS_SIGN_ID: u32 = 0x0000008b;
pub const MSR_IA32_CR_PAT: u32 = 0x00000277;

pub const MSR_IA32_APIC_BASE: u32 = 0x0000001b;
pub const MSR_IA32_APIC_BASE_ENABLED: u64 = 1 << 11;
pub const MSR_IA32_APIC_BASE_X2: u64 = 1 << 10;
pub const MSR_IA32_APIC_BASE_BSP: u64 = 1 << 8;
