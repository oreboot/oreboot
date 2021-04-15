#![feature(llvm_asm)]

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

unsafe fn outl(port: u16, val: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{al}"(val));
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    unsafe {
        outl(0x80, 0x11);
    }
    panic!("Exception: Breakpoint.\r\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception: Double fault.\r\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: &mut InterruptStackFrame) {
    panic!("Exception: Division by zero.\r\n{:#?}", stack_frame);
}

//lazy_static! {
//    static ref IDT: InterruptDescriptorTable = {
//        let mut idt = InterruptDescriptorTable::new();
//        idt.breakpoint.set_handler_fn(breakpoint_handler);
//        idt.double_fault.set_handler_fn(double_fault_handler);
//        idt.divide_error.set_handler_fn(divide_error_handler);
//        idt
//    };
//}

pub fn init_idt() {
    //    IDT.load();
    unsafe {
        let mut idt = 0x100000 as *mut InterruptDescriptorTable;
        (*idt).breakpoint.set_handler_fn(breakpoint_handler);
        (*idt).double_fault.set_handler_fn(double_fault_handler);
        (*idt).divide_error.set_handler_fn(divide_error_handler);
        (*idt).load();
    }
}
