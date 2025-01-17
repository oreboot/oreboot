use crate::util::{clear_bit, read32, set_bit, sleep, udelay, write32};

const P_CLOCK_FREQUENCY: u32 = 1400;
const PAGE_SIZE: u32 = 11;

const PSRAM_CONTROLLER: usize = 0x3000_F000;
const TIMING_CTRL: usize = PSRAM_CONTROLLER + 0x0030;

pub fn analog_init() {
    //
}

pub fn init() {
    //
    // TIMING_CTRL_TRFC_CYCLE
    // TIMING_CTRL_TCPHW_CYCLE
    // TIMING_CTRL_TCPHR_CYCLE
    // TIMING_CTRL_TRC_CYCLE
    let timing = (18 << 24) | (2 << 16) | 11;
    write32(TIMING_CTRL, timing);

    println!("PSRAM init done :)");
}
