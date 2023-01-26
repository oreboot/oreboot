use crate::spi::{spi_flash::SpiFlash, BOOT_DEVICE_SPI_FLASH_BUS};
use spin::rwlock::RwLock;

pub static SPI_FLASH_INFO: RwLock<SpiFlash> = RwLock::new(SpiFlash::new());
pub static SPI_FLASH_INIT_DONE: RwLock<bool> = RwLock::new(false);

pub fn boot_device_init() {
    let bus = BOOT_DEVICE_SPI_FLASH_BUS;
    let cs = 0;

    if *SPI_FLASH_INIT_DONE.read() {
        return;
    }

    if (*SPI_FLASH_INFO.write()).probe(bus as u32, cs).is_err() {
        return;
    }

    *SPI_FLASH_INIT_DONE.write() = true;
}

pub fn boot_device_spi_flash() -> Option<&'static RwLock<SpiFlash>> {
    boot_device_init();

    if *SPI_FLASH_INIT_DONE.read() != true {
        return None;
    }

    Some(&SPI_FLASH_INFO)
}
