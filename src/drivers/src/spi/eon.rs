use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_EON,
};

/* EN25*-specific commands */
pub const CMD_EN25_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_EN25_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_EN25_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_EN25_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_EN25_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_EN25_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_EN25_PP: u8 = 0x02; /* Page Program */
pub const CMD_EN25_SE: u8 = 0x20; /* Sector Erase */
pub const CMD_EN25_BE: u8 = 0xd8; /* Block Erase */
pub const CMD_EN25_DP: u8 = 0xb9; /* Deep Power-down */
pub const CMD_EN25_RES: u8 = 0xab; /* Release from DP, and Read Signature */

pub const EON_ID_EN25B80: u16 = 0x2014;
pub const EON_ID_EN25B16: u16 = 0x2015;
pub const EON_ID_EN25B32: u16 = 0x2016;
pub const EON_ID_EN25B64: u16 = 0x2017;
pub const EON_ID_EN25F80: u16 = 0x3114;
pub const EON_ID_EN25F16: u16 = 0x3115;
pub const EON_ID_EN25F32: u16 = 0x3116;
pub const EON_ID_EN25F64: u16 = 0x3117;
pub const EON_ID_EN25Q80: u16 = 0x3014;
pub const EON_ID_EN25Q16: u16 = 0x3015; /* Same as EN25D16 */
pub const EON_ID_EN25Q32: u16 = 0x3016; /* Same as EN25Q32A and EN25Q32B */
pub const EON_ID_EN25Q64: u16 = 0x3017;
pub const EON_ID_EN25Q128: u16 = 0x3018;
pub const EON_ID_EN25QH16: u16 = 0x7015;
pub const EON_ID_EN25QH32: u16 = 0x7016;
pub const EON_ID_EN25QH64: u16 = 0x7017;
pub const EON_ID_EN25QH128: u16 = 0x7018;
pub const EON_ID_EN25S80: u16 = 0x3814;
pub const EON_ID_EN25S16: u16 = 0x3815;
pub const EON_ID_EN25S32: u16 = 0x3816;
pub const EON_ID_EN25S64: u16 = 0x3817;

pub const FLASH_TABLE: [SpiFlashPartId; 21] = [
    /* EN25B80 */
    SpiFlashPartId::create(EON_ID_EN25B80, 8),
    /* EN25B16 */
    SpiFlashPartId::create(EON_ID_EN25B16, 9),
    /* EN25B32 */
    SpiFlashPartId::create(EON_ID_EN25B32, 10),
    /* EN25B64 */
    SpiFlashPartId::create(EON_ID_EN25B64, 11),
    /* EN25F80 */
    SpiFlashPartId::create(EON_ID_EN25F80, 8),
    /* EN25F16 */
    SpiFlashPartId::create(EON_ID_EN25F16, 9),
    /* EN25F32 */
    SpiFlashPartId::create(EON_ID_EN25F32, 10),
    /* EN25F64 */
    SpiFlashPartId::create(EON_ID_EN25F32, 11),
    /* EN25Q80(A) */
    SpiFlashPartId::create(EON_ID_EN25Q80, 8),
    /* EN25Q16(D16) */
    SpiFlashPartId::create(EON_ID_EN25Q16, 9),
    /* EN25Q32(A/B) */
    SpiFlashPartId::create(EON_ID_EN25Q32, 10),
    /* EN25Q64 */
    SpiFlashPartId::create(EON_ID_EN25Q64, 11),
    /* EN25Q128 */
    SpiFlashPartId::create(EON_ID_EN25Q128, 12),
    /* EN25QH16 */
    SpiFlashPartId::create(EON_ID_EN25QH16, 9),
    /* EN25QH32 */
    SpiFlashPartId::create(EON_ID_EN25QH32, 10),
    /* EN25QH64 */
    SpiFlashPartId::create(EON_ID_EN25QH64, 11),
    /* EN25QH128 */
    SpiFlashPartId::create(EON_ID_EN25QH128, 12),
    /* EN25S80 */
    SpiFlashPartId::create(EON_ID_EN25S80, 8),
    /* EN25S16 */
    SpiFlashPartId::create(EON_ID_EN25S16, 9),
    /* EN25S32 */
    SpiFlashPartId::create(EON_ID_EN25S32, 10),
    /* EN25S64 */
    SpiFlashPartId::create(EON_ID_EN25S64, 11),
];

pub const SPI_FLASH_EON_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_EON,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub const SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_EON_VI];
