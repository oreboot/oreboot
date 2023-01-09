pub mod gpio;
pub mod variant;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Sku {
    /// LTE
    Droid37 = 37,
    /// LTE + Touch
    Droid38 = 38,
    /// LTE + KB backlight
    Droid39 = 39,
    /// LTE + Touch + KB backlight
    Droid40 = 40,
    Invalid,
}

impl From<u32> for Sku {
    fn from(u: u32) -> Self {
        match u {
            37 => Self::Droid37,
            38 => Self::Droid38,
            39 => Self::Droid39,
            40 => Self::Droid40,
            _ => Self::Invalid,
        }
    }
}
