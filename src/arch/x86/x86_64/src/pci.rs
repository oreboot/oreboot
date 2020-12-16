use model::*;

/* Device config space registers. */
const REG_VENDOR_ID: u16 = 0x00;
// const REG_DEVICE_ID: u16 = 0x02;
// const REG_COMMAND: u16 = 0x04;
// const REG_STATUS: u16 = 0x06;
// const REG_REVISION_ID: u16 = 0x08;
// const REG_PROG_IF: u16 = 0x09;
// const REG_SUBCLASS: u16 = 0x0A;
// const REG_CLASS: u16 = 0x0B;
// const REG_CACHE_LINE_SIZE: u16 = 0x0C;
// const REG_LATENCY_TIMER: u16 = 0x0D;
// const REG_HEADER_TYPE: u16 = 0x0E;
// const REG_BIST: u16 = 0x0F;
// const REG_BAR0: u16 = 0x10;
// const REG_BAR1: u16 = 0x14;
// const REG_BAR2: u16 = 0x18;
// const REG_BAR3: u16 = 0x1C;
// const REG_BAR4: u16 = 0x20;
// const REG_BAR5: u16 = 0x24;
// const REG_CARDBUS_CIS_POINTER: u16 = 0x28;
// const REG_SUBSYS_VENDOR_ID: u16 = 0x2C;
// const REG_SUBSYS_ID: u16 = 0x2E;
// const REG_DEV_OPROM_BASE: u16 = 0x30;
// const REG_CAP_POINTER: u16 = 0x34;
// const REG_INTERRUPT_LINE: u16 = 0x3C;
// const REG_INTERRUPT_PIN: u16 = 0x3D;
// const REG_MIN_GRANT: u16 = 0x3E;
// const REG_MAX_LATENCY: u16 = 0x3F;

/* Bridge config space registers. */
const REG_PRIMARY_BUS: u16 = 0x18;
// const REG_SECONDARY_BUS: u16 = 0x19;
// const REG_SUBORDINATE_BUS: u16 = 0x1A;
// const REG_SECONDARY_LATENCY: u16 = 0x1B;
// const REG_IO_BASE: u16 = 0x1C;
// const REG_IO_LIMIT: u16 = 0x1D;
// const REG_SECONDARY_STATUS: u16 = 0x1E;
// const REG_MEMORY_BASE: u16 = 0x20;
// const REG_MEMORY_LIMIT: u16 = 0x22;
// const REG_PREFETCH_MEM_BASE: u16 = 0x24;
// const REG_PREFETCH_MEM_LIMIT: u16 = 0x26;
// const REG_PREFETCH_BASE_UPPER: u16 = 0x28;
// const REG_PREFETCH_LIMIT_UPPER: u16 = 0x2C;
// const REG_IO_BASE_UPPER: u16 = 0x30;
// const REG_IO_LIMIT_UPPER: u16 = 0x32;
// const REG_BRIDGE_OPROM_BASE: u16 = 0x38;
// const REG_BRIDGE_CONTROL: u16 = 0x3C;

/// Write 32 bits to port
unsafe fn outl(port: u16, val: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{al}"(val));
}

/// Read 32 bits from port
unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    llvm_asm!("inl %dx, %eax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    ret
}

fn pci_read_config(device: u32, register: u16) -> Result<u32> {
    unsafe {
        outl(0xCF8, device | (register & !3) as u32);
        Ok(inl(0xCFC + (register & 3)))
    }
}

pub fn scan_bus(w: &mut impl core::fmt::Write, bus: u8) {
    for devfn in 0..0x100 {
        let device = (devfn >> 3) & 0x1F;
        let function = devfn & 0x7;

        let port_address = 0x8000_0000 | device << 11 | function << 8;
        let value = pci_read_config(port_address, REG_VENDOR_ID).unwrap();

        if value == 0xFFFF || value == 0xFFFF_0000 || value == 0xFFFF_FFFF || value == 0 {
            continue;
        }

        let hdr_type = pci_read_config(port_address, 0xC).unwrap();
        let hdr_type = (hdr_type >> 16) & 0x7F;
        
        let ven_id = value & 0xFFFF;
        let dev_id = (value >> 8) & 0xFFFF;

        if hdr_type == 0 {
            writeln!(w, "PCI: Found PCI device at {:#04x}:{:x}:{:x}\r", bus, device, function).unwrap();
            writeln!(w, "PCI: \tVendor ID: {:#06X} Device ID: {:#06X}\r",  ven_id, dev_id).unwrap();
            continue;
        }
        if hdr_type == 1 {
            writeln!(w, "PCI: Found PCI bridge at {:#04x}:{:x}:{:x}\r", bus, device, function).unwrap();
        }
        if hdr_type == 2 {
            writeln!(w, "PCI: Found PCI cardbus at {:#04x}:{:x}:{:x}\r", bus, device, function).unwrap();
        }
        if hdr_type == 1 || hdr_type == 2 {
            let next_bus = pci_read_config(port_address, REG_PRIMARY_BUS).unwrap();
            let next_bus = ((next_bus >> 8) & 0xFF) as u8;

            if next_bus != bus {
                scan_bus(w, next_bus);
            }
            continue;
        }
        writeln!(w, "PCI: Found header {:X} at {:04X}:{:02X}:{:X}\r", hdr_type, bus, device, function).unwrap();
        writeln!(w, "PCI: \tVendor ID: {:#06X} Device ID: {:#06X}\r", ven_id, dev_id).unwrap();

    }
}
