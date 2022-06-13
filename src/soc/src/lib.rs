#![no_std]
#![feature(int_abs_diff)]

#[cfg(feature = "amd")]
pub mod amd;

#[cfg(feature = "aspeed")]
pub mod aspeed;

#[cfg(feature = "opentitan")]
pub mod opentitan;

#[cfg(feature = "sifive")]
pub mod sifive;

#[cfg(feature = "starfive")]
pub mod starfive;

#[cfg(feature = "sunxi")]
pub mod sunxi;
