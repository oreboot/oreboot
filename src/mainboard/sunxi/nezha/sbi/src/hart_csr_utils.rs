use alloc::vec::Vec;
use riscv::register::{
    medeleg, mideleg,
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
    let mideleg = mideleg::read();
    let mut delegs = Vec::new();
    if mideleg.usoft() {
        delegs.push("usoft")
    }
    if mideleg.utimer() {
        delegs.push("utimer")
    }
    if mideleg.uext() {
        delegs.push("uext")
    }
    if mideleg.ssoft() {
        delegs.push("ssoft")
    }
    if mideleg.stimer() {
        delegs.push("stimer")
    }
    if mideleg.sext() {
        delegs.push("sext")
    }
    println!(
        "[rustsbi] mideleg: {} ({:#x})\r",
        delegs.join(", "),
        mideleg.bits()
    );
}

#[inline]
fn print_medeleg() {
    let medeleg = medeleg::read();
    let mut delegs = Vec::new();
    if medeleg.instruction_misaligned() {
        delegs.push("ima")
    }
    if medeleg.instruction_fault() {
        delegs.push("ia") // instruction access
    }
    if medeleg.illegal_instruction() {
        delegs.push("illinsn")
    }
    if medeleg.breakpoint() {
        delegs.push("bkpt")
    }
    if medeleg.load_misaligned() {
        delegs.push("lma")
    }
    if medeleg.load_fault() {
        delegs.push("la") // load access
    }
    if medeleg.store_misaligned() {
        delegs.push("sma")
    }
    if medeleg.store_fault() {
        delegs.push("sa") // store access
    }
    if medeleg.user_env_call() {
        delegs.push("uecall")
    }
    if medeleg.supervisor_env_call() {
        delegs.push("secall")
    }
    if medeleg.machine_env_call() {
        delegs.push("mecall")
    }
    if medeleg.instruction_page_fault() {
        delegs.push("ipage")
    }
    if medeleg.load_page_fault() {
        delegs.push("lpage")
    }
    if medeleg.store_page_fault() {
        delegs.push("spage")
    }
    println!(
        "[rustsbi] medeleg: {} ({:#x})\r",
        delegs.join(", "),
        medeleg.bits()
    );
}

fn pmp_get(n: usize) -> Option<(usize, usize, usize)> {
    if n >= PMP_COUNT {
        return None;
    }
    let t1;
    let mut addr;
    let log2len;
    let pmpcfg_shift = (n & 7) << 3;
    let cfgmask = 0xff << pmpcfg_shift;
    let pmpcfg = if n <= 8 {
        pmpcfg0::read() & cfgmask
    } else {
        pmpcfg2::read() & cfgmask
    };
    let port = pmpcfg >> pmpcfg_shift;
    addr = match n {
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
