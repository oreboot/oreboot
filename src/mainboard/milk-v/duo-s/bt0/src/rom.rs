// 64k mask ROM
const MASK_ROM_BASE: usize = 0x0440_0000;
// The mask ROM provides us with helper functions.
// plat/cv180x/include/riscv/rom_api_refer.h
// plat/cv181x/include/riscv/rom_api_refer.h
#[cfg(soc = "CV1800B")]
const MASK_ROM_FN_BASE: usize = MASK_ROM_BASE;
// On later SoCs, the mask ROM functions are off
#[cfg(not(soc = "CV1800B"))]
const MASK_ROM_FN_BASE: usize = MASK_ROM_BASE + 0x0001_8000;

const ROM_GET_BOOT_SRC: usize = MASK_ROM_FN_BASE + 0x0020;
const ROM_SET_BOOT_SRC: usize = MASK_ROM_FN_BASE + 0x0040;
const ROM_LOAD_IMAGE: usize = MASK_ROM_FN_BASE + 0x0060;
const ROM_FLASH_INIT: usize = MASK_ROM_FN_BASE + 0x0080;
const ROM_IMAGE_CRC: usize = MASK_ROM_FN_BASE + 0x00A0;
const ROM_GET_RETRY_COUNT: usize = MASK_ROM_FN_BASE + 0x00C0;
const ROM_VERIFY_RSA: usize = MASK_ROM_FN_BASE + 0x00E0;
const ROM_CRYPTODMA_AES_DECRYPT: usize = MASK_ROM_FN_BASE + 0x0100;

const BOOT_SRC_TAG: u32 = 0xCE00;

#[derive(Debug)]
#[repr(u32)]
pub enum BootSrc {
    SpiNand = BOOT_SRC_TAG | 0x00,
    SpiNor = BOOT_SRC_TAG | 0x02,
    Emmc = BOOT_SRC_TAG | 0x03,
    Sd = BOOT_SRC_TAG | 0xa0,
    Usb = BOOT_SRC_TAG | 0xa3,
    Uart = BOOT_SRC_TAG | 0xa5,
}

impl core::fmt::Display for BootSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            BootSrc::SpiNand => write!(f, "SPI NAND"),
            BootSrc::SpiNor => write!(f, "SPI NOR"),
            BootSrc::Emmc => write!(f, "EMMC"),
            BootSrc::Sd => write!(f, "SD card"),
            BootSrc::Usb => write!(f, "USB"),
            BootSrc::Uart => write!(f, "UART"),
        }
    }
}

type GetBootSrc = unsafe extern "C" fn() -> BootSrc;
type SetBootSrc = unsafe extern "C" fn(src: BootSrc) -> usize;
type LoadImage = unsafe extern "C" fn(addr: usize, offset: u32, size: usize, retry: u32) -> usize;
type GetRetryCount = unsafe extern "C" fn() -> usize;

use core::{fmt::Pointer, mem::transmute};

pub fn get_boot_src() -> BootSrc {
    unsafe {
        let f: GetBootSrc = transmute(ROM_GET_BOOT_SRC);
        f()
    }
}

pub fn get_retry_count() -> usize {
    unsafe {
        let f: GetRetryCount = transmute(ROM_GET_RETRY_COUNT);
        f()
    }
}

pub fn set_boot_src(src: BootSrc) {
    unsafe {
        let f: SetBootSrc = transmute(ROM_GET_BOOT_SRC);
        let r = f(src);
        println!("set boot src: {r:08x}");
    }
}

pub fn load_image(addr: usize, offset: u32, size: usize, retry: u32) {
    unsafe {
        let f: LoadImage = transmute(ROM_LOAD_IMAGE);
        let r = f(addr, offset, size, retry);
        println!("load image: {r}");
    }
}
