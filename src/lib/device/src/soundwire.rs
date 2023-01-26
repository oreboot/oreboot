/// enum soundwire_version - Versions of SoundWire Specification supported by a device.
/// @SOUNDWIRE_VERSION_1_0: SoundWire Specification Version 1.0 released January 2015.
/// @SOUNDWIRE_VERSION_1_1: SoundWire Specification Version 1.1 released June 2016.
/// @SOUNDWIRE_VERSION_1_2: SoundWire Specification Version 1.2 released April 2019.
#[repr(C)]
pub enum SoundwireVersion {
    Version1_0 = 1,
    Version1_1,
    Version1_2,
}

/// enum mipi_class - MIPI class encoding.
/// @MIPI_CLASS_NONE: No further class decoding.
/// @MIPI_CLASS_SDCA: Device implements SoundWire Device Class for Audio (SDCA).
///
/// 0x02-0x7F: Reserved
/// 0x80-0xFF: MIPI Alliance extended device class
#[repr(C)]
pub enum MipiClass {
    ClassNone,
    Sdca,
}

/// struct soundwire_address - SoundWire Device Address Encoding.
/// @version: SoundWire specification version from &enum soundwire_version.
/// @link_id: Zero-based SoundWire master link id.
/// @unique_id: Unique ID for multiple slave devices on the same bus.
/// @manufacturer_id: Manufacturer ID from include/mipi/ids.h.
/// @part_id: Vendor defined part ID.
/// @class: MIPI class encoding in &enum mipi_class.
#[repr(C)]
pub struct SoundwireAddress {
    pub version: SoundwireVersion,
    pub link_id: u8,
    pub unique_id: u8,
    pub manufacturer_id: u16,
    pub part_id: u16,
    pub class: MipiClass,
}
