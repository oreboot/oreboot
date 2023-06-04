use riscv::register::{
    medeleg, mideleg, mie,
    misa::{self, MXL},
    pmpaddr0, pmpaddr1, pmpaddr10, pmpaddr11, pmpaddr12, pmpaddr13, pmpaddr14, pmpaddr15, pmpaddr2,
    pmpaddr3, pmpaddr4, pmpaddr5, pmpaddr6, pmpaddr7, pmpaddr8, pmpaddr9, pmpcfg0, pmpcfg2,
};
use rustsbi::{print, println};

pub const PMP_COUNT: usize = 16;
pub const PMP_SHIFT: usize = 2;
pub const PMP_R: usize = 0x01;
pub const PMP_W: usize = 0x02;
pub const PMP_X: usize = 0x04;
pub const PMP_A: usize = 0x18;
pub const PMP_A_TOR: usize = 0x08;
// pub const PMP_A_NA4:usize = 0x10;
pub const PMP_A_NAPOT: usize = 0x18;
pub const PMP_L: usize = 0x80;

pub fn print_hart_csrs() {
    print_misa();
    print_mideleg();
    print_medeleg();
    print_mie();
}
#[inline]
fn ctz(mut x: usize) -> usize {
    let mut ret = 0;
    while (x & 1usize) != 0 {
        ret += 1;
        x >>= 1
    }
    ret
}

#[inline]
fn print_misa() {
    let isa = misa::read();
    if let Some(isa) = isa {
        let mxl_str = match isa.mxl() {
            MXL::XLEN32 => "RV32",
            MXL::XLEN64 => "RV64",
            MXL::XLEN128 => "RV128",
        };
        print!("[rustsbi] misa: {}", mxl_str);
        for ext in 'A'..='Z' {
            if isa.has_extension(ext) {
                print!("{}", ext);
            }
        }
        println!("\r");
    }
}

#[inline]
fn print_mideleg() {
    print!("[rustsbi] mideleg: ");
    let mideleg = mideleg::read();
    if mideleg.ssoft() {
        print!("ssoft ")
    }
    if mideleg.usoft() {
        print!("usoft ")
    }
    if mideleg.stimer() {
        print!("stimer ")
    }
    if mideleg.utimer() {
        print!("utimer ")
    }
    if mideleg.sext() {
        print!("sext ")
    }
    if mideleg.uext() {
        print!("uext ")
    }
    println!("({:#08x})\r", mideleg.bits());
}

#[inline]
fn print_mie() {
    print!("[rustsbi] mie: ");
    let mie = mie::read();
    if mie.msoft() {
        print!("msoft ")
    }
    if mie.ssoft() {
        print!("ssoft ")
    }
    if mie.usoft() {
        print!("usoft ")
    }
    if mie.mtimer() {
        print!("mtimer ")
    }
    if mie.stimer() {
        print!("stimer ")
    }
    if mie.utimer() {
        print!("utimer ")
    }
    if mie.mext() {
        print!("mext ")
    }
    if mie.sext() {
        print!("sext ")
    }
    if mie.uext() {
        print!("uext ")
    }
    println!("({:#08x})\r", mie.bits());
}

#[inline]
fn print_medeleg() {
    print!("[rustsbi] medeleg: ");
    let medeleg = medeleg::read();
    if medeleg.instruction_misaligned() {
        print!("ima ")
    }
    if medeleg.instruction_fault() {
        print!("ia ") // instruction access
    }
    if medeleg.illegal_instruction() {
        print!("illinsn ")
    }
    if medeleg.breakpoint() {
        print!("bkpt ")
    }
    if medeleg.load_misaligned() {
        print!("lma ")
    }
    if medeleg.load_fault() {
        print!("la ") // load access
    }
    if medeleg.store_misaligned() {
        print!("sma ")
    }
    if medeleg.store_fault() {
        print!("sa ") // store access
    }
    if medeleg.user_env_call() {
        print!("uecall ")
    }
    if medeleg.supervisor_env_call() {
        print!("secall ")
    }
    if medeleg.machine_env_call() {
        print!("mecall ")
    }
    if medeleg.instruction_page_fault() {
        print!("ipage ")
    }
    if medeleg.load_page_fault() {
        print!("lpage ")
    }
    if medeleg.store_page_fault() {
        print!("spage ")
    }
    println!("({:#08x})\r", medeleg.bits());
}

fn pmp_get(n: usize) -> Option<(usize, usize, usize)> {
    if n >= PMP_COUNT {
        return None;
    }
    let t1;
    let log2len;
    let pmpcfg_shift = (n & 7) << 3;
    let cfgmask = 0xff << pmpcfg_shift;
    let pmpcfg = if n <= 8 {
        pmpcfg0::read().bits & cfgmask
    } else {
        pmpcfg2::read().bits & cfgmask
    };
    let port = pmpcfg >> pmpcfg_shift;
    let mut addr = match n {
        0 => pmpaddr0::read(),
        1 => pmpaddr1::read(),
        2 => pmpaddr2::read(),
        3 => pmpaddr3::read(),
        4 => pmpaddr4::read(),
        5 => pmpaddr5::read(),
        6 => pmpaddr6::read(),
        7 => pmpaddr7::read(),
        8 => pmpaddr8::read(),
        9 => pmpaddr9::read(),
        10 => pmpaddr10::read(),
        11 => pmpaddr11::read(),
        12 => pmpaddr12::read(),
        13 => pmpaddr13::read(),
        14 => pmpaddr14::read(),
        15 => pmpaddr15::read(),
        _ => 0,
    };
    if (port & PMP_A) == PMP_A_NAPOT {
        addr |= 0x1ff;
        if addr == usize::MAX {
            addr = 0;
            log2len = 64;
        } else {
            t1 = ctz(!addr);
            addr = (addr & !((1usize << t1) - 1)) << PMP_SHIFT;
            log2len = t1 + PMP_SHIFT + 1;
        }
    } else {
        addr <<= PMP_SHIFT;
        log2len = PMP_SHIFT
    }
    Some((port, addr, log2len))
}

pub fn print_hart_pmp() {
    let mut size;
    for i in 0..PMP_COUNT {
        if let Some((port, addr, l2l)) = pmp_get(i) {
            if (port & PMP_A) == 0 {
                continue;
            }
            size = if l2l < 64 { 1usize << l2l } else { 0 };
            if (port & PMP_A_TOR) == PMP_A_TOR {
                print!(
                    "PMP{}\t 0x{:x} - 0x{:x} (A",
                    i,
                    match i {
                        0 => 0,
                        1 => pmpaddr0::read(),
                        2 => pmpaddr1::read(),
                        3 => pmpaddr2::read(),
                        4 => pmpaddr3::read(),
                        5 => pmpaddr4::read(),
                        6 => pmpaddr5::read(),
                        7 => pmpaddr6::read(),
                        8 => pmpaddr7::read(),
                        9 => pmpaddr8::read(),
                        10 => pmpaddr9::read(),
                        11 => pmpaddr10::read(),
                        12 => pmpaddr11::read(),
                        13 => pmpaddr12::read(),
                        14 => pmpaddr13::read(),
                        15 => pmpaddr14::read(),
                        _ => 0,
                    } << PMP_SHIFT,
                    addr
                )
            } else {
                print!(
                    "PMP{}\t: 0x{:>08x} - 0x{:>08x} (A",
                    i,
                    addr,
                    addr + size - 1
                );
            }

            if (port & PMP_L) != 0 {
                print!(",L");
            }
            if (port & PMP_R) != 0 {
                print!(",R");
            }
            if (port & PMP_W) != 0 {
                print!(",W");
            }
            if (port & PMP_X) != 0 {
                print!(",X");
            }
            print!(")\r\n")
        }
    }
}
