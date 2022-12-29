use crate::google::chromeec::{
    ec::{get_mkbp_event, is_uhepi_supported, set_sci_mask, set_smi_mask, set_wake_mask, Error},
    ec_commands::HostEventCode,
    ec_lpc::get_event,
};
use acpi::AcpiSn;
use cpu::x86::smm::{APM_CNT_ACPI_DISABLE, APM_CNT_ACPI_ENABLE};
use drivers::elog;
use log::debug;

#[cfg(feature = "amd")]
use soc::amd::common::block::acpi::poweroff;
#[cfg(feature = "intel")]
use soc::intel::common::block::pmc::pmclib::poweroff;
#[cfg(feature = "mediatek")]
use soc::mediatek::common::poweroff;

#[cfg(not(any(feature = "intel", feature = "amd", feature = "mediatek")))]
pub fn poweroff() {}

pub fn clear_pending_events() {
    while get_event() != HostEventCode::None {}

    debug!("Clearing pending EC events. Error code EC_RES_UNAVAILABLE(9) is expected.\r\n");

    while get_mkbp_event().is_ok() {}
}

pub fn process_one_event() -> HostEventCode {
    let event = get_event();

    if event != HostEventCode::None {
        let _ = elog::gsmi_add_event_byte(elog::ELOG_TYPE_EC_EVENT, event as u8);
    }

    match event {
        HostEventCode::LidClosed => {
            debug!("LID CLOSED, SHUTDOWN\r\n");
            poweroff();
        }
        _ => (),
    }

    event
}

pub fn smi_process_events() -> u64 {
    let mut num_events = 0;
    // Process all pending events
    while process_one_event() != HostEventCode::None {
        // count events just so the loop doesn't get optimized out
        num_events += 1;
    }
    num_events
}

pub fn smi_sleep(slp_typ: i32, s3_mask: u64, s5_mask: u64) -> Result<(), Error> {
    if !is_uhepi_supported()? {
        match AcpiSn::from(slp_typ as u8) {
            AcpiSn::S3 => set_wake_mask(s3_mask)?,
            AcpiSn::S5 => set_wake_mask(s5_mask)?,
            _ => return Err(Error::Generic),
        }
    }

    // Disable SCI and SMI events
    set_smi_mask(0)?;
    set_sci_mask(0)?;

    // Clear pending events that may trigger immediate wak
    clear_pending_events();

    Ok(())
}

pub fn smi_apmc(apmc: u8, sci_mask: u64, smi_mask: u64) -> Result<(), Error> {
    match apmc {
        APM_CNT_ACPI_ENABLE => {
            set_smi_mask(0)?;
            clear_pending_events();
            set_sci_mask(sci_mask)?;
        }
        APM_CNT_ACPI_DISABLE => {
            set_sci_mask(0)?;
            clear_pending_events();
            set_smi_mask(smi_mask)?;
        }
        _ => return Err(Error::Generic),
    }

    Ok(())
}
