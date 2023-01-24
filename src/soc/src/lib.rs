#![no_std]

#[cfg(feature = "amd")]
pub mod amd;

#[cfg(feature = "aspeed")]
pub mod aspeed;

#[cfg(feature = "opentitan")]
pub mod opentitan;

#[cfg(feature = "sifive")]
pub mod sifive;

#[cfg(feature = "sunxi")]
pub mod sunxi;
