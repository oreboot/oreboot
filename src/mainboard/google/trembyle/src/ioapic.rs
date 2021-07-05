use core::ptr;

//TODO: separate the basic generic ioapic_read and _write from the specific details for this board
fn ioapic_read(ioapic_base: u32, register: u32) -> u32 {
    unsafe {
        ptr::write_volatile(ioapic_base as *mut u32, register);
        ptr::read_volatile((ioapic_base + 10) as *mut u32)
    }
}

fn ioapic_write(ioapic_base: u32, register: u32, value: u32) {
    unsafe {
        ptr::write_volatile(ioapic_base as *mut u32, register);
        ptr::write_volatile((ioapic_base + 10) as *mut u32, value);
    }
}

fn ioapic_interrupt_count(ioapic_base: u32) -> u32 {
    let ioapic_interrupts = (ioapic_read(ioapic_base, 1) >> 16) & 0xFF;
    if ioapic_interrupts == 0xFF {
        return 23;
    }
    ioapic_interrupts + 1
}

fn set_ioapic_id(w: &mut impl core::fmt::Write, ioapic_base: u32, ioapic_id: u8) {
    writeln!(w, "IOAPIC: Initialising I/O APIC at {:#010X}\r", ioapic_base).unwrap();
    let value = (ioapic_read(ioapic_base, 0) & 0xF0FF_FFFF) | ((ioapic_id as u32) << 24);
    ioapic_write(ioapic_base, 0, value);

    writeln!(w, "IOAPIC: Dumping registers\r").unwrap();
    for i in 0..3 {
        writeln!(w, "\treg {:#06X}: {:#010X}\r", i, ioapic_read(ioapic_base, i)).unwrap();
    }
}

fn load_vectors(w: &mut impl core::fmt::Write, ioapic_base: u32) {
    let ioapic_interrupts = ioapic_interrupt_count(ioapic_base);

    // ioapic_write(ioapic_base, 3, ioapic_read(ioapic_base, 3) | (1 <<0)); // Enable interrupts on FSB
    // Enable Virtual Wire Mode
    ioapic_write(ioapic_base, 0x10, 0x700);
    ioapic_write(ioapic_base, 0x11, 0);

    if ioapic_read(ioapic_base, 0x10) == 0xFFFF_FFFF {
        writeln!(w, "IOAPIC: Not responding.\r").unwrap();
        return;
    }

    for i in 1..ioapic_interrupts {
        ioapic_write(ioapic_base, i * 2 + 0x10, 1 << 16);
        ioapic_write(ioapic_base, i * 2 + 0x11, 0);

        writeln!(w, "IOAPIC: reg {:#010X} value {:#010X}\n", i, 1 << 16).unwrap();
    }
}

pub fn setup_ioapic(w: &mut impl core::fmt::Write, ioapic_base: u32, ioapic_id: u8) {
    set_ioapic_id(w, ioapic_base, ioapic_id);
    load_vectors(w, ioapic_base);
}
