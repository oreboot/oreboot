use core::arch::asm;
use riscv::register::{
    mcause::{self, Exception, Trap},
    mepc,
    mtvec::{self, TrapMode},
};

// This function will lookup virtual memory module and page table system
// if memory fault from address `addr` is a page fault, return true
// otherwise when not a page fault, or when paging is disabled, return false
pub fn is_page_fault(addr: usize) -> bool {
    if !is_s1p9_mstatus_sv39_mode() {
        return false;
    }
    if !check_sext_sv39(addr) {
        return true;
    }
    let base_ppn = read_sptbr_ppn();
    let level_2_ppn = unsafe {
        let vpn2 = (addr >> 30) & 0x1FF;
        let ptr = ((base_ppn << 12) as *const usize).add(vpn2);
        let level_2_pte = if let Ok(ans) = try_read_address(ptr) {
            ans
        } else {
            // level 2 ppn read failed
            return true;
        };
        if (level_2_pte & 0b1) == 0 {
            // level 2 pte is not valid
            return true;
        }
        if (level_2_pte & 0b1110) != 0b0000 && (level_2_pte >> 10) & 0x3FFFF != 0 {
            // 大页对齐出错，返回页异常
            // level 2 huge page align not satisfied
            return true;
        }
        (level_2_pte >> 10) & 0x3F_FFFF_FFFF
    };
    let level_1_ppn = unsafe {
        let vpn1 = (addr >> 21) & 0x1FF;
        let ptr = ((level_2_ppn << 12) as *const usize).add(vpn1);
        let level_1_pte = if let Ok(ans) = try_read_address(ptr) {
            ans
        } else {
            // level 1 ppn read failed
            return true;
        };
        if (level_1_pte & 0b1) == 0 {
            // level 1 pte is not valid
            return true;
        }
        if (level_1_pte & 0b1110) != 0b0000 && (level_1_pte >> 10) & 0x1FF != 0 {
            // 大页对齐出错，返回页异常
            // level 1 huge page align not satisfied
            return true;
        }
        (level_1_pte >> 10) & 0x3F_FFFF_FFFF
    };
    let _ppn = unsafe {
        let vpn0 = (addr >> 12) & 0x1FF;
        let ptr = ((level_1_ppn << 12) as *const usize).add(vpn0);
        let final_pte = if let Ok(ans) = try_read_address(ptr) {
            ans
        } else {
            // level 0 ppn read failed
            return true;
        };
        if (final_pte & 0b1) == 0 {
            // level 0 pte is not valid
            return true;
        }
        if (final_pte & 0b1110) == 0b0000 {
            // level 0 page cannot have leaves
            return true;
        }
        (final_pte >> 10) & 0x3F_FFFF_FFFF
    };
    // 到这一步都没有错误，说明查找是成功的，并非页异常
    false
}

// read Privileged Spec v1.9 defined mstatus to decide virtual memory mode
// 9 -> Sv39
fn is_s1p9_mstatus_sv39_mode() -> bool {
    let mut mstatus_bits: usize;
    unsafe { asm!("csrr {}, mstatus", out(reg) mstatus_bits) };
    let mode = (mstatus_bits >> 24) & 0b1_1111;
    mode == 9
}

// if sext is not valid, raise a page fault
fn check_sext_sv39(addr: usize) -> bool {
    let addr_b38 = (addr >> 38) & 0b1 == 1;
    let sext = addr >> 39;
    if addr_b38 && sext == 0x1FFFFFF {
        return true;
    }
    if !addr_b38 && sext == 0 {
        return true;
    }
    false
}

// get Privileged Spec v1.9 defined sptbr root page table base
fn read_sptbr_ppn() -> usize {
    let sptbr_bits: usize;
    unsafe { asm!("csrr {}, 0x180", out(reg) sptbr_bits) };
    sptbr_bits & 0xFFF_FFFF_FFFF
}

#[derive(Debug)]
struct LoadAccessFault;

unsafe fn try_read_address(ptr: *const usize) -> Result<usize, LoadAccessFault> {
    let saved_mtvec_address = init_trap_vector();
    let ans: usize;
    asm!("li    tp, 0");
    asm!("ld    {}, 0({})", out(reg) ans, in(reg) ptr);
    let has_error: usize;
    asm!("mv    {}, tp", out(reg) has_error);
    let ans = if has_error == 1 {
        Err(LoadAccessFault)
    } else {
        Ok(ans)
    };
    recover_trap_vector(saved_mtvec_address);
    return ans;
}

extern "C" fn memory_fault_catch_handler() {
    let cause = mcause::read().cause();
    if cause == Trap::Exception(Exception::LoadFault) {
        unsafe { asm!("li   tp, 1") }; // tp = 1 说明发生了错误
    }
    let bad_ins_addr = mepc::read();
    let ins_16 = unsafe { core::ptr::read_volatile(bad_ins_addr as *const u16) };
    let bytes = if ins_16 & 0b11 != 0b11 { 2 } else { 4 };
    mepc::write(mepc::read().wrapping_add(bytes)); // skip current load instruction
}

fn init_trap_vector() -> usize {
    let mut addr = delegate_catch_trap as usize;
    if addr & 0x2 != 0 {
        addr = addr.wrapping_add(0x2); // 必须对齐到4个字节
    }
    let saved_mtvec_address = mtvec::read().address();
    unsafe { mtvec::write(addr, TrapMode::Direct) };
    saved_mtvec_address
}

fn recover_trap_vector(saved_mtvec_address: usize) {
    unsafe { mtvec::write(saved_mtvec_address, TrapMode::Direct) }
}

#[naked]
#[link_section = ".text"]
unsafe extern "C" fn delegate_catch_trap() -> ! {
    asm!(
        ".align 4", // align to 4 bytes
        "addi   sp, sp, -8*16
        sd      ra, 8*0(sp)
        sd      t0, 8*1(sp)
        sd      t1, 8*2(sp)
        sd      t2, 8*3(sp)
        sd      t3, 8*4(sp)
        sd      t4, 8*5(sp)
        sd      t5, 8*6(sp)
        sd      t6, 8*7(sp)
        sd      a0, 8*8(sp)
        sd      a1, 8*9(sp)
        sd      a2, 8*10(sp)
        sd      a3, 8*11(sp)
        sd      a4, 8*12(sp)
        sd      a5, 8*13(sp)
        sd      a6, 8*14(sp)
        sd      a7, 8*15(sp)",
        "call   {memory_fault_catch_handler}",
        "ld     ra, 8*0(sp)
        ld      t0, 8*1(sp)
        ld      t1, 8*2(sp)
        ld      t2, 8*3(sp)
        ld      t3, 8*4(sp)
        ld      t4, 8*5(sp)
        ld      t5, 8*6(sp)
        ld      t6, 8*7(sp)
        ld      a0, 8*8(sp)
        ld      a1, 8*9(sp)
        ld      a2, 8*10(sp)
        ld      a3, 8*11(sp)
        ld      a4, 8*12(sp)
        ld      a5, 8*13(sp)
        ld      a6, 8*14(sp)
        ld      a7, 8*15(sp)
        addi    sp, sp, 8*16",
        "mret",
        memory_fault_catch_handler = sym memory_fault_catch_handler,
        options(noreturn)
    )
}
