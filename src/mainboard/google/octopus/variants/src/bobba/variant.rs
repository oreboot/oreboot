use crate::baseboard::variant::power_off_lte_module;
use acpi::AcpiSn;
use ec::google::chromeec::ec_skuid::get_board_sku;

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

pub fn smi_sleep(slp_typ: u8) {
    // Currently use cases here all target to S5 therefore we do early return
    // here for saving one transaction to the EC for getting SKU ID.
    if AcpiSn::from(slp_typ) != AcpiSn::S5 {
        return;
    }

    match Sku::from(get_board_sku()) {
        Sku::Droid37 | Sku::Droid38 | Sku::Droid39 | Sku::Droid40 => {
            power_off_lte_module();
            return;
        }
        _ => return,
    }
}
