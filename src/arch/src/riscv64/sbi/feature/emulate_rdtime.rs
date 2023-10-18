use super::super::runtime::SupervisorContext;
use log::println;
use riscv::register::cycle;

const DEBUG: bool = false;
const DEBUG_RDCYCLE: bool = true;
const DEBUG_RDTIME: bool = false;

const RDINS_MASK: usize = 0xFFFF_F07F;
const RDTIME_INST: usize = 0xC010_2073;
const RDCYCLE_INST: usize = 0xC000_2073;

// FIXME: DO NOT HARDCODE; use sbi crate definitions etc
const CLINT_BASE_JH7110: usize = 0x0200_0000;
const CLINT_MTIMER_OFFSET: usize = 0xbff8;

pub fn read32(reg: usize) -> u32 {
    unsafe { core::ptr::read_volatile(reg as *mut u32) }
}

fn get_mtime(clint_base: usize) -> u64 {
    let mtimer = clint_base + CLINT_MTIMER_OFFSET;
    let l = read32(mtimer) as u64;
    let h = read32(mtimer + 4) as u64;
    (h << 32) | l
}

#[inline]
pub fn emulate_rdtime(ctx: &mut SupervisorContext, ins: usize) -> bool {
    match ins & RDINS_MASK {
        RDCYCLE_INST => {
            //  c0002573     rdcycle a0
            let reg = ((ins >> 7) & 0b1_1111) as u8;
            if DEBUG && DEBUG_RDCYCLE {
                println!("[SBI] rdcycle {ins:08x} ({reg})");
            }
            let cycle_usize = cycle::read64() as usize;
            set_register_xi(ctx, reg, cycle_usize);
            // skip current instruction, 4 bytes
            ctx.mepc = ctx.mepc.wrapping_add(4);
            if DEBUG && DEBUG_RDCYCLE {
                println!("[SBI] rdcycle {cycle_usize:x}");
            }
            true
        }
        RDTIME_INST => {
            //  c0102573     rdtime  a0    (reg = 10)
            //  c01027f3     rdtime  a5    (reg = 15)
            // rdtime is actually a csrrw instruction
            let reg = ((ins >> 7) & 0b1_1111) as u8;
            if DEBUG && DEBUG_RDTIME {
                println!("[SBI] rdtime {ins:08x} ({reg})");
            }
            // let mtime = riscv::register::time::read64();
            let mtime = get_mtime(CLINT_BASE_JH7110);
            let time_usize = mtime as usize;
            set_register_xi(ctx, reg, time_usize);
            // skip current instruction, 4 bytes
            ctx.mepc = ctx.mepc.wrapping_add(4);
            if DEBUG && DEBUG_RDTIME {
                println!("[SBI] rdtime {time_usize:x}");
            }
            true
        }
        _ => false, // is not an rdXXX instruction
    }
}

#[inline]
fn set_register_xi(ctx: &mut SupervisorContext, i: u8, data: usize) {
    let registers = unsafe { &mut *(ctx as *mut _ as *mut [usize; 31]) };
    assert!(i <= 31, "i should be valid register target");
    if i == 0 {
        // x0, don't modify
        return;
    }
    registers[(i - 1) as usize] = data;
}
