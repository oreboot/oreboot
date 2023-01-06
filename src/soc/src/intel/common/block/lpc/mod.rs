use crate::intel::{
    apollolake::pci_devs::PCH_DEV_LPC,
    common::block::{
        gpmr::{gpmr_write32, GPMR_LPCGMR, GPMR_LPCIOE},
        lpc::{LPC_GENERIC_MEM_RANGE, LPC_IO_ENABLES, LPC_LGMR_ADDR_MASK, LPC_LGMR_EN},
    },
};

use payload::drivers::pci_map_bus_ops::{
    pci_read_config16, pci_read_config32, pci_write_config16, pci_write_config32,
};

mod lpc_def;
pub use lpc_def::*;

pub const LPC_IOE_EC_62_66: u32 = 1 << 11;
pub const LPC_IOE_KBC_60_64: u32 = 1 << 10;
pub const LPC_IOE_LGE_200: u32 = 1 << 8;

pub fn lpc_enable_fixed_io_ranges(mut io_enables: u16) -> u16 {
    let reg_io_enables = pci_read_config16(PCH_DEV_LPC, LPC_IO_ENABLES as u16);
    io_enables |= reg_io_enables;
    pci_write_config16(PCH_DEV_LPC, LPC_IO_ENABLES as u16, io_enables);

    if cfg!(feature = "soc_intel_common_block_lpc_mirror_to_gpmr") {
        gpmr_write32(GPMR_LPCIOE, io_enables as u32);
    }

    io_enables
}

pub fn lpc_open_mmio_window(base: usize, size: usize) {
    let mut lgmr = pci_read_config32(PCH_DEV_LPC, LPC_GENERIC_MEM_RANGE as u16);

    if lgmr & (LPC_LGMR_EN as u32) != 0 {
        //error!(
        //    "LPC: Cannot open window to resource {:x} size {:x}",
        //    base, size
        //);
        //error!("LPC: MMIO window already in use");
        return;
    }

    if size > LPC_LGMR_WINDOW_SIZE as usize {
        //error!(
        //    "LPC: Resource {:x} size {:x} larger than window({:x})",
        //    base, size, LPC_LGMR_WINDOW_SIZE
        //);
    }

    lgmr = ((base as u32) & LPC_LGMR_ADDR_MASK) | LPC_LGMR_EN as u32;

    pci_write_config32(PCH_DEV_LPC, LPC_GENERIC_MEM_RANGE as u16, lgmr);
    if cfg!(feature = "soc_intel_common_block_lpc_mirror_to_gpmr") {
        gpmr_write32(GPMR_LPCGMR, lgmr as u32);
    }
}
