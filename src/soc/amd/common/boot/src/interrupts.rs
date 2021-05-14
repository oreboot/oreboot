fn outb(port: u16, val: u8) {
    unsafe {
        llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
    }
}

pub fn init_pics() {
    // Topology: CPU <- PIC A <- PIC B

    // Initialize both PICs
    outb(0x20, 0x11); // PIC A: ICW4 needed; ICW1 is being issued; edge trigger mode
    outb(0xA0, 0x11); // PIC B: ICW4 needed; ICW1 is being issued; edge trigger mode

    // Set bases of both PICs
    outb(0x21, 0x20); // PIC A: first interrupt is 0x20
    outb(0xA1, 0x28); // PIC B: first interrupt is 0x28

    // Set up PIC cascade
    outb(0x21, 0x04); // PIC A's third input (input #2) is what PIC B's uplink is connected to
    outb(0xA1, 0x02);

    // Set 80x86 mode on both PICs
    outb(0x21, 0x01); // PIC A: Set to 80x86 mode
    outb(0xA1, 0x01); // PIC B: Set to 80x86 mode

    // Mask all interrupts
    outb(0x21, 0xFB); // PIC A: Mask all interrupts except the one caused by PIC B (Rationale: if someone unmasks a regular interrupt in the PIC, the PIC cascade is an implementation detail and thus the user shouldn't need to set it manually)
    outb(0xA1, 0xFF); // PIC B: Mask all interrupts
}
