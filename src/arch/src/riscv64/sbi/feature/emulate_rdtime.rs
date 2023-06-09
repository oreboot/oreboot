use super::super::runtime::SupervisorContext;
use core::arch::asm;
use log::println;
use riscv::register::cycle;

const DEBUG_THIS: bool = false;

// 0x4102573

#[inline]
pub fn emulate_rdtime(ctx: &mut SupervisorContext, ins: usize) -> bool {
    //  c0002573     rdcycle a0
    if ins & 0xFFFFF07F == 0xC0002073 {
        let rd = ((ins >> 7) & 0b1_1111) as u8;
        let cycle_usize = cycle::read64() as usize;
        set_register_xi(ctx, rd, cycle_usize);
        ctx.mepc = ctx.mepc.wrapping_add(4); // skip current instruction
        println!("[rustsbi] rdcycle {:x}\r", cycle_usize);
        true
    }
    // TODO: IS THIS CORRECT? Linux calls rdtime a *lot*.
    else if ins & 0xFFFFF07F == 0xC0102073 {
        // rdtime is actually a csrrw instruction
        let rd = ((ins >> 7) & 0b1_1111) as u8;
        let mtime: u64;
        unsafe {
            asm!("csrr {}, time", out(reg) mtime);
        }
        let time_usize = mtime as usize;
        set_register_xi(ctx, rd, time_usize);
        ctx.mepc = ctx.mepc.wrapping_add(4); // skip current instruction
        let x = time_usize / 0x1000;
        if DEBUG_THIS && x > 1 && x % 0x200 == 0 {
            println!("[rustsbi] rdtime {:x}\r", time_usize);
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
