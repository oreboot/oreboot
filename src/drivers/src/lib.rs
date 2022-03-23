//! Oreboot drivers
#![no_std]

mod clock;
pub use clock::ClockNode;

mod model;
pub use model::{Driver, Result, EOF, NOT_IMPLEMENTED};

pub mod spi;
pub mod timer;
pub mod uart;
pub mod wrappers;
