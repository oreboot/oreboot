use crate::intel::{
    apollolake::{
        itss::{IRQS_PER_IPC, ITSS_MAX_IRQ},
        pcr_ids::PID_ITSS,
    },
    common::block::pcr::pcr_rmw32,
};
use core::mem::size_of;

pub const PCR_ITSS_IPC0_CONF: usize = 0x3200;

pub fn itss_set_irq_polarity(irq: i32, active_low: i32) {
    let port = PID_ITSS;

    if irq < 0 || irq > ITSS_MAX_IRQ as i32 {
        return;
    }

    let reg = PCR_ITSS_IPC0_CONF + size_of::<u32>() * (irq as usize / IRQS_PER_IPC);
    let mask = 1 << (irq as usize % IRQS_PER_IPC);

    pcr_rmw32(
        port as u8,
        reg as u16,
        !mask,
        if active_low != 0 { mask } else { 0 },
    );
}
