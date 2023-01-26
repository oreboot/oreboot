use util::helpers::KIB;

pub const LPC_IO_ENABLES: u8 = 0x82;
pub const LPC_GENERIC_MEM_RANGE: u8 = 0x98;
pub const LPC_LGMR_ADDR_MASK: u32 = 0xffff_0000;
pub const LPC_LGMR_EN: u8 = 1 << 0;
pub const LPC_LGMR_WINDOW_SIZE: usize = 64 * KIB;
