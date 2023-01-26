use crate::{
    oreboot_tables::{MacAddress, ObFramebuffer, ObGpio},
    sysinfo::{
        Memrange, SpiFlash, Sysinfo, SYSINFO_MAX_GPIOS, SYSINFO_MAX_MACS, SYSINFO_MAX_MEM_RANGES,
    },
};
use alloc::vec::Vec;
use spin::rwlock::RwLock;

pub const CPU_KHZ_DEFAULT: u32 = 200;
#[cfg(LP_SERIAL_CONSOLE)]
pub const SER_IOPORT_DEFAULT: u16 = 0x3f8;
#[cfg(not(LP_SERIAL_CONSOLE))]
pub const SER_IOPORT_DEFAULT: u16 = 0x3f8;

pub static LIB_SYSINFO: RwLock<Sysinfo> = RwLock::new(Sysinfo {
    cpu_khz: CPU_KHZ_DEFAULT,
    cb_serial: 0,
    ser_ioport: SER_IOPORT_DEFAULT,
    ser_base: 0,
    memrange: [Memrange::new(); SYSINFO_MAX_MEM_RANGES],
    cmos_option_table: 0,
    cmos_range_start: 0,
    cmos_range_end: 0,
    cmos_checksum_location: 0,
    vbnv_start: 0,
    vbnv_size: 0,
    version: 0,
    extra_version: 0,
    build: 0,
    compile_time: 0,
    compile_by: 0,
    compile_host: 0,
    compile_domain: 0,
    compiler: 0,
    linker: 0,
    assembler: 0,
    mem_chip_base: 0,
    pcie_ctrl_base: 0,
    cb_version: 0,
    framebuffer: ObFramebuffer::new(),
    num_gpios: 0,
    gpios: [ObGpio::new(); SYSINFO_MAX_GPIOS],
    macs: [MacAddress::new(); SYSINFO_MAX_MACS],
    serialno: 0,
    mbtable: Vec::new(),
    cb_header: 0,
    cb_mainboard: 0,
    vboot_workbuf: 0,
    #[cfg(LP_ARCH_X86)]
    x86_rom_var_mtrr_index: 0,
    tstamp_table: 0,
    cbmem_cons: 0,
    mrc_cache: 0,
    acpi_gnvs: 0,
    acpi_cnvs: 0,
    acpi_rsdp: 0,
    board_id: 0,
    ram_code: 0,
    sku_id: 0,
    fw_config: 0,
    wifi_calibration: 0,
    ramoops_buffer: 0,
    ramoops_buffer_size: 0,
    spi_flash: SpiFlash::new(),
    fmap_offset: 0,
    cbfs_offset: 0,
    cbfs_size: 0,
    boot_media_size: 0,
    mtc_start: 0,
    mtc_size: 0,
    chromeos_vpd: 0,
    mmc_early_wake_status: 0,
    fmap_cache: 0,
    #[cfg(LP_PCI)]
    pacc: PciAccess::new(),
    type_c_info: 0,
    cbfs_ro_mcache_offset: 0,
    cbfs_ro_mcache_size: 0,
    cbfs_rw_mcache_offset: 0,
    cbfs_rw_mcache_size: 0,
});
