use crate::runtime::SupervisorContext;
use riscv::register::{mstatus, satp};

// There is no `sfence.vma` in 1.9.1 privileged spec; however there is a `sfence.vm`.
// For backward compability, here we emulate the first instruction using the second one.
// sfence.vma: | 31..25 funct7=SFENCE.VMA(0001001) | 24..20 rs2/asid | 19..15 rs1/vaddr |
//               14..12 funct3=PRIV(000) | 11..7 rd, =0 | 6..0 opcode=SYSTEM(1110011) |
// sfence.vm(1.9):  | 31..=20 SFENCE.VM(000100000100) | 19..15 rs1/vaddr |
//               14..12 funct3=PRIV(000) | 11..7 rd, =0 | 6..0 opcode=SYSTEM(1110011) |

#[inline]
pub fn emulate_sfence_vma(ctx: &mut SupervisorContext, ins: usize) -> bool {
    if ins & 0xFE007FFF == 0x12000073 {
        // sfence.vma instruction
        // discard rs2 // let _rs2_asid = ((ins >> 20) & 0b1_1111) as u8;
        // let rs1_vaddr = ((ins >> 15) & 0b1_1111) as u8;
        // read paging mode from satp (sptbr)
        let satp_bits = satp::read().bits();
        // bit 63..20 is not readable and writeable on K210, so we cannot
        // decide paging type from the 'satp' register.
        // that also means that the asid function is not usable on this chip.
        // we have to fix it to be Sv39.
        let ppn = satp_bits & 0xFFF_FFFF_FFFF; // 43..0 PPN WARL
                                               // write to sptbr
        let sptbr_bits = ppn & 0x3F_FFFF_FFFF;
        unsafe { asm!("csrw 0x180, {}", in(reg) sptbr_bits) }; // write to sptbr
                                                               // enable paging (in v1.9.1, mstatus: | 28..24 VM[4:0] WARL | ... )
        let mut mstatus_bits: usize;
        unsafe { asm!("csrr {}, mstatus", out(reg) mstatus_bits) };
        mstatus_bits &= !0x1F00_0000;
        mstatus_bits |= 9 << 24;
        unsafe { asm!("csrw mstatus, {}", in(reg) mstatus_bits) };
        ctx.mstatus = mstatus::read();
        // emulate with sfence.vm (declared in privileged spec v1.9)
        unsafe { asm!(".word 0x10400073") }; // sfence.vm x0
                                             // ::"r"(rs1_vaddr)
        ctx.mepc = ctx.mepc.wrapping_add(4); // skip current instruction
        true
    } else {
        false // is not a sfence.vma instruction
    }
}
