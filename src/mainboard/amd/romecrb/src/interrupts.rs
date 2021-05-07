use lazy_static::lazy_static;
use x86_64::structures::idt::EntryOptions;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::PrivilegeLevel;

fn outb(port: u16, val: u8) {
    unsafe {
        llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("Exception: Breakpoint.\r\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception: Double fault.\r\n{:#?}", stack_frame);
    arch::halt();
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("Exception: General protection fault.\r\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    panic!("Exception: Division by zero.\r\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn interrupt_handler(stack_frame: InterruptStackFrame) {
    panic!("Interrupt.\r\n{:#?}", stack_frame);
}


//lazy_static! {
//    static ref IDT: InterruptDescriptorTable = {
//        let mut idt = InterruptDescriptorTable::new();
//        idt.breakpoint.set_handler_fn(breakpoint_handler);
//        idt.double_fault.set_handler_fn(double_fault_handler);
//        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
//        idt.divide_error.set_handler_fn(divide_error_handler);
//        idt[32].set_handler_fn(interrupt_handler);
//        idt
//    };
//}

pub fn init_idt() {
//    IDT.load();

    unsafe {
        let mut idt = 0x100000 as *mut InterruptDescriptorTable;
        (*idt) = InterruptDescriptorTable::new();
        (*idt).breakpoint.set_handler_fn(breakpoint_handler).set_privilege_level(PrivilegeLevel::Ring0);
        (*idt).double_fault.set_handler_fn(double_fault_handler).set_privilege_level(PrivilegeLevel::Ring0);
        (*idt).general_protection_fault.set_handler_fn(general_protection_fault_handler).set_privilege_level(PrivilegeLevel::Ring0);
        (*idt).divide_error.set_handler_fn(divide_error_handler).set_privilege_level(PrivilegeLevel::Ring0);
        //(*idt)[32].set_handler_fn(interrupt_handler);
        (*idt).load();
    }
 unsafe {
            llvm_asm!("xorl %ebx, %ebx\ndiv %ebx" : /* no outputs */ : /* no inputs */ : "ebx" : "volatile");
        }
    panic!("X");
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
