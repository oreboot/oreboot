#[cfg(target_arch = "x86_64")]
use crate::arch::x86::sysinfo::LIB_SYSINFO;
#[cfg(target_arch = "x86_64")]
use arch::x86_64::mmio::{read32, write32};

use crate::pci::{pci_bus, pci_func, pci_slot, PciDevT};
use log::debug;
use types::bit;
use util::{helpers::retry, timer::udelay};

/*
 * iATU Unroll-specific register definitions
 */

pub const PCIE_ATU_UNR_REGION_CTRL1: usize = 0x00;
pub const PCIE_ATU_UNR_REGION_CTRL2: usize = 0x04;
pub const PCIE_ATU_UNR_LOWER_BASE: usize = 0x0c;
pub const PCIE_ATU_UNR_UPPER_BASE: usize = 0x0c;
pub const PCIE_ATU_UNR_LIMIT: usize = 0x10;
pub const PCIE_ATU_UNR_LOWER_TARGET: usize = 0x14;
pub const PCIE_ATU_UNR_UPPER_TARGET: usize = 0x18;
pub const PCIE_ATU_REGION_INDEX0: usize = 0x0;
pub const PCIE_ATU_TYPE_CFG0: usize = 0x4;
pub const PCIE_ATU_TYPE_CFG1: usize = 0x5;
pub const PCIE_ATU_ENABLE: usize = bit(31) as usize;
pub const ATU_CTRL2: usize = PCIE_ATU_UNR_REGION_CTRL2;
pub const ATU_ENABLE: usize = PCIE_ATU_ENABLE;

pub const LINK_WAIT_IATU_US: usize = 1000;
pub const LINK_WAIT_MAX_IATU_RETRIES: usize = 5;

/*
 * ATU & endpoint config space base address offsets relative to
 * PCIe controller base address.
 */
pub const QCOM_ATU_BASE_OFFSET: usize = 0x1000;
pub const QCOM_EP_CFG_OFFSET: usize = 0x100000;
pub const QCOM_EP_CFG_SIZE: usize = 0x1000;

pub fn pcie_atu_bus(x: usize) -> usize {
    (x & 0xff) << 24
}

pub fn pcie_atu_dev(x: usize) -> usize {
    (x & 0x1f) << 19
}

pub fn pcie_atu_func(x: usize) -> usize {
    (x & 0x7) << 16
}

/// Register address builder
pub fn pcie_get_atu_outb_unr_reg_offset(region: usize) -> usize {
    region << 9
}

pub fn lower_32_bits(n: usize) -> u32 {
    n as u32
}

pub fn upper_32_bits(n: usize) -> u32 {
    ((n >> 16) >> 16) as u32
}

pub fn dw_pcie_writel_iatu(atu_base: usize, index: u16, reg: u32, val: u32) {
    let offset = pcie_get_atu_outb_unr_reg_offset(index as usize);
    unsafe { write32(atu_base + offset as usize + reg as usize, val) };
}

pub fn dw_pcie_readl_iatu(atu_base: usize, index: u16, reg: u32) -> u32 {
    let offset = pcie_get_atu_outb_unr_reg_offset(index as usize);
    unsafe { read32(atu_base + offset as usize + reg as usize) }
}

pub fn dw_pcie_prog_outbound_atu(
    atu_base: usize,
    index: u16,
    type_: u32,
    cfg_addr: u64,
    pcie_addr: u64,
    cfg_size: u32,
) {
    dw_pcie_writel_iatu(
        atu_base,
        index,
        PCIE_ATU_UNR_LOWER_BASE as u32,
        lower_32_bits(cfg_addr as usize),
    );
    dw_pcie_writel_iatu(
        atu_base,
        index,
        PCIE_ATU_UNR_UPPER_BASE as u32,
        upper_32_bits(cfg_addr as usize),
    );
    dw_pcie_writel_iatu(
        atu_base,
        index,
        PCIE_ATU_UNR_LIMIT as u32,
        lower_32_bits((cfg_addr + cfg_size as u64 - 1) as usize),
    );
    dw_pcie_writel_iatu(
        atu_base,
        index,
        PCIE_ATU_UNR_LOWER_TARGET as u32,
        lower_32_bits(pcie_addr as usize),
    );
    dw_pcie_writel_iatu(
        atu_base,
        index,
        PCIE_ATU_UNR_UPPER_TARGET as u32,
        upper_32_bits(pcie_addr as usize),
    );
    dw_pcie_writel_iatu(atu_base, index, PCIE_ATU_UNR_REGION_CTRL1 as u32, type_);
    dw_pcie_writel_iatu(
        atu_base,
        index,
        PCIE_ATU_UNR_REGION_CTRL2 as u32,
        PCIE_ATU_ENABLE as u32,
    );

    /*
     * Make sure ATU enable takes effect before any subsequent config
     * and I/O accesses.
     */
    let condition = dw_pcie_readl_iatu(atu_base, index, ATU_CTRL2 as u32) & ATU_ENABLE as u32;
    if retry(
        LINK_WAIT_MAX_IATU_RETRIES as u32,
        condition,
        udelay,
        LINK_WAIT_IATU_US as u32,
    ) != 0
    {
        return;
    }

    debug!("outbound IATU couldn't be enabled after 5ms");
}

/// Get PCIe MMIO configuration space base address
pub fn pci_map_bus(dev: PciDevT) -> usize {
    let out =
        |config_base: usize, devfn: usize| -> usize { config_base + (QCOM_EP_CFG_SIZE * devfn) };

    let current_dev = pci_bus(dev);
    let devfn = (pci_slot(dev) << 3) | pci_func(dev);

    /*
     * Extract PCIe controller base from coreboot and derive the ATU and
     * endpoint config base addresses from it.
     */
    let cntrlr_base = (*LIB_SYSINFO.read()).pcie_ctrl_base as usize;
    let config_base = cntrlr_base + QCOM_EP_CFG_OFFSET as usize;
    let config_size = QCOM_EP_CFG_SIZE as u32;
    let atu_base = cntrlr_base + QCOM_ATU_BASE_OFFSET as usize;

    /*
     * Cache the dev. For same dev, ATU mapping is not needed for each
     * request.
     */
    if current_dev as u32 == dev {
        return out(config_base, devfn as usize);
    }

    let busdev = pcie_atu_bus(current_dev as usize)
        | pcie_atu_dev(pci_slot(dev) as usize)
        | pcie_atu_func(pci_func(dev) as usize);

    let atu_type = if current_dev == 1 {
        PCIE_ATU_TYPE_CFG0 as u32
    } else {
        PCIE_ATU_TYPE_CFG1 as u32
    };

    dw_pcie_prog_outbound_atu(
        atu_base,
        PCIE_ATU_REGION_INDEX0 as u16,
        atu_type,
        config_base as u64,
        busdev as u64,
        config_size,
    );

    out(config_base, devfn as usize)
}
