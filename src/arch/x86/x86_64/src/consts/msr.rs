// ==== This file seems to be unused =======

/* SPDX-License-Identifier: GPL-2.0-only */

// imported from Akaros/kern/arch/x86/ros/msr-index.h

/* CPU model specific register (MSR) numbers */

/* x86-64 specific MSRs */
pub const EFER: u32 = 0xc000_0080; /* extended feature register */
pub const STAR: u32 = 0xc000_0081; /* legacy mode SYSCALL target */
pub const LSTAR: u32 = 0xc000_0082; /* long mode SYSCALL target */
pub const CSTAR: u32 = 0xc000_0083; /* compat mode SYSCALL target */
pub const SYSCALL_MASK: u32 = 0xc000_0084; /* EFLAGS mask for syscall */
pub const FS_BASE: u32 = 0xc000_0100; /* 64bit FS base */
pub const GS_BASE: u32 = 0xc000_0101; /* 64bit GS base */
pub const KERNEL_GS_BASE: u32 = 0xc000_0102; /* SwapGS GS shadow */
pub const TSC_AUX: u32 = 0xc000_0103; /* Auxiliary TSC */

/* EFER bits: */
pub mod efer {
    pub const LME: u64 = 1 << 8; /* Long mode enable */
    pub const LMA: u64 = 1 << 10; /* Long mode active (read-only) */
}

pub const IA32_SYSENTER_CS: u32 = 0x00000174;
pub const IA32_SYSENTER_ESP: u32 = 0x00000175;
pub const IA32_SYSENTER_EIP: u32 = 0x00000176;

pub const IA32_TSC: u32 = 0x00000010;

pub const IA32_MISC_ENABLE: u32 = 0x000001a0;
pub const IA32_BIOS_SIGN_ID: u32 = 0x0000008b;
pub const IA32_CR_PAT: u32 = 0x00000277;

pub const IA32_APIC_BASE: u32 = 0x0000001b;
pub const IA32_APIC_BASE_ENABLED: u64 = 1 << 11;
pub const IA32_APIC_BASE_X2: u64 = 1 << 10;
pub const IA32_APIC_BASE_BSP: u64 = 1 << 8;
