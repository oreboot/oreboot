#![no_std]

//! **NOTE**: This library is no_std by default, used within oreboot platform
//! ports.
//! <https://www.reddit.com/r/rust/comments/1hs6spy/psa_for_std_feature_in_no_std_libraries/>

pub mod areas;

/// This is used in our build system.
#[cfg(feature = "std")]
pub mod layout;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
