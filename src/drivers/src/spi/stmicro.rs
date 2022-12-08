use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SPI_FLASH_PP_0xD8_SECTOR_DESC, SpiFlashPartId,
        SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::{SpiSlave, VENDOR_ID_STMICRO},
    Error,
};

/// Write Enable
pub const CMD_M25PXX_WREN: u8 = 0x06;
/// Write Disable
pub const CMD_M25PXX_WRDI: u8 = 0x04;
/// Read Status Register
pub const CMD_M25PXX_RDSR: u8 = 0x05;
/// Write Status Register
pub const CMD_M25PXX_WRSR: u8 = 0x01;
/// Read Data Bytes
pub const CMD_M25PXX_READ: u8 = 0x03;
/// Read Data Bytes at Higher Speed
pub const CMD_M25PXX_FAST_READ: u8 = 0x0b;
/// Page Program
pub const CMD_M25PXX_PP: u8 = 0x02;
/// Subsector Erase
pub const CMD_M25PXX_SSE: u8 = 0x20;
/// Sector Erase
pub const CMD_M25PXX_SE: u8 = 0xd8;
/// Bulk Erase
pub const CMD_M25PXX_BE: u8 = 0xc7;
/// Deep Power-down
pub const CMD_M25PXX_DP: u8 = 0xb9;
/// Release from DP, and Read Signature
pub const CMD_M25PXX_RES: u8 = 0xab;

/*
 * Device ID = (memory_type << 8) + memory_capacity
 */
pub const STM_ID_M25P10: u16 = 0x2011;
pub const STM_ID_M25P20: u16 = 0x2012;
pub const STM_ID_M25P40: u16 = 0x2013;
pub const STM_ID_M25P80: u16 = 0x2014;
pub const STM_ID_M25P16: u16 = 0x2015;
pub const STM_ID_M25P32: u16 = 0x2016;
pub const STM_ID_M25P64: u16 = 0x2017;
pub const STM_ID_M25P128: u16 = 0x2018;
pub const STM_ID_M25PX80: u16 = 0x7114;
pub const STM_ID_M25PX16: u16 = 0x7115;
pub const STM_ID_M25PX32: u16 = 0x7116;
pub const STM_ID_M25PX64: u16 = 0x7117;
pub const STM_ID_M25PE80: u16 = 0x8014;
pub const STM_ID_M25PE16: u16 = 0x8015;
pub const STM_ID_M25PE32: u16 = 0x8016;
pub const STM_ID_M25PE64: u16 = 0x8017;
pub const STM_ID_N25Q016__3E: u16 = 0xba15;
pub const STM_ID_N25Q032__3E: u16 = 0xba16;
pub const STM_ID_N25Q064__3E: u16 = 0xba17;
pub const STM_ID_N25Q128__3E: u16 = 0xba18;
pub const STM_ID_N25Q256__3E: u16 = 0xba19;
pub const STM_ID_N25Q016__1E: u16 = 0xbb15;
pub const STM_ID_N25Q032__1E: u16 = 0xbb16;
pub const STM_ID_N25Q064__1E: u16 = 0xbb17;
pub const STM_ID_N25Q128__1E: u16 = 0xbb18;
pub const STM_ID_N25Q256__1E: u16 = 0xbb19;

/* M25P10 */
pub const FLASH_TABLE_SE32K: [SpiFlashPartId; 1] = [SpiFlashPartId::create(STM_ID_M25P10, 2)];
pub const FLASH_TABLE_SE64K: [SpiFlashPartId; 14] = [
    /* M25P16 */
    SpiFlashPartId::create(STM_ID_M25P16, 5),
    /* M25P20 */
    SpiFlashPartId::create(STM_ID_M25P20, 2),
    /* M25P32 */
    SpiFlashPartId::create(STM_ID_M25P32, 6),
    /* M25P40 */
    SpiFlashPartId::create(STM_ID_M25P40, 3),
    /* M25P64 */
    SpiFlashPartId::create(STM_ID_M25P64, 7),
    /* M25P80 */
    SpiFlashPartId::create(STM_ID_M25P80, 4),
    /* M25PX80 */
    SpiFlashPartId::create(STM_ID_M25PX80, 4),
    /* M25PX16 */
    SpiFlashPartId::create(STM_ID_M25PX16, 5),
    /* M25PX32 */
    SpiFlashPartId::create(STM_ID_M25PX32, 6),
    /* M25PX64 */
    SpiFlashPartId::create(STM_ID_M25PX64, 7),
    /* M25PE80 */
    SpiFlashPartId::create(STM_ID_M25PE80, 4),
    /* M25PE16 */
    SpiFlashPartId::create(STM_ID_M25PE16, 5),
    /* M25PE32 */
    SpiFlashPartId::create(STM_ID_M25PE32, 6),
    /* M25PE64 */
    SpiFlashPartId::create(STM_ID_M25PE64, 7),
];

pub const FLASH_TABLE_SE256K: [SpiFlashPartId; 1] = [SpiFlashPartId::create(STM_ID_M25P128, 6)];
pub const FLASH_TABLE_SSE: [SpiFlashPartId; 10] = [
    /* N25Q016..3E */
    SpiFlashPartId::create(STM_ID_N25Q016__3E, 9),
    /* N25Q032..3E */
    SpiFlashPartId::create(STM_ID_N25Q032__3E, 10),
    /* N25Q064..3E */
    SpiFlashPartId::create(STM_ID_N25Q064__3E, 11),
    /* N25Q128..3E */
    SpiFlashPartId::create(STM_ID_N25Q128__3E, 12),
    /* N25Q256..3E */
    SpiFlashPartId::create(STM_ID_N25Q256__3E, 13),
    /* N25Q016..1E */
    SpiFlashPartId::create(STM_ID_N25Q016__1E, 9),
    /* N25Q032..1E */
    SpiFlashPartId::create(STM_ID_N25Q032__1E, 10),
    /* N25Q064..1E */
    SpiFlashPartId::create(STM_ID_N25Q064__1E, 11),
    /* N25Q128..1E */
    SpiFlashPartId::create(STM_ID_N25Q128__1E, 12),
    /* N25Q256..1E */
    SpiFlashPartId::create(STM_ID_N25Q256__1E, 13),
];

impl<'a> SpiSlave<'a> {
    pub fn stmicro_release_deep_sleep_identify(&self, idcode: &mut [u8]) -> Result<(), Error> {
        self.spi_flash_cmd(CMD_M25PXX_RES, idcode)?;

        /* Assuming ST parts identify with 0x1X to release from deep
        power down and read electronic signature. */
        if idcode[3] & 0xf0 != 0x10 {
            return Err(Error::Generic);
        }

        /* Fix up the idcode to mimic rdid jedec instruction. */
        idcode[0] = 0x20;
        idcode[1] = 0x20;
        idcode[2] = idcode[3] + 1;

        Ok(())
    }
}

pub const SPI_FLASH_STMICRO1_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_STMICRO,
    8,
    5,
    0xffff,
    &FLASH_TABLE_SE32K,
    &SPI_FLASH_PP_0xD8_SECTOR_DESC,
);

pub const SPI_FLASH_STMICRO2_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_STMICRO,
    8,
    6,
    0xffff,
    &FLASH_TABLE_SE64K,
    &SPI_FLASH_PP_0xD8_SECTOR_DESC,
);

pub const SPI_FLASH_STMICRO3_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_STMICRO,
    8,
    8,
    0xffff,
    &FLASH_TABLE_SE256K,
    &SPI_FLASH_PP_0xD8_SECTOR_DESC,
);

pub const SPI_FLASH_STMICRO4_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_STMICRO,
    8,
    2,
    0xffff,
    &FLASH_TABLE_SSE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [
    SPI_FLASH_STMICRO1_VI,
    SPI_FLASH_STMICRO2_VI,
    SPI_FLASH_STMICRO3_VI,
    SPI_FLASH_STMICRO4_VI,
];
