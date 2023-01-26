use device::Device;

pub const GOOGLE_CHROMEEC_USBC_DEVICE_HID: &str = "GOOG0014";
pub const GOOGLE_CHROMEEC_USBC_DEVICE_NAME: &str = "USBC";

pub fn google_chromeec_acpi_name(_dev: &Device) -> &'static str {
    /*
     * Chrome EC device (CREC - GOOG0004) is really a child of EC device (EC - PNP0C09) in
     * ACPI tables. However, in coreboot device tree, there is no separate chip/device for
     * EC0. Thus, Chrome EC device needs to return "EC0.CREC" as the ACPI name so that the
     * callers can get the correct acpi device path/scope for this device.
     *
     * If we ever enable a separate driver for generating AML for EC0 device, then this
     * function needs to be updated to return "CREC".
     */
    "EC0.CREC"
}
