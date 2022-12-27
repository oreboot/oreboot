use acpi::acpi_is_wakeup_s3;
use log::info;
use soc::intel::common::block::lpc::{lpc_enable_fixed_io_ranges, lpc_open_pmio_window};
use variants::baseboard::ec::{MAINBOARD_EC_LOG_EVENTS, MAINBOARD_EC_SCI_EVENTS,  MAINBOARD_EC_SMI_EVENTS, MAINBOARD_EC_S0IX_WAKE_EVENTS, MAINBOARD_EC_S3_WAKE_EVENTS, MAINBOARD_EC_S5_WAKE_EVENTS};

pub fn ramstage_ec_init() -> Result<(), Error> {
    let info = EventInfo {
        log_events: MAINBOARD_EC_LOG_EVENTS,
        sci_events: MAINBOARD_EC_SCI_EVENTS,
        smi_events: MAINBOARD_EC_SMI_EVENTS,
        s3_wake_events: MAINBOARD_EC_S3_WAKE_EVENTS,
        s3_device_events: 0,
        s5_wake_events: MAINBOARD_EC_S5_WAKE_EVENTS,
        s0ix_wake_events: MAINBOARD_EC_S0IX_WAKE_EVENTS,
    };

    info!("mainboard: EC init");

    info.init(acpi_is_wakeup_s3())
}

pub fn bootblock_ec_init() {
    let mut ec_ioport_base = [0u16];
    let mut ec_ioport_size = [0usize];
	// Set up LPC decoding for the ChromeEC I/O port ranges:
	// - Ports 62/66, 60/64, and 200->208
	// - ChromeEC specific communication I/O ports.
    let _ = lpc_enable_fixed_io_ranges(LPC_IOE_EC_62_66 | LPC_IOE_KBC_60_64 | LPC_IOE_LGE_200);
    ioport_range(ec_ioport_base.as_mut(), ec_ioport_size.as_mut());
    lpc_open_pmio_window(ec_ioport_base[0], ec_ioport_size[0]);
}

pub fn mainboard_ec_init() {
    if ENV_RAMSTAGE != 0 {
        ramstage_ec_init();
    } else if ENV_BOOTBLOCK != 0 {
        bootblock_ec_init();
    } else {
        ()
    }
}
