#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

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
use soc::SOC;
mod mainboard;
use mainboard::MainBoard;
use uart::i8250::I8250;
mod fabric;
use fabric::fabric;
mod msr;
use msr::msrs;
mod c00;
use c00::c00;
use wrappers::DoD;
use x86_64::registers::model_specific::Msr;

use core::ptr;

fn poke32(a: u32, v: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}
fn poke8(a: u32, v: u8) -> () {
    let y = a as *mut u8;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek32(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}

fn smnhack(w: &mut impl core::fmt::Write, reg: u32, want: u32) -> () {
    let got = smn_read(reg);
    write!(w, "{:x}: got {:x}, want {:x}\r\n", reg, got, want).unwrap();
    if got == want {
        return;
    }
    smn_write(reg, want);
    let got = smn_read(reg);
    write!(w, "Try 2: {:x}: got {:x}, want {:x}\r\n", reg, got, want).unwrap();
}

fn smngotwant(w: &mut impl core::fmt::Write, reg: u32, want: u32) -> () {
    let got = smn_read(reg);
    write!(w, "{:x}: GOT {:x}, WANT {:x}\r\n", reg, got, want).unwrap();
}

fn cpu_init<'a>(w: &mut impl core::fmt::Write, soc: &'a mut soc::SOC) -> Result<(), &'a str> {
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
                    soc.init(w)
                }
                _ => {
                    write!(w, "Unsupported AMD CPU\r\n").unwrap();
                    Err("Unsupported AMD CPU")
                }
            }
        }
        Some(0x19) => {
            // Milan
            soc.init(w)
        }
        _ => {
            write!(w, "Unsupported AMD CPU\r\n").unwrap();
            Err("Unsupported AMD CPU")
        }
    }
}

fn start_bootstrap_core(fdt_address: usize) -> ! {
    let m = &mut MainBoard::new();
    m.init().unwrap();
    let mut text_output_drivers = m.text_output_drivers();
    let console = &mut DoD::new(&mut text_output_drivers);

    // todo: this should do the cpu init.
    // soc is a superset of cpu is a superset of architecture.
    let s = &mut SOC::new();

    for _i in 1..32 {
        console.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    }
    let w = &mut print::WriteToDyn::new(console);

    // Logging.
    smnhack(w, 0x13B1_02F4, 0x00000000u32);
    smnhack(w, 0x13B1_02F0, 0xc9280001u32);
    smnhack(w, 0x13C1_02F4, 0x00000000u32);
    smnhack(w, 0x13C1_02F0, 0xf4180001u32);
    smnhack(w, 0x13D1_02F4, 0x00000000u32);
    smnhack(w, 0x13D1_02F0, 0xc8180001u32);
    smnhack(w, 0x13E1_02F4, 0x00000000u32);
    smnhack(w, 0x13E1_02F0, 0xf5180001u32);
    // IOMMU on    smnhack(w, 0x13F0_0044, 0xc9200001u32);
    smnhack(w, 0x13F0_0044, 0xc9200000u32);
    smnhack(w, 0x13F0_0048, 0x00000000u32);
    // IOMMU on smnhack(w, 0x1400_0044, 0xf4100001u32);
    smnhack(w, 0x1400_0044, 0xf4100000u32);
    smnhack(w, 0x1400_0048, 0x00000000u32);
    // IOMMU on smnhack(w, 0x1410_0044, 0xc8100001u32);
    smnhack(w, 0x1410_0044, 0xc8100000u32);
    smnhack(w, 0x1410_0048, 0x00000000u32);
    // IOMMU on smnhack(w, 0x1420_0044, 0xf5100001u32);
    smnhack(w, 0x1420_0044, 0xf5100000u32);
    smnhack(w, 0x1420_0048, 0x00000000u32);
    smnhack(w, 0x1094_2014, 0x00000000u32);
    smnhack(w, 0x1094_2010, 0x0000000cu32);
    smnhack(w, 0x10A4_2014, 0x00000000u32);
    smnhack(w, 0x10A4_2010, 0x0000000cu32);
    smnhack(w, 0x1074_1014, 0x00000000u32);
    smnhack(w, 0x1074_1010, 0x00000000u32);
    smnhack(w, 0x1074_2014, 0x00000000u32);
    smnhack(w, 0x1074_2010, 0x00000000u32);
    smnhack(w, 0x1074_3014, 0x00000000u32);
    smnhack(w, 0x1074_3010, 0xc6000004u32);
    smnhack(w, 0x1074_4014, 0x00000000u32);
    smnhack(w, 0x1074_4010, 0x00000000u32);
    smnhack(w, 0x10B4_2014, 0x00000000u32);
    smnhack(w, 0x10B4_2010, 0x0000000cu32);
    smnhack(w, 0x1084_3014, 0x00000000u32);
    smnhack(w, 0x1084_3010, 0xf8000004u32);
    smnhack(w, 0x10C4_2014, 0x00000000u32);
    smnhack(w, 0x10C4_2010, 0x0000000cu32);
    smnhack(w, 0x13B1_0044, 0x00000160u32);
    smnhack(w, 0x13C1_0044, 0x00000140u32);
    smnhack(w, 0x13D1_0044, 0x00000120u32);
    smnhack(w, 0x13E1_0044, 0x00000100u32);
    smnhack(w, 0x1010_0018, 0x00636360u32);
    smnhack(w, 0x1050_0018, 0x00646460u32);
    smnhack(w, 0x1020_0018, 0x00414140u32);
    smnhack(w, 0x1060_0018, 0x00424240u32);
    smnhack(w, 0x1060_1018, 0x00000000u32);
    smnhack(w, 0x1060_2018, 0x00000000u32);
    smnhack(w, 0x1030_0018, 0x00212120u32);
    smnhack(w, 0x1070_0018, 0x00222220u32);
    smnhack(w, 0x1070_1018, 0x00000000u32);
    smnhack(w, 0x1070_2018, 0x00000000u32);
    smnhack(w, 0x1040_0018, 0x00020200u32);
    smnhack(w, 0x1080_0018, 0x00030300u32);
    smnhack(w, 0x1090_0018, 0x00000000u32);
    smnhack(w, 0x10A0_0018, 0x00000000u32);
    smnhack(w, 0x10B0_0018, 0x00000000u32);
    smnhack(w, 0x10C0_0018, 0x00000000u32);
    smnhack(w, 0x1110_0018, 0x00000000u32);
    smnhack(w, 0x1120_0018, 0x00000000u32);
    smnhack(w, 0x1130_0018, 0x00000000u32);
    smnhack(w, 0x1140_0018, 0x00010100u32);
    smnhack(w, 0x1110_1018, 0x00000000u32);
    smnhack(w, 0x1120_1018, 0x00000000u32);
    smnhack(w, 0x1130_1018, 0x00000000u32);
    smnhack(w, 0x1140_1018, 0x00000000u32);
    smnhack(w, 0x1110_2018, 0x00000000u32);
    smnhack(w, 0x1120_2018, 0x00000000u32);
    smnhack(w, 0x1130_2018, 0x00000000u32);
    smnhack(w, 0x1140_2018, 0x00000000u32);
    smnhack(w, 0x1110_3018, 0x00000000u32);
    smnhack(w, 0x1120_3018, 0x00000000u32);
    smnhack(w, 0x1130_3018, 0x00000000u32);
    smnhack(w, 0x1140_3018, 0x00000000u32);
    smnhack(w, 0x1110_4018, 0x00000000u32);
    smnhack(w, 0x1120_4018, 0x00000000u32);
    smnhack(w, 0x1130_4018, 0x00000000u32);
    smnhack(w, 0x1140_4018, 0x00000000u32);
    smnhack(w, 0x1110_5018, 0x00000000u32);
    smnhack(w, 0x1120_5018, 0x00000000u32);
    smnhack(w, 0x1130_5018, 0x00000000u32);
    smnhack(w, 0x1140_5018, 0x00000000u32);
    smnhack(w, 0x1110_6018, 0x00000000u32);
    smnhack(w, 0x1120_6018, 0x00000000u32);
    smnhack(w, 0x1130_6018, 0x00000000u32);
    smnhack(w, 0x1140_6018, 0x00000000u32);
    smnhack(w, 0x1110_7018, 0x00000000u32);
    smnhack(w, 0x1120_7018, 0x00000000u32);
    smnhack(w, 0x1130_7018, 0x00000000u32);
    smnhack(w, 0x1140_7018, 0x00000000u32);

    // end logging
    smnhack(w, 0x13b1_0030, 0x00000001u32 << 11);
    smnhack(w, 0x13c1_0030, 0x00000001u32 << 11);
    smnhack(w, 0x13d1_0030, 0x00000001u32 << 11);
    smnhack(w, 0x13e1_0030, 0x00000001u32 << 11);

    // PCIE crs count
    smnhack(w, 0x13b1_0028, 0x02620006u32);
    smnhack(w, 0x13c1_0028, 0x02620006u32);
    smnhack(w, 0x13d1_0028, 0x02620006u32);
    smnhack(w, 0x13e1_0028, 0x02620006u32);

    // PCIE 100 mhz
    smnhack(w, 0x13b1_0020, 0x00000001u32);
    smnhack(w, 0x13c1_0020, 0x00000001u32);
    smnhack(w, 0x13d1_0020, 0x00000001u32);
    smnhack(w, 0x13e1_0020, 0x00000001u32);

    // lovely bridges
    smnhack(w, 0x13b3_C004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13b3_9804, 0x00040007 | 0x100u32);
    smnhack(w, 0x13b3_9404, 0x00040007 | 0x100u32);
    smnhack(w, 0x13b3_9004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13b3_8004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13b3_5404, 0x00040000 | 0x100u32);
    smnhack(w, 0x13b3_5004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_4C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_4804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_4404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_4004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_3C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_3804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_3404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_3004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_2C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_2804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_2404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_2004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_1C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_1804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_1404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13b3_1004, 0x00040005 | 0x100u32);

    smnhack(w, 0x13c3_C004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13c3_9804, 0x00040007 | 0x100u32);
    smnhack(w, 0x13c3_9404, 0x00040007 | 0x100u32);
    smnhack(w, 0x13c3_9004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13c3_8004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13c3_5404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_5004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_4C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_4804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_4404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_4004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_3C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_3804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_3404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_3004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_2C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_2804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_2404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_2004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_1C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_1804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_1404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13c3_1004, 0x00040005 | 0x100u32);

    smnhack(w, 0x13d3_C004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13d3_9804, 0x00040007 | 0x100u32);
    smnhack(w, 0x13d3_9404, 0x00040007 | 0x100u32);
    smnhack(w, 0x13d3_9004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13d3_8004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13d3_5404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_5004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_4C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_4804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_4404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_4004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_3C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_3804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_3404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_3004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_2C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_2804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_2404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_2004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_1C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_1804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_1404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13d3_1004, 0x00040005 | 0x100u32);

    smnhack(w, 0x13e3_C004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13e3_9804, 0x00040007 | 0x100u32);
    smnhack(w, 0x13e3_9404, 0x00040007 | 0x100u32);
    smnhack(w, 0x13e3_9004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13e3_8004, 0x00040000 | 0x100u32);
    smnhack(w, 0x13e3_5404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_5004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_4C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_4804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_4404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_4004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_3C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_3804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_3404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_3004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_2C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_2804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_2404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_2004, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_1C04, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_1804, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_1404, 0x00040005 | 0x100u32);
    smnhack(w, 0x13e3_1004, 0x00040001 | 0x100u32);

    smnhack(w, 0x13b102e0, 0xc9000101);
    smnhack(w, 0x13b102e4, 0x00000000);
    smnhack(w, 0x13b102e8, 0xc9100003);
    smnhack(w, 0x13b102ec, 0x00000000);
    smnhack(w, 0x13b10300, 0x00000000);
    smnhack(w, 0x13b10304, 0x00000000);
    smnhack(w, 0x13b10308, 0x00000000);
    smnhack(w, 0x13b1030c, 0x00000000);
    smnhack(w, 0x13b102f0, 0xc9280001);
    smnhack(w, 0x13b102f4, 0x00000000);
    smnhack(w, 0x13c102e0, 0x00000000);
    smnhack(w, 0x13c102e4, 0x00000000);
    smnhack(w, 0x13c102e8, 0xf4000003);
    smnhack(w, 0x13c102ec, 0x00000000);
    smnhack(w, 0x13c10300, 0x00000000);
    smnhack(w, 0x13c10304, 0x00000000);
    smnhack(w, 0x13c10308, 0x00000000);
    smnhack(w, 0x13c1030c, 0x00000000);
    smnhack(w, 0x13c102f0, 0xf4180001);
    smnhack(w, 0x13c102f4, 0x00000000);
    smnhack(w, 0x13d102e0, 0x00000000);
    smnhack(w, 0x13d102e4, 0x00000000);
    smnhack(w, 0x13d102e8, 0xc8000003);
    smnhack(w, 0x13d102ec, 0x00000000);
    smnhack(w, 0x13d10300, 0x00000000);
    smnhack(w, 0x13d10304, 0x00000000);
    smnhack(w, 0x13d10308, 0x00000000);
    smnhack(w, 0x13d1030c, 0x00000000);
    smnhack(w, 0x13d102f0, 0xc8180001);
    smnhack(w, 0x13d102f4, 0x00000000);
    smnhack(w, 0x13e102e0, 0x00000000);
    smnhack(w, 0x13e102e4, 0x00000000);
    smnhack(w, 0x13e102e8, 0xf5000003);
    smnhack(w, 0x13e102ec, 0x00000000);
    smnhack(w, 0x13e10300, 0x00000000);
    smnhack(w, 0x13e10304, 0x00000000);
    smnhack(w, 0x13e10308, 0x00000000);
    smnhack(w, 0x13e1030c, 0x00000000);
    smnhack(w, 0x13e102f0, 0xf5180001);
    smnhack(w, 0x13e102f4, 0x00000000);

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
    // unsafe {
    //    write!(w, "0x1b is {:x} \r\n", apic_base).unwrap();
    //}
    if true {
        msrs(w);
    }

    match cpu_init(w, s) {
        Ok(()) => {}
        Err(_e) => {
            write!(w, "Error from amd_init acknowledged--continuing anyway\r\n").unwrap();
        }
    }

    if true {
        fabric(w);
    }

    if false {
        msrs(w);
    }
    c00(w);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    poke32(0xfee000d0, 0x1000000);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    write!(w, "loading payload with fdt_address {}\r\n", fdt_address).unwrap();

    boot(w, fdt_address);

    write!(w, "Unexpected return from payload\r\n").unwrap();
    arch::halt()
}

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    // See <https://developer.amd.com/resources/epyc-resources/epyc-specifications/>, "Processor Programming Reference (PPR) for Family 17h Model 31h, Revision B0 Processors"
    let apic_base = unsafe { Msr::new(0x1b).read() }; // Note: This MSR is per-thread
    let bootstrap_core = apic_base & (1 << 8) != 0;
    if bootstrap_core {
        start_bootstrap_core(fdt_address);
    } else {
        // See coreboot:src/cpu/x86/smm/smihandler.c function "nodeid".
        // APICx020: bits 31...24 (8 bits): APIC_ID; APIC defaults to 0xFEE0_0000
        let apic_id = (peek32(0xFEE0_0020) >> 24) as u8;

        let post = &mut IOPort;
        let mut p: [u8; 1] = [0x33; 1];
        p[0] = apic_id;
        post.pwrite(&p, 0x80).unwrap();
        loop {
            arch::halt();
        }
    }
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
