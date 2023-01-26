#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Error {
    Unimplemented,
}

pub const ELOG_TYPE_EC_EVENT: u8 = 0x91;

pub fn add_event(_event_type: u8) -> Result<(), Error> {
    Err(Error::Unimplemented)
}

pub fn add_event_byte(_event_type: u8, _data: u8) -> Result<(), Error> {
    Err(Error::Unimplemented)
}

pub fn gsmi_add_event_byte(_event_type: u8, _data: u8) -> Result<(), Error> {
    Ok(())
}
