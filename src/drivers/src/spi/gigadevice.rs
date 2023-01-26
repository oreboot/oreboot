use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_GIGADEVICE,
};

/* GD25Pxx-specific commands */
pub const CMD_GD25_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_GD25_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_GD25_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_GD25_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_GD25_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_GD25_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_GD25_PP: u8 = 0x02; /* Page Program */
pub const CMD_GD25_SE: u8 = 0x20; /* Sector (4K) Erase */
pub const CMD_GD25_BE: u8 = 0xd8; /* Block (64K) Erase */
pub const CMD_GD25_CE: u8 = 0xc7; /* Chip Erase */
pub const CMD_GD25_DP: u8 = 0xb9; /* Deep Power-down */
pub const CMD_GD25_RES: u8 = 0xab; /* Release from DP, and Read Signature */

pub const FLASH_TABLE: [SpiFlashPartId; 13] = [
    /* GD25T80 */
    SpiFlashPartId::create(0x3114, 8),
    /* GD25Q80 */
    SpiFlashPartId::create_fast_read(0x4014, 8, 1, 1), /* also GD25Q80B */
    /* GD25Q16 */
    SpiFlashPartId::create_fast_read(0x4015, 9, 1, 1), /* also GD25Q16B */
    /* GD25Q32 */
    SpiFlashPartId::create_fast_read(0x4016, 10, 1, 1), /* also GD25Q32B */
    /* GD25Q64 */
    SpiFlashPartId::create_fast_read(0x4017, 11, 1, 1), /* also GD25Q64B, GD25B64C */
    /* GD25Q128 */
    SpiFlashPartId::create_fast_read(0x4018, 12, 1, 1), /* also GD25Q128B */
    /* GD25VQ80C */
    SpiFlashPartId::create_fast_read(0x4214, 8, 1, 1),
    /* GD25VQ16C */
    SpiFlashPartId::create_fast_read(0x4215, 9, 1, 1),
    /* GD25LQ80 */
    SpiFlashPartId::create_fast_read(0x6014, 8, 1, 1),
    /* GD25LQ16 */
    SpiFlashPartId::create_fast_read(0x6015, 9, 1, 1),
    /* GD25LQ32 */
    SpiFlashPartId::create_fast_read(0x6016, 10, 1, 1),
    /* GD25LQ64C */
    SpiFlashPartId::create_fast_read(0x6017, 11, 1, 1),
    /* GD25LQ128 */
    SpiFlashPartId::create_fast_read(0x6018, 12, 1, 1),
];

pub const SPI_FLASH_GIGADEVICE_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_GIGADEVICE,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_GIGADEVICE_VI];
