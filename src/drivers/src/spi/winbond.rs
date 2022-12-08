use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::VENDOR_ID_WINBOND,
};
use bitfield::bitfield;

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Bp3(u8);
    pub busy, set_busy: 1, 0;
    pub wel, set_wel: 1, 1;
    pub bp, set_bp: 3, 2;
    pub tb, set_tb: 1, 5;
    pub sec, set_sec: 1, 6;
    pub srp0, set_srp0: 1, 7;
}

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Bp4(u8);
    pub busy, set_busy: 1, 0;
    pub wel, set_wel: 1, 1;
    pub bp, set_bp: 4, 2;
    pub tb, set_tb: 1, 6;
    pub srp0, set_srp0: 1, 7;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union StatusReg1 {
    u: u8,
    bp3: Bp3,
    bp4: Bp4,
}

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Reg2(u8);
    pub srp1, set_srp1: 1, 0;
    pub qe, set_qe: 1, 1;
    pub res, set_res: 1, 2;
    pub lb, set_lb: 3, 3;
    pub cmp, set_cmp: 1, 6;
    pub sus, set_sus: 1, 7;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union StatusReg2 {
    u: u8,
    reg2: Reg2,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StatusReg3 {
    reg2: StatusReg2,
    reg1: StatusReg1,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StatusReg3 {
    reg1: StatusReg1,
    reg2: StatusReg2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union StatusRegsUnion {
    reg3: StatusReg3,
    u: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StatusRegs(StatusRegsUnion);

pub const FLASH_TABLE: [SpiFlashPartId; 23] = [
    /* W25P80 */
    SpiFlashPartId::create(0x2014, 8),
    /* W25P16 */
    SpiFlashPartId::create(0x2015, 9),
    /* W25P32 */
    SpiFlashPartId::create(0x2016, 10),
    /* W25X80 */
    SpiFlashPartId::create_fast_read(0x2016, 8, 1, 0),
    /* W25X16 */
    SpiFlashPartId::create_fast_read(0x3015, 9, 1, 0),
    /* W25X32 */
    SpiFlashPartId::create_fast_read(0x3016, 10, 1, 0),
    /* W25X64 */
    SpiFlashPartId::create_fast_read(0x3017, 11, 1, 0),
    /* W25Q80_V */
    SpiFlashPartId::create_fast_read(0x3017, 11, 1, 1),
    /* W25Q16_V */
    SpiFlashPartId::create_full([0x4015, 0], 9, 1, 1, 16, 3),
    /* W25Q16DW */
    SpiFlashPartId::create_full([0x6015, 0], 9, 1, 1, 16, 3),
    /* W25Q32_V */
    SpiFlashPartId::create_full([0x4016, 0], 10, 1, 1, 16, 3),
    /* W25Q32DW */
    SpiFlashPartId::create_full([0x6016, 0], 10, 1, 1, 16, 3),
    /* W25Q64_V */
    SpiFlashPartId::create_full([0x4017, 0], 11, 1, 1, 17, 3),
    /* W25Q64DW */
    SpiFlashPartId::create_full([0x6017, 0], 11, 1, 1, 17, 3),
    /* W25Q64JW */
    SpiFlashPartId::create_full([0x8017, 0], 11, 1, 1, 17, 3),
    /* W25Q128_V */
    SpiFlashPartId::create_full([0x4018, 0], 12, 1, 1, 18, 3),
    /* W25Q128FW */
    SpiFlashPartId::create_full([0x8018, 0], 12, 1, 1, 18, 3),
    /* W25Q128J */
    SpiFlashPartId::create_full([0x7018, 0], 12, 1, 1, 18, 3),
    /* W25Q128JW */
    SpiFlashPartId::create_full([0x8018, 0], 12, 1, 1, 18, 3),
    /* W25Q512NW-IM */
    SpiFlashPartId::create_full([0x8020, 0], 14, 1, 1, 16, 4),
    /* W25Q256_V */
    SpiFlashPartId::create_full([0x4019, 0], 13, 1, 1, 16, 4),
    /* W25Q256J */
    SpiFlashPartId::create_full([0x7019, 0], 13, 1, 1, 16, 4),
    /* W25Q256JW */
    SpiFlashPartId::create_full([0x6019, 0], 13, 1, 1, 16, 4),
];

// FIXME: implement get_write_protection and set_write_protection
// currently, in coreboot, requires unsafe access to union fields
// basically transmutes union fields between representations
// need a safer design
//
// after implementing, do:
//
// impl SpiFlashProtectionOps for SpiFlashVendorInfo {
//      fn get_write(...) {
//          get_write_protection(...)
//      }
//
//      fn set_write(...) {
//          set_write_protection(...)
//      }
// }

pub const SPI_FLASH_WINBOND_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_WINBOND,
    8,
    2,
    0xffff,
    &FLASH_TABLE,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] = [SPI_FLASH_WINBOND_VI];
