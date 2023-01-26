use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0xD8_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_SPANSION,
};

/* S25FLxx-specific commands */
pub const CMD_S25FLXX_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_S25FLXX_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_S25FLXX_READID: u8 = 0x90; /* Read Manufacture ID and Device ID */
pub const CMD_S25FLXX_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_S25FLXX_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_S25FLXX_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_S25FLXX_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_S25FLXX_PP: u8 = 0x02; /* Page Program */
pub const CMD_S25FLXX_SE: u8 = 0xd8; /* Sector Erase */
pub const CMD_S25FLXX_BE: u8 = 0xc7; /* Bulk Erase */
pub const CMD_S25FLXX_DP: u8 = 0xb9; /* Deep Power-down */
pub const CMD_S25FLXX_RES: u8 = 0xab; /* Release from DP, and Read Signature */

pub const SPSN_ID_S25FL008A: u16 = 0x0213;
pub const SPSN_ID_S25FL016A: u16 = 0x0214;
pub const SPSN_ID_S25FL032A: u16 = 0x0215;
pub const SPSN_ID_S25FL064A: u16 = 0x0216;
pub const SPSN_ID_S25FL128S: u16 = 0x0219;
pub const SPSN_ID_S25FL128P: u16 = 0x2018;
pub const SPSN_ID_S25FL208K: u16 = 0x4014;
pub const SPSN_ID_S25FL116K: u16 = 0x4015;
pub const SPSN_ID_S25FL132K: u16 = 0x4016;
pub const SPSN_ID_S25FL164K: u16 = 0x4017;
pub const SPSN_EXT_ID_S25FL128P_256KB: u16 = 0x0300;
pub const SPSN_EXT_ID_S25FL128P_64KB: u16 = 0x0301;
pub const SPSN_EXT_ID_S25FL032P: u16 = 0x4d00;
pub const SPSN_EXT_ID_S25FLXXS_64KB: u16 = 0x4d01;

pub const FLASH_TABLE_EXT: [SpiFlashPartId; 8] = [
    /* S25FL008A */
    SpiFlashPartId::create(SPSN_ID_S25FL008A, 4),
    /* S25FL016A */
    SpiFlashPartId::create(SPSN_ID_S25FL016A, 5),
    /* S25FL032A */
    SpiFlashPartId::create(SPSN_ID_S25FL032A, 6),
    /* S25FL064A */
    SpiFlashPartId::create(SPSN_ID_S25FL064A, 7),
    /* S25FL128P_64K */
    SpiFlashPartId::create_id([SPSN_ID_S25FL128P, SPSN_EXT_ID_S25FL128P_64KB], 8),
    /* S25FL128S_256K */
    SpiFlashPartId::create_id([SPSN_ID_S25FL128S, SPSN_EXT_ID_S25FLXXS_64KB], 9),
    /* S25FL032P */
    SpiFlashPartId::create_id([SPSN_ID_S25FL032A, SPSN_EXT_ID_S25FL032P], 6),
    /* S25FS128S */
    SpiFlashPartId::create_id([SPSN_ID_S25FL128P, SPSN_EXT_ID_S25FLXXS_64KB], 8),
];

pub const FLASH_TABLE_256K_SECTOR: [SpiFlashPartId; 1] =
    [/* S25FL128P_256K */ SpiFlashPartId::create_id(
        [SPSN_ID_S25FL128P, SPSN_EXT_ID_S25FL128P_256KB],
        6,
    )];

pub const FLASH_TABLE: [SpiFlashPartId; 4] = [
    /* S25FL208K */
    SpiFlashPartId::create(SPSN_ID_S25FL208K, 4),
    /* S25FL116K_16M */
    SpiFlashPartId::create(SPSN_ID_S25FL116K, 5),
    /* S25FL132K */
    SpiFlashPartId::create(SPSN_ID_S25FL132K, 6),
    /* S25FL164K */
    SpiFlashPartId::create(SPSN_ID_S25FL164K, 7),
];

pub const SPI_FLASH_SPANSION_EXT1_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create_id(
    VENDOR_ID_SPANSION,
    8,
    6,
    [0xffff, 0xffff],
    &FLASH_TABLE_EXT,
    &SPI_FLASH_PP_0xD8_SECTOR_DESC,
);

pub const SPI_FLASH_SPANSION_EXT2_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create_id(
    VENDOR_ID_SPANSION,
    8,
    8,
    [0xffff, 0xffff],
    &FLASH_TABLE_256K_SECTOR,
    &SPI_FLASH_PP_0xD8_SECTOR_DESC,
);

pub const SPI_FLASH_SPANSION_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_SPANSION,
    8,
    6,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0xD8_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [
    SPI_FLASH_SPANSION_EXT1_VI,
    SPI_FLASH_SPANSION_EXT2_VI,
    SPI_FLASH_SPANSION_VI,
];
