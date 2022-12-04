/// struct fw_config - Firmware configuration field and option.
/// @field_name: Name of the field that this option belongs to.
/// @option_name: Name of the option within this field.
/// @mask: Bitmask of the field.
/// @value: Value of the option within the mask.
pub struct FwConfig {
    pub field_name: &'static str,
    pub option_name: &'static str,
    pub mask: u64,
    pub value: u64,
}
