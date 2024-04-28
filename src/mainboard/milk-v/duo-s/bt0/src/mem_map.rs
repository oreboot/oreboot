pub const TOP_BASE: usize = 0x0300_0000;
pub const TOP_MISC: usize = TOP_BASE;

// NOTE: The vendor code has PHYD_BASE and PHYD_BASE_ADDR.
// Those are not the same, looks like there was some confusion.
// PHYD_BASE is really PHYD_APB. They also have that. We reduced it.
// What is PHYD anyway?
// TODO: What is the difference between PHY_BASE and PHY_BASE_ADDR?
// plat/cv181x/include/ddr/ddr_sys.h
pub const DDR_SYS_BASE: usize = 0x0800_0000;
pub const PI_BASE: usize = DDR_SYS_BASE + 0x0000;
pub const PHYD_BASE_ADDR: usize = DDR_SYS_BASE; // ?? used in phy_init
pub const PHY_BASE: usize = DDR_SYS_BASE + 0x2000;
pub const DDR_CFG_BASE: usize = DDR_SYS_BASE + 0x4000;
pub const PHYD_APB: usize = DDR_SYS_BASE + 0x6000;
pub const AXI_MON_BASE: usize = DDR_SYS_BASE + 0x8000;
pub const DDR_TOP_BASE: usize = DDR_SYS_BASE + 0xa000;
pub const DDR_BIST_BASE: usize = DDR_SYS_BASE + 0x0001_0000;

pub const DRAM_BASE: usize = 0x8000_0000;
