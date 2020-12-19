/* SPDX-License-Identifier: GPL-2.0-only */

// from github.com:akaros/vmm-akaros/xhype/xhype
// https://uefi.org/sites/default/files/resources/ACPI_6_3_May16.pdf
/* SPDX-License-Identifier: GPL-2.0-only */
// from github.com:akaros/vmm-akaros/xhype/xhype

use core::mem::size_of;
use core::ptr;
// https://uefi.org/sites/default/files/resources/ACPI_6_3_May16.pdf
// Table 5-27 RSDP Structure
#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableRsdp {
    pub signature: [u8; 8],         /* ACPI signature, contains "RSD PTR " */
    pub checksum: u8,               /* ACPI 1.0 checksum */
    pub oem_id: [u8; 6],            /* OEM identification */
    pub revision: u8,               /* Must be (0) for ACPI 1.0 or (2) for ACPI 2.0+ */
    pub rsdt_physical_address: u32, /* 32-bit physical address of the RSDT */
    pub length: u32,                /* Table length in bytes, including header (ACPI 2.0+) */
    pub xsdt_physical_address: u64, /* 64-bit physical address of the XSDT (ACPI 2.0+) */
    pub extended_checksum: u8,      /* Checksum of entire table (ACPI 2.0+) */
    pub reserved: [u8; 3],          /* Reserved, must be zero */
}
pub const ACPI_RSDP_CHECKSUM_LENGTH: usize = 20;
pub const ACPI_RSDP_XCHECKSUM_LENGTH: usize = 36;
pub const ACPI_RSDP_CHECKSUM_OFFSET: usize = 8;
pub const ACPI_RSDP_XCHECKSUM_OFFSET: usize = 32;

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],         /* ASCII table signature */
    pub length: u32,                /* Length of table in bytes, including this header */
    pub revision: u8,               /* ACPI Specification minor version number */
    pub checksum: u8,               /* To make sum of entire table == 0 */
    pub oem_id: [u8; 6],            /* ASCII OEM identification */
    pub oem_table_id: [u8; 8],      /* ASCII OEM table identification */
    pub oem_revision: u32,          /* OEM revision number */
    pub asl_compiler_id: [u8; 4],   /* ASCII ASL compiler vendor ID */
    pub asl_compiler_revision: u32, /* ASL compiler version */
}
pub const ACPI_TABLE_HEADER_CHECKSUM_OFFSET: usize = 9;

impl AcpiTableHeader {
    pub fn new() -> Self {
        AcpiTableHeader { revision: 2, checksum: 0, oem_id: *b"OREORE", oem_table_id: *b"xOREBOOT", oem_revision: 0, asl_compiler_id: *b"RUST", asl_compiler_revision: 0, ..Default::default() }
    }
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableXsdt {
    pub header: AcpiTableHeader, /* Common ACPI table header */
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiGenericAddress {
    pub space_id: u8,     /* Address space where pub struct or register exists */
    pub bit_width: u8,    /* Size in bits of given register */
    pub bit_offset: u8,   /* Bit offset within the register */
    pub access_width: u8, /* Minimum Access size (ACPI 3.0) */
    pub address: u64,     /* 64-bit address of pub struct or register */
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableFadt {
    pub header: AcpiTableHeader,                 /* Common ACPI table header */
    pub facs: u32,                               /* 32-bit physical address of FACS */
    pub dsdt: u32,                               /* 32-bit physical address of DSDT */
    pub model: u8,                               /* System Interrupt Model (ACPI 1.0) - not used in ACPI 2.0+ */
    pub preferred_profile: u8,                   /* Conveys preferred power management profile to OSPM. */
    pub sci_interrupt: u16,                      /* System vector of SCI interrupt */
    pub smi_command: u32,                        /* 32-bit Port address of SMI command port */
    pub acpi_enable: u8,                         /* Value to write to SMI_CMD to enable ACPI */
    pub acpi_disable: u8,                        /* Value to write to SMI_CMD to disable ACPI */
    pub s4_bios_request: u8,                     /* Value to write to SMI_CMD to enter S4BIOS state */
    pub pstate_control: u8,                      /* Processor performance state control */
    pub pm1a_event_block: u32,                   /* 32-bit port address of Power Mgt 1a Event Reg Blk */
    pub pm1b_event_block: u32,                   /* 32-bit port address of Power Mgt 1b Event Reg Blk */
    pub pm1a_control_block: u32,                 /* 32-bit port address of Power Mgt 1a Control Reg Blk */
    pub pm1b_control_block: u32,                 /* 32-bit port address of Power Mgt 1b Control Reg Blk */
    pub pm2_control_block: u32,                  /* 32-bit port address of Power Mgt 2 Control Reg Blk */
    pub pm_timer_block: u32,                     /* 32-bit port address of Power Mgt Timer Ctrl Reg Blk */
    pub gpe0_block: u32,                         /* 32-bit port address of General Purpose Event 0 Reg Blk */
    pub gpe1_block: u32,                         /* 32-bit port address of General Purpose Event 1 Reg Blk */
    pub pm1_event_length: u8,                    /* Byte Length of ports at pm1x_event_block */
    pub pm1_control_length: u8,                  /* Byte Length of ports at pm1x_control_block */
    pub pm2_control_length: u8,                  /* Byte Length of ports at pm2_control_block */
    pub pm_timer_length: u8,                     /* Byte Length of ports at pm_timer_block */
    pub gpe0_block_length: u8,                   /* Byte Length of ports at gpe0_block */
    pub gpe1_block_length: u8,                   /* Byte Length of ports at gpe1_block */
    pub gpe1_base: u8,                           /* Offset in GPE number space where GPE1 events start */
    pub cst_control: u8,                         /* Support for the _CST object and C-States change notification */
    pub c2_latency: u16,                         /* Worst case HW latency to enter/exit C2 state */
    pub c3_latency: u16,                         /* Worst case HW latency to enter/exit C3 state */
    pub flush_size: u16,                         /* Processor memory cache line width, in bytes */
    pub flush_stride: u16,                       /* Number of flush strides that need to be read */
    pub duty_offset: u8,                         /* Processor duty cycle index in processor P_CNT reg */
    pub duty_width: u8,                          /* Processor duty cycle value bit width in P_CNT register */
    pub day_alarm: u8,                           /* Index to day-of-month alarm in RTC CMOS RAM */
    pub month_alarm: u8,                         /* Index to month-of-year alarm in RTC CMOS RAM */
    pub century: u8,                             /* Index to century in RTC CMOS RAM */
    pub boot_flags: u16,                         /* IA-PC Boot Architecture Flags (see below for individual flags) */
    pub reserved: u8,                            /* Reserved, must be zero */
    pub flags: u32,                              /* Miscellaneous flag bits (see below for individual flags) */
    pub reset_register: AcpiGenericAddress,      /* 64-bit address of the Reset register */
    pub reset_value: u8,                         /* Value to write to the reset_register port to reset the system */
    pub arm_boot_flags: u16,                     /* ARM-Specific Boot Flags (see below for individual flags) (ACPI 5.1) */
    pub minor_revision: u8,                      /* FADT Minor Revision (ACPI 5.1) */
    pub xfacs: u64,                              /* 64-bit physical address of FACS */
    pub xdsdt: u64,                              /* 64-bit physical address of DSDT */
    pub xpm1a_event_block: AcpiGenericAddress,   /* 64-bit Extended Power Mgt 1a Event Reg Blk address */
    pub xpm1b_event_block: AcpiGenericAddress,   /* 64-bit Extended Power Mgt 1b Event Reg Blk address */
    pub xpm1a_control_block: AcpiGenericAddress, /* 64-bit Extended Power Mgt 1a Control Reg Blk address */
    pub xpm1b_control_block: AcpiGenericAddress, /* 64-bit Extended Power Mgt 1b Control Reg Blk address */
    pub xpm2_control_block: AcpiGenericAddress,  /* 64-bit Extended Power Mgt 2 Control Reg Blk address */
    pub xpm_timer_block: AcpiGenericAddress,     /* 64-bit Extended Power Mgt Timer Ctrl Reg Blk address */
    pub xgpe0_block: AcpiGenericAddress,         /* 64-bit Extended General Purpose Event 0 Reg Blk address */
    pub xgpe1_block: AcpiGenericAddress,         /* 64-bit Extended General Purpose Event 1 Reg Blk address */
    pub sleep_control: AcpiGenericAddress,       /* 64-bit Sleep Control register (ACPI 5.0) */
    pub sleep_status: AcpiGenericAddress,        /* 64-bit Sleep Status register (ACPI 5.0) */
    pub hypervisor_id: u64,                      /* Hypervisor Vendor ID (ACPI 6.0) */
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableFacs {
    pub signature: [u8; 4], /* ASCII table signature */
    pub length: u32,        /* Length of table in bytes, including this header */
    pub hardware_signature: u32,
    pub firmware_waking_vector: u32,
    pub global_lock: u32,
    pub flags: u32,
    pub xfirm_waking_vector: u64,
    pub version: u8,
    pub reserved: [u8; 3],
    pub ospm_flags: u32,
    pub reserved_0: [u8; 24],
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableMadt {
    pub header: AcpiTableHeader, /* Common ACPI table header */
    pub address: u32,            /* Physical address of local APIC */
    pub flags: u32,
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiSubtableHeader {
    pub r#type: u8,
    pub length: u8,
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiMadtLocalApic {
    pub header: AcpiSubtableHeader,
    pub processor_id: u8, /* ACPI processor id */
    pub id: u8,           /* Processor's local APIC id */
    pub lapic_flags: u32,
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiMadtLocalX2Apic {
    pub header: AcpiSubtableHeader,
    pub reserved: u16,      /* reserved - must be zero */
    pub local_apic_id: u32, /* Processor x2APIC ID  */
    pub lapic_flags: u32,
    pub uid: u32, /* ACPI processor UID */
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiMadtIoApic {
    pub header: AcpiSubtableHeader,
    pub id: u8,               /* I/O APIC ID */
    pub reserved: u8,         /* reserved - must be zero */
    pub address: u32,         /* APIC physical address */
    pub global_irq_base: u32, /* Global system interrupt where INTI lines start */
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiMadtInterruptOverride {
    pub header: AcpiSubtableHeader,
    pub bus: u8,
    pub sourceirq: u8,
    pub globalirq: u32,
    pub flags: u16,
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiMadtNMI {
    pub header: AcpiSubtableHeader,
    pub flags: u16,
    pub globalirq: u32,
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiMadtLocalX2ApicNMI {
    pub header: AcpiSubtableHeader,
    pub flags: u16,
    pub acpi_processor_uid: u32,
    pub local_interrupt: u8,
    pub reserved: [u8; 3],
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableMcfg {
    pub header: AcpiTableHeader, /* Common ACPI table header */
    pub reserved: [u8; 8],
    pub base_address: u64,
    pub pci_seg_group: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub reserved_0: [u8; 4],
}

#[repr(packed)]
#[derive(Default)]
pub struct AcpiTableHpet {
    pub header: AcpiTableHeader,
    pub hw_block_id: u32,
    pub base_address: AcpiGenericAddress,
    pub hpet_number: u8,
    pub min_clock_ticks: u16,
    pub flags: u8,
}

pub fn write<T>(w: &mut impl core::fmt::Write, val: T, offset: usize, index: usize) {
    let y = (offset + index * size_of::<T>()) as *mut T;
    unsafe {
        ptr::write_volatile(y, val);
    }
    write!(w, "write to {:x?}: \r\n", y).unwrap();
}

pub fn read<T>(offset: usize, index: usize) -> T {
    let y = (offset + index * size_of::<T>()) as *mut T;
    unsafe { ptr::read_volatile(y) }
}

/*
 *
 * Intel ACPI Component Architecture
 * ASL Optimizing Compiler version 20140214-64 [Mar 29 2014]
 * Copyright (c) 2000 - 2014 Intel Corporation
 *
 * Compilation of "vmm_acpi_dsdt.dsl" - Fri Apr  1 13:34:26 2016
 *
 */

/*
 *
 * Based on the example at osdev wiki wiki.osdev.org/AML,
 * and the other example in http://www.acpi.info/DOWNLOADS/ACPI_5_Errata%20A.pdf
 * on page 194. Or https://uefi.org/acpi/specs. It keeps moving.
 *
 * Compiled with `iasl -sc input_file.dsl`
 */

/*
 *       9:  DefinitionBlock (
 *      10:      "vmm_acpi_dsdt.aml", // Output AML Filename : String
 *      11:      "DSDT",              // Signature : String
 *      12:      0x2,                 // DSDT Compliance Revision : ByteConst
 *      13:      "MIKE",              // OEMID : String
 *      14:      "DSDTTBL",           // TABLE ID : String
 *      15:      0x0                  // OEM Revision : DWordConst
 *      16:  ){}
 */

pub const DSDT_DSDTTBL_HEADER: [u8; 36] = [
    0x44, 0x53, 0x44, 0x54, 0x24, 0x00, 0x00, 0x00, /* 00000000    "DSDT$..." */
    0x02, 0xF3, 0x4D, 0x49, 0x4B, 0x45, 0x00, 0x00, /* 00000008    "..MIKE.." */
    0x44, 0x53, 0x44, 0x54, 0x54, 0x42, 0x4C, 0x00, /* 00000010    "DSDTTBL." */
    0x00, 0x00, 0x00, 0x00, 0x49, 0x4E, 0x54, 0x4C, /* 00000018    "....INTL" */
    0x14, 0x02, 0x14, 0x20, /* 00000020    "... " */
];

#[inline]
pub fn gencsum(start: usize, end: usize) -> u8 {
    let mut tot: u16 = 0;
    for i in start..end {
        let b: u8 = read(i, 0);
        tot += b as u16;
    }
    (!tot as u8) + 1
}

#[inline]
pub fn acpi_tb_checksum(start: usize, end: usize) -> u8 {
    gencsum(start, end)
}

pub const SIG_RSDP: [u8; 8] = *b"RSD PTR ";
pub const SIG_XSDT: [u8; 4] = *b"XSDT";
pub const SIG_FADT: [u8; 4] = *b"FACP";
pub const SIG_FACS: [u8; 4] = *b"FACS";
pub const SIG_MADT: [u8; 4] = *b"APIC";
pub const SIG_MCFG: [u8; 4] = *b"MCFG";
pub const SIG_HPET: [u8; 4] = *b"HPET";

pub const MADT_LOCAL_APIC: u8 = 0;
pub const MADT_IO_APIC: u8 = 1;
pub const MADT_LOCAL_X2APIC: u8 = 9;
pub const MADT_LOCAL_X2APIC_NMI: u8 = 0xA;
pub const MADT_LOCAL_ISOR: u8 = 2;
