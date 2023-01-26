use crate::oreboot_tables::{FlashMmapWindow, MacAddress, ObFramebuffer, ObGpio};
#[cfg(LP_PCI)]
use crate::pci::PciAccess;
use alloc::vec::Vec;

pub const SYSINFO_MAX_GPIOS: usize = 8;
pub const SYSINFO_MAX_MACS: usize = 10;
pub const SYSINFO_MAX_MEM_RANGES: usize = 32;
pub const SYSINFO_MAX_MMAP_WINDOWS: usize = 2;

#[repr(C)]
pub struct Sysinfo {
    pub cpu_khz: u32,
    pub cb_serial: usize,
    pub ser_ioport: u16,
    pub ser_base: u32,
    pub memrange: [Memrange; SYSINFO_MAX_MEM_RANGES],
    pub cmos_option_table: usize,
    pub cmos_range_start: u32,
    pub cmos_range_end: u32,
    pub cmos_checksum_location: u32,
    pub vbnv_start: u32,
    pub vbnv_size: u32,
    pub version: usize,
    pub extra_version: usize,
    pub build: usize,
    pub compile_time: usize,
    pub compile_by: usize,
    pub compile_host: usize,
    pub compile_domain: usize,
    pub compiler: usize,
    pub linker: usize,
    pub assembler: usize,
    pub mem_chip_base: usize,
    /* Base address of PCIe controller */
    pub pcie_ctrl_base: usize,
    pub cb_version: usize,
    pub framebuffer: ObFramebuffer,
    pub num_gpios: i32,
    pub gpios: [ObGpio; SYSINFO_MAX_GPIOS],
    pub macs: [MacAddress; SYSINFO_MAX_MACS],
    pub serialno: usize,
    /* Pointer to the multiboot table */
    pub mbtable: Vec<u32>,
    pub cb_header: usize,
    pub cb_mainboard: usize,
    pub vboot_workbuf: usize,
    #[cfg(LP_ARCH_X86)]
    pub x86_rom_var_mtrr_index: i32,
    pub tstamp_table: usize,
    pub cbmem_cons: usize,
    pub mrc_cache: usize,
    pub acpi_gnvs: usize,
    pub acpi_cnvs: usize,
    pub acpi_rsdp: usize,
    pub board_id: u32,
    pub ram_code: u32,
    pub sku_id: u32,
    /// A payload using this field is responsible for ensuring it checks its
    /// value against UNDEFINED_FW_CONFIG before using it.
    pub fw_config: u64,
    pub wifi_calibration: usize,
    pub ramoops_buffer: u64,
    pub ramoops_buffer_size: u32,
    pub spi_flash: SpiFlash,
    pub fmap_offset: u64,
    pub cbfs_offset: u64,
    pub cbfs_size: u64,
    pub boot_media_size: u64,
    pub mtc_start: u64,
    pub mtc_size: u32,
    pub chromeos_vpd: usize,
    pub mmc_early_wake_status: i32,
    /* Pointer to FMAP cache in CBMEM */
    pub fmap_cache: usize,
    #[cfg(LP_PCI)]
    pub pacc: PciAccess,
    /* USB Type-C Port Configuration Info */
    pub type_c_info: usize,
    /* CBFS RW/RO Metadata Cache */
    pub cbfs_ro_mcache_offset: usize,
    pub cbfs_ro_mcache_size: u32,
    pub cbfs_rw_mcache_offset: usize,
    pub cbfs_rw_mcache_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Memrange {
    base: u64,
    size: u64,
    type_: u32,
}

impl Memrange {
    pub const fn new() -> Self {
        Self {
            base: 0,
            size: 0,
            type_: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpiFlash {
    size: u32,
    sector_size: u32,
    erase_cmd: u32,
    mmap_window_count: u32,
    mmap_table: [FlashMmapWindow; SYSINFO_MAX_MMAP_WINDOWS],
}

impl SpiFlash {
    pub const fn new() -> Self {
        Self {
            size: 0,
            sector_size: 0,
            erase_cmd: 0,
            mmap_window_count: 0,
            mmap_table: [FlashMmapWindow::new(); SYSINFO_MAX_MMAP_WINDOWS],
        }
    }
}
