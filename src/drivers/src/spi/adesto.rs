use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_ADESTO,
};

pub const FLASH_TABLE: [SpiFlashPartId; 12] = [
    /* AT25SL128A */
    SpiFlashPartId::create(0x4218, 12),
    /* AT25DF081A Yes, 81A id < 81 */
    SpiFlashPartId::create(0x4501, 8),
    /* AT25DF081 */
    SpiFlashPartId::create(0x4502, 8),
    /* AT25DF161 */
    SpiFlashPartId::create(0x4602, 9),
    /* AT25DL161 */
    SpiFlashPartId::create(0x4603, 9),
    /* AT25DF321 */
    SpiFlashPartId::create(0x4700, 10),
    /* AT25DF321A */
    SpiFlashPartId::create(0x4701, 10),
    /* AT25DF641 */
    SpiFlashPartId::create(0x4800, 11),
    /* AT25SF081 */
    SpiFlashPartId::create(0x8501, 8),
    /* AT25DQ161 */
    SpiFlashPartId::create(0x8600, 9),
    /* AT25SF161 */
    SpiFlashPartId::create(0x8601, 9),
    /* AT25DQ321 */
    SpiFlashPartId::create(0x8700, 10),
];

pub const SPI_FLASH_ADESTO_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_ADESTO,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub const SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_ADESTO_VI];
