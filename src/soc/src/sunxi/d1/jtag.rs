//! JTAG sink interface

use super::gpio::{
    portf::{PF0, PF1, PF3, PF5},
    Function,
};

/// Reserved ownership for JTAG interface
///
/// This structure does not provide any working operation of jtag pins,
/// it only represents ownership that these pins are dedicated for jtag use.
pub struct Jtag<PINS> {
    pins: PINS,
}

impl<PINS: Pins> Jtag<PINS> {
    /// Create JTAG instance
    #[inline]
    pub fn new(pins: PINS) -> Self {
        Self { pins }
    }
    /// Release pins from JTAG instance
    #[allow(unused)]
    #[inline]
    pub fn free(self) -> PINS {
        self.pins
    }
}

pub trait Pins {}

// parameter order: tms, tck, tdi, tdo
impl Pins
    for (
        PF0<Function<4>>,
        PF5<Function<4>>,
        PF1<Function<4>>,
        PF3<Function<4>>,
    )
{
}
