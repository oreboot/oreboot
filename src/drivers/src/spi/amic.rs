use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_AMIC,
};

/* A25L-specific commands */
pub const CMD_A25_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_A25_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_A25_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_A25_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_A25_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_A25_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_A25_PP: u8 = 0x02; /* Page Program */
pub const CMD_A25_SE: u8 = 0x20; /* Sector (4K) Erase */
pub const CMD_A25_BE: u8 = 0xd8; /* Block (64K) Erase */
pub const CMD_A25_CE: u8 = 0xc7; /* Chip Erase */
pub const CMD_A25_DP: u8 = 0xb9; /* Deep Power-down */
pub const CMD_A25_RES: u8 = 0xab; /* Release from DP, and Read Signature */

pub const FLASH_TABLE: [SpiFlashPartId; 9] = [
    /* A25L16PU */
    SpiFlashPartId::create(0x2015, 9),
    /* A25L16PT */
    SpiFlashPartId::create(0x2025, 9),
    /* A25L080 */
    SpiFlashPartId::create(0x3014, 8),
    /* A25L016 */
    SpiFlashPartId::create(0x3015, 9),
    /* A25L032 */
    SpiFlashPartId::create(0x3016, 10),
    /* A25LQ080 */
    SpiFlashPartId::create(0x4014, 8),
    /* A25LQ16 */
    SpiFlashPartId::create(0x4015, 9),
    /* A25LQ32 */
    SpiFlashPartId::create(0x4016, 10),
    /* A25LQ64 */
    SpiFlashPartId::create(0x4017, 11),
];

pub const SPI_FLASH_AMIC_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_AMIC,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_AMIC_VI];
