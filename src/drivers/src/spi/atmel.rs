use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_ATMEL,
};

/* M25Pxx-specific commands */
pub const CMD_AT25_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_AT25_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_AT25_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_AT25_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_AT25_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_AT25_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_AT25_PP: u8 = 0x02; /* Page Program */
pub const CMD_AT25_SE: u8 = 0x20; /* Sector (4K) Erase */
pub const CMD_AT25_BE: u8 = 0xd8; /* Block (64K) Erase */
pub const CMD_AT25_CE: u8 = 0xc7; /* Chip Erase */
pub const CMD_AT25_DP: u8 = 0xb9; /* Deep Power-down */
pub const CMD_AT25_RES: u8 = 0xab; /* Release from DP, and Read Signature */

pub const FLASH_TABLE: [SpiFlashPartId; 7] = [
    /* AT25X16 */
    SpiFlashPartId::create(0x3015, 9),
    /* AT25DF32 */
    SpiFlashPartId::create(0x47, 10),
    /* AT25X64 */
    SpiFlashPartId::create(0x3017, 11),
    /* AT25Q16 */
    SpiFlashPartId::create(0x4015, 9),
    /* AT25Q32 */
    SpiFlashPartId::create(0x4016, 10),
    /* AT25Q64 */
    SpiFlashPartId::create(0x4017, 11),
    /* AT25Q128 */
    SpiFlashPartId::create(0x4018, 12),
];

pub const SPI_FLASH_ATMEL_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_ATMEL,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_ATMEL_VI];
