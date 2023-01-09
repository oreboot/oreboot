use crate::{baseboard::variant::power_off_lte_module, bobba::Sku};
use acpi::AcpiSn;
use ec::google::chromeec::ec_skuid::get_board_sku;

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
