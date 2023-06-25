use super::super::runtime::SupervisorContext;
use core::arch::asm;
use log::println;
use riscv::register::cycle;

const DEBUG: bool = true;
const DEBUG_RDCYCLE: bool = true;
const DEBUG_RDTIME: bool = false;

// 0x4102573

pub fn read32(reg: usize) -> u32 {
    unsafe { core::ptr::read_volatile(reg as *mut u32) }
}

const CLINT_BASE: usize = 0x0200_0000;
const CLINT_MTIMER: usize = CLINT_BASE + 0xbff8;
fn get_mtime() -> u64 {
    let l = read32(CLINT_MTIMER) as u64;
    let h = read32(CLINT_MTIMER + 4) as u64;
    (h << 32) | l
}

/*

[SBI] DEBUG: instruction 0xc0102573 at 0xffffffff80406e2e: Exception(IllegalInstruction)
[    0.007642] sched_clock: 64 bits at 4MHz, resolution 250ns, wraps every 2199023255500ns
[SBI] DEBUG: instruction 0xc01027f3 at 0xffffffff80406f5a: Exception(IllegalInstruction)
[SBI] DEBUG: instruction 0xc0102573 at 0x0000000040003f86: Exception(IllegalInstruction)
[SBI] DEBUG: instruction 0xc0102573 at 0x0000000040003f8a: Exception(IllegalInstruction)
[SBI] DEBUG: instruction 0x0000000c at 0x000000000000000c: Exception(InstructionFault)

*/

#[inline]
pub fn emulate_rdtime(ctx: &mut SupervisorContext, ins: usize) -> bool {
    //  c0002573     rdcycle a0
    if ins & 0xFFFFF07F == 0xC0002073 {
        if DEBUG && DEBUG_RDTIME {
            println!("[SBI] rdcycle");
        }
        let rd = ((ins >> 7) & 0b1_1111) as u8;
        let cycle_usize = cycle::read64() as usize;
        set_register_xi(ctx, rd, cycle_usize);
        // skip current instruction, 4 bytes
        ctx.mepc = ctx.mepc.wrapping_add(4);
        if DEBUG && DEBUG_RDTIME {
            println!("[SBI] rdcycle {cycle_usize:x}");
        }
        true
    }
    // TODO: IS THIS CORRECT? Linux calls rdtime a *lot*.
    //  c0102573     rdtime  a0
    else if ins & 0xFFFFF07F == 0xC0102073 {
        if DEBUG && DEBUG_RDTIME {
            println!("[SBI] rdtime");
        }
        // rdtime is actually a csrrw instruction
        let rd = ((ins >> 7) & 0b1_1111) as u8;
        // let mtime = riscv::register::time::read64();
        let mtime = get_mtime();
        let time_usize = mtime as usize;
        set_register_xi(ctx, rd, time_usize);
        // skip current instruction, 4 bytes
        ctx.mepc = ctx.mepc.wrapping_add(4);
        let x = time_usize / 0x1000;
        if DEBUG && DEBUG_RDTIME
        /*&& x > 1 && x % 0x200 == 0*/
        {
            println!("[SBI] rdtime {time_usize:x}");
        }
        true
    } else {
        false // is not a rdtime instruction
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
