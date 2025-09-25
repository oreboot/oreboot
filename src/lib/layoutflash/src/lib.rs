#![no_std]
pub mod areas;

#[cfg(feature = "std")]
pub mod layout;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
