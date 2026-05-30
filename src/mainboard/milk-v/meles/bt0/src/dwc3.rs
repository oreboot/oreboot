use crate::util::{read32, write32};

const DWC3_DCTL: usize = 0xc704;
const DWC3_DCTL_RUN_STOP: u32 = 1 << 31;

pub fn dwc3_gadget_run(base: usize) {
    println!("USB gadget run");
    write32(base + DWC3_DCTL, DWC3_DCTL_RUN_STOP);
}
