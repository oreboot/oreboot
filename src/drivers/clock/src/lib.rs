#![no_std]
/// Trait to set the input clock rate on a driver.

pub trait ClockNode {
    fn set_clock_rate(&mut self, rate: u32);
}
