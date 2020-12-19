use model::*;

/* Device config space registers. */
const REG_VENDOR_ID: u16 = 0x00;
const REG_DEVICE_ID: u16 = 0x02;
// const REG_COMMAND: u16 = 0x04;
// const REG_STATUS: u16 = 0x06;
// const REG_REVISION_ID: u16 = 0x08;
// const REG_PROG_IF: u16 = 0x09;
// const REG_SUBCLASS: u16 = 0x0A;
// const REG_CLASS: u16 = 0x0B;
// const REG_CACHE_LINE_SIZE: u16 = 0x0C;
// const REG_LATENCY_TIMER: u16 = 0x0D;
const REG_HEADER_TYPE: u16 = 0x0E;
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
// const REG_PRIMARY_BUS: u16 = 0x18;
const REG_SECONDARY_BUS: u16 = 0x19;
const REG_SUBORDINATE_BUS: u16 = 0x1A;
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
const REG_BRIDGE_CONTROL: u16 = 0x3C;

/// Write 32 bits to port
unsafe fn outl(port: u16, val: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{al}"(val));
}
/// Write 8 bits to port
unsafe fn outb(port: u16, val: u8) {
    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
}

// Write 16 bits to port
unsafe fn outw(port: u16, val: u16) {
    llvm_asm!("outw %ax, %dx" :: "{dx}"(port), "{al}"(val));
}
// Read 8 bits from port
fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        llvm_asm!("inb %dx, %al" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    }
    ret
}
// /// Read 16 bits from port
unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    llvm_asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    ret
}
fn pci_read_config16(device: u32, register: u16) -> Result<u16> {
    unsafe {
        outl(0xCF8, device | (register & !3) as u32);
        Ok(inw(0xCFC + (register & 3)))
    }
}
fn pci_read_config8(device: u32, register: u16) -> Result<u8> {
    unsafe {
        outl(0xCF8, device | (register & !3) as u32);
        Ok(inb(0xCFC + (register & 3)))
    }
}
fn pci_write_config8(device: u32, register: u16, val: u8){
    unsafe {
        outl(0xCF8, device | (register & !3) as u32);
        outb(0xCFC + (register & 3), val)
    }
}
fn pci_write_config16(device: u32, register: u16, val: u16){
    unsafe {
        outl(0xCF8, device | (register & !3) as u32);
        outw(0xCFC + (register & 3), val)
    }
}

pub fn scan_bus(w: &mut impl core::fmt::Write, bus: u16) {
    for devfn in 0..0x100 {
        let device = (devfn >> 3) & 0x1F;
        let function = devfn & 0x7;

        let port_address: u32 = 0x8000_0000 | (bus as u32) << 16 | device << 11 | function << 8;
        let ven_id = pci_read_config16(port_address, REG_VENDOR_ID).unwrap();
        let dev_id = pci_read_config16(port_address, REG_DEVICE_ID).unwrap();

        if dev_id == 0xFFFF || ven_id == 0xFFFF || dev_id == 0 {
            continue;
        }

        let hdr_type = pci_read_config16(port_address, REG_HEADER_TYPE).unwrap();
        let hdr_type = hdr_type & 0x7F;

        if hdr_type == 0 {
            writeln!(w, "\n\rPCI: Found PCI device at {:04x}:{:x}.{:x}\r", bus, device, function).unwrap();
            writeln!(w, "PCI: \tVendor ID: {:#06X} Device ID: {:#06X}\r", ven_id, dev_id).unwrap();
            // pci_write_config8(port_address, REG_COMMAND, 0x14);
            if  device == 0x18 && function == 0 {
                pci_write_config8(port_address, 0xc0, 0x03);
                pci_write_config16(port_address, 0xc4, 0x301B);
                pci_write_config16(port_address, 0xc8, 0x4003);
                pci_write_config16(port_address, 0xcc, 0x701A);
                pci_write_config16(port_address, 0xd0, 0x8003);
                pci_write_config16(port_address, 0xd4, 0xB019);
                pci_write_config16(port_address, 0xd8, 0xc003);
                pci_write_config16(port_address, 0xdc, 0xf018);
                write!(w, "Written\r\n").unwrap();
            }
            write!(w, "NN: 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F").unwrap();
            for i in 0..0x100 {
                if i % 0x10 == 0 {
                    write!(w, "\n\r{:02x}:",i).unwrap();
                }
                let reg = pci_read_config8(port_address, i).unwrap();
                write!(w, " {:02x}", reg).unwrap();    
            }
            continue;
        }
        if hdr_type == 1 {
            writeln!(w, "\n\rPCI: Found PCI bridge at {:04x}:{:x}.{:x}\r", bus, device, function).unwrap();
            // devfn += devfn + 8;
            // pci_write_config8(port_address, REG_COMMAND, 0x14);
            
            let b_ctrl = pci_read_config16(port_address, REG_BRIDGE_CONTROL).unwrap();
            writeln!(w, "\t Bridge Ctrl Reg: {:X}\r", b_ctrl).unwrap();
            
            
            
            if  device == 7 && function == 1 {
                pci_write_config8(port_address, REG_SECONDARY_BUS, 1);
                pci_write_config8(port_address, REG_SUBORDINATE_BUS, 1);
                scan_bus(w, 1);
            }
            if  device == 8 && function == 1 {
                pci_write_config8(port_address, REG_SECONDARY_BUS, 2);
                pci_write_config8(port_address, REG_SUBORDINATE_BUS, 2);
                scan_bus(w, 2);
            }

            write!(w, "NN: 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F").unwrap();

            for i in 0..0x100 {
                // if device == 0 && function == 0 {
                //     if i == 0x4 {
                //         pci_write_config8(port_address, i, 0x14);
                //     }
                //     if i == 0x8 {
                //         pci_write_config8(port_address, i, 0x16)
                //     }
                // }
                if i % 0x10 == 0 {
                    write!(w, "\n\r{:x}:",i).unwrap();
                }
                let reg = pci_read_config8(port_address, i).unwrap();
                write!(w, " {:02x}", reg).unwrap();    
            }


        }
        if hdr_type == 2 {
            writeln!(w, "PCI: Found PCI cardbus at {:04x}:{:x}.{:x}\r", bus, device, function).unwrap();
        }
        if hdr_type == 1 || hdr_type == 2 {
            let next_bus = pci_read_config16(port_address, REG_SECONDARY_BUS).unwrap();
            let next_bus = next_bus & 0xFF;

            if next_bus != bus {
                writeln!(w, "PCI: scanning bridge with bus {:X}\r", next_bus).unwrap();
                scan_bus(w, next_bus);
            }
            // 0xB00_0000
            // 0xE00_0000
            continue;
        }
        writeln!(w, "\nPCI: Found header {:X} at {:04X}:{:02X}.{:X}\r", hdr_type, bus, device, function).unwrap();
        writeln!(w, "PCI: \tVendor ID: {:#06X} Device ID: {:#06X}\r", ven_id, dev_id).unwrap();
    }
}

// fn pci_bus_reset(device: u32, bus: u16) {
//     let ctrl_reg = pci_read_config16(device, REG_BRIDGE_CONTROL);
// }
