#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(abi_x86_interrupt)]

use arch::ioport::IOPort;
use boot::boot;
use core::fmt::Write;
use core::panic::PanicInfo;
use cpu::model::amd_family_id;
use cpu::model::amd_model_id;
use model::Driver;
use print;
use raw_cpuid::CpuId;
use smn::{smn_read, smn_write};
use soc::soc_init;
// use uart::amdmmio::UART;
use uart::debug_port::DebugPort;
use uart::i8250::I8250;
mod mainboard;
use mainboard::MainBoard;
mod msr;
use msr::msrs;
// mod c00;
// use c00::c00;
mod interrupts;
use interrupts::init_idt;
use x86_64::registers::model_specific::Msr;
extern crate heapless; // v0.4.x
use heapless::consts::*;
use heapless::Vec;
use wrappers::DoD;
use x86_64::instructions::interrupts::int3;

use core::ptr;

fn smnhack(w: &mut impl core::fmt::Write, reg: u32, want: u32) -> () {
    let got = smn_read(reg);
    write!(w, "{:x}: got {:x}, want {:x}\r\n", reg, got, want).unwrap();
    if got == want {
        return;
    }
    // smn_write(reg, want);
    // let got = smn_read(reg);
    // write!(w, "Try 2: {:x}: got {:x}, want {:x}\r\n", reg, got, want).unwrap();
}

fn cpu_init(w: &mut impl core::fmt::Write) -> Result<(), &str> {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info() {
        Some(vendor) => {
            if vendor.as_string() != "AuthenticAMD" {
                panic!("Only AMD is supported");
            }
        }
        None => {
            panic!("Could not determine whether or not CPU is AMD");
        }
    }
    // write!(w, "CPU Model is: {}\r\n", cpuid.get_extended_function_info().as_ref().map_or_else(|| "n/a", |extfuninfo| extfuninfo.processor_brand_string().unwrap_or("unreadable"),)); // "AMD EPYC TITUS N-Core Processor"
    let amd_family_id = cpuid.get_feature_info().map(|info| amd_family_id(&info));
    let amd_model_id = cpuid.get_feature_info().map(|info| amd_model_id(&info));
    match amd_family_id {
        Some(family_id) => {
            match amd_model_id {
                Some(model_id) => {
                    write!(w, "AMD CPU: family {:X}h, model {:X}h\r\n", family_id, model_id)
                        .unwrap();
                }
                None => (),
            }
        }
        None => (),
    }
    match amd_family_id {
        Some(0x17) => {
            match amd_model_id {
                Some(0x18) => {
                    // Picasso :-)
                    soc_init(w)
                }
                _ => {
                    write!(w, "Unsupported AMD CPU\r\n").unwrap();
                    Err("Unsupported AMD CPU")
                }
            }
        }
        _ => {
            write!(w, "Unsupported AMD CPU\r\n").unwrap();
            Err("Unsupported AMD CPU")
        }
    }
}

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let m = &mut MainBoard::new();
    m.init().unwrap();
    let mut text_output_drivers = m.text_output_drivers();
    let console = &mut DoD::new(&mut text_output_drivers);

    console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    let w = &mut print::WriteToDyn::new(console);

    init_idt();

    if false {
        write!(w, "Let's go BOOM!\r\n").unwrap();
        //panic!("AAAAAAAAAH"); <-- this works :)
        unsafe {
            // llvm_asm!("int3" :::: "volatile");
            int3();
            // llvm_asm!("xorl %ebx, %ebx\ndiv %ebx" : /* no outputs */ : /* no inputs */ : "ebx" : "volatile");
        }
        write!(w, "Didn't explode :(\r\n").unwrap();
    }

    // disable legacy interrupts
    // smnhack(w, 0x13F0_0004, 0x0010_0400u32);
    // smnhack(w, 0x13F0_0064, 0x0010_0400u32); // 847405

    // It is hard to say if we need to do this.
    if true {
        let v = unsafe { Msr::new(0xc001_1004).read() };
        write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
    if true {
        let v = unsafe { Msr::new(0xc001_1005).read() };
        write!(w, "c001_1005 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
    unsafe {
        write!(w, "0x1b is {:x} \r\n", Msr::new(0x1b).read()).unwrap();
    }

    if true {
        msrs(w);
    }

    match cpu_init(w) {
        Ok(()) => {}
        Err(_e) => {
            write!(w, "Error from amd_init acknowledged--continuing anyway\r\n").unwrap();
        }
    }

    if false {
        msrs(w);
    }
    // TODO: Is this specific to Rome?
    // c00(w);

    boot(w, fdt_address);

    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let uart0 = &mut I8250::new(0x3f8, 0, IOPort {});
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
