use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_MACRONIX,
};

/* MX25xx-specific commands */
pub const CMD_MX25XX_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_MX25XX_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_MX25XX_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_MX25XX_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_MX25XX_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_MX25XX_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_MX25XX_PP: u8 = 0x02; /* Page Program */
pub const CMD_MX25XX_SE: u8 = 0x20; /* Sector Erase */
pub const CMD_MX25XX_BE: u8 = 0xD8; /* Block Erase */
pub const CMD_MX25XX_CE: u8 = 0xc7; /* Chip Erase */
pub const CMD_MX25XX_DP: u8 = 0xb9; /* Deep Power-down */
pub const CMD_MX25XX_RES: u8 = 0xab; /* Release from DP, and Read Signature */

pub const MACRONIX_SR_WIP: u8 = 1 << 0; /* Write-in-Progress */

pub const FLASH_TABLE: [SpiFlashPartId; 20] = [
    /* MX25L8005 */
    SpiFlashPartId::create(0x2014, 8),
    /* MX25L1605D */
    SpiFlashPartId::create(0x2015, 9),
    /* MX25L3205D */
    SpiFlashPartId::create(0x2016, 10),
    /* MX25L6405D */
    SpiFlashPartId::create(0x2017, 11),
    /* MX25L12805D */
    SpiFlashPartId::create(0x2018, 12),
    /* MX25L25635F */
    SpiFlashPartId::create(0x2019, 13),
    /* MX25L51235F */
    SpiFlashPartId::create(0x201a, 14),
    /* MX25L1635D */
    SpiFlashPartId::create(0x2415, 9),
    /*
     * NOTE: C225xx JEDEC IDs are basically useless because Macronix keeps
     * reusing the same IDs for vastly different chips. 35E versions always
     * seem to support Dual I/O but not Dual Output, while 35F versions seem
     * to support both, so we only set Dual I/O here to improve our chances
     * of compatibility. Since Macronix makes it impossible to search all
     * different parts that it recklessly assigned the same IDs to, it's
     * hard to know if there may be parts that don't even support Dual I/O
     * with these IDs, though (or what we should do if there are).
     */
    /* MX25L1635E */
    SpiFlashPartId::create_fast_read(0x2515, 9, 0, 1),
    /* MX25U8032E */
    SpiFlashPartId::create_fast_read(0x2534, 8, 0, 1),
    /* MX25U1635E/MX25U1635F */
    SpiFlashPartId::create_fast_read(0x2535, 9, 0, 1),
    /* MX25U3235E/MX25U3235F */
    SpiFlashPartId::create_fast_read(0x2536, 10, 0, 1),
    /* MX25U6435E/MX25U6435F */
    SpiFlashPartId::create_fast_read(0x2537, 11, 0, 1),
    /* MX25U12835F */
    SpiFlashPartId::create_fast_read(0x2538, 12, 0, 1),
    /* MX25U25635F */
    SpiFlashPartId::create_fast_read(0x2539, 13, 0, 1),
    /* MX25U51235F */
    SpiFlashPartId::create_fast_read(0x253a, 14, 0, 1),
    /* MX25L12855E */
    SpiFlashPartId::create_fast_read(0x2618, 12, 0, 1),
    /* MX25L3235D/MX25L3225D/MX25L3236D/MX25L3237D */
    SpiFlashPartId::create_fast_read(0x5e16, 10, 0, 1),
    /* MX25L6495F */
    SpiFlashPartId::create(0x9517, 11),
    /* MX77U25650F */
    SpiFlashPartId::create(0x7539, 13),
];

pub const SPI_FLASH_MACRONIX_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_MACRONIX,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_MACRONIX_VI];
