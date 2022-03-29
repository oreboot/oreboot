//! AMD SoC support

// code common for all AMD SoCs
pub mod common;

#[cfg(feature = "amd_picasso")]
pub mod picasso;

#[cfg(feature = "amd_rome")]
pub mod rome;
