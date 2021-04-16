#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
use boot::boot;
use core::panic::PanicInfo;
use cpu::model::amd_family_id;
use cpu::model::amd_model_id;
use raw_cpuid::CpuId;
use soc::soc_init;
use wrappers::DoD;
mod c00;
mod mainboard;
mod msr;
use arch::ioport::IOPort;
use core::fmt::Write;
use mainboard::MainBoard;
use model::Driver;
use uart::i8250::I8250;

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
                    write!(w, "AMD CPU: family {:X}h, model {:X}h\r\n", family_id, model_id).unwrap();
                }
                None => (),
            }
        }
        None => (),
    }
    match amd_family_id {
        Some(0x17) => {
            match amd_model_id {
                Some(v) if v >= 0x31 => {
                    // Rome
                    soc_init(w)
                }
                _ => {
                    write!(w, "Unsupported AMD CPU\r\n").unwrap();
                    Err("Unsupported AMD CPU")
                }
            }
        }
        Some(0x19) => {
            // Milan
            soc_init(w)
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
    let mut text_outputs = m.text_outputs();
    let console = &mut DoD::new(&mut text_outputs);
    let w = &mut print::WriteTo::new(console);
    cpu_init(w);
    boot(w, fdt_address);
    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let io = IOPort {};
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
