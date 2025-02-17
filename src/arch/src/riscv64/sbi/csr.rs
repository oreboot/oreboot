use log::{print, println};
use riscv::register::{self as reg, medeleg, mideleg, mie, misa};

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

pub fn print_info() {
    print_misa();
    print_mideleg();
    print_medeleg();
    print_mie();
    print_pmp();
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
            misa::XLEN::XLEN32 => "RV32",
            misa::XLEN::XLEN64 => "RV64",
            misa::XLEN::XLEN128 => "RV128",
        };
        print!("[rustsbi] misa: {mxl_str}");
        for ext in 'A'..='Z' {
            if isa.has_extension(ext) {
                print!("{}", ext);
            }
        }
        println!("");
    }
}

#[inline]
fn print_mideleg() {
    print!("[rustsbi] mideleg: ");
    let mideleg = mideleg::read();
    if mideleg.ssoft() {
        print!("ssoft ")
    }
    if mideleg.stimer() {
        print!("stimer ")
    }
    if mideleg.sext() {
        print!("sext ")
    }
    println!("({:#08x})", mideleg.bits());
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
    if mie.mtimer() {
        print!("mtimer ")
    }
    if mie.stimer() {
        print!("stimer ")
    }
    if mie.mext() {
        print!("mext ")
    }
    if mie.sext() {
        print!("sext ")
    }
    println!("({:#08x})", mie.bits());
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
    if medeleg.instruction_page_fault() {
        print!("ipage ")
    }
    if medeleg.load_page_fault() {
        print!("lpage ")
    }
    if medeleg.store_page_fault() {
        print!("spage ")
    }
    println!("({:#08x})", medeleg.bits());
}

fn read_pmp(n: usize) -> usize {
    match n {
        0 => reg::pmpaddr0::read(),
        1 => reg::pmpaddr1::read(),
        2 => reg::pmpaddr2::read(),
        3 => reg::pmpaddr3::read(),
        4 => reg::pmpaddr4::read(),
        5 => reg::pmpaddr5::read(),
        6 => reg::pmpaddr6::read(),
        7 => reg::pmpaddr7::read(),
        8 => reg::pmpaddr8::read(),
        9 => reg::pmpaddr9::read(),
        10 => reg::pmpaddr10::read(),
        11 => reg::pmpaddr11::read(),
        12 => reg::pmpaddr12::read(),
        13 => reg::pmpaddr13::read(),
        14 => reg::pmpaddr14::read(),
        15 => reg::pmpaddr15::read(),
        _ => 0,
    }
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
        reg::pmpcfg0::read().bits & cfgmask
    } else {
        reg::pmpcfg2::read().bits & cfgmask
    };
    let port = pmpcfg >> pmpcfg_shift;
    let mut addr = read_pmp(n);
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

fn print_pmp() {
    let mut size;
    for i in 0..PMP_COUNT {
        if let Some((port, addr, l2l)) = pmp_get(i) {
            if (port & PMP_A) == 0 {
                continue;
            }
            size = if l2l < 64 { 1usize << l2l } else { 0 };
            if (port & PMP_A_TOR) == PMP_A_TOR {
                let start = read_pmp(i - 1) << PMP_SHIFT;
                let end = addr;
                print!("[rustsbi] PMP{i}: 0x{start:>08x} - 0x{end:>08x} (A",);
            } else {
                let start = addr;
                let end = addr + size - 1;
                print!("[rustsbi] PMP{i}: 0x{start:>08x} - 0x{end:>08x} (A",);
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
            println!(")");
        }
    }
}
