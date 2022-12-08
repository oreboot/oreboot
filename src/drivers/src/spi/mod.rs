#[cfg(feature = "adesto")]
pub mod adesto;
#[cfg(feature = "amic")]
pub mod amic;
#[cfg(feature = "atmel")]
pub mod atmel;
pub mod cbfs_spi;
#[cfg(feature = "eon")]
pub mod eon;
#[cfg(feature = "gigadevice")]
pub mod gigadevice;
#[cfg(feature = "macronix")]
pub mod macronix;
#[cfg(feature = "spansion")]
pub mod spansion;
pub mod spi_flash;
pub mod spi_generic;
#[cfg(feature = "sst")]
pub mod sst;
#[cfg(feature = "stmicro")]
pub mod stmicro;
#[cfg(feature = "winbond")]
pub mod winbond;

/// Keep at 0 because lots of boards assume this default
/// Use features to define board-specific values
pub const BOOT_DEVICE_SPI_FLASH_BUS: u8 = 0;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Error {
    Generic = -1,
    MissingSpiBus,
    MissingSpiCtrlr,
    Unimplemented,
}
