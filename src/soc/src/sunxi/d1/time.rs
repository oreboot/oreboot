/// Bits per second
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Hz(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;
    /// Wrap in `Hz`
    fn hz(self) -> Hz;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }
    fn hz(self) -> Hz {
        Hz(self)
    }
}
