pub mod spi_flash;
pub mod spi_generic;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Error {
    Generic = -1,
    MissingSpiBus,
    MissingSpiCtrlr,
    Unimplemented,
}
