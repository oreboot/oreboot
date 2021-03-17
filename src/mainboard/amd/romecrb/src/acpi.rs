use arch::acpi::*;
use arch::consts::x86;
use core::mem::size_of;
use util::round_up_4k;

/// Setup the BIOS tables in the low memory
///
/// `start` is the base address of the BIOS tables in the physical address space
/// of the guest. `low_mem` is a host virtual memory block which is mapped to
/// the lowest memory of the guest. `cores` is the number of logical CPUs of the guest.
/// Total number of bytes occupied by the BIOS tables is returned.
pub fn setup_acpi_tables(w: &mut impl core::fmt::Write, start: usize, cores: u32) -> usize {
    // calculate offsets first
    // variables with suffix `_offset` mean the offset in `low_mem`.

    const NUM_XSDT_ENTRIES: usize = 4;

    let rsdp_offset = start;
    let xsdt_offset = rsdp_offset + size_of::<AcpiTableRsdp>();
    let xsdt_entry_offset = xsdt_offset + size_of::<AcpiTableHeader>();
    let fadt_offset = xsdt_entry_offset + NUM_XSDT_ENTRIES * size_of::<u64>();
    let facs_offset = fadt_offset + size_of::<AcpiTableFadt>();
    let dsdt_offset = facs_offset + size_of::<AcpiTableFacs>();

    // let madt_offset = dsdt_offset + 69887;
    let madt_offset = dsdt_offset + DSDT_DSDTTBL_HEADER.len();
    let madt_local_apic_offset = madt_offset + size_of::<AcpiTableMadt>();
    let madt_local_x2apic_offset = madt_local_apic_offset + cores as usize * size_of::<AcpiMadtLocalApic>();
    let madt_local_x2apic_nmi_offset = madt_local_x2apic_offset + cores as usize * size_of::<AcpiMadtLocalX2Apic>();
    let madt_io_apic_offset = madt_local_x2apic_nmi_offset + size_of::<AcpiMadtLocalX2ApicNMI>();
    let madt_local_isor_offset = madt_io_apic_offset + size_of::<AcpiMadtIoApic>();

    let mcfg_offset = madt_local_isor_offset + 2 * size_of::<AcpiMadtInterruptOverride>();
    let hpet_offset = mcfg_offset + size_of::<AcpiTableMcfg>();
    let total_size = hpet_offset + size_of::<AcpiTableHpet>() - start;

    // setup rsdp - Root System Description Pointer
    let rsdp = AcpiTableRsdp { signature: SIG_RSDP, revision: 2, length: 36, xsdt_physical_address: xsdt_offset as u64, ..Default::default() };

    write!(w, "Write rsdp  at {:x?} \r\n", rsdp_offset).unwrap();
    write(w, rsdp, rsdp_offset, 0);
    write(w, gencsum(rsdp_offset, rsdp_offset + ACPI_RSDP_CHECKSUM_LENGTH), rsdp_offset, ACPI_RSDP_CHECKSUM_OFFSET); // XXX
    debug_assert_eq!(acpi_tb_checksum(rsdp_offset, rsdp_offset + ACPI_RSDP_CHECKSUM_LENGTH), 0);

    write(w, gencsum(rsdp_offset, rsdp_offset + ACPI_RSDP_XCHECKSUM_LENGTH), rsdp_offset, ACPI_RSDP_XCHECKSUM_OFFSET); // XXX
    debug_assert_eq!(acpi_tb_checksum(rsdp_offset, rsdp_offset + ACPI_RSDP_XCHECKSUM_LENGTH), 0);

    // xsdt - Extended System Description Table
    let xsdt_total_length = size_of::<AcpiTableHeader>() + size_of::<u64>() * NUM_XSDT_ENTRIES;
    let xsdt = AcpiTableHeader { signature: SIG_XSDT, length: xsdt_total_length as u32, revision: 1, ..AcpiTableHeader::new() };
    write(w, xsdt, xsdt_offset, 0);
    // xsdt entries
    let xsdt_entries: [u64; NUM_XSDT_ENTRIES] = [fadt_offset as u64, madt_offset as u64, mcfg_offset as u64, hpet_offset as u64];

    write(w, xsdt_entries, xsdt_entry_offset, 0);
    write(w, gencsum(xsdt_offset, xsdt_offset + xsdt_total_length), xsdt_offset, ACPI_TABLE_HEADER_CHECKSUM_OFFSET); // XXX
    debug_assert_eq!(acpi_tb_checksum(xsdt_offset, xsdt_offset + xsdt_total_length), 0);

    const FADT_FLAGS: u32 = 0b0011_0000_0101_1010_0101;
    // fadt - Fixed ACPI Description Table
    let fadt = AcpiTableFadt {
        header: AcpiTableHeader { signature: SIG_FADT, length: size_of::<AcpiTableFadt>() as u32, ..AcpiTableHeader::new() },
        preferred_profile: 0x04,
        sci_interrupt: 0x09,
        smi_command: 0xB2,
        acpi_enable: 0xa0,
        acpi_disable: 0xa1,
        pm1a_event_block: 0x800,
        pm1a_control_block: 0x804,
        pm_timer_block: 0x808,
        gpe0_block: 0x820,
        pm1_event_length: 4,
        pm1_control_length: 2,
        pm_timer_length: 4,
        gpe0_block_length: 8,
        c2_latency: 0x65,
        c3_latency: 0x3e9,
        flush_size: 0x400,
        flush_stride: 0x10,
        duty_offset: 1,
        duty_width: 0,
        day_alarm: 0xD,
        century: 0x32,
        boot_flags: 1,
        flags: FADT_FLAGS,
        reset_register: AcpiGenericAddress { space_id: 1, bit_width: 8, bit_offset: 0, access_width: 0, address: 0xCF9 },
        reset_value: 0x06,
        minor_revision: 2,
        xfacs: facs_offset as u64,
        xdsdt: dsdt_offset as u64,
        xpm1a_event_block: AcpiGenericAddress { space_id: 1, bit_width: 32, bit_offset: 0, access_width: 2, address: 0x800 },
        xpm1a_control_block: AcpiGenericAddress { space_id: 1, bit_width: 16, bit_offset: 0, access_width: 2, address: 0x804 },
        xpm_timer_block: AcpiGenericAddress { space_id: 1, bit_width: 32, bit_offset: 0, access_width: 3, address: 0x808 },
        xgpe0_block: AcpiGenericAddress { space_id: 1, bit_width: 64, bit_offset: 0, access_width: 1, address: 0x820 },
        ..Default::default()
    };
    write(w, fadt, fadt_offset, 0);
    write(w, gencsum(fadt_offset, fadt_offset + size_of::<AcpiTableFadt>()), fadt_offset, ACPI_TABLE_HEADER_CHECKSUM_OFFSET); // XXX
    debug_assert_eq!(acpi_tb_checksum(fadt_offset, fadt_offset + size_of::<AcpiTableFadt>()), 0);

    // facs - Firmware ACPI Control Structure
    let facs = AcpiTableFacs { signature: SIG_FACS, length: size_of::<AcpiTableFacs>() as u32, flags: 0, version: 2, ..Default::default() };
    write(w, facs, facs_offset, 0);

    // dsdt - Differentiated System Description Table
    write(w, DSDT_DSDTTBL_HEADER, dsdt_offset, 0);

    // madt - Multiple APIC Description Table
    // TODO: Recalculate for SMP
    // let madt_total_length = size_of::<AcpiTableMadt>() + cores as usize * (size_of::<AcpiMadtLocalApic>() + size_of::<AcpiMadtLocalX2apic>());
    let madt_total_length = size_of::<AcpiTableMadt>() + cores as usize * (size_of::<AcpiMadtLocalApic>() + size_of::<AcpiMadtLocalX2Apic>()) + size_of::<AcpiMadtLocalX2ApicNMI>() + size_of::<AcpiMadtIoApic>() + 2 * size_of::<AcpiMadtInterruptOverride>();

    let madt = AcpiTableMadt { header: AcpiTableHeader { signature: SIG_MADT, length: madt_total_length as u32, revision: 4, ..AcpiTableHeader::new() }, address: x86::APIC_BASE as u32, flags: 1 };
    write(w, madt, madt_offset, 0);
    // Processor Local APIC
    for i in 0..cores {
        let local_apic = AcpiMadtLocalApic { header: AcpiSubtableHeader { r#type: MADT_LOCAL_APIC, length: size_of::<AcpiMadtLocalApic>() as u8 }, processor_id: i as u8, id: i as u8, lapic_flags: 1 };
        write(w, local_apic, madt_local_apic_offset, i as usize)
    }

    // Processor Local x2APIC
    for i in 0..cores {
        let local_x2apic = AcpiMadtLocalX2Apic {
            header: AcpiSubtableHeader { r#type: MADT_LOCAL_X2APIC, length: size_of::<AcpiMadtLocalX2Apic>() as u8 },
            local_apic_id: i * 2, /* Per even SMT thread */
            lapic_flags: 1,
            uid: i * 2, /* uid = apic_id */
            ..Default::default()
        };
        write(w, local_x2apic, madt_local_x2apic_offset, i as usize)
    }

    // Local x2APIC NMI
    let local_x2apic_nmi = AcpiMadtLocalX2ApicNMI {
        header: AcpiSubtableHeader { r#type: MADT_LOCAL_X2APIC_NMI, length: size_of::<AcpiMadtLocalX2ApicNMI>() as u8 },
        flags: 5,                        /* polarity and trigger mode */
        acpi_processor_uid: 0xFFFF_FFFF, /* FF.. Applies to all processors */
        local_interrupt: 1,
        ..Default::default()
    };
    write(w, local_x2apic_nmi, madt_local_x2apic_nmi_offset, 0 as usize);

    // I/O APICs
    let io_apic = AcpiMadtIoApic { header: AcpiSubtableHeader { r#type: MADT_IO_APIC, length: size_of::<AcpiMadtIoApic>() as u8 }, id: 0xf0, address: 0xFEC0_0000 as u32, global_irq_base: 0, ..Default::default() };
    write(w, io_apic, madt_io_apic_offset, 0);

    // isor - interrupt source override0
    write!(w, "First ISOR\r\n").unwrap();
    let isor = AcpiMadtInterruptOverride { header: AcpiSubtableHeader { r#type: MADT_LOCAL_ISOR, length: size_of::<AcpiMadtInterruptOverride>() as u8 }, bus: 0, sourceirq: 0, globalirq: 2, flags: 0 /* polarity and trigger mode = 0 */ };
    write(w, isor, madt_local_isor_offset, 0 as usize);

    write!(w, "Second ISOR\r\n").unwrap();
    let isor = AcpiMadtInterruptOverride { header: AcpiSubtableHeader { r#type: MADT_LOCAL_ISOR, length: size_of::<AcpiMadtInterruptOverride>() as u8 }, bus: 0, sourceirq: 9, globalirq: 9, flags: 0xf /* polarity and trigger mode = 3 */ };
    write(w, isor, madt_local_isor_offset, 1 as usize);

    write!(w, "MADT checksum from {:x} to {:x} store into {:x}\r\n", madt_offset, madt_offset + madt_total_length, ACPI_TABLE_HEADER_CHECKSUM_OFFSET).unwrap();
    write(w, gencsum(madt_offset, madt_offset + madt_total_length), madt_offset, ACPI_TABLE_HEADER_CHECKSUM_OFFSET); // XXX
    debug_assert_eq!(acpi_tb_checksum(madt_offset, madt_offset + madt_total_length), 0);

    // MCFG - Memory Mapped Configuration Space Structure
    let mcfg = AcpiTableMcfg { header: AcpiTableHeader { signature: SIG_MCFG, length: size_of::<AcpiTableMcfg>() as u32, ..AcpiTableHeader::new() }, base_address: 0xE000_0000, end_bus: 0xFF, ..Default::default() };
    write(w, mcfg, mcfg_offset, 0);
    write(w, gencsum(mcfg_offset, mcfg_offset + size_of::<AcpiTableMcfg>()), mcfg_offset, ACPI_TABLE_HEADER_CHECKSUM_OFFSET);

    // HPET - High Precision Event Timer Table
    let hpet = AcpiTableHpet {
        header: AcpiTableHeader { signature: SIG_HPET, length: size_of::<AcpiTableHpet>() as u32, ..AcpiTableHeader::new() },
        hw_block_id: 0x1022_8201, /* PCI Vendor ID, HW Rev ID */
        base_address: AcpiGenericAddress { space_id: 0, bit_width: 64, bit_offset: 0, access_width: 0, address: 0xFED0_0000 },
        hpet_number: 0,
        min_clock_ticks: 0x37EE,
        flags: 0,
    };
    write(w, hpet, hpet_offset, 0);
    write(w, gencsum(hpet_offset, hpet_offset + size_of::<AcpiTableHpet>()), hpet_offset, ACPI_TABLE_HEADER_CHECKSUM_OFFSET);

    round_up_4k(total_size)
}
